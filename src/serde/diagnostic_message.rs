use crate::{proto::payload::BorrowedPayload, DoIpError, LogicalAddress, Payload, PayloadType};
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use crate::proto::diagnostic_message::*;

fn read_addrs<T: Read>(reader: &mut T) -> Result<(LogicalAddress, LogicalAddress), DoIpError> {
    let source_address = reader.read_u16::<BigEndian>()?;
    let target_address = reader.read_u16::<BigEndian>()?;
    Ok((source_address, target_address))
}

fn get_addrs(buffer: &[u8]) -> (LogicalAddress, LogicalAddress) {
    let source_address = BigEndian::read_u16(&buffer[0..2]);
    let target_address = BigEndian::read_u16(&buffer[2..4]);
    (source_address, target_address)
}

fn write_addrs<T: Write>(
    writer: &mut T,
    source_address: &LogicalAddress,
    target_address: &LogicalAddress,
) -> Result<(), DoIpError> {
    writer.write_u16::<BigEndian>(*source_address)?;
    writer.write_u16::<BigEndian>(*target_address)?;
    Ok(())
}

impl<'a> Payload for DiagnosticMessage<'a> {
    fn length(&self) -> usize {
        size::DIAGREQ_DEFAULT_SIZE + self.user_data.get_ref().len()
    }

    fn payload_type() -> PayloadType {
        PayloadType::DiagnosticMessage
    }

    fn read<T: Read>(
        reader: &mut T,
        payload_length: usize,
    ) -> Result<DiagnosticMessage<'a>, DoIpError> {
        let mut me = size::DIAGREQ_ZEROED;
        /*let mut me = Self {
            source_address: 0,
            target_address: 0,
            user_data: UdsBuffer::Owned(vec![]),
        };*/
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        use DoIpError::*;
        if payload_length < size::DIAGREQ_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: size::DIAGREQ_DEFAULT_SIZE as u32,
            });
        }
        (self.source_address, self.target_address) = read_addrs(reader)?;
        let user_data_len = payload_length - 4; // 4 == source + target address
        let buffer = match self.user_data {
            UdsBuffer::Borrowed(_) => Err(DoIpError::BufferTooSmall),
            UdsBuffer::Owned(ref mut buf) => Ok(buf),
        }?;
        buffer.resize(user_data_len, 0u8);
        reader.read_exact(buffer.as_mut())?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        write_addrs(writer, &self.source_address, &self.target_address)?;
        writer.write_all(self.user_data.get_ref())?;
        Ok(())
    }
}

impl<'a> BorrowedPayload<'a> for DiagnosticMessage<'a> {
    fn read_borrowed(payload: &'a [u8]) -> Result<Self, DoIpError> {
        use DoIpError::*;
        if payload.len() < size::DIAGREQ_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload.len() as u32,
                expected: size::DIAGREQ_DEFAULT_SIZE as u32,
            });
        }
        let (source_address, target_address) = get_addrs(payload);
        Ok(DiagnosticMessage {
            source_address,
            target_address,
            user_data: UdsBuffer::Borrowed(&payload[4..]),
        })
    }
}

impl<'a> Payload for DiagnosticMessagePositiveAck<'a> {
    fn length(&self) -> usize {
        size::DIAGRSPACK_DEFAULT_SIZE + self.previous_diagnostic_message_data.get_ref().len()
    }

    fn payload_type() -> PayloadType {
        PayloadType::DiagnosticMessagePositiveAcknowledgement
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        let mut me = size::DIAGRSPACK_ZEROES;
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        use DoIpError::*;
        if payload_length < size::DIAGRSPACK_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: size::DIAGRSPACK_DEFAULT_SIZE as u32,
            });
        }
        (self.source_address, self.target_address) = read_addrs(reader)?;
        let previous_diagnostic_message_data_len = payload_length - 5; // 5 == Length
        let buffer = match self.previous_diagnostic_message_data {
            UdsBuffer::Borrowed(_) => Err(DoIpError::BufferTooSmall),
            UdsBuffer::Owned(ref mut buf) => Ok(buf),
        }?;
        buffer.resize(previous_diagnostic_message_data_len, 0u8);
        let ack_code_raw = reader.read_u8()?;
        self.ack_code = DiagnosticMessagePositiveAckCode::from(ack_code_raw);
        reader.read_exact(buffer)?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        write_addrs(writer, &self.source_address, &self.target_address)?;
        writer.write_u8(self.ack_code.into())?;
        writer.write_all(self.previous_diagnostic_message_data.get_ref())?;
        Ok(())
    }
}

impl<'a> BorrowedPayload<'a> for DiagnosticMessagePositiveAck<'a> {
    fn read_borrowed(payload: &'a [u8]) -> Result<Self, DoIpError> {
        use DoIpError::*;
        if payload.len() < size::DIAGRSPACK_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload.len() as u32,
                expected: size::DIAGRSPACK_DEFAULT_SIZE as u32,
            });
        }
        let (source_address, target_address) = get_addrs(payload);
        let ack_code_raw = payload[4];
        let ack_code = DiagnosticMessagePositiveAckCode::from(ack_code_raw);
        Ok(DiagnosticMessagePositiveAck {
            source_address,
            target_address,
            ack_code,
            previous_diagnostic_message_data: UdsBuffer::Borrowed(&payload[5..]),
        })
    }
}

impl<'a> Payload for DiagnosticMessageNegativeAck<'a> {
    fn length(&self) -> usize {
        size::DIAGRSPNACK_DEFAULT_SIZE + self.previous_diagnostic_message_data.get_ref().len()
    }

    fn payload_type() -> PayloadType {
        PayloadType::DiagnosticMessageNegativeAcknowledgement
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, DoIpError> {
        let mut me = size::DIAGRSPNACK_ZEROES;
        me.read_replace(reader, payload_length)?;
        Ok(me)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), DoIpError> {
        (self.source_address, self.target_address) = read_addrs(reader)?;
        let previous_diagnostic_message_data_len = payload_length - 5; // 5 == Length
        let buffer = match self.previous_diagnostic_message_data {
            UdsBuffer::Borrowed(_) => Err(DoIpError::BufferTooSmall),
            UdsBuffer::Owned(ref mut buf) => Ok(buf),
        }?;
        buffer.resize(previous_diagnostic_message_data_len, 0u8);
        let ack_code_raw = reader.read_u8()?;
        self.ack_code = DiagnosticMessageNegativeAckCode::from(ack_code_raw);
        reader.read_exact(buffer)?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        write_addrs(writer, &self.source_address, &self.target_address)?;
        writer.write_u8(self.ack_code.into())?;
        writer.write_all(self.previous_diagnostic_message_data.get_ref())?;
        Ok(())
    }
}

impl<'a> BorrowedPayload<'a> for DiagnosticMessageNegativeAck<'a> {
    fn read_borrowed(payload: &'a [u8]) -> Result<Self, DoIpError> {
        use DoIpError::*;
        if payload.len() < size::DIAGRSPACK_DEFAULT_SIZE {
            return Err(PayloadLengthTooShort {
                value: payload.len() as u32,
                expected: size::DIAGRSPACK_DEFAULT_SIZE as u32,
            });
        }
        let (source_address, target_address) = get_addrs(payload);
        let ack_code_raw = payload[4];
        let ack_code = DiagnosticMessageNegativeAckCode::from(ack_code_raw);
        Ok(DiagnosticMessageNegativeAck {
            source_address,
            target_address,
            ack_code,
            previous_diagnostic_message_data: UdsBuffer::Borrowed(&payload[5..]),
        })
    }
}

mod size {
    use super::LogicalAddress;
    use super::{
        DiagnosticMessage, DiagnosticMessageNegativeAck, DiagnosticMessageNegativeAckCode,
        DiagnosticMessagePositiveAck, DiagnosticMessagePositiveAckCode, UdsBuffer,
    };
    use std::mem::size_of;

    pub const DIAGREQ_ZEROED: DiagnosticMessage = DiagnosticMessage {
        source_address: 0u16,
        target_address: 0u16,
        user_data: UdsBuffer::Owned(vec![]),
    };
    pub const DIAGREQ_DEFAULT_SIZE: usize =
        size_of::<LogicalAddress>() + size_of::<LogicalAddress>();

    pub const DIAGRSPACK_ZEROES: DiagnosticMessagePositiveAck = DiagnosticMessagePositiveAck {
        source_address: 0u16,
        target_address: 0u16,
        ack_code: DiagnosticMessagePositiveAckCode::RoutingConfirmationAck,
        previous_diagnostic_message_data: UdsBuffer::Owned(vec![]),
    };
    pub const DIAGRSPACK_DEFAULT_SIZE: usize =
        size_of::<LogicalAddress>() + size_of::<LogicalAddress>() + size_of::<u8>();

    #[allow(dead_code)]
    pub const DIAGRSPNACK_ZEROES: DiagnosticMessageNegativeAck = DiagnosticMessageNegativeAck {
        source_address: 0u16,
        target_address: 0u16,
        ack_code: DiagnosticMessageNegativeAckCode::InvalidSourceAddress,
        previous_diagnostic_message_data: UdsBuffer::Owned(vec![]),
    };
    pub const DIAGRSPNACK_DEFAULT_SIZE: usize =
        size_of::<LogicalAddress>() + size_of::<LogicalAddress>() + size_of::<u8>();
}

impl From<u8> for DiagnosticMessagePositiveAckCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => DiagnosticMessagePositiveAckCode::RoutingConfirmationAck,
            _ => DiagnosticMessagePositiveAckCode::Reserved(value),
        }
    }
}

impl From<u8> for DiagnosticMessageNegativeAckCode {
    fn from(value: u8) -> Self {
        use DiagnosticMessageNegativeAckCode::*;
        match value {
            0x02 => InvalidSourceAddress,
            0x03 => UnknownTargetAddress,
            0x04 => DiagnosticMessageTooLarge,
            0x05 => OutOfMemory,
            0x06 => TargetUnreachable,
            0x07 => UnknownNetwork,
            0x08 => TransportProtocolError,
            _ => Reserved(value),
        }
    }
}

impl From<DiagnosticMessagePositiveAckCode> for u8 {
    fn from(value: DiagnosticMessagePositiveAckCode) -> Self {
        use DiagnosticMessagePositiveAckCode::*;
        match value {
            RoutingConfirmationAck => 0,
            _ => 0xff,
        }
    }
}

impl From<DiagnosticMessageNegativeAckCode> for u8 {
    fn from(value: DiagnosticMessageNegativeAckCode) -> Self {
        use DiagnosticMessageNegativeAckCode::*;
        match value {
            InvalidSourceAddress => 0x02,
            UnknownTargetAddress => 0x03,
            DiagnosticMessageTooLarge => 0x04,
            OutOfMemory => 0x05,
            TargetUnreachable => 0x06,
            UnknownNetwork => 0x07,
            TransportProtocolError => 0x08,
            Reserved(value) => value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::tests::*;
    use super::*;

    #[test]
    fn diagnostic_message() {
        let payload = DiagnosticMessage {
            source_address: 0x0123,
            target_address: 0x00ed,
            user_data: UdsBuffer::Owned(vec![0x22, 0xf0, 0x12]),
        };
        let v = vec![
            0x02, 0xfd, // Protocol version
            0x80, 0x01, // Payload type
            0x00, 0x00, 0x00, 0x07, // Payload length
            0x01, 0x23, 0x00, 0xed, 0x22, 0xf0, 0x12,
        ];
        assert_encode(&payload, &v);
        assert_decode_no_length_change(&payload, &v);
    }

    #[test]
    fn diagnostic_message_positive_ack() {
        let payload = DiagnosticMessagePositiveAck {
            source_address: 0x0123,
            target_address: 0x00ed,
            ack_code: DiagnosticMessagePositiveAckCode::RoutingConfirmationAck,
            previous_diagnostic_message_data: UdsBuffer::Owned(vec![0x22, 0xf0, 0x12]),
        };
        let v = vec![
            0x02, 0xfd, // Protocol version
            0x80, 0x02, // Payload type
            0x00, 0x00, 0x00, 0x08, // Payload length
            0x01, 0x23, 0x00, 0xed, 0x00, 0x22, 0xf0, 0x12,
        ];
        assert_encode(&payload, &v);
        assert_decode_no_length_change(&payload, &v);
    }

    #[test]
    fn diagnostic_message_negative_ack() {
        let payload = DiagnosticMessageNegativeAck {
            source_address: 0x0123,
            target_address: 0x00ed,
            ack_code: DiagnosticMessageNegativeAckCode::OutOfMemory,
            previous_diagnostic_message_data: UdsBuffer::Owned(vec![0x22, 0xf0, 0x12]),
        };
        let v = vec![
            0x02, 0xfd, // Protocol version
            0x80, 0x03, // Payload type
            0x00, 0x00, 0x00, 0x08, // Payload length
            0x01, 0x23, 0x00, 0xed, 0x05, 0x22, 0xf0, 0x12,
        ];
        assert_encode(&payload, &v);
        assert_decode_no_length_change(&payload, &v);
    }
}
