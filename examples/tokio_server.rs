use doip_rw::message::{
    DiagnosticMessage, DiagnosticMessageNegativeAck, DiagnosticMessageNegativeAckCode,
    RoutingActivationRequest,
    RoutingActivationResponse, RoutingActivationResponseCode, DOIP_HEADER_LENGTH,
};
use doip_rw::LogicalAddress;
use doip_rw::{DoIpError, Payload, PayloadType};

const SOURCE_LOGICAL_ADDRESS: LogicalAddress = 0x0077;

use std::io::{self, Cursor};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

enum CnxState {
    NoRouting,
    Idle,
}

async fn doip_rw_tcp_send(tcp: &mut TcpStream, buf: &[u8]) -> io::Result<()> {
    tcp.try_write(buf)?;
    Ok(())
}

async fn doip_rw_tcp_receive_exact(tcp: &mut TcpStream, buf: &mut [u8]) -> io::Result<usize> {
    let idx = 0;
    loop {
        tcp.readable().await?;
        match tcp.try_read(&mut buf[idx..]) {
            Ok(0) => return Err(io::Error::from(io::ErrorKind::UnexpectedEof)),
            Ok(nb_bytes) => {
                let idx = idx + nb_bytes;
                if idx >= buf.len() {
                    break;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(buf.len())
}

fn msg_to_vec<P: Payload>(payload: &P) -> io::Result<Vec<u8>> {
    let mut buf = Vec::<u8>::new();
    doip_rw::write_message(payload, &mut buf)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "DoIP message incorrectly formed"))?;

    Ok(buf)
}

async fn doip_rw_msg_send<P: Payload>(tcp: &mut TcpStream, payload: &P) -> io::Result<()> {
    let buf = msg_to_vec(payload)?;
    doip_rw_tcp_send(tcp, &buf[0..buf.len()]).await?;
    Ok(())
}

async fn handle_routing(
    tcp: &mut TcpStream,
    state: CnxState,
    rareq: RoutingActivationRequest,
) -> Result<CnxState, DoIpError> {
    match state {
        CnxState::Idle => {
            let rsp = RoutingActivationResponse {
		logical_address_tester: rareq.source_address,
		logical_address_of_doip_entity: SOURCE_LOGICAL_ADDRESS,
		routing_activation_response_code: RoutingActivationResponseCode::RoutingActivationDeniedSourceAddressAlreadyRegistred,
		reserved_oem: [0u8; 4],
		oem_specific: None,
	    };
            doip_rw_msg_send(tcp, &rsp).await?;
            Ok(CnxState::Idle)
        }
        CnxState::NoRouting => {
            let rsp = RoutingActivationResponse {
                logical_address_tester: rareq.source_address,
                logical_address_of_doip_entity: SOURCE_LOGICAL_ADDRESS,
                routing_activation_response_code:
                    RoutingActivationResponseCode::RoutingSuccessfullyActivated,
                reserved_oem: [0u8; 4],
                oem_specific: None,
            };
            doip_rw_msg_send(tcp, &rsp).await?;
            Ok(CnxState::Idle)
        }
    }
}

async fn handle_uds_msg<'a>(
    _tcp: &mut TcpStream,
    _state: &CnxState,
    _dreq: DiagnosticMessage<'a>,
) -> Result<CnxState, DoIpError> {
    todo!();
}

async fn handle_uds<'a>(
    tcp: &mut TcpStream,
    state: CnxState,
    dreq: DiagnosticMessage<'a>,
) -> Result<CnxState, DoIpError> {
    match state {
        CnxState::Idle => {
            handle_uds_msg(tcp, &state, dreq).await?;
        }
        _ => {
            let nack = DiagnosticMessageNegativeAck {
                source_address: dreq.target_address,
                target_address: dreq.source_address,
                ack_code: DiagnosticMessageNegativeAckCode::InvalidSourceAddress,
                previous_diagnostic_message_data: dreq.user_data,
            };
            doip_rw_msg_send(tcp, &nack).await?;
        }
    };
    Ok(state)
}

async fn handle_cnx(mut tcp: TcpStream) -> Result<(), DoIpError> {
    let mut buf = [0u8; 1024];
    let mut state = CnxState::NoRouting;

    loop {
        let _ = doip_rw_tcp_receive_exact(&mut tcp, &mut buf[0..DOIP_HEADER_LENGTH]).await?;
        let hdr = doip_rw::read_header(&mut Cursor::new(&buf[0..])).unwrap();
        match hdr.payload_type {
            PayloadType::RoutingActivationRequest => {
                let plen = hdr.payload_length as usize;
                let nb = doip_rw_tcp_receive_exact(
                    &mut tcp,
                    &mut buf[DOIP_HEADER_LENGTH..DOIP_HEADER_LENGTH + plen],
                )
                .await?;
                let rareq: RoutingActivationRequest =
                    doip_rw::read_payload(&mut Cursor::new(&buf[..nb]), nb)?;
                state = handle_routing(&mut tcp, state, rareq).await?;
            }
            PayloadType::DiagnosticMessage => {
                let plen = hdr.payload_length as usize;
                let nb = doip_rw_tcp_receive_exact(
                    &mut tcp,
                    &mut buf[DOIP_HEADER_LENGTH..DOIP_HEADER_LENGTH + plen],
                )
                .await?;
                let dreq: DiagnosticMessage = doip_rw::read_payload(&mut Cursor::new(&buf[..nb]), nb)?;
                state = handle_uds(&mut tcp, state, dreq).await?;
            }
            _ => break,
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:13400").await?;
    loop {
        let (client, _) = listener.accept().await?;
        task::spawn_local(async move { handle_cnx(client) });
    }
}
