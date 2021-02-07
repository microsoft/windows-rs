use super::*;
use std::convert::TryInto;

pub struct Blob {
    pub reader: &'static TypeReader,
    pub file_index: u16,
    pub offset: usize,
    pub size: usize,
}

impl Blob {
    fn bytes(&self) -> &'static [u8] {
        &self.reader.files[self.file_index as usize].bytes[self.offset..]
    }

    pub fn peek_unsigned(&self) -> (u32, usize) {
        let bytes = self.bytes();

        if bytes[0] & 0x80 == 0 {
            (bytes[0] as u32, 1)
        } else if bytes[0] & 0xC0 == 0x80 {
            ((((bytes[0] & 0x3F) as u32) << 8) | bytes[1] as u32, 2)
        } else {
            (
                (((bytes[0] & 0x1F) as u32) << 24)
                    | (bytes[1] as u32) << 16
                    | (bytes[2] as u32) << 8
                    | bytes[3] as u32,
                4,
            )
        }
    }

    pub fn read_unsigned(&mut self) -> u32 {
        let (value, offset) = self.peek_unsigned();
        self.offset += offset;
        value
    }

    pub fn read_expected(&mut self, expected: u32) -> bool {
        let (value, offset) = self.peek_unsigned();
        if value == expected {
            self.offset += offset;
            true
        } else {
            false
        }
    }

    pub fn read_modifiers(&mut self) -> Vec<TypeDefOrRef> {
        let mut mods = vec![];

        loop {
            let (value, offset) = self.peek_unsigned();
            if value != 32 && value != 31 {
                break;
            } else {
                self.offset += offset;
                mods.push(TypeDefOrRef::decode(
                    self.reader,
                    self.read_unsigned(),
                    self.file_index,
                ))
            }
        }

        mods
    }

    pub fn read_str(&mut self) -> &'static str {
        let len = self.read_unsigned() as usize;
        self.offset += len;
        std::str::from_utf8(
            &self.reader.files[self.file_index as usize].bytes[self.offset - len..self.offset],
        )
        .unwrap()
    }

    pub fn read_utf16(&self) -> String {
        let bytes = &self.reader.files[self.file_index as usize].bytes[self.offset..];
        if bytes.as_ptr().align_offset(std::mem::align_of::<u16>()) > 0 {
            let bytes = bytes
                .chunks_exact(2)
                .take(self.size / 2)
                .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
                .collect::<Vec<u16>>();
            String::from_utf16(&bytes).unwrap()
        } else {
            assert!(
                bytes.len() >= self.size,
                "Attempt to read from end of memory"
            );
            let bytes =
                unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u16, self.size / 2) };
            String::from_utf16(bytes).unwrap()
        }
    }

    pub fn read_i8(&mut self) -> i8 {
        let value = i8::from_le_bytes(self.bytes()[..1].try_into().unwrap());
        self.offset += 1;
        value
    }

    pub fn read_u8(&mut self) -> u8 {
        let value = u8::from_le_bytes(self.bytes()[..1].try_into().unwrap());
        self.offset += 1;
        value
    }

    pub fn read_i16(&mut self) -> i16 {
        let value = i16::from_le_bytes(self.bytes()[..2].try_into().unwrap());
        self.offset += 2;
        value
    }

    pub fn read_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes(self.bytes()[..2].try_into().unwrap());
        self.offset += 2;
        value
    }

    pub fn read_i32(&mut self) -> i32 {
        let value = i32::from_le_bytes(self.bytes()[..4].try_into().unwrap());
        self.offset += 4;
        value
    }

    pub fn read_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes(self.bytes()[..4].try_into().unwrap());
        self.offset += 4;
        value
    }

    pub fn read_i64(&mut self) -> i64 {
        let value = i64::from_le_bytes(self.bytes()[..8].try_into().unwrap());
        self.offset += 8;
        value
    }

    pub fn read_u64(&mut self) -> u64 {
        let value = u64::from_le_bytes(self.bytes()[..8].try_into().unwrap());
        self.offset += 8;
        value
    }

    pub fn read_f32(&mut self) -> f32 {
        let value = f32::from_le_bytes(self.bytes()[..4].try_into().unwrap());
        self.offset += 4;
        value
    }

    pub fn read_f64(&mut self) -> f64 {
        let value = f64::from_le_bytes(self.bytes()[..8].try_into().unwrap());
        self.offset += 8;
        value
    }
}
