use crate::*;

/// Provides detailed error information. `IRestrictedErrorInfo` represents the
/// [IRestrictedErrorInfo](https://docs.microsoft.com/en-us/windows/win32/api/restrictederrorinfo/nn-restrictederrorinfo-irestrictederrorinfo)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IRestrictedErrorInfo(IUnknown);

#[repr(C)]
pub struct IRestrictedErrorInfo_vtable(
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> ErrorCode,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: RawPtr,
        description: *mut RawPtr,
        error: *mut ErrorCode,
        restricted: *mut RawPtr,
        sid: *mut RawPtr,
    ) -> ErrorCode, // GetErrorDetails
    pub unsafe extern "system" fn(this: RawPtr, reference: *mut RawPtr) -> ErrorCode, // GetReference
);

impl IRestrictedErrorInfo {
    /// Retrieves any error information stored on the calling thread. An error code indicates the
    /// absence of error information.
    pub fn from_thread() -> Result<Self> {
        IErrorInfo::from_thread().and_then(|e| e.cast())
    }

    /// Gets the error code and description of the error.
    pub fn details(&self) -> (ErrorCode, String) {
        let mut fallback = BString::new();
        let mut message = BString::new();
        let mut unused = BString::new();
        let mut code = ErrorCode(0);

        unsafe {
            (self.vtable().3)(
                self.abi(),
                fallback.set_abi(),
                &mut code,
                message.set_abi(),
                unused.set_abi(),
            );
        }

        let message = if !message.is_empty() {
            message
        } else {
            fallback
        };

        (code, message.into())
    }
}

unsafe impl Interface for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_vtable;

    const IID: Guid = Guid::from_values(
        0x82BA_7092,
        0x4C88,
        0x427D,
        [0xA7, 0xBC, 0x16, 0xDD, 0x93, 0xFE, 0xB6, 0x7E],
    );
}

impl std::fmt::Debug for IRestrictedErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
