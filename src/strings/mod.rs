mod costring;
mod hstring;

pub use costring::*;
pub use hstring::*;

// TODO: add PCWSTR and PCSTR macros for creating null terminated string literals.
// These should automatically bind to PWSTR and PSTR input parameters.
