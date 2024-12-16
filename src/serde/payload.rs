use crate::PayloadType;

impl PayloadType {
    /// Convert a [`PayloadType`] into a u16.
    pub fn into_u16(self) -> u16 {
        use PayloadType::*;

        match self {
            GenericDoIpHeaderNegativeAcknowledge => 0x0000,
            VehicleIdentificationRequest => 0x0001,
            VehicleIdentificationRequestWithEid => 0x0002,
            VehicleIdentificationRequestWithVin => 0x0003,
            VehicleIdentificationResponse => 0x0004,
            RoutingActivationRequest => 0x0005,
            RoutingActivationResponse => 0x0006,
            AliveCheckRequest => 0x0007,
            AliveCheckResponse => 0x0008,
            DoIpEntityStatusRequest => 0x4001,
            DoIpEntityStatusResponse => 0x4002,
            DiagnosticPowerModeInformationRequest => 0x4003,
            DiagnosticPowerModeInformationResponse => 0x4004,
            DiagnosticMessage => 0x8001,
            DiagnosticMessagePositiveAcknowledgement => 0x8002,
            DiagnosticMessageNegativeAcknowledgement => 0x8003,
            Reserved(value) => value,
            ReservedVm(value) => value,
        }
    }
}

impl From<u16> for PayloadType {
    /// Convert a u16 into a [`PayloadType`].
    fn from(value: u16) -> Self {
        use PayloadType::*;
        match value {
            0x0000 => GenericDoIpHeaderNegativeAcknowledge,
            0x0001 => VehicleIdentificationRequest,
            0x0002 => VehicleIdentificationRequestWithEid,
            0x0003 => VehicleIdentificationRequestWithVin,
            0x0004 => VehicleIdentificationResponse,
            0x0005 => RoutingActivationRequest,
            0x0006 => RoutingActivationResponse,
            0x0007 => AliveCheckRequest,
            0x0008 => AliveCheckResponse,
            0x0009..=0x4000 => Reserved(value),
            0x4001 => DoIpEntityStatusRequest,
            0x4002 => DoIpEntityStatusResponse,
            0x4003 => DiagnosticPowerModeInformationRequest,
            0x4004 => DiagnosticPowerModeInformationResponse,
            0x4005..=0x8000 => Reserved(value),
            0x8001 => DiagnosticMessage,
            0x8002 => DiagnosticMessagePositiveAcknowledgement,
            0x8003 => DiagnosticMessageNegativeAcknowledgement,
            0x8004..=0xEFFF => Reserved(value),
            0xF000..=0xFFFF => ReservedVm(value),
        }
    }
}

/*
#[derive(PartialEq, Debug)]
pub enum Payload<U = diagnostic::DefaultUserData> {
    /// Generic DoIP header negative acknowledge
    GenericDoIpHeaderNegativeAcknowledge(generic_doip_header_nack::NackCode),
    /// Vehicle identification request message.
    VehicleIdentificationRequest,
    /// Vehicle identification request message with EID
    VehicleIdentificationRequestWithEid(vehicle_identification::Eid),
    /// Vehicle identification request message with VIN
    VehicleIdentificationRequestWithVin(vehicle_identification::Vin),
    /// Vehicle announcement message/vehicle identification response message.
    VehicleIdentificationResponse(vehicle_identification::ResponsePayload),
    /// Routing activation request.
    RoutingActivationRequest(routing_activation::RequestPayload),
    /// Routing activation response.
    RoutingActivationResponse(routing_activation::ResponsePayload),
    /// Alive check request.
    AliveCheckRequest,
    /// Alive check response.
    AliveCheckResponse(LogicalAddress),
    /// DoIP entity status request.
    DoIpEntityStatusRequest,
    /// DoIP entity status response
    DoIpEntityStatusResponse(doip_entity_status::ResponsePayload),
    /// Diagnostic power mode information request.
    DiagnosticPowerModeInformationRequest,
    /// Diagnostic power mode information response.
    DiagnosticPowerModeInformationResponse(diagnostic::DiagnosticPowerMode),
    /// Diagnostic message.
    DiagnosticMessage(diagnostic::MessagePayload<U>),
    /// Diagnostic message positive acknowledgement.
    DiagnosticMessagePositiveAcknowledgement(diagnostic::PositiveResponsePayload<U>),
    /// Diagnostic message negative acknowledgement.
    DiagnosticMessageNegativeAcknowledgement(diagnostic::NegativeResponsePayload<U>),
}
*/
