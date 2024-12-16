use thiserror::Error;

#[derive(Error, Debug)]
/// DoIp generic error return
///
/// These error codes encompass both encoding and decoding errors.  The specific
/// [`DoIpError::Io`] error is designed to take care or reader and writer
/// issues, when a TCP stream is used as an input for example.
pub enum DoIpError {
    /// The DoIp payload is too short for the payload type.
    #[error("Payload length in header does match expected payload type length: {value:?}, expected: {expected:?}")]
    PayloadLengthTooShort {
        /// Payload length in the received DoIp message.
        value: u32,
        /// Minimum payload length for that type of DoIp message, according to its paylaod type.
        expected: u32,
    },
    /// The activation type for
    /// [`RoutingActivationRequest`](struct@crate::message::RoutingActivationRequest)
    /// is unknown.
    #[error("Unknown activation type value: {0}")]
    UnknownActivationType(u8),
    /// The activation response code for
    /// [`RoutingActivationResponse`](struct@crate::message::RoutingActivationResponse)
    /// is unknown.
    #[error("Unknown routing activation response code value: {0}")]
    UnknownRoutingActivationResponseCode(u8),
    /// The playload type is not valid.
    #[error("Unexpected payload type found: {value:?}")]
    UnexpectedPayloadType {
        /// Payload type received and invalid.
        value: u16,
    },
    /// The provided buffer for reading the message is too small.
    #[error("Buffer to small")]
    BufferTooSmall,
    /// An input/output error occurred while using a reader or a writer.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
