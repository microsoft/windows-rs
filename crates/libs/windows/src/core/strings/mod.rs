mod hstring;
mod literals;
mod pcstr;
mod pcwstr;
mod pstr;
mod pwstr;

pub use hstring::*;
#[doc(hidden)]
pub use literals::*;
pub use pcstr::*;
pub use pcwstr::*;
pub use pstr::*;
pub use pwstr::*;

use super::*;

extern "C" {
    pub fn strlen(s: PCSTR) -> usize;
    pub fn wcslen(s: PCWSTR) -> usize;
}
