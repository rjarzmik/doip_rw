#[derive(Debug, Clone, PartialEq)]
/// Entity status request message
pub struct EntityStatusRequest {}

/// Entity status response
///
/// This response from the DoIP entity gives to the DoIP external tester
/// information about the limitations of the DoIP entity.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityStatusResponse {
    /// Node type
    pub node_type: u8,
    /// Maximum number of connections supported.
    pub max_open_sockets: u8,
    /// Current number of connections opened.
    pub cur_open_sockets: u8,
    /// Maximum length of a DoIP message which can be received.
    pub max_data_size: u32,
}
