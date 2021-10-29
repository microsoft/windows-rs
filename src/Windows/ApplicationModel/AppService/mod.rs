#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub struct AppServiceCatalog {}
impl AppServiceCatalog {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAppServiceProvidersAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appservicename: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>> {
        Self::IAppServiceCatalogStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), appservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>>(result__)
        })
    }
    pub fn IAppServiceCatalogStatics<R, F: FnOnce(&IAppServiceCatalogStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppServiceCatalog, IAppServiceCatalogStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AppServiceCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceCatalog";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppServiceClosedEventArgs(::windows::runtime::IInspectable);
impl AppServiceClosedEventArgs {
    pub fn Status(&self) -> ::windows::runtime::Result<AppServiceClosedStatus> {
        let this = self;
        unsafe {
            let mut result__: AppServiceClosedStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceClosedStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceClosedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceClosedEventArgs;{de6016f6-cb03-4d35-ac8d-cc6303239731})");
}
unsafe impl ::windows::runtime::Interface for AppServiceClosedEventArgs {
    type Vtable = IAppServiceClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3730839286, 51971, 19765, [172, 141, 204, 99, 3, 35, 151, 49]);
}
impl ::windows::runtime::RuntimeName for AppServiceClosedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceClosedEventArgs";
}
impl ::std::convert::From<AppServiceClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppServiceClosedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppServiceClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppServiceClosedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppServiceClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppServiceClosedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppServiceClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppServiceClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppServiceClosedEventArgs {}
unsafe impl ::std::marker::Sync for AppServiceClosedEventArgs {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppServiceClosedStatus(pub i32);
impl AppServiceClosedStatus {
    pub const Completed: AppServiceClosedStatus = AppServiceClosedStatus(0i32);
    pub const Canceled: AppServiceClosedStatus = AppServiceClosedStatus(1i32);
    pub const ResourceLimitsExceeded: AppServiceClosedStatus = AppServiceClosedStatus(2i32);
    pub const Unknown: AppServiceClosedStatus = AppServiceClosedStatus(3i32);
}
impl ::std::convert::From<i32> for AppServiceClosedStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppServiceClosedStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceClosedStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceClosedStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppServiceConnection(::windows::runtime::IInspectable);
impl AppServiceConnection {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppServiceConnection, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AppServiceName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAppServiceName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SendMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, message: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AppServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceRequestReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ServiceClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceClosedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn OpenRemoteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::RemoteSystems::RemoteSystemConnectionRequest>>(&self, remotesystemconnectionrequest: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>> {
        let this = &::windows::runtime::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn SendStatelessMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, AppServiceConnection>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::RemoteSystems::RemoteSystemConnectionRequest>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(connection: Param0, connectionrequest: Param1, message: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>> {
        Self::IAppServiceConnectionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), connection.into_param().abi(), connectionrequest.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>>(result__)
        })
    }
    pub fn IAppServiceConnectionStatics<R, F: FnOnce(&IAppServiceConnectionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppServiceConnection, IAppServiceConnectionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceConnection;{9dd474a2-871f-4d52-89a9-9e090531bd27})");
}
unsafe impl ::windows::runtime::Interface for AppServiceConnection {
    type Vtable = IAppServiceConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2647946402, 34591, 19794, [137, 169, 158, 9, 5, 49, 189, 39]);
}
impl ::windows::runtime::RuntimeName for AppServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceConnection";
}
impl ::std::convert::From<AppServiceConnection> for ::windows::runtime::IUnknown {
    fn from(value: AppServiceConnection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppServiceConnection> for ::windows::runtime::IUnknown {
    fn from(value: &AppServiceConnection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppServiceConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppServiceConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppServiceConnection> for ::windows::runtime::IInspectable {
    fn from(value: AppServiceConnection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppServiceConnection> for ::windows::runtime::IInspectable {
    fn from(value: &AppServiceConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppServiceConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppServiceConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AppServiceConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppServiceConnection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AppServiceConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppServiceConnection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for AppServiceConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &AppServiceConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AppServiceConnection {}
unsafe impl ::std::marker::Sync for AppServiceConnection {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppServiceConnectionStatus(pub i32);
impl AppServiceConnectionStatus {
    pub const Success: AppServiceConnectionStatus = AppServiceConnectionStatus(0i32);
    pub const AppNotInstalled: AppServiceConnectionStatus = AppServiceConnectionStatus(1i32);
    pub const AppUnavailable: AppServiceConnectionStatus = AppServiceConnectionStatus(2i32);
    pub const AppServiceUnavailable: AppServiceConnectionStatus = AppServiceConnectionStatus(3i32);
    pub const Unknown: AppServiceConnectionStatus = AppServiceConnectionStatus(4i32);
    pub const RemoteSystemUnavailable: AppServiceConnectionStatus = AppServiceConnectionStatus(5i32);
    pub const RemoteSystemNotSupportedByApp: AppServiceConnectionStatus = AppServiceConnectionStatus(6i32);
    pub const NotAuthorized: AppServiceConnectionStatus = AppServiceConnectionStatus(7i32);
    pub const AuthenticationError: AppServiceConnectionStatus = AppServiceConnectionStatus(8i32);
    pub const NetworkNotAvailable: AppServiceConnectionStatus = AppServiceConnectionStatus(9i32);
    pub const DisabledByPolicy: AppServiceConnectionStatus = AppServiceConnectionStatus(10i32);
    pub const WebServiceUnavailable: AppServiceConnectionStatus = AppServiceConnectionStatus(11i32);
}
impl ::std::convert::From<i32> for AppServiceConnectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppServiceConnectionStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceConnectionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceConnectionStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppServiceDeferral(::windows::runtime::IInspectable);
impl AppServiceDeferral {
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceDeferral;{7e1b5322-eab0-4248-ae04-fdf93838e472})");
}
unsafe impl ::windows::runtime::Interface for AppServiceDeferral {
    type Vtable = IAppServiceDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2115719970, 60080, 16968, [174, 4, 253, 249, 56, 56, 228, 114]);
}
impl ::windows::runtime::RuntimeName for AppServiceDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceDeferral";
}
impl ::std::convert::From<AppServiceDeferral> for ::windows::runtime::IUnknown {
    fn from(value: AppServiceDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppServiceDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &AppServiceDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppServiceDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppServiceDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppServiceDeferral> for ::windows::runtime::IInspectable {
    fn from(value: AppServiceDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppServiceDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &AppServiceDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppServiceDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppServiceDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppServiceDeferral {}
unsafe impl ::std::marker::Sync for AppServiceDeferral {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppServiceRequest(::windows::runtime::IInspectable);
impl AppServiceRequest {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SendResponseAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, message: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceRequest;{20e58d9d-18de-4b01-80ba-90a76204e3c8})");
}
unsafe impl ::windows::runtime::Interface for AppServiceRequest {
    type Vtable = IAppServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(551914909, 6366, 19201, [128, 186, 144, 167, 98, 4, 227, 200]);
}
impl ::windows::runtime::RuntimeName for AppServiceRequest {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequest";
}
impl ::std::convert::From<AppServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: AppServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &AppServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: AppServiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &AppServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppServiceRequest {}
unsafe impl ::std::marker::Sync for AppServiceRequest {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppServiceRequestReceivedEventArgs(::windows::runtime::IInspectable);
impl AppServiceRequestReceivedEventArgs {
    pub fn Request(&self) -> ::windows::runtime::Result<AppServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<AppServiceDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceRequestReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs;{6e122360-ff65-44ae-9e45-857fe4180681})");
}
unsafe impl ::windows::runtime::Interface for AppServiceRequestReceivedEventArgs {
    type Vtable = IAppServiceRequestReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1846682464, 65381, 17582, [158, 69, 133, 127, 228, 24, 6, 129]);
}
impl ::windows::runtime::RuntimeName for AppServiceRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs";
}
impl ::std::convert::From<AppServiceRequestReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppServiceRequestReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppServiceRequestReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppServiceRequestReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppServiceRequestReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppServiceRequestReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppServiceRequestReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppServiceRequestReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppServiceRequestReceivedEventArgs {}
unsafe impl ::std::marker::Sync for AppServiceRequestReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppServiceResponse(::windows::runtime::IInspectable);
impl AppServiceResponse {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::runtime::Result<AppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__: AppServiceResponseStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceResponseStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceResponse {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceResponse;{8d503cec-9aa3-4e68-9559-9de63e372ce4})");
}
unsafe impl ::windows::runtime::Interface for AppServiceResponse {
    type Vtable = IAppServiceResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2370845932, 39587, 20072, [149, 89, 157, 230, 62, 55, 44, 228]);
}
impl ::windows::runtime::RuntimeName for AppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceResponse";
}
impl ::std::convert::From<AppServiceResponse> for ::windows::runtime::IUnknown {
    fn from(value: AppServiceResponse) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppServiceResponse> for ::windows::runtime::IUnknown {
    fn from(value: &AppServiceResponse) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppServiceResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppServiceResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppServiceResponse> for ::windows::runtime::IInspectable {
    fn from(value: AppServiceResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppServiceResponse> for ::windows::runtime::IInspectable {
    fn from(value: &AppServiceResponse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppServiceResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppServiceResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppServiceResponse {}
unsafe impl ::std::marker::Sync for AppServiceResponse {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppServiceResponseStatus(pub i32);
impl AppServiceResponseStatus {
    pub const Success: AppServiceResponseStatus = AppServiceResponseStatus(0i32);
    pub const Failure: AppServiceResponseStatus = AppServiceResponseStatus(1i32);
    pub const ResourceLimitsExceeded: AppServiceResponseStatus = AppServiceResponseStatus(2i32);
    pub const Unknown: AppServiceResponseStatus = AppServiceResponseStatus(3i32);
    pub const RemoteSystemUnavailable: AppServiceResponseStatus = AppServiceResponseStatus(4i32);
    pub const MessageSizeTooLarge: AppServiceResponseStatus = AppServiceResponseStatus(5i32);
    pub const AppUnavailable: AppServiceResponseStatus = AppServiceResponseStatus(6i32);
    pub const AuthenticationError: AppServiceResponseStatus = AppServiceResponseStatus(7i32);
    pub const NetworkNotAvailable: AppServiceResponseStatus = AppServiceResponseStatus(8i32);
    pub const DisabledByPolicy: AppServiceResponseStatus = AppServiceResponseStatus(9i32);
    pub const WebServiceUnavailable: AppServiceResponseStatus = AppServiceResponseStatus(10i32);
}
impl ::std::convert::From<i32> for AppServiceResponseStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppServiceResponseStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceResponseStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceResponseStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppServiceTriggerDetails(::windows::runtime::IInspectable);
impl AppServiceTriggerDetails {
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn AppServiceConnection(&self) -> ::windows::runtime::Result<AppServiceConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceConnection>(result__)
        }
    }
    pub fn IsRemoteSystemConnection(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppServiceTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CheckCallerForCapabilityAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, capabilityname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IAppServiceTriggerDetails3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), capabilityname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CallerRemoteConnectionToken(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppServiceTriggerDetails4>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppServiceTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceTriggerDetails;{88a2dcac-ad28-41b8-80bb-bdf1b2169e19})");
}
unsafe impl ::windows::runtime::Interface for AppServiceTriggerDetails {
    type Vtable = IAppServiceTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2292374700, 44328, 16824, [128, 187, 189, 241, 178, 22, 158, 25]);
}
impl ::windows::runtime::RuntimeName for AppServiceTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceTriggerDetails";
}
impl ::std::convert::From<AppServiceTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: AppServiceTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppServiceTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &AppServiceTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppServiceTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppServiceTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppServiceTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: AppServiceTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppServiceTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &AppServiceTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppServiceTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppServiceTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppServiceTriggerDetails {}
unsafe impl ::std::marker::Sync for AppServiceTriggerDetails {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceCatalogStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceCatalogStatics {
    type Vtable = IAppServiceCatalogStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4010616071, 53554, 19589, [131, 149, 60, 49, 213, 161, 233, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceCatalogStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appservicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceClosedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceClosedEventArgs {
    type Vtable = IAppServiceClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3730839286, 51971, 19765, [172, 141, 204, 99, 3, 35, 151, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppServiceClosedStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceConnection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceConnection {
    type Vtable = IAppServiceConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2647946402, 34591, 19794, [137, 169, 158, 9, 5, 49, 189, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceConnection2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceConnection2 {
    type Vtable = IAppServiceConnection2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2346700127, 8962, 20413, [128, 97, 82, 81, 28, 47, 139, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnection2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotesystemconnectionrequest: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceConnectionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceConnectionStatics {
    type Vtable = IAppServiceConnectionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2915396841, 54280, 22131, [134, 55, 130, 122, 75, 39, 65, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connection: ::windows::runtime::RawPtr, connectionrequest: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceDeferral {
    type Vtable = IAppServiceDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2115719970, 60080, 16968, [174, 4, 253, 249, 56, 56, 228, 114]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceRequest {
    type Vtable = IAppServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(551914909, 6366, 19201, [128, 186, 144, 167, 98, 4, 227, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceRequestReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceRequestReceivedEventArgs {
    type Vtable = IAppServiceRequestReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1846682464, 65381, 17582, [158, 69, 133, 127, 228, 24, 6, 129]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceRequestReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceResponse(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceResponse {
    type Vtable = IAppServiceResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2370845932, 39587, 20072, [149, 89, 157, 230, 62, 55, 44, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceResponse_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppServiceResponseStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceTriggerDetails {
    type Vtable = IAppServiceTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2292374700, 44328, 16824, [128, 187, 189, 241, 178, 22, 158, 25]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceTriggerDetails2 {
    type Vtable = IAppServiceTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3896333490, 10444, 17394, [180, 101, 192, 72, 46, 89, 226, 220]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceTriggerDetails3 {
    type Vtable = IAppServiceTriggerDetails3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4225179169, 31033, 20072, [158, 60, 119, 128, 20, 122, 171, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilityname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppServiceTriggerDetails4 {
    type Vtable = IAppServiceTriggerDetails4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(293974400, 34913, 24112, [171, 85, 28, 244, 208, 139, 191, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStatelessAppServiceResponse(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStatelessAppServiceResponse {
    type Vtable = IStatelessAppServiceResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1131760375, 43500, 21246, [130, 231, 147, 155, 104, 220, 147, 136]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatelessAppServiceResponse_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut StatelessAppServiceResponseStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct StatelessAppServiceResponse(::windows::runtime::IInspectable);
impl StatelessAppServiceResponse {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::runtime::Result<StatelessAppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__: StatelessAppServiceResponseStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<StatelessAppServiceResponseStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StatelessAppServiceResponse {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.StatelessAppServiceResponse;{43754af7-a9ec-52fe-82e7-939b68dc9388})");
}
unsafe impl ::windows::runtime::Interface for StatelessAppServiceResponse {
    type Vtable = IStatelessAppServiceResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1131760375, 43500, 21246, [130, 231, 147, 155, 104, 220, 147, 136]);
}
impl ::windows::runtime::RuntimeName for StatelessAppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.StatelessAppServiceResponse";
}
impl ::std::convert::From<StatelessAppServiceResponse> for ::windows::runtime::IUnknown {
    fn from(value: StatelessAppServiceResponse) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&StatelessAppServiceResponse> for ::windows::runtime::IUnknown {
    fn from(value: &StatelessAppServiceResponse) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for StatelessAppServiceResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &StatelessAppServiceResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<StatelessAppServiceResponse> for ::windows::runtime::IInspectable {
    fn from(value: StatelessAppServiceResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StatelessAppServiceResponse> for ::windows::runtime::IInspectable {
    fn from(value: &StatelessAppServiceResponse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for StatelessAppServiceResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a StatelessAppServiceResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for StatelessAppServiceResponse {}
unsafe impl ::std::marker::Sync for StatelessAppServiceResponse {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct StatelessAppServiceResponseStatus(pub i32);
impl StatelessAppServiceResponseStatus {
    pub const Success: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(0i32);
    pub const AppNotInstalled: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(1i32);
    pub const AppUnavailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(2i32);
    pub const AppServiceUnavailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(3i32);
    pub const RemoteSystemUnavailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(4i32);
    pub const RemoteSystemNotSupportedByApp: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(5i32);
    pub const NotAuthorized: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(6i32);
    pub const ResourceLimitsExceeded: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(7i32);
    pub const MessageSizeTooLarge: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(8i32);
    pub const Failure: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(9i32);
    pub const Unknown: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(10i32);
    pub const AuthenticationError: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(11i32);
    pub const NetworkNotAvailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(12i32);
    pub const DisabledByPolicy: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(13i32);
    pub const WebServiceUnavailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(14i32);
}
impl ::std::convert::From<i32> for StatelessAppServiceResponseStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for StatelessAppServiceResponseStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for StatelessAppServiceResponseStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.StatelessAppServiceResponseStatus;i4)");
}
