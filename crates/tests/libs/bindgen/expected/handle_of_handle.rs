pub type AliasHandle = Handle;
pub const FLAG: Grbit = 1;
pub type Grbit = u32;
pub type Handle = *mut core::ffi::c_void;
pub const INVALID: Handle = -1 as _;
pub type PGrbit = *mut Grbit;
pub type PHandle = *mut Handle;
