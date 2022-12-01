#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDevicePortalConnection {
    type Vtable = IDevicePortalConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for IDevicePortalConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f447f51_1198_4da1_8d54_bdef393e09b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnectionClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDevicePortalConnectionClosedEventArgs {
    type Vtable = IDevicePortalConnectionClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IDevicePortalConnectionClosedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcf70e38_7032_428c_9f50_945c15a9f0cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionClosedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePortalConnectionClosedReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDevicePortalConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalConnectionRequestReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IDevicePortalConnectionRequestReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64dae045_6fda_4459_9ebd_ecce22e38559);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Http")]
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    RequestMessage: usize,
    #[cfg(feature = "Web_Http")]
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    ResponseMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnectionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDevicePortalConnectionStatics {
    type Vtable = IDevicePortalConnectionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDevicePortalConnectionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bbe31e7_e9b9_4645_8fed_a53eea0edbd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub GetForAppServiceConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserviceconnection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    GetForAppServiceConnection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDevicePortalWebSocketConnection {
    type Vtable = IDevicePortalWebSocketConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for IDevicePortalWebSocketConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67657920_d65a_42f0_aef4_787808098b7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest2: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: *mut ::core::ffi::c_void, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest3: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, protocol: *mut ::core::ffi::c_void, outboundbuffersizeinbytes: u32, nodelay: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalWebSocketConnectionRequestReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79fdcaba_175c_4739_9f74_dda797c35b3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsWebSocketUpgradeRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub WebSocketProtocolsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebSocketProtocolsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"System_Diagnostics_DevicePortal\"`*"]
#[repr(transparent)]
pub struct DevicePortalConnection(::windows::core::IUnknown);
impl DevicePortalConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestReceived(&self, handler: &super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRequestReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn GetForAppServiceConnection(appserviceconnection: &super::super::super::ApplicationModel::AppService::AppServiceConnection) -> ::windows::core::Result<DevicePortalConnection> {
        Self::IDevicePortalConnectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForAppServiceConnection)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appserviceconnection), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest(&self, request: &super::super::super::Web::Http::HttpRequestMessage) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetServerMessageWebSocketForRequest)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(request), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest2(&self, request: &super::super::super::Web::Http::HttpRequestMessage, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetServerMessageWebSocketForRequest2)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(request), messagetype, ::core::mem::transmute_copy(protocol), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest3(&self, request: &super::super::super::Web::Http::HttpRequestMessage, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &::windows::core::HSTRING, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetServerMessageWebSocketForRequest3)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(request), messagetype, ::core::mem::transmute_copy(protocol), outboundbuffersizeinbytes, maxmessagesize, receivemode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest(&self, request: &super::super::super::Web::Http::HttpRequestMessage) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetServerStreamWebSocketForRequest)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(request), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest2(&self, request: &super::super::super::Web::Http::HttpRequestMessage, protocol: &::windows::core::HSTRING, outboundbuffersizeinbytes: u32, nodelay: bool) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetServerStreamWebSocketForRequest2)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(request), ::core::mem::transmute_copy(protocol), outboundbuffersizeinbytes, nodelay, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IDevicePortalConnectionStatics<R, F: FnOnce(&IDevicePortalConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DevicePortalConnection, IDevicePortalConnectionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DevicePortalConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for DevicePortalConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnection;{0f447f51-1198-4da1-8d54-bdef393e09b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DevicePortalConnection {
    type Vtable = IDevicePortalConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for DevicePortalConnection {
    const IID: ::windows::core::GUID = <IDevicePortalConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DevicePortalConnection {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnection";
}
::windows::core::interface_hierarchy!(DevicePortalConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DevicePortalConnection {}
unsafe impl ::core::marker::Sync for DevicePortalConnection {}
#[doc = "*Required features: `\"System_Diagnostics_DevicePortal\"`*"]
#[repr(transparent)]
pub struct DevicePortalConnectionClosedEventArgs(::windows::core::IUnknown);
impl DevicePortalConnectionClosedEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<DevicePortalConnectionClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for DevicePortalConnectionClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for DevicePortalConnectionClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs;{fcf70e38-7032-428c-9f50-945c15a9f0cb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DevicePortalConnectionClosedEventArgs {
    type Vtable = IDevicePortalConnectionClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for DevicePortalConnectionClosedEventArgs {
    const IID: ::windows::core::GUID = <IDevicePortalConnectionClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DevicePortalConnectionClosedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs";
}
::windows::core::interface_hierarchy!(DevicePortalConnectionClosedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DevicePortalConnectionClosedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePortalConnectionClosedEventArgs {}
#[doc = "*Required features: `\"System_Diagnostics_DevicePortal\"`*"]
#[repr(transparent)]
pub struct DevicePortalConnectionRequestReceivedEventArgs(::windows::core::IUnknown);
impl DevicePortalConnectionRequestReceivedEventArgs {
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn RequestMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn ResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsWebSocketUpgradeRequest(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsWebSocketUpgradeRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebSocketProtocolsRequested(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WebSocketProtocolsRequested)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = &::windows::core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for DevicePortalConnectionRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for DevicePortalConnectionRequestReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs;{64dae045-6fda-4459-9ebd-ecce22e38559})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DevicePortalConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalConnectionRequestReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for DevicePortalConnectionRequestReceivedEventArgs {
    const IID: ::windows::core::GUID = <IDevicePortalConnectionRequestReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DevicePortalConnectionRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs";
}
::windows::core::interface_hierarchy!(DevicePortalConnectionRequestReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Abi for DevicePortalConnectionClosedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for DevicePortalConnectionClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePortalConnectionClosedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePortalConnectionClosedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
