use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use super::super::proto::power_mode_info::*;
use crate::{DoIpError, Payload, PayloadType};

impl Payload for PowerModeRequest {
    fn length(&self) -> usize {
        0
    }

    fn payload_type() -> PayloadType {
        PayloadType::DiagnosticPowerModeInformationRequest
    }

    fn read<T: Read>(_reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        assert_empty_payload(payload_length)?;
        Ok(PowerModeRequest {})
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

impl Payload for PowerModeResponse {
    fn length(&self) -> usize {
        size::PMRSP_DEFAULT_SIZE
    }

    fn payload_type() -> PayloadType {
        PayloadType::DiagnosticPowerModeInformationResponse
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        let mut me = size::PMRSP_ZEROED;
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        use DoIpError::*;
        if payload_length != size::PMRSP_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: size::PMRSP_DEFAULT_SIZE as u32,
            });
        }
        self.power_mode = reader.read_u8()?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        writer.write_u8(self.power_mode)?;
        Ok(())
    }
}

mod size {
    use super::PowerModeResponse;
    use std::mem::size_of;

    pub const PMRSP_ZEROED: PowerModeResponse = PowerModeResponse { power_mode: 0 };
    pub const PMRSP_DEFAULT_SIZE: usize = size_of::<u8>();
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
    fn power_mode_request() {
        let v = vec![
            0x02, 0xfd, // Protocol version
            0x40, 0x03, // Payload type
            0x00, 0x00, 0x00, 0x00, // Payload length
        ];
        assert_encode(&PowerModeRequest {}, &v);
        assert_decode(&PowerModeRequest {}, &v);
    }
    #[test]
    fn power_mode_response() {
        let payload = PowerModeResponse { power_mode: 0x01 };
        let v = vec![
            0x02, 0xfd, // Protocol version
            0x40, 0x04, // Payload type
            0x00, 0x00, 0x00, 0x01, // Payload length
            0x01,
        ];

        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }
}
