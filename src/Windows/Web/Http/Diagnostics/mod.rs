#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HttpDiagnosticProvider(::windows::runtime::IInspectable);
impl HttpDiagnosticProvider {
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestSent<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::TypedEventHandler<
                HttpDiagnosticProvider,
                HttpDiagnosticProviderRequestSentEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestSent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResponseReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::TypedEventHandler<
                HttpDiagnosticProvider,
                HttpDiagnosticProviderResponseReceivedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveResponseReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestResponseCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::TypedEventHandler<
                HttpDiagnosticProvider,
                HttpDiagnosticProviderRequestResponseCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestResponseCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System_Diagnostics")]
    pub fn CreateFromProcessDiagnosticInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::System::Diagnostics::ProcessDiagnosticInfo,
        >,
    >(
        processdiagnosticinfo: Param0,
    ) -> ::windows::runtime::Result<HttpDiagnosticProvider> {
        Self::IHttpDiagnosticProviderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                processdiagnosticinfo.into_param().abi(),
                &mut result__,
            )
            .from_abi::<HttpDiagnosticProvider>(result__)
        })
    }
    pub fn IHttpDiagnosticProviderStatics<
        R,
        F: FnOnce(&IHttpDiagnosticProviderStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            HttpDiagnosticProvider,
            IHttpDiagnosticProviderStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticProvider {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProvider;{bd811501-a056-4d39-b174-833b7b03b02c})" ) ;
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3179353345,
        41046,
        19769,
        [177, 116, 131, 59, 123, 3, 176, 44],
    );
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProvider {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProvider";
}
impl ::std::convert::From<HttpDiagnosticProvider> for ::windows::runtime::IUnknown {
    fn from(value: HttpDiagnosticProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpDiagnosticProvider> for ::windows::runtime::IUnknown {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HttpDiagnosticProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HttpDiagnosticProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HttpDiagnosticProvider> for ::windows::runtime::IInspectable {
    fn from(value: HttpDiagnosticProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpDiagnosticProvider> for ::windows::runtime::IInspectable {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HttpDiagnosticProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HttpDiagnosticProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HttpDiagnosticProvider {}
unsafe impl ::std::marker::Sync for HttpDiagnosticProvider {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(
    ::windows::runtime::IInspectable,
);
impl HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    pub fn ActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Timestamps(
        &self,
    ) -> ::windows::runtime::Result<HttpDiagnosticProviderRequestResponseTimestamps> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HttpDiagnosticProviderRequestResponseTimestamps>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ThreadId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Initiator(&self) -> ::windows::runtime::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__: HttpDiagnosticRequestInitiator = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Collections::IVectorView<
                HttpDiagnosticSourceLocation,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType
    for HttpDiagnosticProviderRequestResponseCompletedEventArgs
{
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs;{735f98ee-94f6-4532-b26e-61e1b1e4efd4})" ) ;
}
unsafe impl ::windows::runtime::Interface
    for HttpDiagnosticProviderRequestResponseCompletedEventArgs
{
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1935644910,
        38134,
        17714,
        [178, 110, 97, 225, 177, 228, 239, 212],
    );
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const NAME: &'static str =
        "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs";
}
impl ::std::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HttpDiagnosticProviderRequestResponseCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HttpDiagnosticProviderRequestResponseCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HttpDiagnosticProviderRequestResponseCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HttpDiagnosticProviderRequestResponseCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
unsafe impl ::std::marker::Sync for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HttpDiagnosticProviderRequestResponseTimestamps(::windows::runtime::IInspectable);
impl HttpDiagnosticProviderRequestResponseTimestamps {
    #[cfg(feature = "Foundation")]
    pub fn CacheCheckedTimestamp(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::DateTime,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionInitiatedTimestamp(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::DateTime,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NameResolvedTimestamp(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::DateTime,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SslNegotiatedTimestamp(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::DateTime,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionCompletedTimestamp(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::DateTime,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestSentTimestamp(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::DateTime,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestCompletedTimestamp(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::DateTime,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResponseReceivedTimestamp(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::DateTime,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResponseCompletedTimestamp(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::DateTime,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticProviderRequestResponseTimestamps {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps;{e0afde10-55cf-4c01-91d4-a20557d849f0})" ) ;
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3769622032,
        21967,
        19457,
        [145, 212, 162, 5, 87, 216, 73, 240],
    );
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProviderRequestResponseTimestamps {
    const NAME: &'static str =
        "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps";
}
impl ::std::convert::From<HttpDiagnosticProviderRequestResponseTimestamps>
    for ::windows::runtime::IUnknown
{
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps>
    for ::windows::runtime::IUnknown
{
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HttpDiagnosticProviderRequestResponseTimestamps
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HttpDiagnosticProviderRequestResponseTimestamps
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HttpDiagnosticProviderRequestResponseTimestamps>
    for ::windows::runtime::IInspectable
{
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps>
    for ::windows::runtime::IInspectable
{
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HttpDiagnosticProviderRequestResponseTimestamps
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HttpDiagnosticProviderRequestResponseTimestamps
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HttpDiagnosticProviderRequestResponseTimestamps {}
unsafe impl ::std::marker::Sync for HttpDiagnosticProviderRequestResponseTimestamps {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HttpDiagnosticProviderRequestSentEventArgs(::windows::runtime::IInspectable);
impl HttpDiagnosticProviderRequestSentEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Message(&self) -> ::windows::runtime::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::HttpRequestMessage>(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ThreadId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Initiator(&self) -> ::windows::runtime::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__: HttpDiagnosticRequestInitiator = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Collections::IVectorView<
                HttpDiagnosticSourceLocation,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticProviderRequestSentEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs;{3f5196d0-4c1f-4ebe-a57a-06930771c50d})" ) ;
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1062311632,
        19487,
        20158,
        [165, 122, 6, 147, 7, 113, 197, 13],
    );
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProviderRequestSentEventArgs {
    const NAME: &'static str =
        "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs";
}
impl ::std::convert::From<HttpDiagnosticProviderRequestSentEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpDiagnosticProviderRequestSentEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HttpDiagnosticProviderRequestSentEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HttpDiagnosticProviderRequestSentEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HttpDiagnosticProviderRequestSentEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpDiagnosticProviderRequestSentEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HttpDiagnosticProviderRequestSentEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HttpDiagnosticProviderRequestSentEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HttpDiagnosticProviderRequestSentEventArgs {}
unsafe impl ::std::marker::Sync for HttpDiagnosticProviderRequestSentEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(::windows::runtime::IInspectable);
impl HttpDiagnosticProviderResponseReceivedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Message(&self) -> ::windows::runtime::Result<super::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::HttpResponseMessage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticProviderResponseReceivedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs;{a0a2566c-ab5f-4d66-bb2d-084cf41635d0})" ) ;
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2694993516,
        43871,
        19814,
        [187, 45, 8, 76, 244, 22, 53, 208],
    );
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticProviderResponseReceivedEventArgs {
    const NAME: &'static str =
        "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs";
}
impl ::std::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HttpDiagnosticProviderResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HttpDiagnosticProviderResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HttpDiagnosticProviderResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HttpDiagnosticProviderResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HttpDiagnosticProviderResponseReceivedEventArgs {}
unsafe impl ::std::marker::Sync for HttpDiagnosticProviderResponseReceivedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    pub const CrossOriginPreFlight: HttpDiagnosticRequestInitiator =
        HttpDiagnosticRequestInitiator(10i32);
    pub const Fetch: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(11i32);
    pub const Beacon: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(12i32);
}
impl ::std::convert::From<i32> for HttpDiagnosticRequestInitiator {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HttpDiagnosticRequestInitiator {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticRequestInitiator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Web.Http.Diagnostics.HttpDiagnosticRequestInitiator;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HttpDiagnosticSourceLocation(::windows::runtime::IInspectable);
impl HttpDiagnosticSourceLocation {
    #[cfg(feature = "Foundation")]
    pub fn SourceUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    pub fn LineNumber(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    pub fn ColumnNumber(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDiagnosticSourceLocation {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation;{54a9d260-8860-423f-b6fa-d77716f647a7})" ) ;
}
unsafe impl ::windows::runtime::Interface for HttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1420415584,
        34912,
        16959,
        [182, 250, 215, 119, 22, 246, 71, 167],
    );
}
impl ::windows::runtime::RuntimeName for HttpDiagnosticSourceLocation {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation";
}
impl ::std::convert::From<HttpDiagnosticSourceLocation> for ::windows::runtime::IUnknown {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpDiagnosticSourceLocation> for ::windows::runtime::IUnknown {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HttpDiagnosticSourceLocation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HttpDiagnosticSourceLocation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HttpDiagnosticSourceLocation> for ::windows::runtime::IInspectable {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpDiagnosticSourceLocation> for ::windows::runtime::IInspectable {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HttpDiagnosticSourceLocation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HttpDiagnosticSourceLocation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HttpDiagnosticSourceLocation {}
unsafe impl ::std::marker::Sync for HttpDiagnosticSourceLocation {}
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct HttpDiagnosticsContract(pub u8);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHttpDiagnosticProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3179353345,
        41046,
        19769,
        [177, 116, 131, 59, 123, 3, 176, 44],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProvider_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs(
    ::windows::runtime::IInspectable,
);
unsafe impl ::windows::runtime::Interface
    for IHttpDiagnosticProviderRequestResponseCompletedEventArgs
{
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1935644910,
        38134,
        17714,
        [178, 110, 97, 225, 177, 228, 239, 212],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut HttpDiagnosticRequestInitiator,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3769622032,
        21967,
        19457,
        [145, 212, 162, 5, 87, 216, 73, 240],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1062311632,
        19487,
        20158,
        [165, 122, 6, 147, 7, 113, 197, 13],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::DateTime,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut HttpDiagnosticRequestInitiator,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2694993516,
        43871,
        19814,
        [187, 45, 8, 76, 244, 22, 53, 208],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::DateTime,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticProviderStatics {
    type Vtable = IHttpDiagnosticProviderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1535266497,
        27244,
        18380,
        [175, 236, 30, 134, 188, 38, 5, 59],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System_Diagnostics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        processdiagnosticinfo: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System_Diagnostics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHttpDiagnosticSourceLocation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1420415584,
        34912,
        16959,
        [182, 250, 215, 119, 22, 246, 71, 167],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticSourceLocation_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u64,
    ) -> ::windows::runtime::HRESULT,
);
