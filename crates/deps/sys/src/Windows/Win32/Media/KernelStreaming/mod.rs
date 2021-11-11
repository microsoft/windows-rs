#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateAllocator(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateAllocator2(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateClock(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateClock2(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreatePin(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreatePin2(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateTopologyNode(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateTopologyNode2(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
}
