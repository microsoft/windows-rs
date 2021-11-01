#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Notifications`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INotificationActivationCallback(::windows::runtime::IUnknown);
impl INotificationActivationCallback {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Notifications`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, appusermodelid: Param0, invokedargs: Param1, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), appusermodelid.into_param().abi(), invokedargs.into_param().abi(), ::std::mem::transmute(data), ::std::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INotificationActivationCallback {
    type Vtable = INotificationActivationCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1407391799, 26112, 19073, [147, 149, 117, 207, 254, 116, 111, 148]);
}
impl ::std::convert::From<INotificationActivationCallback> for ::windows::runtime::IUnknown {
    fn from(value: INotificationActivationCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INotificationActivationCallback> for ::windows::runtime::IUnknown {
    fn from(value: &INotificationActivationCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INotificationActivationCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INotificationActivationCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationActivationCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appusermodelid: super::super::Foundation::PWSTR, invokedargs: super::super::Foundation::PWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Notifications`, `Win32_Foundation`*"]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: super::super::Foundation::PWSTR,
    pub Value: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl NOTIFICATION_USER_INPUT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NOTIFICATION_USER_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NOTIFICATION_USER_INPUT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NOTIFICATION_USER_INPUT_DATA").field("Key", &self.Key).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NOTIFICATION_USER_INPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NOTIFICATION_USER_INPUT_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NOTIFICATION_USER_INPUT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
