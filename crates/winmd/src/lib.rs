//! A Windows Metadata (winmd) parser
mod file;
pub mod parsed;
mod traits;
mod type_reader;
mod windows;

pub use file::{File, TableIndex};
pub use parsed::*;
pub use traits::*;
pub use type_reader::TypeReader;
pub use windows::*;
