use crate::file::WinmdFile;
use crate::TypeReader;

use std::convert::TryInto;

pub struct Blob<'a> {
    pub reader: &'a TypeReader,
    pub file_index: u16,
    offset: usize,
}

impl<'a> Blob<'a> {
    pub fn new(reader: &'a TypeReader, file_index: u16, offset: usize) -> Self {
        Blob {
            reader,
            file_index,
            offset,
        }
    }

    fn bytes(&self) -> &[u8] {
        &self.file().bytes[self.offset..]
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

    pub fn read_modifiers(&mut self) {
        loop {
            let (value, offset) = self.peek_unsigned();
            if value != 32 && value != 31 {
                break;
            } else {
                self.offset += offset;
                self.read_unsigned();
            }
        }
    }

    pub fn read_str(&mut self) -> &str {
        let len = self.read_unsigned() as usize;
        self.offset += len;
        // TODO: for some reason we can't read using bytes() and then adjust the offset because it trips
        // over reference lifetimes
        std::str::from_utf8(&self.file().bytes[self.offset - len..self.offset]).unwrap()
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

    fn file(&self) -> &WinmdFile {
        &self.reader.files[self.file_index as usize]
    }
}
