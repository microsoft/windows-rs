#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalConnection {
    type Vtable = IDevicePortalConnection_Vtbl;
}
impl ::core::clone::Clone for IDevicePortalConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePortalConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f447f51_1198_4da1_8d54_bdef393e09b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnectionClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalConnectionClosedEventArgs {
    type Vtable = IDevicePortalConnectionClosedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDevicePortalConnectionClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePortalConnectionClosedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcf70e38_7032_428c_9f50_945c15a9f0cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePortalConnectionClosedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalConnectionRequestReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDevicePortalConnectionRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePortalConnectionRequestReceivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64dae045_6fda_4459_9ebd_ecce22e38559);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Http")]
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    RequestMessage: usize,
    #[cfg(feature = "Web_Http")]
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    ResponseMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalConnectionStatics {
    type Vtable = IDevicePortalConnectionStatics_Vtbl;
}
impl ::core::clone::Clone for IDevicePortalConnectionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePortalConnectionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bbe31e7_e9b9_4645_8fed_a53eea0edbd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub GetForAppServiceConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserviceconnection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    GetForAppServiceConnection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalWebSocketConnection {
    type Vtable = IDevicePortalWebSocketConnection_Vtbl;
}
impl ::core::clone::Clone for IDevicePortalWebSocketConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePortalWebSocketConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67657920_d65a_42f0_aef4_787808098b7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest2: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::std::mem::MaybeUninit<::windows_core::HSTRING>, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest3: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, protocol: ::std::mem::MaybeUninit<::windows_core::HSTRING>, outboundbuffersizeinbytes: u32, nodelay: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalWebSocketConnectionRequestReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79fdcaba_175c_4739_9f74_dda797c35b3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsWebSocketUpgradeRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub WebSocketProtocolsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebSocketProtocolsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"System_Diagnostics_DevicePortal\"`*"]
#[repr(transparent)]
pub struct DevicePortalConnection(::windows_core::IUnknown);
impl DevicePortalConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestReceived<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRequestReceived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn GetForAppServiceConnection<P0>(appserviceconnection: P0) -> ::windows_core::Result<DevicePortalConnection>
    where
        P0: ::windows_core::IntoParam<super::super::super::ApplicationModel::AppService::AppServiceConnection>,
    {
        Self::IDevicePortalConnectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForAppServiceConnection)(::windows_core::Interface::as_raw(this), appserviceconnection.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest<P0>(&self, request: P0) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>
    where
        P0: ::windows_core::IntoParam<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &::windows_core::ComInterface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetServerMessageWebSocketForRequest)(::windows_core::Interface::as_raw(this), request.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest2<P0>(&self, request: P0, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>
    where
        P0: ::windows_core::IntoParam<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &::windows_core::ComInterface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetServerMessageWebSocketForRequest2)(::windows_core::Interface::as_raw(this), request.into_param().abi(), messagetype, ::core::mem::transmute_copy(protocol), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest3<P0>(&self, request: P0, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &::windows_core::HSTRING, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>
    where
        P0: ::windows_core::IntoParam<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &::windows_core::ComInterface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetServerMessageWebSocketForRequest3)(::windows_core::Interface::as_raw(this), request.into_param().abi(), messagetype, ::core::mem::transmute_copy(protocol), outboundbuffersizeinbytes, maxmessagesize, receivemode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest<P0>(&self, request: P0) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket>
    where
        P0: ::windows_core::IntoParam<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &::windows_core::ComInterface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetServerStreamWebSocketForRequest)(::windows_core::Interface::as_raw(this), request.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest2<P0>(&self, request: P0, protocol: &::windows_core::HSTRING, outboundbuffersizeinbytes: u32, nodelay: bool) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket>
    where
        P0: ::windows_core::IntoParam<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &::windows_core::ComInterface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetServerStreamWebSocketForRequest2)(::windows_core::Interface::as_raw(this), request.into_param().abi(), ::core::mem::transmute_copy(protocol), outboundbuffersizeinbytes, nodelay, &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IDevicePortalConnectionStatics<R, F: FnOnce(&IDevicePortalConnectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DevicePortalConnection, IDevicePortalConnectionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DevicePortalConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePortalConnection {}
impl ::core::fmt::Debug for DevicePortalConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePortalConnection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePortalConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnection;{0f447f51-1198-4da1-8d54-bdef393e09b6})");
}
impl ::core::clone::Clone for DevicePortalConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DevicePortalConnection {
    type Vtable = IDevicePortalConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DevicePortalConnection {
    const IID: ::windows_core::GUID = <IDevicePortalConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DevicePortalConnection {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnection";
}
::windows_core::imp::interface_hierarchy!(DevicePortalConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DevicePortalConnection {}
unsafe impl ::core::marker::Sync for DevicePortalConnection {}
#[doc = "*Required features: `\"System_Diagnostics_DevicePortal\"`*"]
#[repr(transparent)]
pub struct DevicePortalConnectionClosedEventArgs(::windows_core::IUnknown);
impl DevicePortalConnectionClosedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<DevicePortalConnectionClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DevicePortalConnectionClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePortalConnectionClosedEventArgs {}
impl ::core::fmt::Debug for DevicePortalConnectionClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePortalConnectionClosedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePortalConnectionClosedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs;{fcf70e38-7032-428c-9f50-945c15a9f0cb})");
}
impl ::core::clone::Clone for DevicePortalConnectionClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DevicePortalConnectionClosedEventArgs {
    type Vtable = IDevicePortalConnectionClosedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DevicePortalConnectionClosedEventArgs {
    const IID: ::windows_core::GUID = <IDevicePortalConnectionClosedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DevicePortalConnectionClosedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DevicePortalConnectionClosedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DevicePortalConnectionClosedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePortalConnectionClosedEventArgs {}
#[doc = "*Required features: `\"System_Diagnostics_DevicePortal\"`*"]
#[repr(transparent)]
pub struct DevicePortalConnectionRequestReceivedEventArgs(::windows_core::IUnknown);
impl DevicePortalConnectionRequestReceivedEventArgs {
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn RequestMessage(&self) -> ::windows_core::Result<super::super::super::Web::Http::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn ResponseMessage(&self) -> ::windows_core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsWebSocketUpgradeRequest(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsWebSocketUpgradeRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebSocketProtocolsRequested(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebSocketProtocolsRequested)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = &::windows_core::ComInterface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DevicePortalConnectionRequestReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePortalConnectionRequestReceivedEventArgs {}
impl ::core::fmt::Debug for DevicePortalConnectionRequestReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePortalConnectionRequestReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePortalConnectionRequestReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs;{64dae045-6fda-4459-9ebd-ecce22e38559})");
}
impl ::core::clone::Clone for DevicePortalConnectionRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DevicePortalConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalConnectionRequestReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DevicePortalConnectionRequestReceivedEventArgs {
    const IID: ::windows_core::GUID = <IDevicePortalConnectionRequestReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DevicePortalConnectionRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DevicePortalConnectionRequestReceivedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DevicePortalConnectionRequestReceivedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePortalConnectionRequestReceivedEventArgs {}
#[doc = "*Required features: `\"System_Diagnostics_DevicePortal\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DevicePortalConnectionClosedReason(pub i32);
impl DevicePortalConnectionClosedReason {
    pub const Unknown: Self = Self(0i32);
    pub const ResourceLimitsExceeded: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const NotAuthorized: Self = Self(3i32);
    pub const UserNotPresent: Self = Self(4i32);
    pub const ServiceTerminated: Self = Self(5i32);
}
impl ::core::marker::Copy for DevicePortalConnectionClosedReason {}
impl ::core::clone::Clone for DevicePortalConnectionClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePortalConnectionClosedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DevicePortalConnectionClosedReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DevicePortalConnectionClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePortalConnectionClosedReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePortalConnectionClosedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedReason;i4)");
}
