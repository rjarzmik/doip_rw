use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use super::super::proto::entity_status::*;
use crate::{DoIpError, Payload, PayloadType};

impl Payload for EntityStatusRequest {
    fn length(&self) -> usize {
        0
    }

    fn payload_type() -> PayloadType {
        PayloadType::DoIpEntityStatusRequest
    }

    fn read<T: Read>(_reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        assert_empty_payload(payload_length)?;
        Ok(EntityStatusRequest {})
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

impl Payload for EntityStatusResponse {
    fn length(&self) -> usize {
        size::ESRSP_DEFAULT_SIZE
    }

    fn payload_type() -> PayloadType {
        PayloadType::DoIpEntityStatusResponse
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        let mut me = size::ESRSP_ZEROED;
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        use DoIpError::*;
        if payload_length != size::ESRSP_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: size::ESRSP_DEFAULT_SIZE as u32,
            });
        }
        self.node_type = reader.read_u8()?;
        self.max_open_sockets = reader.read_u8()?;
        self.cur_open_sockets = reader.read_u8()?;
        self.max_data_size = reader.read_u32::<BigEndian>()?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        writer.write_u8(self.node_type)?;
        writer.write_u8(self.max_open_sockets)?;
        writer.write_u8(self.cur_open_sockets)?;
        writer.write_u32::<BigEndian>(self.max_data_size)?;
        Ok(())
    }
}

mod size {
    use super::EntityStatusResponse;
    use std::mem::size_of;

    pub const ESRSP_ZEROED: EntityStatusResponse = EntityStatusResponse {
        node_type: 0,
        max_open_sockets: 0,
        cur_open_sockets: 0,
        max_data_size: 0,
    };
    pub const ESRSP_DEFAULT_SIZE: usize =
        size_of::<u8>() + size_of::<u8>() + size_of::<u8>() + size_of::<u32>();
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
    fn entity_status_request() {
        let v = [
            0x02, 0xfd, // Protocol version
            0x40, 0x01, // Payload type
            0x00, 0x00, 0x00, 0x00, // Payload length
        ];
        assert_encode(&EntityStatusRequest {}, &v);
        assert_decode(&EntityStatusRequest {}, &v);
    }
    #[test]
    fn entity_status_response() {
        let payload = EntityStatusResponse {
            node_type: 0x12,
            max_open_sockets: 2,
            cur_open_sockets: 1,
            max_data_size: 1024,
        };
        let v = vec![
            0x02, 0xfd, // Protocol version
            0x40, 0x02, // Payload type
            0x00, 0x00, 0x00, 0x07, // Payload length
            0x12, 0x02, 0x01, 0x00, 0x00, 0x04, 0x00,
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }
}
