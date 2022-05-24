use super::*;
use std::convert::*;

pub struct Blob<'a> {
    pub file: usize,
    pub slice: &'a [u8],
}

impl<'a> std::ops::Deref for Blob<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.slice
    }
}

impl<'a> Blob<'a> {
    pub fn new(file: usize, slice: &'a [u8]) -> Self {
        Self { file, slice }
    }
    pub fn peek_usize(&self) -> (usize, usize) {
        if self[0] & 0x80 == 0 {
            (self[0] as usize, 1)
        } else if self[0] & 0xC0 == 0x80 {
            ((((self[0] & 0x3F) as usize) << 8) | self[1] as usize, 2)
        } else {
            ((((self[0] & 0x1F) as usize) << 24) | (self[1] as usize) << 16 | (self[2] as usize) << 8 | self[3] as usize, 4)
        }
    }
    pub fn read_usize(&mut self) -> usize {
        let (value, offset) = self.peek_usize();
        self.offset(offset);
        value
    }
    pub fn read_expected(&mut self, expected: usize) -> bool {
        let (value, offset) = self.peek_usize();
        if value == expected {
            self.offset(offset);
            true
        } else {
            false
        }
    }
    pub fn read_modifiers(&mut self) -> Vec<TypeDefOrRef> {
        let mut mods = vec![];
        loop {
            let (value, offset) = self.peek_usize();
            if value != 32 && value != 31 {
                break;
            } else {
                self.offset(offset);
                mods.push(TypeDefOrRef::decode(self.file, self.read_usize()))
            }
        }
        mods
    }
    pub fn read_str(&mut self) -> &str {
        let len = self.read_usize();
        let value = unsafe { std::str::from_utf8_unchecked(&self.slice[..len]) };
        self.offset(len);
        value
    }
    pub fn read_string(self) -> String {
        let slice = self.slice;
        if slice.as_ptr().align_offset(core::mem::align_of::<u16>()) > 0 {
            let slice = slice.chunks_exact(2).take(slice.len() / 2).map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap())).collect::<Vec<u16>>();
            String::from_utf16_lossy(&slice)
        } else {
            let slice = unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u16, slice.len() / 2) };
            String::from_utf16_lossy(slice)
        }
    }
    pub fn read_i8(&mut self) -> i8 {
        let value = i8::from_le_bytes(self[..1].try_into().unwrap());
        self.offset(1);
        value
    }
    pub fn read_u8(&mut self) -> u8 {
        let value = u8::from_le_bytes(self[..1].try_into().unwrap());
        self.offset(1);
        value
    }
    pub fn read_i16(&mut self) -> i16 {
        let value = i16::from_le_bytes(self[..2].try_into().unwrap());
        self.offset(2);
        value
    }
    pub fn read_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes(self[..2].try_into().unwrap());
        self.offset(2);
        value
    }
    pub fn read_i32(&mut self) -> i32 {
        let value = i32::from_le_bytes(self[..4].try_into().unwrap());
        self.offset(4);
        value
    }
    pub fn read_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes(self[..4].try_into().unwrap());
        self.offset(4);
        value
    }
    pub fn read_i64(&mut self) -> i64 {
        let value = i64::from_le_bytes(self[..8].try_into().unwrap());
        self.offset(8);
        value
    }
    pub fn read_u64(&mut self) -> u64 {
        let value = u64::from_le_bytes(self[..8].try_into().unwrap());
        self.offset(8);
        value
    }
    pub fn read_f32(&mut self) -> f32 {
        let value = f32::from_le_bytes(self[..4].try_into().unwrap());
        self.offset(4);
        value
    }
    pub fn read_f64(&mut self) -> f64 {
        let value = f64::from_le_bytes(self[..8].try_into().unwrap());
        self.offset(8);
        value
    }
    fn offset(&mut self, offset: usize) {
        self.slice = &self.slice[offset..];
    }
}
