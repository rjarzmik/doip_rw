use crate::DoIpError;
use std::io::{Read, Write};

/// A DoIP payload
///
/// This trait is implemented by each DoIP payload in the [`crate::message`]
/// module.  It provides both the encoding and decoding of a DoIP payload
/// following a [`DoIpHeader`](struct@crate::DoIpHeader).
pub trait Payload {
    /// Get the payload type for this payload.
    fn payload_type() -> PayloadType
    where
        Self: Sized;
    /// Get the length of this payload.
    fn length(&self) -> usize;
    /// Reads from the reader `payload_length` bytes and decodes the message.
    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError>
    where
        Self: Sized;
    /// Replace this payload with a decoded one from the reader.
    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError>
    where
        Self: Sized;
    /// Writes the DoIP payload to a writer.
    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError>
    where
        Self: Sized;
}

/// A DoIP payload with references
///
/// This trait is implemented by only
/// [`DiagnosticMessage`](struct@crate::message::DiagnosticMessage) DoIP
/// payload. Its purpose is to create a payload with references rather that
/// owned data.
pub trait BorrowedPayload<'a> {
    /// Reads from the reader `payload_length` bytes and decodes the message,
    /// using borrowed reference on the input [`Payload`](trait@crate::Payload).
    fn read_borrowed(payload: &'a [u8]) -> Result<Self, DoIpError>
    where
        Self: Sized;
}

/// Supported DoIP payload types.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PayloadType {
    /// GenericDoIpHeaderNegativeAcknowledge
    GenericDoIpHeaderNegativeAcknowledge,
    /// VehicleIdentificationRequest
    VehicleIdentificationRequest,
    /// VehicleIdentificationRequestWithEid
    VehicleIdentificationRequestWithEid,
    /// VehicleIdentificationRequestWithVin
    VehicleIdentificationRequestWithVin,
    /// VehicleIdentificationResponse
    VehicleIdentificationResponse,
    /// RoutingActivationRequest
    RoutingActivationRequest,
    /// RoutingActivationResponse
    RoutingActivationResponse,
    /// AliveCheckRequest
    AliveCheckRequest,
    /// AliveCheckResponse
    AliveCheckResponse,
    /// DoIpEntityStatusRequest
    DoIpEntityStatusRequest,
    /// DoIpEntityStatusResponse
    DoIpEntityStatusResponse,
    /// DiagnosticPowerModeInformationRequest
    DiagnosticPowerModeInformationRequest,
    /// DiagnosticPowerModeInformationResponse
    DiagnosticPowerModeInformationResponse,
    /// DiagnosticMessage
    DiagnosticMessage,
    /// DiagnosticMessagePositiveAcknowledgement
    DiagnosticMessagePositiveAcknowledgement,
    /// DiagnosticMessageNegativeAcknowledgement
    DiagnosticMessageNegativeAcknowledgement,
    /// Reserved by specification for future use
    Reserved(u16),
    /// Reserved for use by vehicle manufacturer
    ReservedVm(u16),
}
