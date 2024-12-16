use crate::LogicalAddress;

#[derive(Debug, Clone, PartialEq)]
/// Alive check request message
///
/// The alive check request is a message sent by the DoIp entity tested to the
/// external tester to verify the tester is still plugged in.
pub struct AliveCheckRequest {}

#[derive(Debug, Clone, PartialEq)]
/// Alive check response message
///
/// The alive check response is a message sent by the DoIp external tester to the
/// DoIp entity to notify it is still plugged in.
pub struct AliveCheckResponse {
    /// Logical address of the tester
    pub source_address: LogicalAddress,
}
