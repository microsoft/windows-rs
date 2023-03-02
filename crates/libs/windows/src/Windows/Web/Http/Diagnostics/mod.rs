#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_Vtbl;
}
impl ::core::clone::Clone for IHttpDiagnosticProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpDiagnosticProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd811501_a056_4d39_b174_833b7b03b02c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSent: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestSent: usize,
    #[cfg(feature = "Foundation")]
    pub ResponseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResponseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResponseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResponseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RequestResponseCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestResponseCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestResponseCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestResponseCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x735f98ee_94f6_4532_b26e_61e1b1e4efd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Timestamps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestedUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestedUri: usize,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Initiator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl;
}
impl ::core::clone::Clone for IHttpDiagnosticProviderRequestResponseTimestamps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpDiagnosticProviderRequestResponseTimestamps {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0afde10_55cf_4c01_91d4_a20557d849f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CacheCheckedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CacheCheckedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionInitiatedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionInitiatedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub NameResolvedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NameResolvedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub SslNegotiatedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SslNegotiatedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionCompletedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionCompletedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSentTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSentTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCompletedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCompletedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ResponseReceivedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResponseReceivedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ResponseCompletedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResponseCompletedTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_Vtbl;
}
impl ::core::clone::Clone for IHttpDiagnosticProviderRequestSentEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpDiagnosticProviderRequestSentEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f5196d0_4c1f_4ebe_a57a_06930771c50d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Initiator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IHttpDiagnosticProviderResponseReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpDiagnosticProviderResponseReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0a2566c_ab5f_4d66_bb2d_084cf41635d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderStatics {
    type Vtable = IHttpDiagnosticProviderStatics_Vtbl;
}
impl ::core::clone::Clone for IHttpDiagnosticProviderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpDiagnosticProviderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b824ec1_6a6c_47cc_afec_1e86bc26053b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System_Diagnostics")]
    pub CreateFromProcessDiagnosticInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processdiagnosticinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System_Diagnostics"))]
    CreateFromProcessDiagnosticInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticSourceLocation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_Vtbl;
}
impl ::core::clone::Clone for IHttpDiagnosticSourceLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpDiagnosticSourceLocation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54a9d260_8860_423f_b6fa_d77716f647a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticSourceLocation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceUri: usize,
    pub LineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub ColumnNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProvider(::windows::core::IUnknown);
impl HttpDiagnosticProvider {
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSent(&self, handler: &super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).RequestSent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestSent(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRequestSent)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResponseReceived(&self, handler: &super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ResponseReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResponseReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveResponseReceived)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestResponseCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).RequestResponseCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestResponseCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRequestResponseCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"System_Diagnostics\"`*"]
    #[cfg(feature = "System_Diagnostics")]
    pub fn CreateFromProcessDiagnosticInfo(processdiagnosticinfo: &super::super::super::System::Diagnostics::ProcessDiagnosticInfo) -> ::windows::core::Result<HttpDiagnosticProvider> {
        Self::IHttpDiagnosticProviderStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpDiagnosticProvider>();
            (::windows::core::Interface::vtable(this).CreateFromProcessDiagnosticInfo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(processdiagnosticinfo), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHttpDiagnosticProviderStatics<R, F: FnOnce(&IHttpDiagnosticProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpDiagnosticProvider, IHttpDiagnosticProviderStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProvider {}
impl ::core::fmt::Debug for HttpDiagnosticProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProvider").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpDiagnosticProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProvider;{bd811501-a056-4d39-b174-833b7b03b02c})");
}
impl ::core::clone::Clone for HttpDiagnosticProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpDiagnosticProvider {
    const IID: ::windows::core::GUID = <IHttpDiagnosticProvider as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProvider {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProvider";
}
::windows::imp::interface_hierarchy!(HttpDiagnosticProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HttpDiagnosticProvider {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProvider {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(::windows::core::IUnknown);
impl HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ActivityId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Timestamps(&self) -> ::windows::core::Result<HttpDiagnosticProviderRequestResponseTimestamps> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpDiagnosticProviderRequestResponseTimestamps>();
            (::windows::core::Interface::vtable(this).Timestamps)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).RequestedUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ProcessId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ThreadId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ThreadId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Initiator(&self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpDiagnosticRequestInitiator>();
            (::windows::core::Interface::vtable(this).Initiator)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>();
            (::windows::core::Interface::vtable(this).SourceLocations)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
impl ::core::fmt::Debug for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProviderRequestResponseCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs;{735f98ee-94f6-4532-b26e-61e1b1e4efd4})");
}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const IID: ::windows::core::GUID = <IHttpDiagnosticProviderRequestResponseCompletedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs";
}
::windows::imp::interface_hierarchy!(HttpDiagnosticProviderRequestResponseCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseTimestamps(::windows::core::IUnknown);
impl HttpDiagnosticProviderRequestResponseTimestamps {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CacheCheckedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).CacheCheckedTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionInitiatedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).ConnectionInitiatedTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NameResolvedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).NameResolvedTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SslNegotiatedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).SslNegotiatedTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).ConnectionCompletedTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSentTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).RequestSentTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).RequestCompletedTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResponseReceivedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).ResponseReceivedTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResponseCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).ResponseCompletedTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProviderRequestResponseTimestamps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProviderRequestResponseTimestamps {}
impl ::core::fmt::Debug for HttpDiagnosticProviderRequestResponseTimestamps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProviderRequestResponseTimestamps").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpDiagnosticProviderRequestResponseTimestamps {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps;{e0afde10-55cf-4c01-91d4-a20557d849f0})");
}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestResponseTimestamps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpDiagnosticProviderRequestResponseTimestamps {
    const IID: ::windows::core::GUID = <IHttpDiagnosticProviderRequestResponseTimestamps as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderRequestResponseTimestamps {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps";
}
::windows::imp::interface_hierarchy!(HttpDiagnosticProviderRequestResponseTimestamps, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseTimestamps {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseTimestamps {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestSentEventArgs(::windows::core::IUnknown);
impl HttpDiagnosticProviderRequestSentEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ActivityId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HttpRequestMessage>();
            (::windows::core::Interface::vtable(this).Message)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ProcessId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ThreadId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ThreadId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Initiator(&self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpDiagnosticRequestInitiator>();
            (::windows::core::Interface::vtable(this).Initiator)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>();
            (::windows::core::Interface::vtable(this).SourceLocations)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProviderRequestSentEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProviderRequestSentEventArgs {}
impl ::core::fmt::Debug for HttpDiagnosticProviderRequestSentEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProviderRequestSentEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpDiagnosticProviderRequestSentEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs;{3f5196d0-4c1f-4ebe-a57a-06930771c50d})");
}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestSentEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpDiagnosticProviderRequestSentEventArgs {
    const IID: ::windows::core::GUID = <IHttpDiagnosticProviderRequestSentEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderRequestSentEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs";
}
::windows::imp::interface_hierarchy!(HttpDiagnosticProviderRequestSentEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestSentEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestSentEventArgs {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(::windows::core::IUnknown);
impl HttpDiagnosticProviderResponseReceivedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ActivityId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<super::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HttpResponseMessage>();
            (::windows::core::Interface::vtable(this).Message)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProviderResponseReceivedEventArgs {}
impl ::core::fmt::Debug for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProviderResponseReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpDiagnosticProviderResponseReceivedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs;{a0a2566c-ab5f-4d66-bb2d-084cf41635d0})");
}
impl ::core::clone::Clone for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpDiagnosticProviderResponseReceivedEventArgs {
    const IID: ::windows::core::GUID = <IHttpDiagnosticProviderResponseReceivedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderResponseReceivedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs";
}
::windows::imp::interface_hierarchy!(HttpDiagnosticProviderResponseReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HttpDiagnosticProviderResponseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderResponseReceivedEventArgs {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticSourceLocation(::windows::core::IUnknown);
impl HttpDiagnosticSourceLocation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).SourceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LineNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).LineNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ColumnNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).ColumnNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticSourceLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticSourceLocation {}
impl ::core::fmt::Debug for HttpDiagnosticSourceLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticSourceLocation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpDiagnosticSourceLocation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation;{54a9d260-8860-423f-b6fa-d77716f647a7})");
}
impl ::core::clone::Clone for HttpDiagnosticSourceLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpDiagnosticSourceLocation {
    const IID: ::windows::core::GUID = <IHttpDiagnosticSourceLocation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticSourceLocation {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation";
}
::windows::imp::interface_hierarchy!(HttpDiagnosticSourceLocation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HttpDiagnosticSourceLocation {}
unsafe impl ::core::marker::Sync for HttpDiagnosticSourceLocation {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HttpDiagnosticRequestInitiator(pub i32);
impl HttpDiagnosticRequestInitiator {
    pub const ParsedElement: Self = Self(0i32);
    pub const Script: Self = Self(1i32);
    pub const Image: Self = Self(2i32);
    pub const Link: Self = Self(3i32);
    pub const Style: Self = Self(4i32);
    pub const XmlHttpRequest: Self = Self(5i32);
    pub const Media: Self = Self(6i32);
    pub const HtmlDownload: Self = Self(7i32);
    pub const Prefetch: Self = Self(8i32);
    pub const Other: Self = Self(9i32);
    pub const CrossOriginPreFlight: Self = Self(10i32);
    pub const Fetch: Self = Self(11i32);
    pub const Beacon: Self = Self(12i32);
}
impl ::core::marker::Copy for HttpDiagnosticRequestInitiator {}
impl ::core::clone::Clone for HttpDiagnosticRequestInitiator {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpDiagnosticRequestInitiator {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HttpDiagnosticRequestInitiator {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HttpDiagnosticRequestInitiator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticRequestInitiator").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpDiagnosticRequestInitiator {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Diagnostics.HttpDiagnosticRequestInitiator;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
