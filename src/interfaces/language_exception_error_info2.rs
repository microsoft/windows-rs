use crate::*;

#[repr(transparent)]
#[derive(Clone)]
pub struct ILanguageExceptionErrorInfo2(IUnknown);

#[repr(C)]
pub struct ILanguageExceptionErrorInfo2_vtable(
    pub IUnknown_QueryInterface,
    pub IUnknown_AddRef,
    pub IUnknown_Release,
    pub extern "system" fn(this: RawComPtr, exception: *mut RawPtr) -> ErrorCode, // GetLanguageException
    pub extern "system" fn(this: RawComPtr, previous: *mut RawPtr) -> ErrorCode, // GetPreviousLanguageExceptionErrorInfo
    pub extern "system" fn(this: RawComPtr, exception: RawPtr) -> ErrorCode, // CapturePropagationContext
    pub extern "system" fn(this: RawComPtr, head: *mut RawPtr) -> ErrorCode, // GetPropagationContextHead
);

unsafe impl ComInterface for ILanguageExceptionErrorInfo2 {
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

unsafe impl GetAbi for ILanguageExceptionErrorInfo2 {
    type Abi = RawComPtr;

    unsafe fn get_abi(&self) -> RawComPtr {
        self.0.get_abi()
    }
}

impl ILanguageExceptionErrorInfo2 {
    pub fn capture_propagation_context(&self) {
        unsafe {
            (self.vtable().5)(self.get_abi(), std::ptr::null_mut());
        }
    }
}
