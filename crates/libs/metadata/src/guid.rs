#![allow(clippy::many_single_char_names)]

use super::*;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct GUID(pub u32, pub u16, pub u16, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

impl GUID {
    pub fn from_args(args: &[(String, Value)]) -> Self {
        fn unwrap_u32(value: &Value) -> u32 {
            match value {
                Value::U32(value) => *value,
                rest => unimplemented!("{rest:?}"),
            }
        }
        fn unwrap_u16(value: &Value) -> u16 {
            match value {
                Value::U16(value) => *value,
                rest => unimplemented!("{rest:?}"),
            }
        }
        fn unwrap_u8(value: &Value) -> u8 {
            match value {
                Value::U8(value) => *value,
                rest => unimplemented!("{rest:?}"),
            }
        }
        Self(unwrap_u32(&args[0].1), unwrap_u16(&args[1].1), unwrap_u16(&args[2].1), unwrap_u8(&args[3].1), unwrap_u8(&args[4].1), unwrap_u8(&args[5].1), unwrap_u8(&args[6].1), unwrap_u8(&args[7].1), unwrap_u8(&args[8].1), unwrap_u8(&args[9].1), unwrap_u8(&args[10].1))
    }

    pub fn from_string_args(args: &[&str]) -> Self {
        Self(args[0].parse().unwrap(), args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap(), args[5].parse().unwrap(), args[6].parse().unwrap(), args[7].parse().unwrap(), args[8].parse().unwrap(), args[9].parse().unwrap(), args[10].parse().unwrap())
    }
}

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x?}-{:04x?}-{:04x?}-{:02x?}{:02x?}-{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}", self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
