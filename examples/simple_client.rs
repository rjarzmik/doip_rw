use doip_rw::message::{
    ActivationType, RoutingActivationRequest, RoutingActivationResponse,
    RoutingActivationResponseCode,
};
use doip_rw::message::{
    DiagnosticMessage, DiagnosticMessagePositiveAck, DiagnosticMessagePositiveAckCode, UdsBuffer,
};
use doip_rw::LogicalAddress;

use std::net::TcpStream;

const SOURCE_LOGICAL_ADDRESS: LogicalAddress = 0x0f02;
const TARGET_LOGICAL_ADDRESS: LogicalAddress = 0x0077;

fn main() -> Result<(), doip_rw::DoIpError> {
    let mut tcp = TcpStream::connect("127.0.0.1:13400")?;

    // Activate the routing
    let routing_request = RoutingActivationRequest {
        source_address: SOURCE_LOGICAL_ADDRESS,
        activation_type: ActivationType::Default,
        reserved: [0; 4],
        reserved_oem: Some([0; 4]),
    };
    doip_rw::write_message(&routing_request, &mut tcp)?;
    let routing_response: RoutingActivationResponse = doip_rw::read_message(&mut tcp)?;
    assert_eq!(
        routing_response.routing_activation_response_code,
        RoutingActivationResponseCode::RoutingSuccessfullyActivated
    );

    // Send a ReadDID(0xf012)
    let uds = DiagnosticMessage {
        source_address: SOURCE_LOGICAL_ADDRESS,
        target_address: TARGET_LOGICAL_ADDRESS,
        user_data: UdsBuffer::Owned(vec![0x22, 0xf0, 0xa0]),
    };
    doip_rw::write_message(&uds, &mut tcp)?;
    let uds_ack: DiagnosticMessagePositiveAck = doip_rw::read_message(&mut tcp)?;
    assert_eq!(
        uds_ack.ack_code,
        DiagnosticMessagePositiveAckCode::RoutingConfirmationAck
    );

    // Read back the ReadDID response
    let uds: DiagnosticMessage = doip_rw::read_message(&mut tcp)?;
    println!("ReadDID() result : {:2x?}", uds.user_data);
    let uds_ack = DiagnosticMessagePositiveAck {
        source_address: SOURCE_LOGICAL_ADDRESS,
        target_address: TARGET_LOGICAL_ADDRESS,
        ack_code: DiagnosticMessagePositiveAckCode::RoutingConfirmationAck,
        previous_diagnostic_message_data: uds.user_data,
    };
    doip_rw::write_message(&uds_ack, &mut tcp)?;
    Ok(())
}
