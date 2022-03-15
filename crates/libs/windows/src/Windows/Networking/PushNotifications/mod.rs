#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b28102e_ef0b_4f39_9b8a_a3c194de7081);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannel_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PushNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PushNotificationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePushNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePushNotificationReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerForUser {
    type Vtable = IPushNotificationChannelManagerForUser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4c45704_1182_42c7_8890_f563c4890dc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerForUser_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsyncWithId: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForSecondaryTileAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerForUser2 {
    type Vtable = IPushNotificationChannelManagerForUser2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc38b066a_7cc1_4dac_87fd_be6e920414a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerForUser2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserverkey: ::windows::core::RawPtr, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserverkey: ::windows::core::RawPtr, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerStatics {
    type Vtable = IPushNotificationChannelManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8baf9b65_77a1_4588_bd19_861529a9dcf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsyncWithId: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForSecondaryTileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerStatics2 {
    type Vtable = IPushNotificationChannelManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb444a65d_a7e9_4b28_950e_f375a907f9df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerStatics3 {
    type Vtable = IPushNotificationChannelManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4701fefe_0ede_4a3f_ae78_bfa471496925);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerStatics4 {
    type Vtable = IPushNotificationChannelManagerStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc540efb_7820_5a5b_9c01_b4757f774025);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ChannelsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelsRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChannelsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChannelsRevoked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelsRevokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationChannelsRevokedEventArgs {
    type Vtable = IPushNotificationChannelsRevokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20e1a24c_1a34_5beb_aae2_40c232c8c140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelsRevokedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1065e0c_36cd_484c_b935_0a99b753cf00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub NotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PushNotificationType) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub ToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    ToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub TileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    TileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub BadgeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    BadgeNotification: usize,
    pub RawNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRawNotification {
    type Vtable = IRawNotification_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a227281_3b79_42ac_9963_22ab00d4f0b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawNotification2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRawNotification2 {
    type Vtable = IRawNotification2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6d0cf19_0c6f_4cdd_9424_eec5be014d26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
    pub ChannelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawNotification3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRawNotification3 {
    type Vtable = IRawNotification3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62737dde_8a73_424c_ab44_5635f40a96e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub ContentBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentBytes: usize,
}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationChannel(::windows::core::IUnknown);
impl PushNotificationChannel {
    #[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExpirationTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PushNotificationReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PushNotificationReceived)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePushNotificationReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePushNotificationReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PushNotificationChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannel {}
impl ::core::fmt::Debug for PushNotificationChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannel;{2b28102e-ef0b-4f39-9b8a-a3c194de7081})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
    const IID: ::windows::core::GUID = <IPushNotificationChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannel";
}
impl ::core::convert::From<PushNotificationChannel> for ::windows::core::IUnknown {
    fn from(value: PushNotificationChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannel> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationChannel> for ::windows::core::IInspectable {
    fn from(value: PushNotificationChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannel> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationChannel {}
unsafe impl ::core::marker::Sync for PushNotificationChannel {}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
pub struct PushNotificationChannelManager {}
impl PushNotificationChannelManager {
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(applicationid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForSecondaryTileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(tileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<PushNotificationChannelManagerForUser> {
        Self::IPushNotificationChannelManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<PushNotificationChannelManagerForUser>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<PushNotificationChannelManagerForUser> {
        Self::IPushNotificationChannelManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PushNotificationChannelManagerForUser>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelsRevoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<PushNotificationChannelsRevokedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelsRevoked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChannelsRevoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveChannelsRevoked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics<R, F: FnOnce(&IPushNotificationChannelManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics2<R, F: FnOnce(&IPushNotificationChannelManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics3<R, F: FnOnce(&IPushNotificationChannelManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics4<R, F: FnOnce(&IPushNotificationChannelManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PushNotificationChannelManager {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManager";
}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationChannelManagerForUser(::windows::core::IUnknown);
impl PushNotificationChannelManagerForUser {
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, applicationid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(::core::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForSecondaryTileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, appserverkey: Param0, channelid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = &::windows::core::Interface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync)(::core::mem::transmute_copy(this), appserverkey.into_param().abi(), channelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, appserverkey: Param0, channelid: Param1, appid: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = &::windows::core::Interface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId)(::core::mem::transmute_copy(this), appserverkey.into_param().abi(), channelid.into_param().abi(), appid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
}
impl ::core::clone::Clone for PushNotificationChannelManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannelManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannelManagerForUser {}
impl ::core::fmt::Debug for PushNotificationChannelManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannelManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannelManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser;{a4c45704-1182-42c7-8890-f563c4890dc4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PushNotificationChannelManagerForUser {
    type Vtable = IPushNotificationChannelManagerForUser_Vtbl;
    const IID: ::windows::core::GUID = <IPushNotificationChannelManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationChannelManagerForUser {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser";
}
impl ::core::convert::From<PushNotificationChannelManagerForUser> for ::windows::core::IUnknown {
    fn from(value: PushNotificationChannelManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannelManagerForUser> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationChannelManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PushNotificationChannelManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PushNotificationChannelManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationChannelManagerForUser> for ::windows::core::IInspectable {
    fn from(value: PushNotificationChannelManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannelManagerForUser> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationChannelManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PushNotificationChannelManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PushNotificationChannelManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationChannelManagerForUser {}
unsafe impl ::core::marker::Sync for PushNotificationChannelManagerForUser {}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationChannelsRevokedEventArgs(::windows::core::IUnknown);
impl PushNotificationChannelsRevokedEventArgs {}
impl ::core::clone::Clone for PushNotificationChannelsRevokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannelsRevokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannelsRevokedEventArgs {}
impl ::core::fmt::Debug for PushNotificationChannelsRevokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannelsRevokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannelsRevokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs;{20e1a24c-1a34-5beb-aae2-40c232c8c140})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PushNotificationChannelsRevokedEventArgs {
    type Vtable = IPushNotificationChannelsRevokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPushNotificationChannelsRevokedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationChannelsRevokedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs";
}
impl ::core::convert::From<PushNotificationChannelsRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PushNotificationChannelsRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannelsRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationChannelsRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PushNotificationChannelsRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PushNotificationChannelsRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationChannelsRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PushNotificationChannelsRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannelsRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationChannelsRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PushNotificationChannelsRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PushNotificationChannelsRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationChannelsRevokedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationChannelsRevokedEventArgs {}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationReceivedEventArgs(::windows::core::IUnknown);
impl PushNotificationReceivedEventArgs {
    #[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCancel)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
    pub fn NotificationType(&self) -> ::windows::core::Result<PushNotificationType> {
        let this = self;
        unsafe {
            let mut result__: PushNotificationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NotificationType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PushNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn ToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToastNotification)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn TileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TileNotification)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn BadgeNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::BadgeNotification> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BadgeNotification)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::BadgeNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
    pub fn RawNotification(&self) -> ::windows::core::Result<RawNotification> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawNotification)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RawNotification>(result__)
        }
    }
}
impl ::core::clone::Clone for PushNotificationReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationReceivedEventArgs {}
impl ::core::fmt::Debug for PushNotificationReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs;{d1065e0c-36cd-484c-b935-0a99b753cf00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPushNotificationReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs";
}
impl ::core::convert::From<PushNotificationReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PushNotificationReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PushNotificationReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PushNotificationReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PushNotificationReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationReceivedEventArgs {}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PushNotificationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PushNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.PushNotifications.PushNotificationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct RawNotification(::windows::core::IUnknown);
impl RawNotification {
    #[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Content)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Headers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
    pub fn ChannelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_PushNotifications\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentBytes(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IRawNotification3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentBytes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for RawNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RawNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RawNotification {}
impl ::core::fmt::Debug for RawNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RawNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RawNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.RawNotification;{1a227281-3b79-42ac-9963-22ab00d4f0b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RawNotification {
    type Vtable = IRawNotification_Vtbl;
    const IID: ::windows::core::GUID = <IRawNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RawNotification {
    const NAME: &'static str = "Windows.Networking.PushNotifications.RawNotification";
}
impl ::core::convert::From<RawNotification> for ::windows::core::IUnknown {
    fn from(value: RawNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RawNotification> for ::windows::core::IUnknown {
    fn from(value: &RawNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RawNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RawNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RawNotification> for ::windows::core::IInspectable {
    fn from(value: RawNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RawNotification> for ::windows::core::IInspectable {
    fn from(value: &RawNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RawNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RawNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RawNotification {}
unsafe impl ::core::marker::Sync for RawNotification {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
