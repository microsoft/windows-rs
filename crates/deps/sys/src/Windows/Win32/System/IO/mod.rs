#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BindIoCompletionCallback();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelIo();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelIoEx();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelSynchronousIo();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateIoCompletionPort();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeviceIoControl();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOverlappedResult();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOverlappedResultEx();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetQueuedCompletionStatus();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetQueuedCompletionStatusEx();
    #[doc = "*Required features: `Win32_System_IO`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PostQueuedCompletionStatus();
}
