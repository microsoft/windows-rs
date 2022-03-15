#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingConnection(::windows::core::IUnknown);
impl CastingConnection {
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn State(&self) -> ::windows::core::Result<CastingConnectionState> {
        let this = self;
        unsafe {
            let mut result__: CastingConnectionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CastingConnectionState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn Device(&self) -> ::windows::core::Result<CastingDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Device)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CastingDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn Source(&self) -> ::windows::core::Result<CastingSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CastingSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, CastingSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CastingConnection, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CastingConnection, CastingConnectionErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorOccurred)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveErrorOccurred)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStartCastingAsync<'a, Param0: ::windows::core::IntoParam<'a, CastingSource>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStartCastingAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisconnectAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisconnectAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for CastingConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CastingConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingConnection {}
impl ::core::fmt::Debug for CastingConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CastingConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingConnection;{cd951653-c2f1-4498-8b78-5fb4cd3640dd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CastingConnection {
    type Vtable = ICastingConnection_Vtbl;
    const IID: ::windows::core::GUID = <ICastingConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CastingConnection {
    const NAME: &'static str = "Windows.Media.Casting.CastingConnection";
}
impl ::core::convert::From<CastingConnection> for ::windows::core::IUnknown {
    fn from(value: CastingConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingConnection> for ::windows::core::IUnknown {
    fn from(value: &CastingConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CastingConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CastingConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CastingConnection> for ::windows::core::IInspectable {
    fn from(value: CastingConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingConnection> for ::windows::core::IInspectable {
    fn from(value: &CastingConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CastingConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CastingConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CastingConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CastingConnection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CastingConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CastingConnection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for CastingConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &CastingConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CastingConnection {}
unsafe impl ::core::marker::Sync for CastingConnection {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingConnectionErrorOccurredEventArgs(::windows::core::IUnknown);
impl CastingConnectionErrorOccurredEventArgs {
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn ErrorStatus(&self) -> ::windows::core::Result<CastingConnectionErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: CastingConnectionErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CastingConnectionErrorStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CastingConnectionErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CastingConnectionErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingConnectionErrorOccurredEventArgs {}
impl ::core::fmt::Debug for CastingConnectionErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnectionErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CastingConnectionErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs;{a7fb3c69-8719-4f00-81fb-961863c79a32})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CastingConnectionErrorOccurredEventArgs {
    type Vtable = ICastingConnectionErrorOccurredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICastingConnectionErrorOccurredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CastingConnectionErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs";
}
impl ::core::convert::From<CastingConnectionErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: CastingConnectionErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingConnectionErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CastingConnectionErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CastingConnectionErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CastingConnectionErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CastingConnectionErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: CastingConnectionErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingConnectionErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CastingConnectionErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CastingConnectionErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CastingConnectionErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CastingConnectionErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for CastingConnectionErrorOccurredEventArgs {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CastingConnectionErrorStatus(pub i32);
impl CastingConnectionErrorStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const DeviceDidNotRespond: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
    pub const InvalidCastingSource: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl ::core::marker::Copy for CastingConnectionErrorStatus {}
impl ::core::clone::Clone for CastingConnectionErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CastingConnectionErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CastingConnectionErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for CastingConnectionErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnectionErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CastingConnectionErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingConnectionErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CastingConnectionState(pub i32);
impl CastingConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Connecting: Self = Self(4i32);
}
impl ::core::marker::Copy for CastingConnectionState {}
impl ::core::clone::Clone for CastingConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CastingConnectionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CastingConnectionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for CastingConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnectionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CastingConnectionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingConnectionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingDevice(::windows::core::IUnknown);
impl CastingDevice {
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FriendlyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Icon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Icon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSupportedCastingPlaybackTypesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingPlaybackTypes>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedCastingPlaybackTypesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CastingPlaybackTypes>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn CreateCastingConnection(&self) -> ::windows::core::Result<CastingConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCastingConnection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CastingConnection>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn GetDeviceSelector(r#type: CastingPlaybackTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeviceSelectorFromCastingSourceAsync<'a, Param0: ::windows::core::IntoParam<'a, CastingSource>>(castingsource: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromCastingSourceAsync)(::core::mem::transmute_copy(this), castingsource.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingDevice>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CastingDevice>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn DeviceInfoSupportsCastingAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>>(device: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInfoSupportsCastingAsync)(::core::mem::transmute_copy(this), device.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICastingDeviceStatics<R, F: FnOnce(&ICastingDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CastingDevice, ICastingDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CastingDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CastingDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingDevice {}
impl ::core::fmt::Debug for CastingDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CastingDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevice;{de721c83-4a43-4ad1-a6d2-2492a796c3f2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CastingDevice {
    type Vtable = ICastingDevice_Vtbl;
    const IID: ::windows::core::GUID = <ICastingDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CastingDevice {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevice";
}
impl ::core::convert::From<CastingDevice> for ::windows::core::IUnknown {
    fn from(value: CastingDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingDevice> for ::windows::core::IUnknown {
    fn from(value: &CastingDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CastingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CastingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CastingDevice> for ::windows::core::IInspectable {
    fn from(value: CastingDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingDevice> for ::windows::core::IInspectable {
    fn from(value: &CastingDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CastingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CastingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CastingDevice {}
unsafe impl ::core::marker::Sync for CastingDevice {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingDevicePicker(::windows::core::IUnknown);
impl CastingDevicePicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CastingDevicePicker, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn Filter(&self) -> ::windows::core::Result<CastingDevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Filter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CastingDevicePickerFilter>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Appearance(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Appearance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DevicePickerAppearance>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CastingDeviceSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CastingDevicePicker, CastingDeviceSelectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CastingDeviceSelected)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCastingDeviceSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCastingDeviceSelected)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CastingDevicePickerDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CastingDevicePicker, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CastingDevicePickerDismissed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCastingDevicePickerDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCastingDevicePickerDismissed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Show)(::core::mem::transmute_copy(this), selection.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowWithPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowWithPlacement)(::core::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn Hide(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Hide)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for CastingDevicePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CastingDevicePicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingDevicePicker {}
impl ::core::fmt::Debug for CastingDevicePicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingDevicePicker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CastingDevicePicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevicePicker;{dcd39924-0591-49be-aacb-4b82ee756a95})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CastingDevicePicker {
    type Vtable = ICastingDevicePicker_Vtbl;
    const IID: ::windows::core::GUID = <ICastingDevicePicker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CastingDevicePicker {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevicePicker";
}
impl ::core::convert::From<CastingDevicePicker> for ::windows::core::IUnknown {
    fn from(value: CastingDevicePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingDevicePicker> for ::windows::core::IUnknown {
    fn from(value: &CastingDevicePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CastingDevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CastingDevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CastingDevicePicker> for ::windows::core::IInspectable {
    fn from(value: CastingDevicePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingDevicePicker> for ::windows::core::IInspectable {
    fn from(value: &CastingDevicePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CastingDevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CastingDevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CastingDevicePicker {}
unsafe impl ::core::marker::Sync for CastingDevicePicker {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingDevicePickerFilter(::windows::core::IUnknown);
impl CastingDevicePickerFilter {
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn SupportsAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportsAudio)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn SetSupportsAudio(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSupportsAudio)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn SupportsVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportsVideo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn SetSupportsVideo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSupportsVideo)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn SupportsPictures(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportsPictures)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn SetSupportsPictures(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSupportsPictures)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCastingSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<CastingSource>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCastingSources)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<CastingSource>>(result__)
        }
    }
}
impl ::core::clone::Clone for CastingDevicePickerFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CastingDevicePickerFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingDevicePickerFilter {}
impl ::core::fmt::Debug for CastingDevicePickerFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingDevicePickerFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CastingDevicePickerFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevicePickerFilter;{be8c619c-b563-4354-ae33-9fdaad8c6291})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CastingDevicePickerFilter {
    type Vtable = ICastingDevicePickerFilter_Vtbl;
    const IID: ::windows::core::GUID = <ICastingDevicePickerFilter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CastingDevicePickerFilter {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevicePickerFilter";
}
impl ::core::convert::From<CastingDevicePickerFilter> for ::windows::core::IUnknown {
    fn from(value: CastingDevicePickerFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingDevicePickerFilter> for ::windows::core::IUnknown {
    fn from(value: &CastingDevicePickerFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CastingDevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CastingDevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CastingDevicePickerFilter> for ::windows::core::IInspectable {
    fn from(value: CastingDevicePickerFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingDevicePickerFilter> for ::windows::core::IInspectable {
    fn from(value: &CastingDevicePickerFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CastingDevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CastingDevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CastingDevicePickerFilter {}
unsafe impl ::core::marker::Sync for CastingDevicePickerFilter {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingDeviceSelectedEventArgs(::windows::core::IUnknown);
impl CastingDeviceSelectedEventArgs {
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    pub fn SelectedCastingDevice(&self) -> ::windows::core::Result<CastingDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectedCastingDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CastingDevice>(result__)
        }
    }
}
impl ::core::clone::Clone for CastingDeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CastingDeviceSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingDeviceSelectedEventArgs {}
impl ::core::fmt::Debug for CastingDeviceSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingDeviceSelectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CastingDeviceSelectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDeviceSelectedEventArgs;{dc439e86-dd57-4d0d-9400-af45e4fb3663})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CastingDeviceSelectedEventArgs {
    type Vtable = ICastingDeviceSelectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICastingDeviceSelectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CastingDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.CastingDeviceSelectedEventArgs";
}
impl ::core::convert::From<CastingDeviceSelectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CastingDeviceSelectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingDeviceSelectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CastingDeviceSelectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CastingDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CastingDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CastingDeviceSelectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CastingDeviceSelectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingDeviceSelectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CastingDeviceSelectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CastingDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CastingDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CastingDeviceSelectedEventArgs {}
unsafe impl ::core::marker::Sync for CastingDeviceSelectedEventArgs {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CastingPlaybackTypes(pub u32);
impl CastingPlaybackTypes {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const Picture: Self = Self(4u32);
}
impl ::core::marker::Copy for CastingPlaybackTypes {}
impl ::core::clone::Clone for CastingPlaybackTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CastingPlaybackTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CastingPlaybackTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for CastingPlaybackTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingPlaybackTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CastingPlaybackTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CastingPlaybackTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CastingPlaybackTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CastingPlaybackTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CastingPlaybackTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CastingPlaybackTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingPlaybackTypes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingSource(::windows::core::IUnknown);
impl CastingSource {
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredSourceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreferredSourceUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Casting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredSourceUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferredSourceUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CastingSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CastingSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingSource {}
impl ::core::fmt::Debug for CastingSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CastingSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingSource;{f429ea72-3467-47e6-a027-522923e9d727})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CastingSource {
    type Vtable = ICastingSource_Vtbl;
    const IID: ::windows::core::GUID = <ICastingSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CastingSource {
    const NAME: &'static str = "Windows.Media.Casting.CastingSource";
}
impl ::core::convert::From<CastingSource> for ::windows::core::IUnknown {
    fn from(value: CastingSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingSource> for ::windows::core::IUnknown {
    fn from(value: &CastingSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CastingSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CastingSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CastingSource> for ::windows::core::IInspectable {
    fn from(value: CastingSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CastingSource> for ::windows::core::IInspectable {
    fn from(value: &CastingSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CastingSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CastingSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CastingSource {}
unsafe impl ::core::marker::Sync for CastingSource {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingConnection {
    type Vtable = ICastingConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd951653_c2f1_4498_8b78_5fb4cd3640dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingConnection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CastingConnectionState) -> ::windows::core::HRESULT,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStartCastingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStartCastingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisconnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingConnectionErrorOccurredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingConnectionErrorOccurredEventArgs {
    type Vtable = ICastingConnectionErrorOccurredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7fb3c69_8719_4f00_81fb_961863c79a32);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingConnectionErrorOccurredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CastingConnectionErrorStatus) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDevice {
    type Vtable = ICastingDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde721c83_4a43_4ad1_a6d2_2492a796c3f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Icon: usize,
    #[cfg(feature = "Foundation")]
    pub GetSupportedCastingPlaybackTypesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSupportedCastingPlaybackTypesAsync: usize,
    pub CreateCastingConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDevicePicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDevicePicker {
    type Vtable = ICastingDevicePicker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcd39924_0591_49be_aacb_4b82ee756a95);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevicePicker_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Appearance: usize,
    #[cfg(feature = "Foundation")]
    pub CastingDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CastingDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCastingDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCastingDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub CastingDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CastingDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCastingDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCastingDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Show: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDevicePickerFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDevicePickerFilter {
    type Vtable = ICastingDevicePickerFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe8c619c_b563_4354_ae33_9fdaad8c6291);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevicePickerFilter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SupportsPictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSupportsPictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCastingSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCastingSources: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDeviceSelectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDeviceSelectedEventArgs {
    type Vtable = ICastingDeviceSelectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc439e86_dd57_4d0d_9400_af45e4fb3663);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDeviceSelectedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SelectedCastingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDeviceStatics {
    type Vtable = ICastingDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7d958d7_4d13_4237_a365_4c4f6a4cfd2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CastingPlaybackTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeviceSelectorFromCastingSourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, castingsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeviceSelectorFromCastingSourceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub DeviceInfoSupportsCastingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    DeviceInfoSupportsCastingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingSource {
    type Vtable = ICastingSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf429ea72_3467_47e6_a027_522923e9d727);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredSourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredSourceUri: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
