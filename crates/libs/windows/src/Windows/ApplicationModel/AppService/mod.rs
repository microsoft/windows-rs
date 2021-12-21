#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
pub struct AppServiceCatalog {}
impl AppServiceCatalog {
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAppServiceProvidersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appservicename: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>> {
        Self::IAppServiceCatalogStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppServiceCatalogStatics<R, F: FnOnce(&IAppServiceCatalogStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppServiceCatalog, IAppServiceCatalogStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppServiceCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceCatalog";
}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceClosedEventArgs(::windows::core::IUnknown);
impl AppServiceClosedEventArgs {
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn Status(&self) -> ::windows::core::Result<AppServiceClosedStatus> {
        let this = self;
        unsafe {
            let mut result__: AppServiceClosedStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceClosedStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceClosedEventArgs {}
impl ::core::fmt::Debug for AppServiceClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceClosedEventArgs;{de6016f6-cb03-4d35-ac8d-cc6303239731})");
}
unsafe impl ::windows::core::Interface for AppServiceClosedEventArgs {
    type Vtable = IAppServiceClosedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde6016f6_cb03_4d35_ac8d_cc6303239731);
}
impl ::windows::core::RuntimeName for AppServiceClosedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceClosedEventArgs";
}
impl ::core::convert::From<AppServiceClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppServiceClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppServiceClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppServiceClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppServiceClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceClosedEventArgs {}
unsafe impl ::core::marker::Sync for AppServiceClosedEventArgs {}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceClosedStatus(pub i32);
impl AppServiceClosedStatus {
    pub const Completed: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for AppServiceClosedStatus {}
impl ::core::clone::Clone for AppServiceClosedStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppServiceClosedStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppServiceClosedStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceClosedStatus {}
impl ::core::fmt::Debug for AppServiceClosedStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceClosedStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceClosedStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceClosedStatus;i4)");
}
impl ::windows::core::DefaultType for AppServiceClosedStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceConnection(::windows::core::IUnknown);
impl AppServiceConnection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppServiceConnection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn AppServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn SetAppServiceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn SetPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SendMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, message: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceResponse>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceRequestReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation', 'System_RemoteSystems'*"]
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn OpenRemoteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::RemoteSystems::RemoteSystemConnectionRequest>>(&self, remotesystemconnectionrequest: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>> {
        let this = &::windows::core::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'System'*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'System'*"]
    #[cfg(feature = "System")]
    pub fn SetUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation', 'Foundation_Collections', 'System_RemoteSystems'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn SendStatelessMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, AppServiceConnection>, Param1: ::windows::core::IntoParam<'a, super::super::System::RemoteSystems::RemoteSystemConnectionRequest>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(connection: Param0, connectionrequest: Param1, message: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>> {
        Self::IAppServiceConnectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connection.into_param().abi(), connectionrequest.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAppServiceConnectionStatics<R, F: FnOnce(&IAppServiceConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppServiceConnection, IAppServiceConnectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppServiceConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceConnection {}
impl ::core::fmt::Debug for AppServiceConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceConnection;{9dd474a2-871f-4d52-89a9-9e090531bd27})");
}
unsafe impl ::windows::core::Interface for AppServiceConnection {
    type Vtable = IAppServiceConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dd474a2_871f_4d52_89a9_9e090531bd27);
}
impl ::windows::core::RuntimeName for AppServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceConnection";
}
impl ::core::convert::From<AppServiceConnection> for ::windows::core::IUnknown {
    fn from(value: AppServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceConnection> for ::windows::core::IUnknown {
    fn from(value: &AppServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceConnection> for ::windows::core::IInspectable {
    fn from(value: AppServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceConnection> for ::windows::core::IInspectable {
    fn from(value: &AppServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AppServiceConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AppServiceConnection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AppServiceConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppServiceConnection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppServiceConnection {}
unsafe impl ::core::marker::Sync for AppServiceConnection {}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceConnectionStatus(pub i32);
impl AppServiceConnectionStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const RemoteSystemUnavailable: Self = Self(5i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(6i32);
    pub const NotAuthorized: Self = Self(7i32);
    pub const AuthenticationError: Self = Self(8i32);
    pub const NetworkNotAvailable: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const WebServiceUnavailable: Self = Self(11i32);
}
impl ::core::marker::Copy for AppServiceConnectionStatus {}
impl ::core::clone::Clone for AppServiceConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppServiceConnectionStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppServiceConnectionStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceConnectionStatus {}
impl ::core::fmt::Debug for AppServiceConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceConnectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceConnectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceConnectionStatus;i4)");
}
impl ::windows::core::DefaultType for AppServiceConnectionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceDeferral(::windows::core::IUnknown);
impl AppServiceDeferral {
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AppServiceDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceDeferral {}
impl ::core::fmt::Debug for AppServiceDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceDeferral;{7e1b5322-eab0-4248-ae04-fdf93838e472})");
}
unsafe impl ::windows::core::Interface for AppServiceDeferral {
    type Vtable = IAppServiceDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e1b5322_eab0_4248_ae04_fdf93838e472);
}
impl ::windows::core::RuntimeName for AppServiceDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceDeferral";
}
impl ::core::convert::From<AppServiceDeferral> for ::windows::core::IUnknown {
    fn from(value: AppServiceDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceDeferral> for ::windows::core::IUnknown {
    fn from(value: &AppServiceDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppServiceDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppServiceDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceDeferral> for ::windows::core::IInspectable {
    fn from(value: AppServiceDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceDeferral> for ::windows::core::IInspectable {
    fn from(value: &AppServiceDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppServiceDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppServiceDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceDeferral {}
unsafe impl ::core::marker::Sync for AppServiceDeferral {}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceRequest(::windows::core::IUnknown);
impl AppServiceRequest {
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SendResponseAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, message: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceRequest {}
impl ::core::fmt::Debug for AppServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceRequest;{20e58d9d-18de-4b01-80ba-90a76204e3c8})");
}
unsafe impl ::windows::core::Interface for AppServiceRequest {
    type Vtable = IAppServiceRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20e58d9d_18de_4b01_80ba_90a76204e3c8);
}
impl ::windows::core::RuntimeName for AppServiceRequest {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequest";
}
impl ::core::convert::From<AppServiceRequest> for ::windows::core::IUnknown {
    fn from(value: AppServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &AppServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceRequest> for ::windows::core::IInspectable {
    fn from(value: AppServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &AppServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceRequest {}
unsafe impl ::core::marker::Sync for AppServiceRequest {}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceRequestReceivedEventArgs(::windows::core::IUnknown);
impl AppServiceRequestReceivedEventArgs {
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn Request(&self) -> ::windows::core::Result<AppServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<AppServiceDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceRequestReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceRequestReceivedEventArgs {}
impl ::core::fmt::Debug for AppServiceRequestReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceRequestReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceRequestReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs;{6e122360-ff65-44ae-9e45-857fe4180681})");
}
unsafe impl ::windows::core::Interface for AppServiceRequestReceivedEventArgs {
    type Vtable = IAppServiceRequestReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e122360_ff65_44ae_9e45_857fe4180681);
}
impl ::windows::core::RuntimeName for AppServiceRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs";
}
impl ::core::convert::From<AppServiceRequestReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppServiceRequestReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceRequestReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppServiceRequestReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceRequestReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppServiceRequestReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceRequestReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppServiceRequestReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceRequestReceivedEventArgs {}
unsafe impl ::core::marker::Sync for AppServiceRequestReceivedEventArgs {}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceResponse(::windows::core::IUnknown);
impl AppServiceResponse {
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn Status(&self) -> ::windows::core::Result<AppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__: AppServiceResponseStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceResponseStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceResponse {}
impl ::core::fmt::Debug for AppServiceResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceResponse;{8d503cec-9aa3-4e68-9559-9de63e372ce4})");
}
unsafe impl ::windows::core::Interface for AppServiceResponse {
    type Vtable = IAppServiceResponseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d503cec_9aa3_4e68_9559_9de63e372ce4);
}
impl ::windows::core::RuntimeName for AppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceResponse";
}
impl ::core::convert::From<AppServiceResponse> for ::windows::core::IUnknown {
    fn from(value: AppServiceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceResponse> for ::windows::core::IUnknown {
    fn from(value: &AppServiceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppServiceResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppServiceResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceResponse> for ::windows::core::IInspectable {
    fn from(value: AppServiceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceResponse> for ::windows::core::IInspectable {
    fn from(value: &AppServiceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppServiceResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppServiceResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceResponse {}
unsafe impl ::core::marker::Sync for AppServiceResponse {}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceResponseStatus(pub i32);
impl AppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const Failure: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const MessageSizeTooLarge: Self = Self(5i32);
    pub const AppUnavailable: Self = Self(6i32);
    pub const AuthenticationError: Self = Self(7i32);
    pub const NetworkNotAvailable: Self = Self(8i32);
    pub const DisabledByPolicy: Self = Self(9i32);
    pub const WebServiceUnavailable: Self = Self(10i32);
}
impl ::core::marker::Copy for AppServiceResponseStatus {}
impl ::core::clone::Clone for AppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppServiceResponseStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppServiceResponseStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceResponseStatus {}
impl ::core::fmt::Debug for AppServiceResponseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceResponseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceResponseStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceResponseStatus;i4)");
}
impl ::windows::core::DefaultType for AppServiceResponseStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct AppServiceTriggerDetails(::windows::core::IUnknown);
impl AppServiceTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn AppServiceConnection(&self) -> ::windows::core::Result<AppServiceConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppServiceConnection>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn IsRemoteSystemConnection(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppServiceTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckCallerForCapabilityAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, capabilityname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppServiceTriggerDetails3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), capabilityname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn CallerRemoteConnectionToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppServiceTriggerDetails4>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceTriggerDetails {}
impl ::core::fmt::Debug for AppServiceTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppServiceTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceTriggerDetails;{88a2dcac-ad28-41b8-80bb-bdf1b2169e19})");
}
unsafe impl ::windows::core::Interface for AppServiceTriggerDetails {
    type Vtable = IAppServiceTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88a2dcac_ad28_41b8_80bb_bdf1b2169e19);
}
impl ::windows::core::RuntimeName for AppServiceTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceTriggerDetails";
}
impl ::core::convert::From<AppServiceTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: AppServiceTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &AppServiceTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppServiceTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppServiceTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: AppServiceTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &AppServiceTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppServiceTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppServiceTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceTriggerDetails {}
unsafe impl ::core::marker::Sync for AppServiceTriggerDetails {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceCatalogStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceCatalogStatics {
    type Vtable = IAppServiceCatalogStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef0d2507_d132_4c85_8395_3c31d5a1e941);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceCatalogStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceClosedEventArgs {
    type Vtable = IAppServiceClosedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde6016f6_cb03_4d35_ac8d_cc6303239731);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceClosedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppServiceClosedStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceConnection {
    type Vtable = IAppServiceConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dd474a2_871f_4d52_89a9_9e090531bd27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceConnection2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceConnection2 {
    type Vtable = IAppServiceConnection2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bdfcd5f_2302_4fbd_8061_52511c2f8bf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnection2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceConnectionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceConnectionStatics {
    type Vtable = IAppServiceConnectionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadc56ce9_d408_5673_8637_827a4b274168);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connection: ::windows::core::RawPtr, connectionrequest: ::windows::core::RawPtr, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceDeferral {
    type Vtable = IAppServiceDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e1b5322_eab0_4248_ae04_fdf93838e472);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceRequest {
    type Vtable = IAppServiceRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20e58d9d_18de_4b01_80ba_90a76204e3c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceRequestReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceRequestReceivedEventArgs {
    type Vtable = IAppServiceRequestReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e122360_ff65_44ae_9e45_857fe4180681);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceRequestReceivedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceResponse {
    type Vtable = IAppServiceResponseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d503cec_9aa3_4e68_9559_9de63e372ce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceResponseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppServiceResponseStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceTriggerDetails {
    type Vtable = IAppServiceTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88a2dcac_ad28_41b8_80bb_bdf1b2169e19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceTriggerDetails2 {
    type Vtable = IAppServiceTriggerDetails2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe83d54b2_28cc_43f2_b465_c0482e59e2dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceTriggerDetails3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceTriggerDetails3 {
    type Vtable = IAppServiceTriggerDetails3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbd71e21_7939_4e68_9e3c_7780147aabb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceTriggerDetails4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppServiceTriggerDetails4 {
    type Vtable = IAppServiceTriggerDetails4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1185b180_8861_5e30_ab55_1cf4d08bbf6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStatelessAppServiceResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStatelessAppServiceResponse {
    type Vtable = IStatelessAppServiceResponseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43754af7_a9ec_52fe_82e7_939b68dc9388);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatelessAppServiceResponseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StatelessAppServiceResponseStatus) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct StatelessAppServiceResponse(::windows::core::IUnknown);
impl StatelessAppServiceResponse {
    #[doc = "*Required features: 'ApplicationModel_AppService', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_AppService'*"]
    pub fn Status(&self) -> ::windows::core::Result<StatelessAppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__: StatelessAppServiceResponseStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StatelessAppServiceResponseStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StatelessAppServiceResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StatelessAppServiceResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatelessAppServiceResponse {}
impl ::core::fmt::Debug for StatelessAppServiceResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatelessAppServiceResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StatelessAppServiceResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.StatelessAppServiceResponse;{43754af7-a9ec-52fe-82e7-939b68dc9388})");
}
unsafe impl ::windows::core::Interface for StatelessAppServiceResponse {
    type Vtable = IStatelessAppServiceResponseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43754af7_a9ec_52fe_82e7_939b68dc9388);
}
impl ::windows::core::RuntimeName for StatelessAppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.StatelessAppServiceResponse";
}
impl ::core::convert::From<StatelessAppServiceResponse> for ::windows::core::IUnknown {
    fn from(value: StatelessAppServiceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatelessAppServiceResponse> for ::windows::core::IUnknown {
    fn from(value: &StatelessAppServiceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StatelessAppServiceResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StatelessAppServiceResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StatelessAppServiceResponse> for ::windows::core::IInspectable {
    fn from(value: StatelessAppServiceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatelessAppServiceResponse> for ::windows::core::IInspectable {
    fn from(value: &StatelessAppServiceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StatelessAppServiceResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StatelessAppServiceResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StatelessAppServiceResponse {}
unsafe impl ::core::marker::Sync for StatelessAppServiceResponse {}
#[doc = "*Required features: 'ApplicationModel_AppService'*"]
#[repr(transparent)]
pub struct StatelessAppServiceResponseStatus(pub i32);
impl StatelessAppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(5i32);
    pub const NotAuthorized: Self = Self(6i32);
    pub const ResourceLimitsExceeded: Self = Self(7i32);
    pub const MessageSizeTooLarge: Self = Self(8i32);
    pub const Failure: Self = Self(9i32);
    pub const Unknown: Self = Self(10i32);
    pub const AuthenticationError: Self = Self(11i32);
    pub const NetworkNotAvailable: Self = Self(12i32);
    pub const DisabledByPolicy: Self = Self(13i32);
    pub const WebServiceUnavailable: Self = Self(14i32);
}
impl ::core::marker::Copy for StatelessAppServiceResponseStatus {}
impl ::core::clone::Clone for StatelessAppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StatelessAppServiceResponseStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for StatelessAppServiceResponseStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatelessAppServiceResponseStatus {}
impl ::core::fmt::Debug for StatelessAppServiceResponseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatelessAppServiceResponseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StatelessAppServiceResponseStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.StatelessAppServiceResponseStatus;i4)");
}
impl ::windows::core::DefaultType for StatelessAppServiceResponseStatus {
    type DefaultType = Self;
}
