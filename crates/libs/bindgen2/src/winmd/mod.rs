use super::*;

mod attributes;
mod codes;
mod file;
mod reader;
mod row;
mod signature;
mod tables;
mod value;

pub use attributes::*;
pub use codes::*;
pub use file::*;
pub use reader::*;
pub use row::*;
pub use signature::*;
pub use tables::*;
pub use value::*;

pub struct GUID(
    pub u32,
    pub u16,
    pub u16,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
);

impl std::fmt::Display for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x?}-{:04x?}-{:04x?}-{:02x?}{:02x?}-{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10
        )
    }
}
