use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use super::super::proto::generic_header_nack::*;
use crate::{DoIpError, Payload, PayloadType};

impl Payload for GenericDoIpHeaderNegativeAcknowledge {
    fn length(&self) -> usize {
        size::GENERIC_HEADER_NACK_DEFAULT_SIZE
    }

    fn payload_type() -> PayloadType {
        PayloadType::GenericDoIpHeaderNegativeAcknowledge
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        let mut me = size::GENERIC_HEADER_NACK_ZEROED;
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        use DoIpError::*;
        if payload_length != size::GENERIC_HEADER_NACK_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: size::GENERIC_HEADER_NACK_DEFAULT_SIZE as u32,
            });
        }
        let nack_code = reader.read_u8()?;
        self.nack_code = nack_code.into();
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        let nack_code: u8 = self.nack_code.into();
        writer.write_u8(nack_code)?;
        Ok(())
    }
}

mod size {
    use super::{GenericDoIpHeaderNegativeAcknowledge, NegativeAckCode};
    use std::mem::size_of;

    pub const GENERIC_HEADER_NACK_ZEROED: GenericDoIpHeaderNegativeAcknowledge =
        GenericDoIpHeaderNegativeAcknowledge {
            nack_code: NegativeAckCode::IncorrectPatternFormat,
        };

    pub const GENERIC_HEADER_NACK_DEFAULT_SIZE: usize = size_of::<u8>();
}

impl From<NegativeAckCode> for u8 {
    fn from(value: NegativeAckCode) -> Self {
        use NegativeAckCode::*;
        match value {
            IncorrectPatternFormat => 0,
            UnknownPayloadType => 1,
            MessageTooLarge => 2,
            OutOfMemory => 3,
            InvalidPayloadLength => 4,
            Reserved(value) => value,
        }
    }
}

impl From<u8> for NegativeAckCode {
    fn from(value: u8) -> Self {
        use NegativeAckCode::*;
        match value {
            0 => IncorrectPatternFormat,
            1 => UnknownPayloadType,
            2 => MessageTooLarge,
            3 => OutOfMemory,
            4 => InvalidPayloadLength,
            5..=0xff => Reserved(value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::tests::*;
    use super::*;
    #[test]
    fn generic_header_nack() {
        let payload = GenericDoIpHeaderNegativeAcknowledge {
            nack_code: NegativeAckCode::OutOfMemory,
        };
        let v = vec![
            0x02, 0xfd, // Protocol version
            0x00, 0x00, // Payload type
            0x00, 0x00, 0x00, 0x01, // Payload length
            0x03,
        ];
        assert_encode(&payload, &v);
        assert_decode(&payload, &v);
    }
}
