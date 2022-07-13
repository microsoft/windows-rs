#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotificationListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserNotificationListener {
    type Vtable = IUserNotificationListener_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62553e41_8a06_4cef_8215_6033a5be4b03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationListener_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    pub GetAccessStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserNotificationListenerAccessStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NotificationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotificationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotificationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotificationChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNotificationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kinds: super::NotificationKinds, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNotificationsAsync: usize,
    pub GetNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationid: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotificationListenerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserNotificationListenerStatics {
    type Vtable = IUserNotificationListenerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff6123cf_4386_4aa3_b73d_b804e5b63b23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationListenerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Notifications_Management\"`*"]
#[repr(transparent)]
pub struct UserNotificationListener(::windows::core::IUnknown);
impl UserNotificationListener {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserNotificationListenerAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<UserNotificationListenerAccessStatus>>(result__)
        }
    }
    pub fn GetAccessStatus(&self) -> ::windows::core::Result<UserNotificationListenerAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAccessStatus)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserNotificationListenerAccessStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotificationChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::TypedEventHandler<UserNotificationListener, super::UserNotificationChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NotificationChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotificationChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNotificationChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNotificationsAsync(&self, kinds: super::NotificationKinds) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::UserNotification>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetNotificationsAsync)(::windows::core::Interface::as_raw(this), kinds, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::UserNotification>>>(result__)
        }
    }
    pub fn GetNotification(&self, notificationid: u32) -> ::windows::core::Result<super::UserNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetNotification)(::windows::core::Interface::as_raw(this), notificationid, result__.as_mut_ptr()).from_abi::<super::UserNotification>(result__)
        }
    }
    pub fn ClearNotifications(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearNotifications)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoveNotification(&self, notificationid: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNotification)(::windows::core::Interface::as_raw(this), notificationid).ok() }
    }
    pub fn Current() -> ::windows::core::Result<UserNotificationListener> {
        Self::IUserNotificationListenerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Current)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserNotificationListener>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserNotificationListenerStatics<R, F: FnOnce(&IUserNotificationListenerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserNotificationListener, IUserNotificationListenerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UserNotificationListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserNotificationListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotificationListener {}
impl ::core::fmt::Debug for UserNotificationListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserNotificationListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.Management.UserNotificationListener;{62553e41-8a06-4cef-8215-6033a5be4b03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserNotificationListener {
    type Vtable = IUserNotificationListener_Vtbl;
    const IID: ::windows::core::GUID = <IUserNotificationListener as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserNotificationListener {
    const NAME: &'static str = "Windows.UI.Notifications.Management.UserNotificationListener";
}
impl ::core::convert::From<UserNotificationListener> for ::windows::core::IUnknown {
    fn from(value: UserNotificationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotificationListener> for ::windows::core::IUnknown {
    fn from(value: &UserNotificationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UserNotificationListener> for &::windows::core::IUnknown {
    fn from(value: &UserNotificationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<UserNotificationListener> for ::windows::core::IInspectable {
    fn from(value: UserNotificationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotificationListener> for ::windows::core::IInspectable {
    fn from(value: &UserNotificationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UserNotificationListener> for &::windows::core::IInspectable {
    fn from(value: &UserNotificationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for UserNotificationListener {}
unsafe impl ::core::marker::Sync for UserNotificationListener {}
#[doc = "*Required features: `\"UI_Notifications_Management\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserNotificationListenerAccessStatus(pub i32);
impl UserNotificationListenerAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for UserNotificationListenerAccessStatus {}
impl ::core::clone::Clone for UserNotificationListenerAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserNotificationListenerAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserNotificationListenerAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserNotificationListenerAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationListenerAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserNotificationListenerAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.Management.UserNotificationListenerAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
