use crate::Error;

/// A checked cursor over one metadata blob.
pub struct BlobReader<'a> {
    bytes: &'a [u8],
    offset: usize,
    position: usize,
}

impl<'a> BlobReader<'a> {
    pub(crate) const fn new(bytes: &'a [u8], offset: usize) -> Self {
        Self {
            bytes,
            offset,
            position: 0,
        }
    }

    /// Returns the absolute offset of the next byte.
    pub const fn offset(&self) -> usize {
        self.offset + self.position
    }

    /// Returns the number of unread bytes.
    pub const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }

    /// Returns the next byte without consuming it.
    pub fn peek_u8(&self) -> Result<u8, Error> {
        self.bytes
            .get(self.position)
            .copied()
            .ok_or_else(|| Error::invalid(self.offset(), "unexpected end of blob"))
    }

    /// Reads one byte.
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        let value = self.peek_u8()?;
        self.position += 1;
        Ok(value)
    }

    /// Reads a little-endian 16-bit integer.
    pub fn read_u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian 32-bit integer.
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian 64-bit integer.
    pub fn read_u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.read_array()?))
    }

    /// Reads a canonical ECMA-335 compressed unsigned integer.
    pub fn read_compressed_u32(&mut self) -> Result<u32, Error> {
        let start = self.offset();
        let first = self.read_u8()?;
        match first {
            0x00..=0x7f => Ok(first as u32),
            0x80..=0xbf => {
                let value = ((first as u32 & 0x3f) << 8) | self.read_u8()? as u32;
                if value < 0x80 {
                    Err(Error::invalid(start, "noncanonical compressed integer"))
                } else {
                    Ok(value)
                }
            }
            0xc0..=0xdf => {
                let value = ((first as u32 & 0x1f) << 24)
                    | (self.read_u8()? as u32) << 16
                    | (self.read_u8()? as u32) << 8
                    | self.read_u8()? as u32;
                if value < 0x4000 {
                    Err(Error::invalid(start, "noncanonical compressed integer"))
                } else {
                    Ok(value)
                }
            }
            _ => Err(Error::invalid(start, "invalid compressed integer")),
        }
    }

    /// Reads an ECMA-335 compressed signed integer.
    pub fn read_compressed_i32(&mut self) -> Result<i32, Error> {
        let start = self.position;
        let value = self.read_compressed_u32()?;
        let width = self.position - start;
        let payload_bits = match width {
            1 => 7,
            2 => 14,
            4 => 29,
            _ => unreachable!(),
        };
        let signed = (value >> 1) as i32;
        if value & 1 == 0 {
            Ok(signed)
        } else {
            Ok(signed | (!0 << (payload_bits - 1)))
        }
    }

    /// Reads exactly `length` bytes.
    pub fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], Error> {
        let start = self.position;
        let end = start
            .checked_add(length)
            .ok_or_else(|| Error::invalid(self.offset(), "blob length overflow"))?;
        let bytes = self
            .bytes
            .get(start..end)
            .ok_or_else(|| Error::invalid(self.offset(), "unexpected end of blob"))?;
        self.position = end;
        Ok(bytes)
    }

    /// Requires that the complete blob has been consumed.
    pub fn finish(self) -> Result<(), Error> {
        if self.remaining() == 0 {
            Ok(())
        } else {
            Err(Error::invalid(self.offset(), "trailing bytes in blob"))
        }
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        Ok(self.read_bytes(N)?.try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reader(bytes: &[u8]) -> BlobReader<'_> {
        BlobReader::new(bytes, 100)
    }

    #[test]
    fn compressed_signed_boundaries() {
        assert_eq!(reader(&[0x00]).read_compressed_i32().unwrap(), 0);
        assert_eq!(reader(&[0x02]).read_compressed_i32().unwrap(), 1);
        assert_eq!(reader(&[0x7f]).read_compressed_i32().unwrap(), -1);
        assert_eq!(reader(&[0x7b]).read_compressed_i32().unwrap(), -3);
        assert_eq!(reader(&[0x80, 0x80]).read_compressed_i32().unwrap(), 64);
    }
}
