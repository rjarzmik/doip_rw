use crate::{LogicalAddress, Vin};

/// A vehicle EID
pub type Eid = [u8; 6];
/// A vehicle GID
pub type Gid = [u8; 6];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Vehicle Identification Request.
///
/// This is a monocast/broadcast sent by a DoIP external tester to find all DoIP
/// entities available on this IPv4 address (which might by a broadcast address
/// such as 255.255.255.255).
pub struct VehicleIdentificationRequest {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Vehicle Identification Request with Eid.
///
/// This is the same as [`VehicleIdentificationRequest`], but asking to have an
/// [`Eid`] in the response.
pub struct VehicleIdentificationRequestWithEid {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Vehicle Identification Request with a VIN.
///
/// This is the same as [`VehicleIdentificationRequest`], but asking to have an
/// [`Vin`] in the response.
pub struct VehicleIdentificationRequestWithVin {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// VinGidSyncStatus.
pub enum VinGidSyncStatus {
    /// VIN and/or GID are synchronized
    Synchronized,
    /// Reserved
    Reserved(u8),
    /// VIN and GID are NOT synchronized
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// FurtherActionRequired
///
/// This is a field in [`VehicleIdentificationResponse`], asking for another
/// action.
pub enum FurtherActionRequired {
    /// No further action is required.
    NoFurtherActionRequired,
    /// Reserved.
    Reserved(u8),
    /// Routing activation required to initiate central security.
    RoutingActivationRequiredToInitiateCentralSecurity,
    /// VM Specific
    VmSpecific(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Vehicle identiifcation response / Vehicle announcement
///
/// This is the response to [`VehicleIdentificationRequest`], sent by a DoIP
/// entity to a DoIP external tester.
pub struct VehicleIdentificationResponse {
    /// Vehicle Identification Number.
    pub vin: Vin,
    /// Logical address of the DoIP entity.
    pub logical_address: LogicalAddress,
    /// Unique entitiy identification (EID), e.g. MAC address of network interface.
    pub eid: Eid,
    //// Unique group identification of entities within a vehicle.
    /// None when value not set (as indicated by `0x00` or `0xFF`).
    pub gid: Option<Gid>,
    /// Further action to be taken by the external tester.
    pub further_action: FurtherActionRequired,
    /// Indicates whether all entites have synced information about VIN or GID.
    pub vin_gid_sync_status: VinGidSyncStatus,
}
