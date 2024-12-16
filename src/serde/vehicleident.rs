use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use super::super::proto::vehicleident::*;
use crate::{DoIpError, Payload, PayloadType};

impl Payload for VehicleIdentificationRequest {
    fn length(&self) -> usize {
        0
    }

    fn payload_type() -> PayloadType {
        PayloadType::VehicleIdentificationRequest
    }

    fn read<T: Read>(_reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        assert_empty_payload(payload_length)?;
        Ok(VehicleIdentificationRequest {})
    }

    fn read_replace<T: Read>(
        &mut self,
        _reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        assert_empty_payload(payload_length)
    }

    fn write<T: Write>(&self, _writer: &mut T) -> Result<(), DoIpError> {
        Ok(())
    }
}

impl Payload for VehicleIdentificationRequestWithEid {
    fn payload_type() -> PayloadType {
        PayloadType::VehicleIdentificationRequestWithEid
    }

    fn length(&self) -> usize {
        0
    }

    fn read<T: Read>(_reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        assert_empty_payload(payload_length)?;
        Ok(VehicleIdentificationRequestWithEid {})
    }

    fn read_replace<T: Read>(
        &mut self,
        _reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        assert_empty_payload(payload_length)
    }

    fn write<T: Write>(&self, _writer: &mut T) -> Result<(), DoIpError> {
        Ok(())
    }
}

impl Payload for VehicleIdentificationRequestWithVin {
    fn payload_type() -> PayloadType {
        PayloadType::VehicleIdentificationRequestWithVin
    }

    fn length(&self) -> usize {
        0
    }

    fn read<T: Read>(_reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        assert_empty_payload(payload_length)?;
        Ok(VehicleIdentificationRequestWithVin {})
    }

    fn read_replace<T: Read>(
        &mut self,
        _reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        assert_empty_payload(payload_length)
    }

    fn write<T: Write>(&self, _writer: &mut T) -> Result<(), DoIpError> {
        Ok(())
    }
}

impl Payload for VehicleIdentificationResponse {
    fn length(&self) -> usize {
        size::VIR_DEFAULT_SIZE
    }

    fn payload_type() -> PayloadType {
        PayloadType::VehicleIdentificationResponse
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        let mut me = size::VIR_ZEROED;
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        use DoIpError::*;
        if payload_length != size::VIR_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: size::VIR_DEFAULT_SIZE as u32,
            });
        }
        reader.read_exact(&mut self.vin)?;
        self.logical_address = reader.read_u16::<BigEndian>()?;
        reader.read_exact(&mut self.eid)?;

        let mut gid: Gid = [0x00; 6];
        reader.read_exact(&mut gid)?;
        // Table 1 - value not set
        self.gid = if gid == [0x00; 6] || gid == [0xFF; 6] {
            None
        } else {
            Some(gid)
        };

        let further_action_byte = reader.read_u8()?;
        self.further_action = FurtherActionRequired::from(further_action_byte);

        let vin_gid_sync_status_byte = reader.read_u8()?;
        self.vin_gid_sync_status = VinGidSyncStatus::from(vin_gid_sync_status_byte);
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        let _ = writer.write(&self.vin)?;
        writer.write_u16::<BigEndian>(self.logical_address)?;
        let _ = writer.write(&self.eid)?;
        let _ = writer.write(&self.gid.unwrap_or([0u8; 6]))?;
        writer.write_u8(self.further_action.into())?;
        writer.write_u8(self.vin_gid_sync_status.into())?;
        Ok(())
    }
}

mod size {
    use super::{Eid, Gid, VehicleIdentificationResponse};
    use super::{FurtherActionRequired, VinGidSyncStatus};
    use crate::{LogicalAddress, Vin};
    use std::mem::size_of;

    pub const VIR_ZEROED: VehicleIdentificationResponse = VehicleIdentificationResponse {
        vin: [0; 17],
        logical_address: 0,
        eid: [0; 6],
        gid: Some([0; 6]),
        further_action: FurtherActionRequired::NoFurtherActionRequired,
        vin_gid_sync_status: VinGidSyncStatus::Synchronized,
    };
    pub const VIR_DEFAULT_SIZE: usize = size_of::<Vin>()
        + size_of::<LogicalAddress>()
        + size_of::<Eid>()
        + size_of::<Gid>()
        + size_of::<u8>()
        + size_of::<u8>();
}

impl From<u8> for VinGidSyncStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => VinGidSyncStatus::Synchronized,
            0x10 => VinGidSyncStatus::Incomplete,
            // 0x01..=0x0F and 0x11..=0xFF
            _ => VinGidSyncStatus::Reserved(value),
        }
    }
}

impl From<VinGidSyncStatus> for u8 {
    fn from(value: VinGidSyncStatus) -> Self {
        use VinGidSyncStatus::*;
        match value {
            Synchronized => 0x00,
            Incomplete => 0x10,
            _ => 0xff,
        }
    }
}

impl From<u8> for FurtherActionRequired {
    fn from(value: u8) -> Self {
        match value {
            0x00 => FurtherActionRequired::NoFurtherActionRequired,
            0x01..=0x0F => FurtherActionRequired::Reserved(value),
            0x10 => FurtherActionRequired::RoutingActivationRequiredToInitiateCentralSecurity,
            0x11..=0xFF => FurtherActionRequired::VmSpecific(value),
        }
    }
}

impl From<FurtherActionRequired> for u8 {
    fn from(value: FurtherActionRequired) -> Self {
        use FurtherActionRequired::*;
        match value {
            NoFurtherActionRequired => 0x00,
            RoutingActivationRequiredToInitiateCentralSecurity => 0x10,
            _ => 0xff,
        }
    }
}

fn assert_empty_payload(payload_length: usize) -> Result<(), DoIpError> {
    use DoIpError::*;
    if payload_length != 0 {
        return Err(PayloadLengthTooShort {
            value: payload_length as u32,
            expected: 0u32,
        });
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::super::tests::*;
    use super::*;

    #[test]
    fn vehicle_identification_request() {
        let payload = VehicleIdentificationRequest {};
        let v = [
            0x02, 0xfd, // Protocol version
            0x00, 0x01, // Payload type
            0x00, 0x00, 0x00, 0x00, // Payload length
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }

    #[test]
    fn vehicle_identification_request_with_eid() {
        let payload = VehicleIdentificationRequestWithEid {};
        let v = [
            0x02, 0xfd, // Protocol version
            0x00, 0x02, // Payload type
            0x00, 0x00, 0x00, 0x00, // Payload length
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }

    #[test]
    fn vehicle_identification_request_with_vin() {
        let payload = VehicleIdentificationRequestWithVin {};
        let v = [
            0x02, 0xfd, // Protocol version
            0x00, 0x03, // Payload type
            0x00, 0x00, 0x00, 0x00, // Payload length
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }

    #[test]
    fn vehicle_identification_response() {
        let vin = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let eid = [0xaa, 0xbb, 0xcc, 0xdd, 0x00, 0x38];
        let gid = None;
        let payload = VehicleIdentificationResponse {
            vin,
            logical_address: 0xed00,
            eid,
            gid,
            further_action: FurtherActionRequired::NoFurtherActionRequired,
            vin_gid_sync_status: VinGidSyncStatus::Synchronized,
        };
        let v = [
            0x02, 0xfd, // Protocol version
            0x00, 0x04, // Payload type
            0x00, 0x00, 0x00, 0x21, // Payload length
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, // Vin
            0xed, 0x00, // LogicalAddress
            0xaa, 0xbb, 0xcc, 0xdd, 0x00, 0x38, // EID
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // GID
            0x00, // FurtherActionRequired,
            0x00, // VinGidSyncStatus
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }
}
