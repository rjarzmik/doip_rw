#[derive(Debug, Clone, PartialEq)]
/// Power Mode Request message.
pub struct PowerModeRequest {}

#[derive(Debug, Clone, PartialEq)]
/// Power Mode Response message.
pub struct PowerModeResponse {
    /// Power Mode
    pub power_mode: u8,
}
