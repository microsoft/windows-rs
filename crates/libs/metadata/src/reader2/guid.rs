#![allow(clippy::many_single_char_names)]

use super::*;

#[derive(Clone, PartialEq, Default)]
pub struct GUID(pub u32, pub u16, pub u16, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

impl GUID {
    pub fn from_args(args: &[(String, Value)]) -> Self {
        fn unwrap_u32(value: &Value) -> u32 {
            match value {
                Value::U32(value) => *value,
                _ => unimplemented!(),
            }
        }
        fn unwrap_u16(value: &Value) -> u16 {
            match value {
                Value::U16(value) => *value,
                _ => unimplemented!(),
            }
        }
        fn unwrap_u8(value: &Value) -> u8 {
            match value {
                Value::U8(value) => *value,
                _ => unimplemented!(),
            }
        }
        Self(unwrap_u32(&args[0].1), unwrap_u16(&args[1].1), unwrap_u16(&args[2].1), unwrap_u8(&args[3].1), unwrap_u8(&args[4].1), unwrap_u8(&args[5].1), unwrap_u8(&args[6].1), unwrap_u8(&args[7].1), unwrap_u8(&args[8].1), unwrap_u8(&args[9].1), unwrap_u8(&args[10].1))
    }
}

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:08x?}-{:04x?}-{:04x?}-{:02x?}{:02x?}-{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}", self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
