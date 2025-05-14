use super::*;

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

pub trait IntoStream {
    fn into_stream(self) -> Vec<u8>;
}

impl IntoStream for Vec<u8> {
    fn into_stream(mut self) -> Self {
        self.resize(round(self.len(), 4), 0);
        self
    }
}

pub trait Write {
    fn write_header<T: Sized>(&mut self, value: &T);
    fn write_u16(&mut self, value: u16);
    fn write_u32(&mut self, value: u32);
    fn write_u64(&mut self, value: u64);
    fn write_code(&mut self, value: u32, size: usize);
    fn write_index(&mut self, index: u32, len: usize);
    fn write_compressed(&mut self, value: usize);
    fn write_value(&mut self, value: &Value);
}

impl Write for Vec<u8> {
    fn write_header<T: Sized>(&mut self, value: &T) {
        unsafe {
            self.extend_from_slice(std::slice::from_raw_parts(
                value as *const _ as _,
                core::mem::size_of::<T>(),
            ));
        }
    }

    fn write_u16(&mut self, value: u16) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u32(&mut self, value: u32) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u64(&mut self, value: u64) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_code(&mut self, value: u32, size: usize) {
        if size == 2 {
            self.write_u16(value as u16);
        } else {
            self.write_u32(value);
        }
    }

    fn write_index(&mut self, index: u32, len: usize) {
        if len < (1 << 16) {
            self.write_u16(index as u16 + 1);
        } else {
            self.write_u32(index + 1);
        }
    }

    fn write_compressed(&mut self, value: usize) {
        // See II.23.2 in ECMA-335
        assert!(value < 0x20000000);

        if value < 0x80 {
            self.push(value as u8);
        } else if value < 0x4000 {
            self.push((0x80 | ((value & 0x3F00) >> 8)) as u8);
            self.push((value & 0xFF) as u8);
        } else {
            self.push((0xC0 | ((value & 0x1F000000) >> 24)) as u8);
            self.push(((value & 0xFF0000) >> 16) as u8);
            self.push(((value & 0xFF00) >> 8) as u8);
            self.push((value & 0xFF) as u8);
        }
    }

    fn write_value(&mut self, value: &Value) {
        match value {
            Value::Bool(value) => {
                if *value {
                    self.push(1)
                } else {
                    self.push(0)
                }
            }
            Value::U8(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::I8(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::U16(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::I16(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::U32(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::I32(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::U64(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::I64(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::F32(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::F64(value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::AttributeEnum(_, value) => self.extend_from_slice(&value.to_le_bytes()),
            Value::Utf8(value) => {
                self.write_compressed(value.len());
                self.extend_from_slice(value.as_bytes());
            }
            Value::Utf16(value) => {
                self.extend(value.encode_utf16().flat_map(|value| value.to_le_bytes()));
            }
        }
    }
}

pub trait PushPos<T> {
    fn push_pos(&mut self, value: T) -> u32;
}

impl<T> PushPos<T> for Vec<T> {
    fn push_pos(&mut self, value: T) -> u32 {
        self.push(value);
        (self.len() - 1).try_into().unwrap()
    }
}
