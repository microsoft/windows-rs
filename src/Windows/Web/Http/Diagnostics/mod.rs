#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProvider(pub ::windows::core::IInspectable);
impl HttpDiagnosticProvider {
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestSent<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestSent<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResponseReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveResponseReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestResponseCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestResponseCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "System_Diagnostics")]
    pub fn CreateFromProcessDiagnosticInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Diagnostics::ProcessDiagnosticInfo>>(processdiagnosticinfo: Param0) -> ::windows::core::Result<HttpDiagnosticProvider> {
        Self::IHttpDiagnosticProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), processdiagnosticinfo.into_param().abi(), &mut result__).from_abi::<HttpDiagnosticProvider>(result__)
        })
    }
    pub fn IHttpDiagnosticProviderStatics<R, F: FnOnce(&IHttpDiagnosticProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpDiagnosticProvider, IHttpDiagnosticProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProvider;{bd811501-a056-4d39-b174-833b7b03b02c})");
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd811501_a056_4d39_b174_833b7b03b02c);
}
impl ::windows::core::RuntimeName for HttpDiagnosticProvider {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProvider";
}
impl ::core::convert::From<HttpDiagnosticProvider> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProvider> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProvider> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProvider> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProvider {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProvider {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(pub ::windows::core::IInspectable);
impl HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Timestamps(&self) -> ::windows::core::Result<HttpDiagnosticProviderRequestResponseTimestamps> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDiagnosticProviderRequestResponseTimestamps>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ThreadId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Initiator(&self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__: HttpDiagnosticRequestInitiator = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs;{735f98ee-94f6-4532-b26e-61e1b1e4efd4})");
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x735f98ee_94f6_4532_b26e_61e1b1e4efd4);
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProviderRequestResponseTimestamps(pub ::windows::core::IInspectable);
impl HttpDiagnosticProviderRequestResponseTimestamps {
    #[cfg(feature = "Foundation")]
    pub fn CacheCheckedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionInitiatedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NameResolvedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SslNegotiatedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestSentTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResponseReceivedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResponseCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProviderRequestResponseTimestamps {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps;{e0afde10-55cf-4c01-91d4-a20557d849f0})");
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0afde10_55cf_4c01_91d4_a20557d849f0);
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderRequestResponseTimestamps {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseTimestamps {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseTimestamps {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProviderRequestSentEventArgs(pub ::windows::core::IInspectable);
impl HttpDiagnosticProviderRequestSentEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HttpRequestMessage>(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ThreadId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Initiator(&self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__: HttpDiagnosticRequestInitiator = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProviderRequestSentEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs;{3f5196d0-4c1f-4ebe-a57a-06930771c50d})");
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f5196d0_4c1f_4ebe_a57a_06930771c50d);
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderRequestSentEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestSentEventArgs> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestSentEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestSentEventArgs> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestSentEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestSentEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestSentEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(pub ::windows::core::IInspectable);
impl HttpDiagnosticProviderResponseReceivedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<super::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HttpResponseMessage>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticProviderResponseReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs;{a0a2566c-ab5f-4d66-bb2d-084cf41635d0})");
}
unsafe impl ::windows::core::Interface for HttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0a2566c_ab5f_4d66_bb2d_084cf41635d0);
}
impl ::windows::core::RuntimeName for HttpDiagnosticProviderResponseReceivedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderResponseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderResponseReceivedEventArgs {}
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
unsafe impl ::windows::core::Abi for HttpDiagnosticRequestInitiator {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticRequestInitiator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Diagnostics.HttpDiagnosticRequestInitiator;i4)");
}
impl ::windows::core::DefaultType for HttpDiagnosticRequestInitiator {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HttpDiagnosticSourceLocation(pub ::windows::core::IInspectable);
impl HttpDiagnosticSourceLocation {
    #[cfg(feature = "Foundation")]
    pub fn SourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    pub fn LineNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn ColumnNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HttpDiagnosticSourceLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation;{54a9d260-8860-423f-b6fa-d77716f647a7})");
}
unsafe impl ::windows::core::Interface for HttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54a9d260_8860_423f_b6fa_d77716f647a7);
}
impl ::windows::core::RuntimeName for HttpDiagnosticSourceLocation {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation";
}
impl ::core::convert::From<HttpDiagnosticSourceLocation> for ::windows::core::IUnknown {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HttpDiagnosticSourceLocation> for ::windows::core::IUnknown {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HttpDiagnosticSourceLocation> for ::windows::core::IInspectable {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HttpDiagnosticSourceLocation> for ::windows::core::IInspectable {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticSourceLocation {}
unsafe impl ::core::marker::Sync for HttpDiagnosticSourceLocation {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct HttpDiagnosticsContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd811501_a056_4d39_b174_833b7b03b02c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
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
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x735f98ee_94f6_4532_b26e_61e1b1e4efd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0afde10_55cf_4c01_91d4_a20557d849f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f5196d0_4c1f_4ebe_a57a_06930771c50d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0a2566c_ab5f_4d66_bb2d_084cf41635d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHttpDiagnosticProviderStatics {
    type Vtable = IHttpDiagnosticProviderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b824ec1_6a6c_47cc_afec_1e86bc26053b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System_Diagnostics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, processdiagnosticinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System_Diagnostics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDiagnosticSourceLocation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54a9d260_8860_423f_b6fa_d77716f647a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticSourceLocation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
