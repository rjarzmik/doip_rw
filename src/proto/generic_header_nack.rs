#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// A DoIP Nack response specifier
///
/// [`NegativeAckCode`] is used to indicate why a message was refused, and is part of a [`GenericDoIpHeaderNegativeAcknowledge`] message.
pub enum NegativeAckCode {
    /// DoIp Header mis-formed
    IncorrectPatternFormat,
    /// DoIp Payload type incorrect
    UnknownPayloadType,
    /// DoIp message too big for input buffer
    MessageTooLarge,
    /// Out of Memory
    OutOfMemory,
    /// Payload length incompatible with payload type
    InvalidPayloadLength,
    /// Reserved
    Reserved(u8),
}

/// Generic DoIp Header Negative Acknowledgement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericDoIpHeaderNegativeAcknowledge {
    /// The nack reason code
    pub nack_code: NegativeAckCode,
}
