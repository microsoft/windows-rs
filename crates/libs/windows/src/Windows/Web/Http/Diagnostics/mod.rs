#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProvider(::windows::core::IUnknown);
impl HttpDiagnosticProvider {
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSent<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestSent)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestSent<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRequestSent)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResponseReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResponseReceived)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResponseReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveResponseReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestResponseCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestResponseCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestResponseCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRequestResponseCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"System_Diagnostics\"`*"]
    #[cfg(feature = "System_Diagnostics")]
    pub fn CreateFromProcessDiagnosticInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Diagnostics::ProcessDiagnosticInfo>>(processdiagnosticinfo: Param0) -> ::windows::core::Result<HttpDiagnosticProvider> {
        Self::IHttpDiagnosticProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromProcessDiagnosticInfo)(::core::mem::transmute_copy(this), processdiagnosticinfo.into_param().abi(), &mut result__).from_abi::<HttpDiagnosticProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHttpDiagnosticProviderStatics<R, F: FnOnce(&IHttpDiagnosticProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpDiagnosticProvider, IHttpDiagnosticProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProvider;{bd811501-a056-4d39-b174-833b7b03b02c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_Vtbl;
    const IID: ::windows::core::GUID = <IHttpDiagnosticProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProvider {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProvider";
}
impl ::core::convert::From<HttpDiagnosticProvider> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProvider> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProvider> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProvider> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProvider {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProvider {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(::windows::core::IUnknown);
impl HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn Timestamps(&self) -> ::windows::core::Result<HttpDiagnosticProviderRequestResponseTimestamps> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamps)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDiagnosticProviderRequestResponseTimestamps>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestedUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProcessId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn ThreadId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThreadId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn Initiator(&self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__: HttpDiagnosticRequestInitiator = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Initiator)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceLocations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs;{735f98ee-94f6-4532-b26e-61e1b1e4efd4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHttpDiagnosticProviderRequestResponseCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseTimestamps(::windows::core::IUnknown);
impl HttpDiagnosticProviderRequestResponseTimestamps {
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CacheCheckedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CacheCheckedTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionInitiatedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionInitiatedTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NameResolvedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NameResolvedTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SslNegotiatedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SslNegotiatedTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionCompletedTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSentTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestSentTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestCompletedTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResponseReceivedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResponseReceivedTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResponseCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResponseCompletedTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestResponseTimestamps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProviderRequestResponseTimestamps {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps;{e0afde10-55cf-4c01-91d4-a20557d849f0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl;
    const IID: ::windows::core::GUID = <IHttpDiagnosticProviderRequestResponseTimestamps as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderRequestResponseTimestamps {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseTimestamps {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseTimestamps {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestSentEventArgs(::windows::core::IUnknown);
impl HttpDiagnosticProviderRequestSentEventArgs {
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProcessId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn ThreadId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThreadId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn Initiator(&self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__: HttpDiagnosticRequestInitiator = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Initiator)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceLocations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestSentEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProviderRequestSentEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs;{3f5196d0-4c1f-4ebe-a57a-06930771c50d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHttpDiagnosticProviderRequestSentEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderRequestSentEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestSentEventArgs> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestSentEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestSentEventArgs> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestSentEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestSentEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestSentEventArgs {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(::windows::core::IUnknown);
impl HttpDiagnosticProviderResponseReceivedEventArgs {
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<super::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HttpResponseMessage>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProviderResponseReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs;{a0a2566c-ab5f-4d66-bb2d-084cf41635d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHttpDiagnosticProviderResponseReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderResponseReceivedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderResponseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderResponseReceivedEventArgs {}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for HttpDiagnosticRequestInitiator {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpDiagnosticRequestInitiator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticRequestInitiator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticRequestInitiator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Diagnostics.HttpDiagnosticRequestInitiator;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticSourceLocation(::windows::core::IUnknown);
impl HttpDiagnosticSourceLocation {
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn LineNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
    pub fn ColumnNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColumnNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticSourceLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticSourceLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation;{54a9d260-8860-423f-b6fa-d77716f647a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_Vtbl;
    const IID: ::windows::core::GUID = <IHttpDiagnosticSourceLocation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpDiagnosticSourceLocation {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation";
}
impl ::core::convert::From<HttpDiagnosticSourceLocation> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticSourceLocation> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticSourceLocation> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticSourceLocation> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticSourceLocation {}
unsafe impl ::core::marker::Sync for HttpDiagnosticSourceLocation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd811501_a056_4d39_b174_833b7b03b02c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSent: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestSent: usize,
    #[cfg(feature = "Foundation")]
    pub ResponseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResponseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResponseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResponseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RequestResponseCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x735f98ee_94f6_4532_b26e_61e1b1e4efd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Timestamps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestedUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestedUri: usize,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Initiator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0afde10_55cf_4c01_91d4_a20557d849f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CacheCheckedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CacheCheckedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionInitiatedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionInitiatedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub NameResolvedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NameResolvedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub SslNegotiatedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SslNegotiatedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionCompletedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionCompletedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSentTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSentTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCompletedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCompletedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ResponseReceivedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResponseReceivedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ResponseCompletedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResponseCompletedTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f5196d0_4c1f_4ebe_a57a_06930771c50d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Initiator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0a2566c_ab5f_4d66_bb2d_084cf41635d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderStatics {
    type Vtable = IHttpDiagnosticProviderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b824ec1_6a6c_47cc_afec_1e86bc26053b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System_Diagnostics")]
    pub CreateFromProcessDiagnosticInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processdiagnosticinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System_Diagnostics"))]
    CreateFromProcessDiagnosticInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticSourceLocation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54a9d260_8860_423f_b6fa_d77716f647a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticSourceLocation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceUri: usize,
    pub LineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub ColumnNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
