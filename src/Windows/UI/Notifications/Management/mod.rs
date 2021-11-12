#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserNotificationListener(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserNotificationListener {
    type Vtable = IUserNotificationListener_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62553e41_8a06_4cef_8215_6033a5be4b03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationListener_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserNotificationListenerAccessStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kinds: super::NotificationKinds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notificationid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notificationid: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserNotificationListenerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserNotificationListenerStatics {
    type Vtable = IUserNotificationListenerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff6123cf_4386_4aa3_b73d_b804e5b63b23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationListenerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserNotificationListener(pub ::windows::core::IInspectable);
impl UserNotificationListener {
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserNotificationListenerAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<UserNotificationListenerAccessStatus>>(result__)
        }
    }
    pub fn GetAccessStatus(&self) -> ::windows::core::Result<UserNotificationListenerAccessStatus> {
        let this = self;
        unsafe {
            let mut result__: UserNotificationListenerAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserNotificationListenerAccessStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NotificationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<UserNotificationListener, super::UserNotificationChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotificationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetNotificationsAsync(&self, kinds: super::NotificationKinds) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::UserNotification>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), kinds, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::UserNotification>>>(result__)
        }
    }
    pub fn GetNotification(&self, notificationid: u32) -> ::windows::core::Result<super::UserNotification> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), notificationid, &mut result__).from_abi::<super::UserNotification>(result__)
        }
    }
    pub fn ClearNotifications(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn RemoveNotification(&self, notificationid: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), notificationid).ok() }
    }
    pub fn Current() -> ::windows::core::Result<UserNotificationListener> {
        Self::IUserNotificationListenerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserNotificationListener>(result__)
        })
    }
    pub fn IUserNotificationListenerStatics<R, F: FnOnce(&IUserNotificationListenerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserNotificationListener, IUserNotificationListenerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for UserNotificationListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.Management.UserNotificationListener;{62553e41-8a06-4cef-8215-6033a5be4b03})");
}
unsafe impl ::windows::core::Interface for UserNotificationListener {
    type Vtable = IUserNotificationListener_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62553e41_8a06_4cef_8215_6033a5be4b03);
}
impl ::windows::core::RuntimeName for UserNotificationListener {
    const NAME: &'static str = "Windows.UI.Notifications.Management.UserNotificationListener";
}
impl ::core::convert::From<UserNotificationListener> for ::windows::core::IUnknown {
    fn from(value: UserNotificationListener) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserNotificationListener> for ::windows::core::IUnknown {
    fn from(value: &UserNotificationListener) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserNotificationListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserNotificationListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserNotificationListener> for ::windows::core::IInspectable {
    fn from(value: UserNotificationListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserNotificationListener> for ::windows::core::IInspectable {
    fn from(value: &UserNotificationListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserNotificationListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserNotificationListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserNotificationListener {}
unsafe impl ::core::marker::Sync for UserNotificationListener {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserNotificationListenerAccessStatus(pub i32);
impl UserNotificationListenerAccessStatus {
    pub const Unspecified: UserNotificationListenerAccessStatus = UserNotificationListenerAccessStatus(0i32);
    pub const Allowed: UserNotificationListenerAccessStatus = UserNotificationListenerAccessStatus(1i32);
    pub const Denied: UserNotificationListenerAccessStatus = UserNotificationListenerAccessStatus(2i32);
}
impl ::core::convert::From<i32> for UserNotificationListenerAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserNotificationListenerAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserNotificationListenerAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.Management.UserNotificationListenerAccessStatus;i4)");
}
impl ::windows::core::DefaultType for UserNotificationListenerAccessStatus {
    type DefaultType = Self;
}
