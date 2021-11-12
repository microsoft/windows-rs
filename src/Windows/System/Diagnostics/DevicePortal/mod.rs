#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePortalConnection(pub ::windows::core::IInspectable);
impl DevicePortalConnection {
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn GetForAppServiceConnection<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::AppService::AppServiceConnection>>(appserviceconnection: Param0) -> ::windows::core::Result<DevicePortalConnection> {
        Self::IDevicePortalConnectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appserviceconnection.into_param().abi(), &mut result__).from_abi::<DevicePortalConnection>(result__)
        })
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>>(&self, request: Param0) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Networking::Sockets::ServerMessageWebSocket>(result__)
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, request: Param0, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: Param2) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), request.into_param().abi(), messagetype, protocol.into_param().abi(), &mut result__).from_abi::<super::super::super::Networking::Sockets::ServerMessageWebSocket>(result__)
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest3<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        request: Param0,
        messagetype: super::super::super::Networking::Sockets::SocketMessageType,
        protocol: Param2,
        outboundbuffersizeinbytes: u32,
        maxmessagesize: u32,
        receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode,
    ) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), request.into_param().abi(), messagetype, protocol.into_param().abi(), outboundbuffersizeinbytes, maxmessagesize, receivemode, &mut result__).from_abi::<super::super::super::Networking::Sockets::ServerMessageWebSocket>(result__)
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>>(&self, request: Param0) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Networking::Sockets::ServerStreamWebSocket>(result__)
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, request: Param0, protocol: Param1, outboundbuffersizeinbytes: u32, nodelay: bool) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), request.into_param().abi(), protocol.into_param().abi(), outboundbuffersizeinbytes, nodelay, &mut result__).from_abi::<super::super::super::Networking::Sockets::ServerStreamWebSocket>(result__)
        }
    }
    pub fn IDevicePortalConnectionStatics<R, F: FnOnce(&IDevicePortalConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DevicePortalConnection, IDevicePortalConnectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePortalConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnection;{0f447f51-1198-4da1-8d54-bdef393e09b6})");
}
unsafe impl ::windows::core::Interface for DevicePortalConnection {
    type Vtable = IDevicePortalConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f447f51_1198_4da1_8d54_bdef393e09b6);
}
impl ::windows::core::RuntimeName for DevicePortalConnection {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnection";
}
impl ::core::convert::From<DevicePortalConnection> for ::windows::core::IUnknown {
    fn from(value: DevicePortalConnection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePortalConnection> for ::windows::core::IUnknown {
    fn from(value: &DevicePortalConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePortalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DevicePortalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePortalConnection> for ::windows::core::IInspectable {
    fn from(value: DevicePortalConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePortalConnection> for ::windows::core::IInspectable {
    fn from(value: &DevicePortalConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePortalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DevicePortalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePortalConnection {}
unsafe impl ::core::marker::Sync for DevicePortalConnection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePortalConnectionClosedEventArgs(pub ::windows::core::IInspectable);
impl DevicePortalConnectionClosedEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<DevicePortalConnectionClosedReason> {
        let this = self;
        unsafe {
            let mut result__: DevicePortalConnectionClosedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePortalConnectionClosedReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePortalConnectionClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs;{fcf70e38-7032-428c-9f50-945c15a9f0cb})");
}
unsafe impl ::windows::core::Interface for DevicePortalConnectionClosedEventArgs {
    type Vtable = IDevicePortalConnectionClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcf70e38_7032_428c_9f50_945c15a9f0cb);
}
impl ::windows::core::RuntimeName for DevicePortalConnectionClosedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs";
}
impl ::core::convert::From<DevicePortalConnectionClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DevicePortalConnectionClosedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePortalConnectionClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DevicePortalConnectionClosedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePortalConnectionClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DevicePortalConnectionClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePortalConnectionClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DevicePortalConnectionClosedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePortalConnectionClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DevicePortalConnectionClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePortalConnectionClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DevicePortalConnectionClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePortalConnectionClosedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePortalConnectionClosedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DevicePortalConnectionClosedReason(pub i32);
impl DevicePortalConnectionClosedReason {
    pub const Unknown: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(0i32);
    pub const ResourceLimitsExceeded: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(1i32);
    pub const ProtocolError: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(2i32);
    pub const NotAuthorized: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(3i32);
    pub const UserNotPresent: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(4i32);
    pub const ServiceTerminated: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(5i32);
}
impl ::core::convert::From<i32> for DevicePortalConnectionClosedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DevicePortalConnectionClosedReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DevicePortalConnectionClosedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedReason;i4)");
}
impl ::windows::core::DefaultType for DevicePortalConnectionClosedReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePortalConnectionRequestReceivedEventArgs(pub ::windows::core::IInspectable);
impl DevicePortalConnectionRequestReceivedEventArgs {
    #[cfg(feature = "Web_Http")]
    pub fn RequestMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Web::Http::HttpRequestMessage>(result__)
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn ResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Web::Http::HttpResponseMessage>(result__)
        }
    }
    pub fn IsWebSocketUpgradeRequest(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebSocketProtocolsRequested(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePortalConnectionRequestReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs;{64dae045-6fda-4459-9ebd-ecce22e38559})");
}
unsafe impl ::windows::core::Interface for DevicePortalConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalConnectionRequestReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64dae045_6fda_4459_9ebd_ecce22e38559);
}
impl ::windows::core::RuntimeName for DevicePortalConnectionRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs";
}
impl ::core::convert::From<DevicePortalConnectionRequestReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DevicePortalConnectionRequestReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePortalConnectionRequestReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DevicePortalConnectionRequestReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePortalConnectionRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DevicePortalConnectionRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePortalConnectionRequestReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DevicePortalConnectionRequestReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePortalConnectionRequestReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DevicePortalConnectionRequestReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePortalConnectionRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DevicePortalConnectionRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePortalConnectionRequestReceivedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePortalConnectionRequestReceivedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePortalConnection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePortalConnection {
    type Vtable = IDevicePortalConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f447f51_1198_4da1_8d54_bdef393e09b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePortalConnectionClosedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePortalConnectionClosedEventArgs {
    type Vtable = IDevicePortalConnectionClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcf70e38_7032_428c_9f50_945c15a9f0cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DevicePortalConnectionClosedReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePortalConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalConnectionRequestReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64dae045_6fda_4459_9ebd_ecce22e38559);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web_Http")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))] usize,
    #[cfg(feature = "Web_Http")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePortalConnectionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePortalConnectionStatics {
    type Vtable = IDevicePortalConnectionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bbe31e7_e9b9_4645_8fed_a53eea0edbd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_AppService")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appserviceconnection: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePortalWebSocketConnection {
    type Vtable = IDevicePortalWebSocketConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67657920_d65a_42f0_aef4_787808098b7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))] usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))] usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))] usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))] usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, protocol: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, outboundbuffersizeinbytes: u32, nodelay: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalWebSocketConnectionRequestReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79fdcaba_175c_4739_9f74_dda797c35b3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
