use crate::LogicalAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// Activation type.
///
/// The activation type used in [`RoutingActivationRequest`].
pub enum ActivationType {
    /// ISO 14229
    Default = 0x00,
    /// WWH-OBD for OBD
    WwhObd = 0x01,
    /// OEM specific authentication,
    CentralSecurity = 0x02,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Routing activation request message.
///
/// This is usually the first message from a DoIP external tester to a DoIP
/// entity, to begin the diagnostic stream. It also notifies the DoIP entity of
/// the tester's [`LogicalAddress`], for further messages.
pub struct RoutingActivationRequest {
    /// Address of DoIP tester that requests routing activation.
    pub source_address: LogicalAddress,
    /// Activation type
    pub activation_type: ActivationType,
    /// Reserved
    pub reserved: [u8; 4],
    /// Reserved OEM
    pub reserved_oem: Option<[u8; 4]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// Routing Activation Response Code
///
/// Code replied by the tested DoIP entity, to either accept or deny a DoIP
/// connection. If the code is
/// [`RoutingActivationResponseCode::RoutingSuccessfullyActivated`], the DoIP
/// connection is established, and the diagnostic can further proceed.
pub enum RoutingActivationResponseCode {
    /// The [`LogicalAddress`] of the external tester is unknown to the DoIP
    /// entity.
    RoutingActivationDeniedUnknownSourceAddress = 0x00,
    /// The DoIP entity doesn't have Tcp ressources available anymore.
    RoutingActivationDeniedAllTcpSocketsRegisteredAndActive = 0x01,
    /// The [`LogicalAddress`] of the external tester was already activated.
    RoutingActivationDeniedSourceAddressAlreadyActivated = 0x02,
    /// The [`LogicalAddress`] of the external tester was already registered.
    RoutingActivationDeniedSourceAddressAlreadyRegistred = 0x03,
    /// Authentication was not provided.
    RoutingActivationDeniedMissingAuthentication = 0x04,
    /// Routing rejected.
    RoutingActivationDeniedRejectedConfirmation = 0x05,
    /// Routing was rejected due to unknown [`ActivationType`] in [`RoutingActivationRequest`].
    RoutingActivationDeniedUnsupportedRoutingActivationType = 0x06,
    /// Encrypted activation requires encryption.
    RoutingActivationDeniedEncryptedConnectionViaTLSRequired = 0x07,
    /// Routing is accepted, this is the OK message.
    RoutingSuccessfullyActivated = 0x10,
    /// Routing is activated, but confirmation is required.
    RoutingSuccessfullyActivatedConfirmationRequired = 0x11,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Routing activation response message.
///
/// This is the reply to [`RoutingActivationRequest`], sent by a DoIP entity to
/// a DoIP external tester.
///
/// The tester hopes to get a
/// [`RoutingActivationResponseCode::RoutingSuccessfullyActivated`].
pub struct RoutingActivationResponse {
    /// External DoIP test equipment address.
    pub logical_address_tester: LogicalAddress,
    /// DoIP entity address.
    pub logical_address_of_doip_entity: u16,
    /// Routing activation status information.
    pub routing_activation_response_code: RoutingActivationResponseCode,
    /// Reserved OEM.
    pub reserved_oem: [u8; 4],
    /// OEM specific.
    pub oem_specific: Option<[u8; 4]>,
}
