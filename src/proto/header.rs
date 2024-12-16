use crate::PayloadType;
use core::mem::size_of;

/// Length of the DoIP header.
pub const DOIP_HEADER_LENGTH: usize =
    size_of::<u8>() + size_of::<u8>() + size_of::<u16>() + size_of::<u32>(); // 8 byte

#[derive(Debug, PartialEq)]
/// Generic DoIP header data structure.
///
/// This header always preceeds a DoIP payload
/// [`Payload`](trait@crate::Payload).
pub struct DoIpHeader {
    /// DoIP protocol version :
    /// - 0x01: ISO13400-2:2010.
    /// - 0x02: ISO13400-2:2012.
    /// - 0x03: ISO13400-2:2019.
    pub protocol_version: u8,
    /// The complement-to-1 to [`DoIpHeader::protocol_version`] field.
    pub inverse_protocol_version: u8,
    /// DoIP payload type.
    pub payload_type: PayloadType, // u16
    /// DoIP payload length.
    pub payload_length: u32,
}

impl DoIpHeader {
    /// Creates a [`DoIpHeader`] from the provided fields.
    ///
    /// The created [`DoIpHeader`] will have protocol set to [`ProtocolVersion::DoIpIso`].
    pub fn new(payload_type: PayloadType, payload_length: u32) -> Self {
        Self {
            protocol_version: ProtocolVersion::DoIpIso as u8,
            inverse_protocol_version: !(ProtocolVersion::DoIpIso as u8),
            payload_type,
            payload_length,
        }
    }
    /// Creates a [`DoIpHeader`] from the provided fields.
    pub fn new_versionned(
        protocol_version: ProtocolVersion,
        payload_type: PayloadType,
        payload_length: u32,
    ) -> Self {
        Self {
            protocol_version: protocol_version as u8,
            inverse_protocol_version: !(protocol_version as u8),
            payload_type,
            payload_length,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
/// DoIP protocol versions.
pub enum ProtocolVersion {
    /// DoIP ISO Dis
    DoIpIsoDis = 0x1,
    /// DoIP ISO
    DoIpIso = 0x2,
    /// VehicleIdentificationRequest
    VehicleIdentificationRequest = 0xFF,
}
