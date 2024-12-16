use crate::DoIpError;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use crate::{DoIpHeader, PayloadType};

/// DoIP header
///
/// This is the first part of a DoIP message, always followed by a DoIP payload.
/// The header has always a fixed number of bytes.
impl DoIpHeader {
    /// Read a DoIP header from the reader.
    pub fn read<T: Read>(reader: &mut T) -> Result<Self, DoIpError> {
        let protocol_version = reader.read_u8()?;
        let inverse_protocol_version = reader.read_u8()?;
        let payload_type_bytes = reader.read_u16::<BigEndian>()?;
        let payload_type = PayloadType::from(payload_type_bytes);
        let payload_length = reader.read_u32::<BigEndian>()?;

        Ok(DoIpHeader {
            protocol_version,
            inverse_protocol_version,
            payload_type,
            payload_length,
        })
    }
    /// Writes a DoIP header to the writer.
    pub fn write<T: Write>(&self, writer: &mut T) -> Result<(), DoIpError> {
        writer.write_u8(self.protocol_version)?;
        writer.write_u8(self.inverse_protocol_version)?;
        writer.write_u16::<BigEndian>(self.payload_type.into_u16())?;
        writer.write_u32::<BigEndian>(self.payload_length)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{message::ProtocolVersion, DOIP_HEADER_LENGTH};

    #[test]
    fn test_serialize() {
        use std::io::Cursor;
        let mut buff = Cursor::new(vec![0; DOIP_HEADER_LENGTH]);
        let header = DoIpHeader::new(PayloadType::RoutingActivationRequest, 11);
        let res = header.write(&mut buff);
        assert!(res.is_ok());
        let expected = vec![0x02u8, 0xfd, 0x00, 0x05, 0x00, 0x00, 0x00, 0x0b];
        assert_eq!(&expected, buff.get_ref());
    }

    #[test]
    fn test_derialize() {
        use std::io::Cursor;
        let mut buff = Cursor::new(vec![0x02u8, 0xfd, 0x00, 0x05, 0x00, 0x00, 0x00, 0x0b]);
        let header = DoIpHeader::read(&mut buff).unwrap();
        assert_eq!(header.protocol_version, ProtocolVersion::DoIpIso as u8);
        assert_eq!(header.payload_type, PayloadType::RoutingActivationRequest);
        assert_eq!(header.payload_length, 11u32);
    }
}
