use crate::*;

#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct IRestrictedErrorInfo(IUnknown);

#[repr(C)]
pub struct IRestrictedErrorInfo_vtable(
    pub IUnknown_QueryInterface,
    pub IUnknown_AddRef,
    pub IUnknown_Release,
    pub  extern "system" fn(
        this: RawPtr,
        description: *mut RawPtr,
        error: *mut ErrorCode,
        restricted: *mut RawPtr,
        sid: *mut RawPtr,
    ) -> ErrorCode, // GetErrorDetails
    pub extern "system" fn(this: RawPtr, reference: *mut RawPtr) -> ErrorCode, // GetReference
);

unsafe impl Interface for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_vtable;

    const IID: Guid = {
        Guid::from_values(
            0x82BA_7092,
            0x4C88,
            0x427D,
            [0xA7, 0xBC, 0x16, 0xDD, 0x93, 0xFE, 0xB6, 0x7E],
        )
    };
}

impl std::fmt::Debug for IRestrictedErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl IRestrictedErrorInfo {
    pub fn from_thread() -> Result<Self> {
        let result = IErrorInfo::from_thread();

        match result {
            Ok(result) => result.query::<Self>(),
            Err(err) => Err(err),
        }
    }

    pub fn details(&self) -> (ErrorCode, String) {
        let mut fallback = BString::new();
        let mut message = BString::new();
        let mut unused = BString::new();
        let mut code = ErrorCode(0);

        unsafe {
            (self.vtable().3)(
                self.get_abi(),
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
