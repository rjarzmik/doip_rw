use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use super::super::proto::alive_check::*;
use crate::{DoIpError, Payload, PayloadType};

impl Payload for AliveCheckRequest {
    fn length(&self) -> usize {
        0
    }

    fn payload_type() -> PayloadType {
        PayloadType::AliveCheckRequest
    }

    fn read<T: Read>(_reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        assert_empty_payload(payload_length)?;
        Ok(AliveCheckRequest {})
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

impl Payload for AliveCheckResponse {
    fn length(&self) -> usize {
        size::ALRSP_DEFAULT_SIZE
    }

    fn payload_type() -> PayloadType {
        PayloadType::AliveCheckResponse
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        let mut me = size::ALRSP_ZEROED;
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        use DoIpError::*;
        if payload_length != size::ALRSP_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: size::ALRSP_DEFAULT_SIZE as u32,
            });
        }
        self.source_address = reader.read_u16::<BigEndian>()?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        writer.write_u16::<BigEndian>(self.source_address)?;
        Ok(())
    }
}

mod size {
    use super::AliveCheckResponse;
    use crate::LogicalAddress;
    use std::mem::size_of;

    pub const ALRSP_ZEROED: AliveCheckResponse = AliveCheckResponse { source_address: 0 };
    pub const ALRSP_DEFAULT_SIZE: usize = size_of::<LogicalAddress>();
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
    fn alive_check_request() {
        let payload = AliveCheckRequest {};
        let v = [
            0x02, 0xfd, // Protocol version
            0x00, 0x07, // Payload type
            0x00, 0x00, 0x00, 0x00, // Payload length
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }
    #[test]
    fn alive_check_response() {
        let payload = crate::message::AliveCheckResponse {
            source_address: 0x045e,
        };
        let v = [
            0x02, 0xfd, // Protocol version
            0x00, 0x08, // Payload type
            0x00, 0x00, 0x00, 0x02, // Payload length
            0x04, 0x5e,
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }
}
