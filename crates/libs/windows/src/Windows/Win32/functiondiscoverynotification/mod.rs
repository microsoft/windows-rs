#[cfg(feature = "Win32_functiondiscoveryapi")]
windows_core::imp::define_interface!(CFunctionDiscoveryNotificationWrapper, CFunctionDiscoveryNotificationWrapper_Vtbl, 0);
#[cfg(feature = "Win32_functiondiscoveryapi")]
impl core::ops::Deref for CFunctionDiscoveryNotificationWrapper {
    type Target = super::functiondiscoveryapi::IFunctionDiscoveryNotification;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_functiondiscoveryapi")]
windows_core::imp::interface_hierarchy!(CFunctionDiscoveryNotificationWrapper, windows_core::IUnknown, super::functiondiscoveryapi::IFunctionDiscoveryNotification);
#[cfg(feature = "Win32_functiondiscoveryapi")]
#[repr(C)]
#[doc(hidden)]
pub struct CFunctionDiscoveryNotificationWrapper_Vtbl {
    pub base__: super::functiondiscoveryapi::IFunctionDiscoveryNotification_Vtbl,
}
#[cfg(all(feature = "Win32_functiondiscoveryapi", feature = "Win32_servprov", feature = "Win32_winnt"))]
pub trait CFunctionDiscoveryNotificationWrapper_Impl: super::functiondiscoveryapi::IFunctionDiscoveryNotification_Impl {}
#[cfg(all(feature = "Win32_functiondiscoveryapi", feature = "Win32_servprov", feature = "Win32_winnt"))]
impl CFunctionDiscoveryNotificationWrapper_Vtbl {
    pub const fn new<Identity: CFunctionDiscoveryNotificationWrapper_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::functiondiscoveryapi::IFunctionDiscoveryNotification_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<CFunctionDiscoveryNotificationWrapper as windows_core::Interface>::IID || iid == &<super::functiondiscoveryapi::IFunctionDiscoveryNotification as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_functiondiscoveryapi", feature = "Win32_servprov", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for CFunctionDiscoveryNotificationWrapper {}
pub const FD_EVENTID: u32 = 1000;
pub const FD_EVENTID_ASYNCTHREADEXIT: u32 = 1001;
pub const FD_EVENTID_IPADDRESSCHANGE: u32 = 1003;
pub const FD_EVENTID_PRIVATE: u32 = 100;
pub const FD_EVENTID_QUERYREFRESH: u32 = 1004;
pub const FD_EVENTID_SEARCHCOMPLETE: u32 = 1000;
pub const FD_EVENTID_SEARCHSTART: u32 = 1002;
