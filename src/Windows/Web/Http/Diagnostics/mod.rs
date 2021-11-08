#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Web_Http_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProvider(pub ::windows::runtime::IInspectable);
impl HttpDiagnosticProvider {
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn RequestSent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn RemoveRequestSent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn ResponseReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn RemoveResponseReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn RequestResponseCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn RemoveRequestResponseCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "System_Diagnostics")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `System_Diagnostics`*"]
    pub fn CreateFromProcessDiagnosticInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Diagnostics::ProcessDiagnosticInfo>>(processdiagnosticinfo: Param0) -> ::windows::runtime::Result<HttpDiagnosticProvider> {
        Self::IHttpDiagnosticProviderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), processdiagnosticinfo.into_param().abi(), &mut result__).from_abi::<HttpDiagnosticProvider>(result__)
        })
    }
    pub fn IHttpDiagnosticProviderStatics<R, F: FnOnce(&IHttpDiagnosticProviderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpDiagnosticProvider, IHttpDiagnosticProviderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProvider;{bd811501-a056-4d39-b174-833b7b03b02c})");
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3179353345, 41046, 19769, [177, 116, 131, 59, 123, 3, 176, 44]);
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProvider {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProvider";
}
impl ::core::convert::From<HttpDiagnosticProvider> for ::windows::runtime::IUnknown {
    fn from(value: HttpDiagnosticProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProvider> for ::windows::runtime::IUnknown {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProvider> for ::windows::runtime::IInspectable {
    fn from(value: HttpDiagnosticProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProvider> for ::windows::runtime::IInspectable {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProvider {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProvider {}
#[doc = "*Required features: `Web_Http_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(pub ::windows::runtime::IInspectable);
impl HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn ActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn Timestamps(&self) -> ::windows::runtime::Result<HttpDiagnosticProviderRequestResponseTimestamps> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDiagnosticProviderRequestResponseTimestamps>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn RequestedUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn ProcessId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn ThreadId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn Initiator(&self) -> ::windows::runtime::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__: HttpDiagnosticRequestInitiator = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation_Collections`*"]
    pub fn SourceLocations(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs;{735f98ee-94f6-4532-b26e-61e1b1e4efd4})");
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935644910, 38134, 17714, [178, 110, 97, 225, 177, 228, 239, 212]);
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
#[doc = "*Required features: `Web_Http_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProviderRequestResponseTimestamps(pub ::windows::runtime::IInspectable);
impl HttpDiagnosticProviderRequestResponseTimestamps {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn CacheCheckedTimestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn ConnectionInitiatedTimestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn NameResolvedTimestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn SslNegotiatedTimestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn ConnectionCompletedTimestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn RequestSentTimestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn RequestCompletedTimestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn ResponseReceivedTimestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn ResponseCompletedTimestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticProviderRequestResponseTimestamps {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps;{e0afde10-55cf-4c01-91d4-a20557d849f0})");
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3769622032, 21967, 19457, [145, 212, 162, 5, 87, 216, 73, 240]);
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProviderRequestResponseTimestamps {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::runtime::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::runtime::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::runtime::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::runtime::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseTimestamps {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseTimestamps {}
#[doc = "*Required features: `Web_Http_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProviderRequestSentEventArgs(pub ::windows::runtime::IInspectable);
impl HttpDiagnosticProviderRequestSentEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn ActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn ProcessId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn ThreadId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn Initiator(&self) -> ::windows::runtime::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__: HttpDiagnosticRequestInitiator = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation_Collections`*"]
    pub fn SourceLocations(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticProviderRequestSentEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs;{3f5196d0-4c1f-4ebe-a57a-06930771c50d})");
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1062311632, 19487, 20158, [165, 122, 6, 147, 7, 113, 197, 13]);
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProviderRequestSentEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestSentEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestSentEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestSentEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestSentEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestSentEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestSentEventArgs {}
#[doc = "*Required features: `Web_Http_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(pub ::windows::runtime::IInspectable);
impl HttpDiagnosticProviderResponseReceivedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn ActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<super::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HttpResponseMessage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticProviderResponseReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs;{a0a2566c-ab5f-4d66-bb2d-084cf41635d0})");
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2694993516, 43871, 19814, [187, 45, 8, 76, 244, 22, 53, 208]);
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProviderResponseReceivedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderResponseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderResponseReceivedEventArgs {}
#[doc = "*Required features: `Web_Http_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HttpDiagnosticRequestInitiator(pub i32);
impl HttpDiagnosticRequestInitiator {
    pub const ParsedElement: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(0i32);
    pub const Script: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(1i32);
    pub const Image: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(2i32);
    pub const Link: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(3i32);
    pub const Style: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(4i32);
    pub const XmlHttpRequest: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(5i32);
    pub const Media: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(6i32);
    pub const HtmlDownload: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(7i32);
    pub const Prefetch: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(8i32);
    pub const Other: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(9i32);
    pub const CrossOriginPreFlight: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(10i32);
    pub const Fetch: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(11i32);
    pub const Beacon: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(12i32);
}
impl ::core::convert::From<i32> for HttpDiagnosticRequestInitiator {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HttpDiagnosticRequestInitiator {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticRequestInitiator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Diagnostics.HttpDiagnosticRequestInitiator;i4)");
}
impl ::windows::runtime::DefaultType for HttpDiagnosticRequestInitiator {
    type DefaultType = Self;
}
#[doc = "*Required features: `Web_Http_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticSourceLocation(pub ::windows::runtime::IInspectable);
impl HttpDiagnosticSourceLocation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Diagnostics`, `Foundation`*"]
    pub fn SourceUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn LineNumber(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Diagnostics`*"]
    pub fn ColumnNumber(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticSourceLocation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation;{54a9d260-8860-423f-b6fa-d77716f647a7})");
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1420415584, 34912, 16959, [182, 250, 215, 119, 22, 246, 71, 167]);
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticSourceLocation {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation";
}
impl ::core::convert::From<HttpDiagnosticSourceLocation> for ::windows::runtime::IUnknown {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticSourceLocation> for ::windows::runtime::IUnknown {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticSourceLocation> for ::windows::runtime::IInspectable {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticSourceLocation> for ::windows::runtime::IInspectable {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticSourceLocation {}
unsafe impl ::core::marker::Sync for HttpDiagnosticSourceLocation {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct HttpDiagnosticsContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProvider(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3179353345, 41046, 19769, [177, 116, 131, 59, 123, 3, 176, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935644910, 38134, 17714, [178, 110, 97, 225, 177, 228, 239, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3769622032, 21967, 19457, [145, 212, 162, 5, 87, 216, 73, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1062311632, 19487, 20158, [165, 122, 6, 147, 7, 113, 197, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2694993516, 43871, 19814, [187, 45, 8, 76, 244, 22, 53, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProviderStatics {
    type Vtable = IHttpDiagnosticProviderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1535266497, 27244, 18380, [175, 236, 30, 134, 188, 38, 5, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System_Diagnostics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, processdiagnosticinfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System_Diagnostics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticSourceLocation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1420415584, 34912, 16959, [182, 250, 215, 119, 22, 246, 71, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticSourceLocation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
