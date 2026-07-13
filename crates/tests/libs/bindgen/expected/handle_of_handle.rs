pub type AliasHandle = Handle;
pub const FLAG: Grbit = Grbit(1);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Grbit(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Handle(pub *mut core::ffi::c_void);
impl Default for Handle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INVALID: Handle = Handle(-1 as _);
pub type PGrbit = *mut Grbit;
pub type PHandle = *mut Handle;
