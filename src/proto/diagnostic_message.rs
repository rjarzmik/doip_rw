use crate::LogicalAddress;

#[derive(Debug, Clone, PartialEq)]
/// Diagnostic message
///
/// This is the main purpose of DoIp, ie. to convey a [`DiagnosticMessage`] from
/// the external tester to the DoIp entity (such as a read DID request).
/// It's also conveying the response, such as the read DID response.
///
/// It is the only DoIp message which can be big in size, as the UDS carried
/// data can be quite large, especially for TransferDownload UDS
/// requests. Therefore, the [`UdsBuffer`](enum@crate::message::UdsBuffer) is
/// special, as it can be either owning data or borrowing data.
///
/// When using DoIP gateways, the `target_address` might be inconsistent with
/// the IP target address, in the sense that the DoIP entity target address is
/// different from the one in the [`DiagnosticMessage`]. In that case, it's up
/// to the DoIP entity to forward the message to the final DoIP entity, hence
/// the name of DoIP gateway.
pub struct DiagnosticMessage<'a> {
    /// Logical address of the sender.
    pub source_address: LogicalAddress,
    /// Logical address of the target of the message.
    pub target_address: LogicalAddress,
    /// Buffer carrying the UDS message.
    pub user_data: UdsBuffer<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// Positive acknowledgement code for [`DiagnosticMessagePositiveAck`].
pub enum DiagnosticMessagePositiveAckCode {
    /// Positive acknowledgement of reception
    RoutingConfirmationAck,
    /// Reserved
    Reserved(u8),
}

/// Diagnostic message positive acknowledgement
///
/// Message sent by the DoIP entity to the DoIP external tester to notify it that the previous [`DiagnosticMessage`] was successfully received, and parsed.
#[derive(Debug, PartialEq)]
pub struct DiagnosticMessagePositiveAck<'a> {
    /// Logical address of the sender.
    pub source_address: LogicalAddress,
    /// Logical address of the target of the message.
    pub target_address: LogicalAddress,
    /// Ack code.
    pub ack_code: DiagnosticMessagePositiveAckCode,
    /// Repeated acked message, might be void.
    pub previous_diagnostic_message_data: UdsBuffer<'a>,
}

/// Negative acknowledgement of diagnostic message.
#[derive(Debug, PartialEq)]
pub struct DiagnosticMessageNegativeAck<'a> {
    /// Logical address of the sender
    pub source_address: LogicalAddress,
    /// Logical address of the target of the message
    pub target_address: LogicalAddress,
    /// Nack code.
    pub ack_code: DiagnosticMessageNegativeAckCode,
    /// Repeated acked message, might be void.
    pub previous_diagnostic_message_data: UdsBuffer<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// Negative acknowledgement code for [`DiagnosticMessageNegativeAck`]
pub enum DiagnosticMessageNegativeAckCode {
    /// The source address in the [`DiagnosticMessage`] is invalid.
    InvalidSourceAddress,
    /// The target address in the [`DiagnosticMessage`] is unknown.
    UnknownTargetAddress,
    /// Exceeds maximum supported size of transport protocol.
    DiagnosticMessageTooLarge,
    /// Out of memory
    OutOfMemory,
    /// The target DoIP entity is not reachable.
    TargetUnreachable,
    /// Unknown network
    UnknownNetwork,
    /// Transport error
    TransportProtocolError,
    /// Reserved
    Reserved(u8),
}

#[derive(Debug, Clone, PartialEq)]
/// Uds buffer holding the UDS message
///
/// The buffer has 2 variants, one owned and one borrowed. Usually and by
/// default, it is advisable to use only the owner variant.
///
/// The borrowed variable is usefull to send a DiagnosticMessage with a huge UDS
/// message. This message is usually in an application buffer, and in order to
/// not make a copy, and UdsBuffer can be created referencing this application
/// buffer.
pub enum UdsBuffer<'a> {
    /// Owned variant, ie. UDS message in a [`Vec`]
    Owned(Vec<u8>),
    /// Borrowed variant
    Borrowed(&'a [u8]),
}

impl UdsBuffer<'_> {
    /// Get the slice holding the UDS message
    pub fn get_ref(&self) -> &[u8] {
        match self {
            UdsBuffer::Owned(v) => v.as_ref(),
            UdsBuffer::Borrowed(b) => b,
        }
    }
}
