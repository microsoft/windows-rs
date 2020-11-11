use crate::*;

#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct ILanguageExceptionErrorInfo2(IUnknown);

// TODO: add ILanguageExceptionErrorInfo

#[repr(C)]
pub struct ILanguageExceptionErrorInfo2_vtable(
    usize,
    usize,
    usize,
    extern "system" fn(this: RawPtr, exception: *mut RawPtr) -> ErrorCode, // GetLanguageException
    extern "system" fn(this: RawPtr, previous: *mut RawPtr) -> ErrorCode, // GetPreviousLanguageExceptionErrorInfo
    extern "system" fn(this: RawPtr, exception: RawPtr) -> ErrorCode, // CapturePropagationContext
    extern "system" fn(this: RawPtr, head: *mut RawPtr) -> ErrorCode, // GetPropagationContextHead
);

unsafe impl Interface for ILanguageExceptionErrorInfo2 {
    type Vtable = ILanguageExceptionErrorInfo2_vtable;

    const IID: Guid = {
        Guid::from_values(
            0x5746_E5C4,
            0x5B97,
            0x424C,
            [0xB6, 0x20, 0x28, 0x22, 0x91, 0x57, 0x34, 0xDD],
        )
    };
}

impl std::fmt::Debug for ILanguageExceptionErrorInfo2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl ILanguageExceptionErrorInfo2 {
    pub fn capture_propagation_context(&self) {
        unsafe {
            (self.vtable().5)(self.get_abi(), std::ptr::null_mut());
        }
    }
}
