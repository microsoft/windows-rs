use crate::bstring::*;
use crate::*;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IRestrictedErrorInfo {
    ptr: ComPtr<IRestrictedErrorInfo>,
}

impl IRestrictedErrorInfo {
    pub fn get_error_details(&self) -> Result<(ErrorCode, String)> {
        let mut fallback = BString::new();
        let mut message = BString::new();
        let mut unused = BString::new();
        let mut code = ErrorCode(0);

        unsafe {
            ((*(*(self.ptr.as_raw()))).get_error_details)(
                self.ptr.as_raw(),
                fallback.set_abi(),
                &mut code,
                message.set_abi(),
                unused.set_abi(),
            )
            .and_then(|| {
                let message = if !message.is_empty() {
                    message
                } else {
                    fallback
                };

                (code, message.into())
            })
        }
    }
}

unsafe impl ComInterface for IRestrictedErrorInfo {
    type VTable = abi_IRestrictedErrorInfo;

    fn iid() -> Guid {
        Guid::from_values(
            0x82BA_7092,
            0x4C88,
            0x427D,
            [0xA7, 0xBC, 0x16, 0xDD, 0x93, 0xFE, 0xB6, 0x7E],
        )
    }
}

#[repr(C)]
pub struct abi_IRestrictedErrorInfo {
    __base: [usize; 3],
    get_error_details: extern "system" fn(
        RawComPtr<IRestrictedErrorInfo>,
        *mut RawPtr,
        *mut ErrorCode,
        *mut RawPtr,
        *mut RawPtr,
    ) -> ErrorCode,
}
