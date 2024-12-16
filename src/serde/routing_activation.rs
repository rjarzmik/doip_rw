use crate::DoIpError;
use crate::DoIpError::*;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use crate::proto::routing_activation::*;
use crate::{Payload, PayloadType};

impl Payload for RoutingActivationRequest {
    fn length(&self) -> usize {
        self.reserved_oem
            .map(|_| size::RAREQ_OEM_SIZE)
            .unwrap_or(size::RAREQ_DEFAULT_SIZE)
    }

    fn payload_type() -> PayloadType {
        PayloadType::RoutingActivationRequest
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        let mut me = size::RAREQ_ZEROED;
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        let has_oem_data: bool = match payload_length {
            size::RAREQ_DEFAULT_SIZE => Ok(false),
            size::RAREQ_OEM_SIZE => Ok(true),
            _ => Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: size::RAREQ_DEFAULT_SIZE as u32,
            }),
        }?;
        self.source_address = reader.read_u16::<BigEndian>()?;
        let activation_type_raw: u8 = reader.read_u8()?;
        self.activation_type = ActivationType::try_from(activation_type_raw)?;
        reader.read_exact(&mut self.reserved)?;

        let mut reserved_oem = [0x00; 4];
        self.reserved_oem = match has_oem_data {
            true => {
                reader.read_exact(&mut reserved_oem)?;
                Some(reserved_oem)
            }
            false => None,
        };
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        writer.write_u16::<BigEndian>(self.source_address)?;
        writer.write_u8(self.activation_type as u8)?;
        writer.write_all(&self.reserved)?;
        if let Some(reserved_oem) = self.reserved_oem {
            writer.write_all(&reserved_oem)?;
        }
        Ok(())
    }
}

impl Payload for RoutingActivationResponse {
    fn length(&self) -> usize {
        self.oem_specific
            .map(|_| size::RARSP_OEM_SIZE)
            .unwrap_or(size::RARSP_DEFAULT_SIZE)
    }

    fn payload_type() -> PayloadType {
        PayloadType::RoutingActivationResponse
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        let mut me = size::RARSP_ZEROED;
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        let has_oem_data: bool = match payload_length {
            size::RARSP_DEFAULT_SIZE => Ok(false),
            size::RARSP_OEM_SIZE => Ok(true),
            _ => Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: size::RAREQ_DEFAULT_SIZE as u32,
            }),
        }?;

        self.logical_address_tester = reader.read_u16::<BigEndian>()?;
        self.logical_address_of_doip_entity = reader.read_u16::<BigEndian>()?;
        let routing_activation_response_code_byte = reader.read_u8()?;
        self.routing_activation_response_code =
            RoutingActivationResponseCode::try_from(routing_activation_response_code_byte)?;
        reader.read_exact(&mut self.reserved_oem)?;

        self.oem_specific = if has_oem_data {
            let mut oem_specific = [0x00u8; 4];
            reader.read_exact(&mut oem_specific)?;
            Some(oem_specific)
        } else {
            None
        };
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        writer.write_all(&self.logical_address_tester.to_be_bytes())?;
        writer.write_all(&self.logical_address_of_doip_entity.to_be_bytes())?;
        writer.write_u8(self.routing_activation_response_code as u8)?;
        writer.write_all(&self.reserved_oem)?;
        if let Some(oem_specific) = self.oem_specific {
            writer.write_all(&oem_specific)?;
        }
        Ok(())
    }
}

mod size {
    use crate::proto::routing_activation::*;
    use crate::LogicalAddress;
    use std::mem::size_of;

    pub const RAREQ_ZEROED: RoutingActivationRequest = RoutingActivationRequest {
        source_address: 0u16,
        activation_type: ActivationType::Default,
        reserved: [0u8; 4],
        reserved_oem: Some([0u8; 4]),
    };
    pub const RAREQ_DEFAULT_SIZE: usize =
        size_of::<LogicalAddress>() + size_of::<u8>() + size_of::<[u8; 4]>();
    pub const RAREQ_OEM_SIZE: usize = RAREQ_DEFAULT_SIZE + size_of::<[u8; 4]>();

    pub const RARSP_ZEROED: RoutingActivationResponse = RoutingActivationResponse {
        logical_address_tester: 0u16,
        logical_address_of_doip_entity: 0u16,
        routing_activation_response_code:
            RoutingActivationResponseCode::RoutingActivationDeniedUnknownSourceAddress,
        reserved_oem: [0; 4],
        oem_specific: Some([0; 4]),
    };
    pub const RARSP_DEFAULT_SIZE: usize = size_of::<LogicalAddress>()
        + size_of::<LogicalAddress>()
        + size_of::<u8>()
        + size_of::<[u8; 4]>();
    pub const RARSP_OEM_SIZE: usize = RARSP_DEFAULT_SIZE + size_of::<[u8; 4]>();
}

impl TryFrom<u8> for ActivationType {
    type Error = DoIpError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ActivationType::Default),
            0x01 => Ok(ActivationType::WwhObd),
            0x02 => Ok(ActivationType::CentralSecurity),
            _ => Err(DoIpError::UnknownActivationType(value)),
        }
    }
}

impl TryFrom<u8> for RoutingActivationResponseCode {
    type Error = DoIpError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use RoutingActivationResponseCode::*;
        match value {
            0x00 => Ok(RoutingActivationDeniedUnknownSourceAddress),
            0x01 => Ok(RoutingActivationDeniedAllTcpSocketsRegisteredAndActive),
            0x02 => Ok(RoutingActivationDeniedSourceAddressAlreadyActivated),
            0x03 => Ok(RoutingActivationDeniedSourceAddressAlreadyRegistred),
            0x04 => Ok(RoutingActivationDeniedMissingAuthentication),
            0x05 => Ok(RoutingActivationDeniedRejectedConfirmation),
            0x06 => Ok(RoutingActivationDeniedUnsupportedRoutingActivationType),
            0x07 => Ok(RoutingActivationDeniedEncryptedConnectionViaTLSRequired),
            0x10 => Ok(RoutingSuccessfullyActivated),
            0x11 => Ok(RoutingSuccessfullyActivatedConfirmationRequired),
            _ => Err(DoIpError::UnknownRoutingActivationResponseCode(value)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::tests::*;
    use super::*;

    #[test]
    fn routing_activation_request() {
        let payload = RoutingActivationRequest {
            source_address: 0x0123,
            activation_type: ActivationType::Default,
            reserved: [0u8; 4],
            reserved_oem: None,
        };
        let v = vec![
            0x02, 0xfd, // Protocol version
            0x00, 0x05, // Payload type
            0x00, 0x00, 0x00, 0x07, // Payload length
            0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }

    #[test]
    fn routing_activation_response() {
        let payload = RoutingActivationResponse {
            logical_address_tester: 0x0123,
            logical_address_of_doip_entity: 0x00ed,
            routing_activation_response_code:
                RoutingActivationResponseCode::RoutingSuccessfullyActivated,
            reserved_oem: [0u8; 4],
            oem_specific: None,
        };
        let v = vec![
            0x02, 0xfd, // Protocol version
            0x00, 0x06, // Payload type
            0x00, 0x00, 0x00, 0x09, // Payload length
            0x01, 0x23, 0x00, 0xed, 0x10, 0x00, 0x00, 0x00, 0x00,
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }
}
