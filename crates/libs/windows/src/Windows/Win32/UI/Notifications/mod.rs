#[doc = "*Required features: `\"Win32_UI_Notifications\"`*"]
#[repr(transparent)]
pub struct INotificationActivationCallback(::windows::core::IUnknown);
impl INotificationActivationCallback {
    pub unsafe fn Activate<'a, P0, P1>(&self, appusermodelid: P0, invokedargs: P1, data: &[NOTIFICATION_USER_INPUT_DATA]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), appusermodelid.into(), invokedargs.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(data)), data.len() as _).ok()
    }
}
impl ::core::convert::From<INotificationActivationCallback> for ::windows::core::IUnknown {
    fn from(value: INotificationActivationCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a INotificationActivationCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a INotificationActivationCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotificationActivationCallback> for ::windows::core::IUnknown {
    fn from(value: &INotificationActivationCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for INotificationActivationCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INotificationActivationCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotificationActivationCallback {}
impl ::core::fmt::Debug for INotificationActivationCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotificationActivationCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INotificationActivationCallback {
    type Vtable = INotificationActivationCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53e31837_6600_4a81_9395_75cffe746f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationActivationCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::windows::core::PCWSTR, invokedargs: ::windows::core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Notifications\"`*"]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: ::windows::core::PCWSTR,
    pub Value: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for NOTIFICATION_USER_INPUT_DATA {}
impl ::core::clone::Clone for NOTIFICATION_USER_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NOTIFICATION_USER_INPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFICATION_USER_INPUT_DATA").field("Key", &self.Key).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for NOTIFICATION_USER_INPUT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NOTIFICATION_USER_INPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NOTIFICATION_USER_INPUT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NOTIFICATION_USER_INPUT_DATA {}
impl ::core::default::Default for NOTIFICATION_USER_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
