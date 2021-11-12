#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INotificationActivationCallback(pub ::windows::core::IUnknown);
impl INotificationActivationCallback {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, appusermodelid: Param0, invokedargs: Param1, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), appusermodelid.into_param().abi(), invokedargs.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::core::Interface for INotificationActivationCallback {
    type Vtable = INotificationActivationCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53e31837_6600_4a81_9395_75cffe746f94);
}
impl ::core::convert::From<INotificationActivationCallback> for ::windows::core::IUnknown {
    fn from(value: INotificationActivationCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INotificationActivationCallback> for ::windows::core::IUnknown {
    fn from(value: &INotificationActivationCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INotificationActivationCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INotificationActivationCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationActivationCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appusermodelid: super::super::Foundation::PWSTR, invokedargs: super::super::Foundation::PWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: super::super::Foundation::PWSTR,
    pub Value: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl NOTIFICATION_USER_INPUT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NOTIFICATION_USER_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NOTIFICATION_USER_INPUT_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("NOTIFICATION_USER_INPUT_DATA").field("Key", &self.Key).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NOTIFICATION_USER_INPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NOTIFICATION_USER_INPUT_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NOTIFICATION_USER_INPUT_DATA {
    type Abi = Self;
}
