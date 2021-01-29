use crate::*;

/// A WinRT error object consists of both an error code as well as detailed error information for debugging.
#[derive(Clone, PartialEq)]
pub struct Error {
    code: ErrorCode,
    info: Option<IRestrictedErrorInfo>,
}

impl Error {
    /// This creates a new WinRT error object, capturing the stack and other information about the
    /// point of failure.
    pub fn new(code: ErrorCode, message: &str) -> Self {
        let message: HString = message.into();

        // RoOriginateError creates the error object and associates it with the thread.
        // Need to ignore the result, as that is the delay-load error, which would mean
        // that there's no WinRT to tell about the error.
        unsafe {
            let _ = RoOriginateError(code, message.abi() as _);
        }

        // The error information is then associated with the returning error object and no longer
        // associated with the thread.
        Self {
            code,
            info: IRestrictedErrorInfo::from_thread().ok(),
        }
    }

    // Use internally to create an Error object without error info. Typically used with QueryInterface
    // (E_NOINTERFACE) or the absense of an object to return (E_POINTER) to avoid the code gen overhead
    // for casts that should be cheap (few instructions). Think of it as a way to create recoverable errors
    // that don't need th overhead of debugging origination info.
    pub fn fast_error(code: ErrorCode) -> Self {
        Self { code, info: None }
    }

    /// The error code describing the error.
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    /// The error information describing the error.
    pub fn info(&self) -> &Option<IRestrictedErrorInfo> {
        &self.info
    }

    /// The error message describing the error.
    pub fn message(&self) -> String {
        // First attempt to retrieve the restricted error information.
        if let Some(info) = &self.info {
            let (code, message) = info.details();

            if self.code == code {
                return message.trim_end().to_owned();
            }
        }

        const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x0000_0100;
        const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x0000_1000;
        const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x0000_0200;
        let mut message = HeapString(std::ptr::null_mut());

        // If that fails simply ask for the generic formatted message for the error code.
        unsafe {
            let size = FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                std::ptr::null_mut(),
                self.code,
                0x0000_0400, // MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT)
                &mut message.0,
                0,
                std::ptr::null_mut(),
            );

            String::from_utf16_lossy(std::slice::from_raw_parts(
                message.0 as *const u16,
                size as usize,
            ))
            .trim_end()
            .to_owned()
        }
    }
}

impl std::convert::From<Error> for ErrorCode {
    fn from(error: Error) -> Self {
        let code = error.code;
        let info = error.info.and_then(|info| info.cast().ok());

        unsafe {
            let _ = SetErrorInfo(0, info);
        }

        code
    }
}

impl std::convert::From<ErrorCode> for Error {
    fn from(code: ErrorCode) -> Self {
        if let Ok(info) = IRestrictedErrorInfo::from_thread() {
            // If it does (and therefore running on a recent version of Windows)
            // then capture_propagation_context adds a breadcrumb to the error
            // info to make debugging easier.
            if let Ok(capture) = info.cast::<ILanguageExceptionErrorInfo2>() {
                capture.capture_propagation_context();
            }

            return Self {
                code,
                info: Some(info),
            };
        }

        if let Ok(info) = IErrorInfo::from_thread() {
            Self::new(code, &info.description())
        } else {
            Self::new(code, "")
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Error")
            .field("code", &format_args!("{:#010X}", self.code.0))
            .field("message", &self.message())
            .finish()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&self.message())
    }
}

impl std::error::Error for Error {}

pub struct HeapString(RawPtr);

impl Drop for HeapString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                heap_free(self.0);
            }
        }
    }
}

#[link(name = "kernel32")]
extern "system" {
    fn FormatMessageW(
        flags: u32,
        source: RawPtr,
        code: ErrorCode,
        language: u32,
        buffer: *mut RawPtr,
        size: u32,
        args: RawPtr,
    ) -> u32;
}

#[link(name = "oleaut32")]
extern "system" {
    fn SetErrorInfo(reserved: u32, info: Option<IErrorInfo>) -> ErrorCode;
}

demand_load! {
    "combase.dll" {
        fn RoOriginateError(code: ErrorCode, message: RawPtr) -> i32;
    }
}
