#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationChannel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationChannel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b28102e_ef0b_4f39_9b8a_a3c194de7081);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PushNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PushNotificationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePushNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePushNotificationReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationChannelManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerForUser {
    type Vtable = IPushNotificationChannelManagerForUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationChannelManagerForUser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4c45704_1182_42c7_8890_f563c4890dc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsyncWithId: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForSecondaryTileAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationChannelManagerForUser2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerForUser2 {
    type Vtable = IPushNotificationChannelManagerForUser2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationChannelManagerForUser2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc38b066a_7cc1_4dac_87fd_be6e920414a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerForUser2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserverkey: *mut ::core::ffi::c_void, channelid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserverkey: *mut ::core::ffi::c_void, channelid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationChannelManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerStatics {
    type Vtable = IPushNotificationChannelManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationChannelManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8baf9b65_77a1_4588_bd19_861529a9dcf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsyncWithId: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForSecondaryTileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationChannelManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerStatics2 {
    type Vtable = IPushNotificationChannelManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationChannelManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb444a65d_a7e9_4b28_950e_f375a907f9df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationChannelManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerStatics3 {
    type Vtable = IPushNotificationChannelManagerStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationChannelManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4701fefe_0ede_4a3f_ae78_bfa471496925);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationChannelManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerStatics4 {
    type Vtable = IPushNotificationChannelManagerStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationChannelManagerStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc540efb_7820_5a5b_9c01_b4757f774025);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ChannelsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelsRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChannelsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChannelsRevoked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationChannelsRevokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelsRevokedEventArgs {
    type Vtable = IPushNotificationChannelsRevokedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationChannelsRevokedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20e1a24c_1a34_5beb_aae2_40c232c8c140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelsRevokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationReceivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1065e0c_36cd_484c_b935_0a99b753cf00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub NotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PushNotificationType) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub ToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    ToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub TileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    TileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub BadgeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    BadgeNotification: usize,
    pub RawNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRawNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRawNotification {
    type Vtable = IRawNotification_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRawNotification {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a227281_3b79_42ac_9963_22ab00d4f0b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRawNotification2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRawNotification2 {
    type Vtable = IRawNotification2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRawNotification2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6d0cf19_0c6f_4cdd_9424_eec5be014d26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
    pub ChannelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRawNotification3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRawNotification3 {
    type Vtable = IRawNotification3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRawNotification3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62737dde_8a73_424c_ab44_5635f40a96e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub ContentBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentBytes: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PushNotificationChannel(::windows_core::IUnknown);
impl PushNotificationChannel {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PushNotificationReceived<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PushNotificationReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePushNotificationReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePushNotificationReceived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for PushNotificationChannel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannel;{2b28102e-ef0b-4f39-9b8a-a3c194de7081})");
}
unsafe impl ::windows_core::Interface for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PushNotificationChannel {
    const IID: ::windows_core::GUID = <IPushNotificationChannel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannel";
}
::windows_core::imp::interface_hierarchy!(PushNotificationChannel, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PushNotificationChannel {}
unsafe impl ::core::marker::Sync for PushNotificationChannel {}
pub struct PushNotificationChannelManager;
impl PushNotificationChannelManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId(applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForSecondaryTileAsync(tileid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<PushNotificationChannelManagerForUser>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IPushNotificationChannelManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefault() -> ::windows_core::Result<PushNotificationChannelManagerForUser> {
        Self::IPushNotificationChannelManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelsRevoked<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<PushNotificationChannelsRevokedEventArgs>>,
    {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelsRevoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChannelsRevoked(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveChannelsRevoked)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics<R, F: FnOnce(&IPushNotificationChannelManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics2<R, F: FnOnce(&IPushNotificationChannelManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics3<R, F: FnOnce(&IPushNotificationChannelManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics4<R, F: FnOnce(&IPushNotificationChannelManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PushNotificationChannelManager {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PushNotificationChannelManagerForUser(::windows_core::IUnknown);
impl PushNotificationChannelManagerForUser {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId(&self, applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForSecondaryTileAsync(&self, tileid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync<P0>(&self, appserverkey: P0, channelid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync)(::windows_core::Interface::as_raw(this), appserverkey.try_into_param()?.abi(), ::core::mem::transmute_copy(channelid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId<P0>(&self, appserverkey: P0, channelid: &::windows_core::HSTRING, appid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId)(::windows_core::Interface::as_raw(this), appserverkey.try_into_param()?.abi(), ::core::mem::transmute_copy(channelid), ::core::mem::transmute_copy(appid), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PushNotificationChannelManagerForUser {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser;{a4c45704-1182-42c7-8890-f563c4890dc4})");
}
unsafe impl ::windows_core::Interface for PushNotificationChannelManagerForUser {
    type Vtable = IPushNotificationChannelManagerForUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PushNotificationChannelManagerForUser {
    const IID: ::windows_core::GUID = <IPushNotificationChannelManagerForUser as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationChannelManagerForUser {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser";
}
::windows_core::imp::interface_hierarchy!(PushNotificationChannelManagerForUser, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PushNotificationChannelManagerForUser {}
unsafe impl ::core::marker::Sync for PushNotificationChannelManagerForUser {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PushNotificationChannelsRevokedEventArgs(::windows_core::IUnknown);
impl PushNotificationChannelsRevokedEventArgs {}
impl ::windows_core::RuntimeType for PushNotificationChannelsRevokedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs;{20e1a24c-1a34-5beb-aae2-40c232c8c140})");
}
unsafe impl ::windows_core::Interface for PushNotificationChannelsRevokedEventArgs {
    type Vtable = IPushNotificationChannelsRevokedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PushNotificationChannelsRevokedEventArgs {
    const IID: ::windows_core::GUID = <IPushNotificationChannelsRevokedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationChannelsRevokedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PushNotificationChannelsRevokedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PushNotificationChannelsRevokedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationChannelsRevokedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PushNotificationReceivedEventArgs(::windows_core::IUnknown);
impl PushNotificationReceivedEventArgs {
    pub fn SetCancel(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCancel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NotificationType(&self) -> ::windows_core::Result<PushNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_Notifications\"`"]
    #[cfg(feature = "UI_Notifications")]
    pub fn ToastNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToastNotification)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_Notifications\"`"]
    #[cfg(feature = "UI_Notifications")]
    pub fn TileNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileNotification)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_Notifications\"`"]
    #[cfg(feature = "UI_Notifications")]
    pub fn BadgeNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::BadgeNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BadgeNotification)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RawNotification(&self) -> ::windows_core::Result<RawNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawNotification)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs;{d1065e0c-36cd-484c-b935-0a99b753cf00})");
}
unsafe impl ::windows_core::Interface for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PushNotificationReceivedEventArgs {
    const IID: ::windows_core::GUID = <IPushNotificationReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PushNotificationReceivedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PushNotificationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationReceivedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RawNotification(::windows_core::IUnknown);
impl RawNotification {
    pub fn Content(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChannelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentBytes(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IRawNotification3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentBytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for RawNotification {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.RawNotification;{1a227281-3b79-42ac-9963-22ab00d4f0b7})");
}
unsafe impl ::windows_core::Interface for RawNotification {
    type Vtable = IRawNotification_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RawNotification {
    const IID: ::windows_core::GUID = <IRawNotification as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RawNotification {
    const NAME: &'static str = "Windows.Networking.PushNotifications.RawNotification";
}
::windows_core::imp::interface_hierarchy!(RawNotification, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RawNotification {}
unsafe impl ::core::marker::Sync for RawNotification {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PushNotificationType(pub i32);
impl PushNotificationType {
    pub const Toast: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Badge: Self = Self(2i32);
    pub const Raw: Self = Self(3i32);
    pub const TileFlyout: Self = Self(4i32);
}
impl ::core::marker::Copy for PushNotificationType {}
impl ::core::clone::Clone for PushNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PushNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PushNotificationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PushNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PushNotificationType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.PushNotifications.PushNotificationType;i4)");
}
