#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct FILE {
    pub _Placeholder: *mut core::ffi::c_void,
}
