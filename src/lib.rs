#![warn(missing_docs)]
//! DoIP protocol encoding and decoding library
//!
//! This crate provides all the Diagnostic over IP messages, with their
//! associated :
//! - encoding into a writer
//! - decoding from a reader
//!
//! For reference on the protocol, see ISO-14229-1.
//!
//! A DoIP message is composed of a header and a payload, ie. [`DoIpHeader`] and [`Payload`].
//!
//! The payload is the meaningfull part of the message, and is one of the
//! [`message`] module structs.
//!
//! A typical reception sequence using the library would be :
//! - call [`read_header()`].
//! - call [`read_payload()`] on the correct type.
//! - see documentation of [`read_message()`].
//!
//! A typical emission sequence using the library would be :
//! - build a message struct which implements [`Payload`].
//! - send it with [`write_message()`].
//! - see documentation of [`write_message()`].
mod error;
mod proto;
mod serde;
use std::io::{Read, Write};

pub use error::DoIpError;
pub use proto::header::{DoIpHeader, DOIP_HEADER_LENGTH};
pub use proto::payload::{BorrowedPayload, Payload, PayloadType};

/// A DoIP logical address, both for a tester or a tested entity
pub type LogicalAddress = u16;
/// A Vehicle Identifier Number
pub type Vin = [u8; 17];

/// Reads a DoIP header and attemps to read a DoIp payload
///
/// This function is only usable if it is known beforehand which message is
/// coming next, in which case it returns that DoIp payload.  If used with the
/// wrong payload type, it will return a PayloadLengthTooShort error, and stops
/// reading after the header.
///
/// A sounder use would be :
/// ```
/// use doip_rw::{read_header, read_payload, PayloadType, Payload, message::AliveCheckRequest, };
/// use std::io::Cursor;
///
/// // let mut tcp = TcpStream::connect("127.0.0.1:13400").unwrap();
/// let mut tcp = Cursor::new([0x02, 0xfd, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00]);
/// let header = read_header(&mut tcp).unwrap();
/// match header.payload_type {
///   PayloadType::AliveCheckRequest => {
///     let acr : AliveCheckRequest = read_payload(&mut tcp, header.payload_length as usize).unwrap();
///   },
///   _ => {},
/// }
/// ```
pub fn read_message<R: Read, P: Payload>(reader: &mut R) -> Result<P, DoIpError> {
    let header = read_header(reader)?;
    if P::payload_type() == header.payload_type {
        read_payload(reader, header.payload_length as usize)
    } else {
        Err(DoIpError::UnexpectedPayloadType {
            value: header.payload_type.into_u16(),
        })
    }
}

/// Writes a DoIP header and its DoIp payload
///
/// This function calculates the DoIp Header from the payload, and then sends them
/// both in the writer.
/// This is the main and probably the _only_ function which should be used to send
/// DoIP messages.
///
/// Example:
/// ```
/// use doip_rw::{write_message, message::RoutingActivationRequest, message::ActivationType};
/// use std::net::TcpStream;
///
/// // let mut tcp = TcpStream::connect("127.0.0.1:13400").unwrap();
/// let mut tcp = vec![];
/// let routing_activation = RoutingActivationRequest {
///     source_address: 0x00ed,
///     activation_type: ActivationType::Default,
///     reserved: [0; 4],
///     reserved_oem: Some([0; 4]),
/// };
/// write_message(&routing_activation, &mut tcp).unwrap();
/// ```
pub fn write_message<W: Write, P: Payload>(payload: &P, writer: &mut W) -> Result<(), DoIpError> {
    let header = DoIpHeader::new(P::payload_type(), payload.length() as u32);
    header.write(writer)?;
    payload.write(writer)
}

/// Length of a DoIp message in bytes
///
/// The length is the fixed number of bytes of the DoIp header, added to the
/// number of bytes of the provided payload.
pub fn length_message<P: Payload>(payload: &P) -> usize {
    DOIP_HEADER_LENGTH + payload.length()
}

/// Read a DoIp header
///
/// Read a full DoIp header, ie. the 8 bytes of a DoIp header.
/// This is the first part of a DoIp message normal read flow.
///
/// Example:
/// ```
/// use std::io::Cursor;
/// use doip_rw::read_header;
///
/// // let mut tcp = TcpStream::connect("127.0.0.1:13400").unwrap();
/// let mut tcp = Cursor::new([0x02, 0xfd, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00]);
/// let hdr = read_header(&mut tcp).unwrap();
/// ```
pub fn read_header<R: Read>(reader: &mut R) -> Result<DoIpHeader, DoIpError> {
    DoIpHeader::read(reader)
}

/// Read a specific DoIp payload, specifying the exact payload through Payload type
///
/// This function should normally be called after `read_header`, depending on the
/// `payload_type`.
///
/// Example: if the header payload_type == RoutingActivationResponse :
/// ```
/// use std::io::Cursor;
/// use doip_rw::{read_header, read_payload, message::AliveCheckResponse};
///
/// // let mut tcp = TcpStream::connect("127.0.0.1:13400").unwrap();
/// let mut tcp = Cursor::new([0x02, 0xfd, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x04, 0x54]);
/// let hdr = read_header(&mut tcp).unwrap();
/// let response : AliveCheckResponse = read_payload(&mut tcp, hdr.payload_length as usize).unwrap();
/// ```
pub fn read_payload<R: Read, P: Payload>(
    reader: &mut R,
    payload_length: usize,
) -> Result<P, DoIpError> {
    P::read(reader, payload_length)
}

/// Read a specific DoIp payload into an existing payload
///
/// This function is `read_payload` without a memory allocation. The difference
/// is that this function reuses the payload's buffer, and if there is an
/// internal buffer, such as in DiagnosticMessage::user_data, the internal
/// buffer is resized.
///
/// Example:
/// ```
/// use std::io::Cursor;
/// use doip_rw::{read_header, read_payload, read_replace_payload, message::AliveCheckResponse};
///
/// // let mut tcp = TcpStream::connect("127.0.0.1:13400").unwrap();
/// let mut tcp = Cursor::new([0x02, 0xfd, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x04, 0x54]);
/// let hdr = read_header(&mut tcp).unwrap();
/// let mut response : AliveCheckResponse = read_payload(&mut tcp, hdr.payload_length as usize).unwrap();
/// let mut tcp = Cursor::new([0x02, 0xfd, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x04, 0x54]);
/// read_replace_payload(&mut response, &mut tcp, hdr.payload_length as usize).unwrap();
/// ```
pub fn read_replace_payload<R: Read, P: Payload>(
    payload: &mut P,
    reader: &mut R,
    payload_length: usize,
) -> Result<(), DoIpError> {
    payload.read_replace(reader, payload_length)
}

/// Read a specific DoIp message into an existing message
///
/// This function is `read_message` without a memory allocation. The difference
/// is that this function reuses the message's buffer, and if there is an
/// internal buffer, such as in DiagnosticMessage::user_data, the internal
/// buffer is resized.
///
/// Example:
/// ```
/// use std::io::Cursor;
/// use doip_rw::{read_header, read_payload, read_replace_message, message::AliveCheckResponse};
///
/// // let mut tcp = TcpStream::connect("127.0.0.1:13400").unwrap();
/// let mut tcp = Cursor::new([0x02, 0xfd, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x04, 0x54]);
/// let hdr = read_header(&mut tcp).unwrap();
/// let mut response : AliveCheckResponse = read_payload(&mut tcp, hdr.payload_length as usize).unwrap();
/// let mut tcp = Cursor::new([0x02, 0xfd, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x04, 0x54]);
/// read_replace_message(&mut response, &mut tcp).unwrap();
/// ```
pub fn read_replace_message<R: Read, P: Payload>(
    payload: &mut P,
    reader: &mut R,
) -> Result<(), DoIpError> {
    let header = read_header(reader)?;
    if P::payload_type() == header.payload_type {
        read_replace_payload(payload, reader, header.payload_length as usize)
    } else {
        Err(DoIpError::UnexpectedPayloadType {
            value: header.payload_type.into_u16(),
        })
    }
}

/// Module containing all the *messages* handled by the API.
///
/// The [`read_message()`], [`read_payload()`], [`write_message()`] all rely on
/// the messages in this module, which represent all the possible DoIP messages.
pub mod message {
    // Export all the DoIp messages handled by the crate
    pub use super::proto::alive_check::*;
    pub use super::proto::diagnostic_message::*;
    pub use super::proto::entity_status::*;
    pub use super::proto::generic_header_nack::*;
    pub use super::proto::header::*;
    pub use super::proto::power_mode_info::*;
    pub use super::proto::routing_activation::*;
    pub use super::proto::vehicleident::*;
}
