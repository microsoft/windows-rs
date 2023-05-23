#![allow(dead_code)]

use std::collections::*;
mod attributes;
mod bindings;
mod error;
mod filter;
mod imp;
pub mod reader;
pub mod writer;

pub use attributes::*;
use bindings::*;
pub use error::*;
pub use filter::*;
use imp::*;

macro_rules! flags {
    ($name:ident, $size:ty) => {
        #[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
        pub struct $name(pub $size);
        impl $name {
            pub fn contains(&self, contains: Self) -> bool {
                *self & contains == contains
            }
        }
        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }
        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0.bitor_assign(other.0)
            }
        }
        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.0.bitand_assign(other.0)
            }
        }
        impl std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self {
                Self(self.0.not())
            }
        }
    };
}

pub(crate) use flags;

#[derive(Copy, Clone, Debug)]
pub enum Integer {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
}
