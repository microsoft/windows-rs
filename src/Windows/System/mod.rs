#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "System_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "System_Display")]
pub mod Display;
#[cfg(feature = "System_Implementation")]
pub mod Implementation;
#[cfg(feature = "System_Inventory")]
pub mod Inventory;
#[cfg(feature = "System_Power")]
pub mod Power;
#[cfg(feature = "System_Preview")]
pub mod Preview;
#[cfg(feature = "System_Profile")]
pub mod Profile;
#[cfg(feature = "System_RemoteDesktop")]
pub mod RemoteDesktop;
#[cfg(feature = "System_RemoteSystems")]
pub mod RemoteSystems;
#[cfg(feature = "System_Threading")]
pub mod Threading;
#[cfg(feature = "System_Update")]
pub mod Update;
#[cfg(feature = "System_UserProfile")]
pub mod UserProfile;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppActivationResult(pub ::windows::core::IInspectable);
impl AppActivationResult {
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppActivationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppActivationResult;{6b528900-f46e-4eb0-aa6c-38af557cf9ed})");
}
unsafe impl ::windows::core::Interface for AppActivationResult {
    type Vtable = IAppActivationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b528900_f46e_4eb0_aa6c_38af557cf9ed);
}
impl ::windows::core::RuntimeName for AppActivationResult {
    const NAME: &'static str = "Windows.System.AppActivationResult";
}
impl ::core::convert::From<AppActivationResult> for ::windows::core::IUnknown {
    fn from(value: AppActivationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppActivationResult> for ::windows::core::IUnknown {
    fn from(value: &AppActivationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppActivationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppActivationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppActivationResult> for ::windows::core::IInspectable {
    fn from(value: AppActivationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppActivationResult> for ::windows::core::IInspectable {
    fn from(value: &AppActivationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppActivationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppActivationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppActivationResult {}
unsafe impl ::core::marker::Sync for AppActivationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppDiagnosticInfo(pub ::windows::core::IInspectable);
impl AppDiagnosticInfo {
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> ::windows::core::Result<super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ApplicationModel::AppInfo>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestInfoAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResourceGroups(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>> {
        let this = &::windows::core::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppResourceGroupInfo>>(result__)
        }
    }
    pub fn CreateResourceGroupWatcher(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcher> {
        let this = &::windows::core::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfoWatcher>(result__)
        }
    }
    pub fn CreateWatcher() -> ::windows::core::Result<AppDiagnosticInfoWatcher> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDiagnosticInfoWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestInfoForPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagefamilyname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestInfoForAppAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestInfoForAppUserModelId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appusermodelid: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), appusermodelid.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppActivationResult>> {
        let this = &::windows::core::Interface::cast::<IAppDiagnosticInfo3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppActivationResult>>(result__)
        }
    }
    pub fn IAppDiagnosticInfoStatics<R, F: FnOnce(&IAppDiagnosticInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppDiagnosticInfoStatics2<R, F: FnOnce(&IAppDiagnosticInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfo;{e348a69a-8889-4ca3-be07-d5ffff5f0804})");
}
unsafe impl ::windows::core::Interface for AppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe348a69a_8889_4ca3_be07_d5ffff5f0804);
}
impl ::windows::core::RuntimeName for AppDiagnosticInfo {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfo";
}
impl ::core::convert::From<AppDiagnosticInfo> for ::windows::core::IUnknown {
    fn from(value: AppDiagnosticInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppDiagnosticInfo> for ::windows::core::IUnknown {
    fn from(value: &AppDiagnosticInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppDiagnosticInfo> for ::windows::core::IInspectable {
    fn from(value: AppDiagnosticInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppDiagnosticInfo> for ::windows::core::IInspectable {
    fn from(value: &AppDiagnosticInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfo {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppDiagnosticInfoWatcher(pub ::windows::core::IInspectable);
impl AppDiagnosticInfoWatcher {
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<AppDiagnosticInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: AppDiagnosticInfoWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDiagnosticInfoWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfoWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcher;{75575070-01d3-489a-9325-52f9cc6ede0a})");
}
unsafe impl ::windows::core::Interface for AppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75575070_01d3_489a_9325_52f9cc6ede0a);
}
impl ::windows::core::RuntimeName for AppDiagnosticInfoWatcher {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcher";
}
impl ::core::convert::From<AppDiagnosticInfoWatcher> for ::windows::core::IUnknown {
    fn from(value: AppDiagnosticInfoWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcher> for ::windows::core::IUnknown {
    fn from(value: &AppDiagnosticInfoWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppDiagnosticInfoWatcher> for ::windows::core::IInspectable {
    fn from(value: AppDiagnosticInfoWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcher> for ::windows::core::IInspectable {
    fn from(value: &AppDiagnosticInfoWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcher {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppDiagnosticInfoWatcherEventArgs(pub ::windows::core::IInspectable);
impl AppDiagnosticInfoWatcherEventArgs {
    pub fn AppDiagnosticInfo(&self) -> ::windows::core::Result<AppDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDiagnosticInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfoWatcherEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcherEventArgs;{7017c716-e1da-4c65-99df-046dff5be71a})");
}
unsafe impl ::windows::core::Interface for AppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7017c716_e1da_4c65_99df_046dff5be71a);
}
impl ::windows::core::RuntimeName for AppDiagnosticInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcherEventArgs";
}
impl ::core::convert::From<AppDiagnosticInfoWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppDiagnosticInfoWatcherEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppDiagnosticInfoWatcherEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppDiagnosticInfoWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppDiagnosticInfoWatcherEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppDiagnosticInfoWatcherEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcherEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcherStatus(pub i32);
impl AppDiagnosticInfoWatcherStatus {
    pub const Created: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(0i32);
    pub const Started: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(1i32);
    pub const EnumerationCompleted: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(2i32);
    pub const Stopping: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(3i32);
    pub const Stopped: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(4i32);
    pub const Aborted: AppDiagnosticInfoWatcherStatus = AppDiagnosticInfoWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for AppDiagnosticInfoWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppDiagnosticInfoWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfoWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppDiagnosticInfoWatcherStatus;i4)");
}
impl ::windows::core::DefaultType for AppDiagnosticInfoWatcherStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppExecutionStateChangeResult(pub ::windows::core::IInspectable);
impl AppExecutionStateChangeResult {
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppExecutionStateChangeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppExecutionStateChangeResult;{6f039bf0-f91b-4df8-ae77-3033ccb69114})");
}
unsafe impl ::windows::core::Interface for AppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f039bf0_f91b_4df8_ae77_3033ccb69114);
}
impl ::windows::core::RuntimeName for AppExecutionStateChangeResult {
    const NAME: &'static str = "Windows.System.AppExecutionStateChangeResult";
}
impl ::core::convert::From<AppExecutionStateChangeResult> for ::windows::core::IUnknown {
    fn from(value: AppExecutionStateChangeResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppExecutionStateChangeResult> for ::windows::core::IUnknown {
    fn from(value: &AppExecutionStateChangeResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppExecutionStateChangeResult> for ::windows::core::IInspectable {
    fn from(value: AppExecutionStateChangeResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppExecutionStateChangeResult> for ::windows::core::IInspectable {
    fn from(value: &AppExecutionStateChangeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppExecutionStateChangeResult {}
unsafe impl ::core::marker::Sync for AppExecutionStateChangeResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppMemoryReport(pub ::windows::core::IInspectable);
impl AppMemoryReport {
    pub fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PeakPrivateCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCommitLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn ExpectedTotalCommitLimit(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IAppMemoryReport2>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppMemoryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryReport;{6d65339b-4d6f-45bc-9c5e-e49b3ff2758d})");
}
unsafe impl ::windows::core::Interface for AppMemoryReport {
    type Vtable = IAppMemoryReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d65339b_4d6f_45bc_9c5e_e49b3ff2758d);
}
impl ::windows::core::RuntimeName for AppMemoryReport {
    const NAME: &'static str = "Windows.System.AppMemoryReport";
}
impl ::core::convert::From<AppMemoryReport> for ::windows::core::IUnknown {
    fn from(value: AppMemoryReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppMemoryReport> for ::windows::core::IUnknown {
    fn from(value: &AppMemoryReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppMemoryReport> for ::windows::core::IInspectable {
    fn from(value: AppMemoryReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppMemoryReport> for ::windows::core::IInspectable {
    fn from(value: &AppMemoryReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppMemoryReport {}
unsafe impl ::core::marker::Sync for AppMemoryReport {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppMemoryUsageLevel(pub i32);
impl AppMemoryUsageLevel {
    pub const Low: AppMemoryUsageLevel = AppMemoryUsageLevel(0i32);
    pub const Medium: AppMemoryUsageLevel = AppMemoryUsageLevel(1i32);
    pub const High: AppMemoryUsageLevel = AppMemoryUsageLevel(2i32);
    pub const OverLimit: AppMemoryUsageLevel = AppMemoryUsageLevel(3i32);
}
impl ::core::convert::From<i32> for AppMemoryUsageLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppMemoryUsageLevel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppMemoryUsageLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppMemoryUsageLevel;i4)");
}
impl ::windows::core::DefaultType for AppMemoryUsageLevel {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppMemoryUsageLimitChangingEventArgs(pub ::windows::core::IInspectable);
impl AppMemoryUsageLimitChangingEventArgs {
    pub fn OldLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn NewLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppMemoryUsageLimitChangingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryUsageLimitChangingEventArgs;{79f86664-feca-4da5-9e40-2bc63efdc979})");
}
unsafe impl ::windows::core::Interface for AppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f86664_feca_4da5_9e40_2bc63efdc979);
}
impl ::windows::core::RuntimeName for AppMemoryUsageLimitChangingEventArgs {
    const NAME: &'static str = "Windows.System.AppMemoryUsageLimitChangingEventArgs";
}
impl ::core::convert::From<AppMemoryUsageLimitChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppMemoryUsageLimitChangingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppMemoryUsageLimitChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppMemoryUsageLimitChangingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppMemoryUsageLimitChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppMemoryUsageLimitChangingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppMemoryUsageLimitChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppMemoryUsageLimitChangingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppMemoryUsageLimitChangingEventArgs {}
unsafe impl ::core::marker::Sync for AppMemoryUsageLimitChangingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupBackgroundTaskReport(pub ::windows::core::IInspectable);
impl AppResourceGroupBackgroundTaskReport {
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Trigger(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EntryPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupBackgroundTaskReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupBackgroundTaskReport;{2566e74e-b05d-40c2-9dc1-1a4f039ea120})");
}
unsafe impl ::windows::core::Interface for AppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2566e74e_b05d_40c2_9dc1_1a4f039ea120);
}
impl ::windows::core::RuntimeName for AppResourceGroupBackgroundTaskReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupBackgroundTaskReport";
}
impl ::core::convert::From<AppResourceGroupBackgroundTaskReport> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupBackgroundTaskReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupBackgroundTaskReport> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupBackgroundTaskReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupBackgroundTaskReport> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupBackgroundTaskReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupBackgroundTaskReport> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupBackgroundTaskReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupBackgroundTaskReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupBackgroundTaskReport {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppResourceGroupEnergyQuotaState(pub i32);
impl AppResourceGroupEnergyQuotaState {
    pub const Unknown: AppResourceGroupEnergyQuotaState = AppResourceGroupEnergyQuotaState(0i32);
    pub const Over: AppResourceGroupEnergyQuotaState = AppResourceGroupEnergyQuotaState(1i32);
    pub const Under: AppResourceGroupEnergyQuotaState = AppResourceGroupEnergyQuotaState(2i32);
}
impl ::core::convert::From<i32> for AppResourceGroupEnergyQuotaState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppResourceGroupEnergyQuotaState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupEnergyQuotaState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupEnergyQuotaState;i4)");
}
impl ::windows::core::DefaultType for AppResourceGroupEnergyQuotaState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppResourceGroupExecutionState(pub i32);
impl AppResourceGroupExecutionState {
    pub const Unknown: AppResourceGroupExecutionState = AppResourceGroupExecutionState(0i32);
    pub const Running: AppResourceGroupExecutionState = AppResourceGroupExecutionState(1i32);
    pub const Suspending: AppResourceGroupExecutionState = AppResourceGroupExecutionState(2i32);
    pub const Suspended: AppResourceGroupExecutionState = AppResourceGroupExecutionState(3i32);
    pub const NotRunning: AppResourceGroupExecutionState = AppResourceGroupExecutionState(4i32);
}
impl ::core::convert::From<i32> for AppResourceGroupExecutionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppResourceGroupExecutionState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupExecutionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupExecutionState;i4)");
}
impl ::windows::core::DefaultType for AppResourceGroupExecutionState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupInfo(pub ::windows::core::IInspectable);
impl AppResourceGroupInfo {
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn IsShared(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBackgroundTaskReports(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>>(result__)
        }
    }
    pub fn GetMemoryReport(&self) -> ::windows::core::Result<AppResourceGroupMemoryReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupMemoryReport>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub fn GetProcessDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>>(result__)
        }
    }
    pub fn GetStateReport(&self) -> ::windows::core::Result<AppResourceGroupStateReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupStateReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartSuspendAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartResumeAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTerminateAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfo;{b913f77a-e807-49f4-845e-7b8bdcfe8ee7})");
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb913f77a_e807_49f4_845e_7b8bdcfe8ee7);
}
impl ::windows::core::RuntimeName for AppResourceGroupInfo {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfo";
}
impl ::core::convert::From<AppResourceGroupInfo> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupInfo> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupInfo> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupInfo> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfo {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupInfoWatcher(pub ::windows::core::IInspectable);
impl AppResourceGroupInfoWatcher {
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExecutionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveExecutionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: AppResourceGroupInfoWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfoWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcher;{d9b0a0fd-6e5a-4c72-8b17-09fec4a212bd})");
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9b0a0fd_6e5a_4c72_8b17_09fec4a212bd);
}
impl ::windows::core::RuntimeName for AppResourceGroupInfoWatcher {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcher";
}
impl ::core::convert::From<AppResourceGroupInfoWatcher> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupInfoWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcher> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcher> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupInfoWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcher> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcher {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupInfoWatcherEventArgs(pub ::windows::core::IInspectable);
impl AppResourceGroupInfoWatcherEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcherEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherEventArgs;{7a787637-6302-4d2f-bf89-1c12d0b2a6b9})");
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a787637_6302_4d2f_bf89_1c12d0b2a6b9);
}
impl ::windows::core::RuntimeName for AppResourceGroupInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherEventArgs";
}
impl ::core::convert::From<AppResourceGroupInfoWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupInfoWatcherEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcherEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupInfoWatcherEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcherEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupInfoWatcherExecutionStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs;{1bdbedd7-fee6-4fd4-98dd-e92a2cc299f3})");
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bdbedd7_fee6_4fd4_98dd_e92a2cc299f3);
}
impl ::windows::core::RuntimeName for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs";
}
impl ::core::convert::From<AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherStatus(pub i32);
impl AppResourceGroupInfoWatcherStatus {
    pub const Created: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(0i32);
    pub const Started: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(1i32);
    pub const EnumerationCompleted: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(2i32);
    pub const Stopping: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(3i32);
    pub const Stopped: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(4i32);
    pub const Aborted: AppResourceGroupInfoWatcherStatus = AppResourceGroupInfoWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for AppResourceGroupInfoWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppResourceGroupInfoWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupInfoWatcherStatus;i4)");
}
impl ::windows::core::DefaultType for AppResourceGroupInfoWatcherStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupMemoryReport(pub ::windows::core::IInspectable);
impl AppResourceGroupMemoryReport {
    pub fn CommitUsageLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn CommitUsageLevel(&self) -> ::windows::core::Result<AppMemoryUsageLevel> {
        let this = self;
        unsafe {
            let mut result__: AppMemoryUsageLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppMemoryUsageLevel>(result__)
        }
    }
    pub fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupMemoryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupMemoryReport;{2c8c06b1-7db1-4c51-a225-7fae2d49e431})");
}
unsafe impl ::windows::core::Interface for AppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c8c06b1_7db1_4c51_a225_7fae2d49e431);
}
impl ::windows::core::RuntimeName for AppResourceGroupMemoryReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupMemoryReport";
}
impl ::core::convert::From<AppResourceGroupMemoryReport> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupMemoryReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupMemoryReport> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupMemoryReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupMemoryReport> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupMemoryReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupMemoryReport> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupMemoryReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupMemoryReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupMemoryReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupStateReport(pub ::windows::core::IInspectable);
impl AppResourceGroupStateReport {
    pub fn ExecutionState(&self) -> ::windows::core::Result<AppResourceGroupExecutionState> {
        let this = self;
        unsafe {
            let mut result__: AppResourceGroupExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupExecutionState>(result__)
        }
    }
    pub fn EnergyQuotaState(&self) -> ::windows::core::Result<AppResourceGroupEnergyQuotaState> {
        let this = self;
        unsafe {
            let mut result__: AppResourceGroupEnergyQuotaState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupEnergyQuotaState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupStateReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupStateReport;{52849f18-2f70-4236-ab40-d04db0c7b931})");
}
unsafe impl ::windows::core::Interface for AppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52849f18_2f70_4236_ab40_d04db0c7b931);
}
impl ::windows::core::RuntimeName for AppResourceGroupStateReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupStateReport";
}
impl ::core::convert::From<AppResourceGroupStateReport> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupStateReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupStateReport> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupStateReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupStateReport> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupStateReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupStateReport> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupStateReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupStateReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupStateReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppUriHandlerHost(pub ::windows::core::IInspectable);
impl AppUriHandlerHost {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUriHandlerHost, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<AppUriHandlerHost> {
        Self::IAppUriHandlerHostFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<AppUriHandlerHost>(result__)
        })
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IAppUriHandlerHostFactory<R, F: FnOnce(&IAppUriHandlerHostFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUriHandlerHost, IAppUriHandlerHostFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppUriHandlerHost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerHost;{5d50cac5-92d2-5409-b56f-7f73e10ea4c3})");
}
unsafe impl ::windows::core::Interface for AppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d50cac5_92d2_5409_b56f_7f73e10ea4c3);
}
impl ::windows::core::RuntimeName for AppUriHandlerHost {
    const NAME: &'static str = "Windows.System.AppUriHandlerHost";
}
impl ::core::convert::From<AppUriHandlerHost> for ::windows::core::IUnknown {
    fn from(value: AppUriHandlerHost) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppUriHandlerHost> for ::windows::core::IUnknown {
    fn from(value: &AppUriHandlerHost) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppUriHandlerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppUriHandlerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppUriHandlerHost> for ::windows::core::IInspectable {
    fn from(value: AppUriHandlerHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppUriHandlerHost> for ::windows::core::IInspectable {
    fn from(value: &AppUriHandlerHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppUriHandlerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppUriHandlerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerHost {}
unsafe impl ::core::marker::Sync for AppUriHandlerHost {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppUriHandlerRegistration(pub ::windows::core::IInspectable);
impl AppUriHandlerRegistration {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetAppAddedHostsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetAppAddedHostsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<AppUriHandlerHost>>>(&self, hosts: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), hosts.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllHosts(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppUriHandlerHost>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateHosts<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<AppUriHandlerHost>>>(&self, hosts: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), hosts.into_param().abi()).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppUriHandlerRegistration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistration;{6f73aeb1-4569-5c3f-9ba0-99123eea32c3})");
}
unsafe impl ::windows::core::Interface for AppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f73aeb1_4569_5c3f_9ba0_99123eea32c3);
}
impl ::windows::core::RuntimeName for AppUriHandlerRegistration {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistration";
}
impl ::core::convert::From<AppUriHandlerRegistration> for ::windows::core::IUnknown {
    fn from(value: AppUriHandlerRegistration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppUriHandlerRegistration> for ::windows::core::IUnknown {
    fn from(value: &AppUriHandlerRegistration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppUriHandlerRegistration> for ::windows::core::IInspectable {
    fn from(value: AppUriHandlerRegistration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppUriHandlerRegistration> for ::windows::core::IInspectable {
    fn from(value: &AppUriHandlerRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerRegistration {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppUriHandlerRegistrationManager(pub ::windows::core::IInspectable);
impl AppUriHandlerRegistrationManager {
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    pub fn TryGetRegistration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<AppUriHandlerRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistration>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, User>>(user: Param0) -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistrationManager2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetForPackage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagefamilyname: Param0) -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    pub fn GetForPackageForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, User>>(packagefamilyname: Param0, user: Param1) -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), user.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    pub fn IAppUriHandlerRegistrationManagerStatics<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppUriHandlerRegistrationManagerStatics2<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppUriHandlerRegistrationManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistrationManager;{e62c9a52-ac94-5750-ac1b-6cfb6f250263})");
}
unsafe impl ::windows::core::Interface for AppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe62c9a52_ac94_5750_ac1b_6cfb6f250263);
}
impl ::windows::core::RuntimeName for AppUriHandlerRegistrationManager {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistrationManager";
}
impl ::core::convert::From<AppUriHandlerRegistrationManager> for ::windows::core::IUnknown {
    fn from(value: AppUriHandlerRegistrationManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppUriHandlerRegistrationManager> for ::windows::core::IUnknown {
    fn from(value: &AppUriHandlerRegistrationManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppUriHandlerRegistrationManager> for ::windows::core::IInspectable {
    fn from(value: AppUriHandlerRegistrationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppUriHandlerRegistrationManager> for ::windows::core::IInspectable {
    fn from(value: &AppUriHandlerRegistrationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerRegistrationManager {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistrationManager {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutoUpdateTimeZoneStatus(pub i32);
impl AutoUpdateTimeZoneStatus {
    pub const Attempted: AutoUpdateTimeZoneStatus = AutoUpdateTimeZoneStatus(0i32);
    pub const TimedOut: AutoUpdateTimeZoneStatus = AutoUpdateTimeZoneStatus(1i32);
    pub const Failed: AutoUpdateTimeZoneStatus = AutoUpdateTimeZoneStatus(2i32);
}
impl ::core::convert::From<i32> for AutoUpdateTimeZoneStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutoUpdateTimeZoneStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutoUpdateTimeZoneStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AutoUpdateTimeZoneStatus;i4)");
}
impl ::windows::core::DefaultType for AutoUpdateTimeZoneStatus {
    type DefaultType = Self;
}
pub struct DateTimeSettings {}
impl DateTimeSettings {
    #[cfg(feature = "Foundation")]
    pub fn SetSystemDateTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::DateTime>>(utcdatetime: Param0) -> ::windows::core::Result<()> {
        Self::IDateTimeSettingsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), utcdatetime.into_param().abi()).ok() })
    }
    pub fn IDateTimeSettingsStatics<R, F: FnOnce(&IDateTimeSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DateTimeSettings, IDateTimeSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for DateTimeSettings {
    const NAME: &'static str = "Windows.System.DateTimeSettings";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DiagnosticAccessStatus(pub i32);
impl DiagnosticAccessStatus {
    pub const Unspecified: DiagnosticAccessStatus = DiagnosticAccessStatus(0i32);
    pub const Denied: DiagnosticAccessStatus = DiagnosticAccessStatus(1i32);
    pub const Limited: DiagnosticAccessStatus = DiagnosticAccessStatus(2i32);
    pub const Allowed: DiagnosticAccessStatus = DiagnosticAccessStatus(3i32);
}
impl ::core::convert::From<i32> for DiagnosticAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DiagnosticAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DiagnosticAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.DiagnosticAccessStatus;i4)");
}
impl ::windows::core::DefaultType for DiagnosticAccessStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueue(pub ::windows::core::IInspectable);
impl DispatcherQueue {
    pub fn CreateTimer(&self) -> ::windows::core::Result<DispatcherQueueTimer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueueTimer>(result__)
        }
    }
    pub fn TryEnqueue<'a, Param0: ::windows::core::IntoParam<'a, DispatcherQueueHandler>>(&self, callback: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), callback.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn TryEnqueueWithPriority<'a, Param1: ::windows::core::IntoParam<'a, DispatcherQueueHandler>>(&self, priority: DispatcherQueuePriority, callback: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), priority, callback.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShutdownStarting<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownStarting<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShutdownCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentThread() -> ::windows::core::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueue>(result__)
        })
    }
    pub fn HasThreadAccess(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDispatcherQueue2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IDispatcherQueueStatics<R, F: FnOnce(&IDispatcherQueueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueue;{603e88e4-a338-4ffe-a457-a5cfb9ceb899})");
}
unsafe impl ::windows::core::Interface for DispatcherQueue {
    type Vtable = IDispatcherQueue_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x603e88e4_a338_4ffe_a457_a5cfb9ceb899);
}
impl ::windows::core::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Windows.System.DispatcherQueue";
}
impl ::core::convert::From<DispatcherQueue> for ::windows::core::IUnknown {
    fn from(value: DispatcherQueue) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DispatcherQueue> for ::windows::core::IUnknown {
    fn from(value: &DispatcherQueue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DispatcherQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DispatcherQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DispatcherQueue> for ::windows::core::IInspectable {
    fn from(value: DispatcherQueue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DispatcherQueue> for ::windows::core::IInspectable {
    fn from(value: &DispatcherQueue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DispatcherQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DispatcherQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DispatcherQueue {}
unsafe impl ::core::marker::Sync for DispatcherQueue {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueueController(pub ::windows::core::IInspectable);
impl DispatcherQueueController {
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShutdownQueueAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn CreateOnDedicatedThread() -> ::windows::core::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueueController>(result__)
        })
    }
    pub fn IDispatcherQueueControllerStatics<R, F: FnOnce(&IDispatcherQueueControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DispatcherQueueController, IDispatcherQueueControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueController;{22f34e66-50db-4e36-a98d-61c01b384d20})");
}
unsafe impl ::windows::core::Interface for DispatcherQueueController {
    type Vtable = IDispatcherQueueController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f34e66_50db_4e36_a98d_61c01b384d20);
}
impl ::windows::core::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Windows.System.DispatcherQueueController";
}
impl ::core::convert::From<DispatcherQueueController> for ::windows::core::IUnknown {
    fn from(value: DispatcherQueueController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DispatcherQueueController> for ::windows::core::IUnknown {
    fn from(value: &DispatcherQueueController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DispatcherQueueController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DispatcherQueueController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DispatcherQueueController> for ::windows::core::IInspectable {
    fn from(value: DispatcherQueueController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DispatcherQueueController> for ::windows::core::IInspectable {
    fn from(value: &DispatcherQueueController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DispatcherQueueController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DispatcherQueueController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueController {}
unsafe impl ::core::marker::Sync for DispatcherQueueController {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueueHandler(::windows::core::IUnknown);
impl DispatcherQueueHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DispatcherQueueHandler_box::<F> { vtable: &DispatcherQueueHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({dfa2dc9c-1a2d-4917-98f2-939af1d6e0c8})");
}
unsafe impl ::windows::core::Interface for DispatcherQueueHandler {
    type Vtable = DispatcherQueueHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfa2dc9c_1a2d_4917_98f2_939af1d6e0c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatcherQueueHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct DispatcherQueueHandler_box<F: FnMut() -> ::windows::core::Result<()> + 'static> {
    vtable: *const DispatcherQueueHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + 'static> DispatcherQueueHandler_box<F> {
    const VTABLE: DispatcherQueueHandler_abi = DispatcherQueueHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DispatcherQueueHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: DispatcherQueuePriority = DispatcherQueuePriority(-10i32);
    pub const Normal: DispatcherQueuePriority = DispatcherQueuePriority(0i32);
    pub const High: DispatcherQueuePriority = DispatcherQueuePriority(10i32);
}
impl ::core::convert::From<i32> for DispatcherQueuePriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DispatcherQueuePriority {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.DispatcherQueuePriority;i4)");
}
impl ::windows::core::DefaultType for DispatcherQueuePriority {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueueShutdownStartingEventArgs(pub ::windows::core::IInspectable);
impl DispatcherQueueShutdownStartingEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueShutdownStartingEventArgs;{c4724c4c-ff97-40c0-a226-cc0aaa545e89})");
}
unsafe impl ::windows::core::Interface for DispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4724c4c_ff97_40c0_a226_cc0aaa545e89);
}
impl ::windows::core::RuntimeName for DispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Windows.System.DispatcherQueueShutdownStartingEventArgs";
}
impl ::core::convert::From<DispatcherQueueShutdownStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DispatcherQueueShutdownStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DispatcherQueueShutdownStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DispatcherQueueShutdownStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueShutdownStartingEventArgs {}
unsafe impl ::core::marker::Sync for DispatcherQueueShutdownStartingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueueTimer(pub ::windows::core::IInspectable);
impl DispatcherQueueTimer {
    #[cfg(feature = "Foundation")]
    pub fn Interval(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetInterval<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsRunning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsRepeating(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRepeating(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Tick<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTick<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueTimer;{5feabb1d-a31c-4727-b1ac-37454649d56a})");
}
unsafe impl ::windows::core::Interface for DispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5feabb1d_a31c_4727_b1ac_37454649d56a);
}
impl ::windows::core::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Windows.System.DispatcherQueueTimer";
}
impl ::core::convert::From<DispatcherQueueTimer> for ::windows::core::IUnknown {
    fn from(value: DispatcherQueueTimer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DispatcherQueueTimer> for ::windows::core::IUnknown {
    fn from(value: &DispatcherQueueTimer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DispatcherQueueTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DispatcherQueueTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DispatcherQueueTimer> for ::windows::core::IInspectable {
    fn from(value: DispatcherQueueTimer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DispatcherQueueTimer> for ::windows::core::IInspectable {
    fn from(value: &DispatcherQueueTimer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DispatcherQueueTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DispatcherQueueTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueTimer {}
unsafe impl ::core::marker::Sync for DispatcherQueueTimer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FolderLauncherOptions(pub ::windows::core::IInspectable);
impl FolderLauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FolderLauncherOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn ItemsToSelect(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<super::Storage::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__: super::UI::ViewManagement::ViewSizePreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FolderLauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.FolderLauncherOptions;{bb91c27d-6b87-432a-bd04-776c6f5fb2ab})");
}
unsafe impl ::windows::core::Interface for FolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb91c27d_6b87_432a_bd04_776c6f5fb2ab);
}
impl ::windows::core::RuntimeName for FolderLauncherOptions {
    const NAME: &'static str = "Windows.System.FolderLauncherOptions";
}
impl ::core::convert::From<FolderLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: FolderLauncherOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FolderLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: &FolderLauncherOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FolderLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: FolderLauncherOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FolderLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: &FolderLauncherOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<FolderLauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderLauncherOptions) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderLauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderLauncherOptions) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILauncherViewOptions> for FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ILauncherViewOptions> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILauncherViewOptions> for &FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ILauncherViewOptions> {
        ::core::convert::TryInto::<ILauncherViewOptions>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FolderLauncherOptions {}
unsafe impl ::core::marker::Sync for FolderLauncherOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppActivationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppActivationResult {
    type Vtable = IAppActivationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b528900_f46e_4eb0_aa6c_38af557cf9ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe348a69a_8889_4ca3_be07_d5ffff5f0804);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfo2 {
    type Vtable = IAppDiagnosticInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf46fbd7_191a_446c_9473_8fbc2374a354);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfo3 {
    type Vtable = IAppDiagnosticInfo3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc895c63d_dd61_4c65_babd_81a10b4f9815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoStatics {
    type Vtable = IAppDiagnosticInfoStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce6925bf_10ca_40c8_a9ca_c5c96501866e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoStatics2 {
    type Vtable = IAppDiagnosticInfoStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05b24b86_1000_4c90_bb9f_7235071c50fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75575070_01d3_489a_9325_52f9cc6ede0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcherEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7017c716_e1da_4c65_99df_046dff5be71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcherEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppExecutionStateChangeResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f039bf0_f91b_4df8_ae77_3033ccb69114);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExecutionStateChangeResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppMemoryReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppMemoryReport {
    type Vtable = IAppMemoryReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d65339b_4d6f_45bc_9c5e_e49b3ff2758d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppMemoryReport2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppMemoryReport2 {
    type Vtable = IAppMemoryReport2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f7f3738_51b7_42dc_b7ed_79ba46d28857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppMemoryUsageLimitChangingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f86664_feca_4da5_9e40_2bc63efdc979);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryUsageLimitChangingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupBackgroundTaskReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2566e74e_b05d_40c2_9dc1_1a4f039ea120);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupBackgroundTaskReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb913f77a_e807_49f4_845e_7b8bdcfe8ee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_Diagnostics")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfo2 {
    type Vtable = IAppResourceGroupInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee9b236d_d305_4d6b_92f7_6afdad72dedc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo2_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9b0a0fd_6e5a_4c72_8b17_09fec4a212bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a787637_6302_4d2f_bf89_1c12d0b2a6b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bdbedd7_fee6_4fd4_98dd_e92a2cc299f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupMemoryReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c8c06b1_7db1_4c51_a225_7fae2d49e431);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupMemoryReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupStateReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52849f18_2f70_4236_ab40_d04db0c7b931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupStateReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppResourceGroupExecutionState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerHost(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d50cac5_92d2_5409_b56f_7f73e10ea4c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerHost2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppUriHandlerHost2 {
    type Vtable = IAppUriHandlerHost2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a0bee95_29e4_51bf_8095_a3c068e3c72a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerHostFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppUriHandlerHostFactory {
    type Vtable = IAppUriHandlerHostFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x257c3c96_ce04_5f98_96bb_3ebd3e9275bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHostFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f73aeb1_4569_5c3f_9ba0_99123eea32c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hosts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistration2 {
    type Vtable = IAppUriHandlerRegistration2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd54dac97_cb39_5f1f_883e_01853730bd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hosts: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe62c9a52_ac94_5750_ac1b_6cfb6f250263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManager2 {
    type Vtable = IAppUriHandlerRegistrationManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbddfcaf1_b51a_5e69_aefd_7088d9f2b123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManagerStatics {
    type Vtable = IAppUriHandlerRegistrationManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5cedd9f_5729_5b76_a1d4_0285f295c124);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManagerStatics2 {
    type Vtable = IAppUriHandlerRegistrationManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14f78379_6890_5080_90a7_98824a7f079e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeSettingsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDateTimeSettingsStatics {
    type Vtable = IDateTimeSettingsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2150d1_47ee_48ab_a52b_9f1954278d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, utcdatetime: super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueue(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDispatcherQueue {
    type Vtable = IDispatcherQueue_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x603e88e4_a338_4ffe_a457_a5cfb9ceb899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, priority: DispatcherQueuePriority, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueue2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDispatcherQueue2 {
    type Vtable = IDispatcherQueue2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc822c647_30ef_506e_bd1e_a647ae6675ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDispatcherQueueController {
    type Vtable = IDispatcherQueueController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f34e66_50db_4e36_a98d_61c01b384d20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDispatcherQueueControllerStatics {
    type Vtable = IDispatcherQueueControllerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a6c98e0_5198_49a2_a313_3f70d1f13c27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4724c4c_ff97_40c0_a226_cc0aaa545e89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDispatcherQueueStatics {
    type Vtable = IDispatcherQueueStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa96d83d7_9371_4517_9245_d0824ac12c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5feabb1d_a31c_4727_b1ac_37454649d56a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFolderLauncherOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb91c27d_6b87_432a_bd04_776c6f5fb2ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderLauncherOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownUserPropertiesStatics {
    type Vtable = IKnownUserPropertiesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7755911a_70c5_48e5_b637_5ba3441e4ee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownUserPropertiesStatics2 {
    type Vtable = IKnownUserPropertiesStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b450782_f620_577e_b1b3_dd56644d79b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILaunchUriResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILaunchUriResult {
    type Vtable = ILaunchUriResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec27a8df_f6d5_45ca_913a_70a40c5c8221);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchUriResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LaunchUriStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherOptions {
    type Vtable = ILauncherOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafa21d8_b071_4cd8_853e_341203e557d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherOptions2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherOptions2 {
    type Vtable = ILauncherOptions2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ba08eb4_6e40_4dce_a1a3_2f53950afb49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Search")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))] usize,
    #[cfg(feature = "Storage_Search")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherOptions3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherOptions3 {
    type Vtable = ILauncherOptions3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0770655_4b63_4e3a_9107_4e687841923a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherOptions4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherOptions4 {
    type Vtable = ILauncherOptions4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef6fd10e_e6fb_4814_a44e_57e8b9d9a01b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherStatics {
    type Vtable = ILauncherStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x277151c3_9e3e_42f6_91a4_5dfdeb232451);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherStatics2 {
    type Vtable = ILauncherStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59ba2fbb_24cb_4c02_a4c4_8294569d54f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, extension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherStatics3 {
    type Vtable = ILauncherStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x234261a8_9db3_4683_aa42_dc6f51d33847);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherStatics4 {
    type Vtable = ILauncherStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9ec819f_b5a5_41c6_b3b3_dd1b3178bcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherStatics5 {
    type Vtable = ILauncherStatics5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b24ef84_d895_5fea_9153_1ac49aed9ba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherUIOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherUIOptions {
    type Vtable = ILauncherUIOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b25da6e_8aa6_41e9_8251_4165f5985f49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherUIOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILauncherViewOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILauncherViewOptions {
    type Vtable = ILauncherViewOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9b29f1_7ca7_49de_9bd3_3c5b7184f616);
}
impl ILauncherViewOptions {
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__: super::UI::ViewManagement::ViewSizePreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ILauncherViewOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8a9b29f1-7ca7-49de-9bd3-3c5b7184f616}");
}
impl ::core::convert::From<ILauncherViewOptions> for ::windows::core::IUnknown {
    fn from(value: ILauncherViewOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILauncherViewOptions> for ::windows::core::IUnknown {
    fn from(value: &ILauncherViewOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILauncherViewOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILauncherViewOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILauncherViewOptions> for ::windows::core::IInspectable {
    fn from(value: ILauncherViewOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILauncherViewOptions> for ::windows::core::IInspectable {
    fn from(value: &ILauncherViewOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILauncherViewOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILauncherViewOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherViewOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_ViewManagement")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))] usize,
    #[cfg(feature = "UI_ViewManagement")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMemoryManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMemoryManagerStatics {
    type Vtable = IMemoryManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c6c279c_d7ca_4779_9188_4057219ce64c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMemoryManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMemoryManagerStatics2 {
    type Vtable = IMemoryManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eee351f_6d62_423f_9479_b01f9c9f7669);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMemoryManagerStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMemoryManagerStatics3 {
    type Vtable = IMemoryManagerStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x149b59ce_92ad_4e35_89eb_50dfb4c0d91c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMemoryManagerStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMemoryManagerStatics4 {
    type Vtable = IMemoryManagerStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5a94828_e84e_4886_8a0d_44b3190e3b72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessLauncherOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3080b9cf_f444_4a83_beaf_a549a0f3229c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessLauncherResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessLauncherResult {
    type Vtable = IProcessLauncherResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x544c8934_86d8_4991_8e75_ece8a43b6b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessLauncherStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessLauncherStatics {
    type Vtable = IProcessLauncherStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ab66e7_2d0e_448b_a6a0_c13c3836d09c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessMemoryReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessMemoryReport {
    type Vtable = IProcessMemoryReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x087305a8_9b70_4782_8741_3a982b6ce5e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtocolForResultsOperation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd581293a_6de9_4d28_9378_f86782e182bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteLauncherOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e3a2788_2891_4cdf_a2d6_9dff7d02e693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteLauncherStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRemoteLauncherStatics {
    type Vtable = IRemoteLauncherStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7db7a93_a30c_48b7_9f21_051026a4e517);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShutdownManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShutdownManagerStatics {
    type Vtable = IShutdownManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72e247ed_dd5b_4d6c_b1d0_c57a7bbb5f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShutdownManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShutdownManagerStatics2 {
    type Vtable = IShutdownManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f69a02f_9c34_43c7_a8c3_70b30a7f7504);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, powerstate: PowerState, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, powerstate: PowerState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITimeZoneSettingsStatics {
    type Vtable = ITimeZoneSettingsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b3b2bea_a101_41ae_9fbd_028728bab73d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timezonedisplayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITimeZoneSettingsStatics2 {
    type Vtable = ITimeZoneSettingsStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x555c0db8_39a8_49fa_b4f6_a2c7fc2842ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeout: super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUser {
    type Vtable = IUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf9a26c6_e746_4bcd_b5d4_120103c4209b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredsize: UserPictureSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUser2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUser2 {
    type Vtable = IUser2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98ba5628_a6e3_518e_89d9_d3b2b1991a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, consentgroup: UserAgeConsentGroup, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangeDeferral(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88b59568_bb30_42fb_a270_e9902e40efa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangeDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c030f28_a711_4c1e_ab48_04179c15938f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x086459dc_18c6_48db_bc99_724fb9203ccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserChangedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserChangedEventArgs2 {
    type Vtable = IUserChangedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b2ccb44_6f01_560c_97ad_fc7f32ec581f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDeviceAssociationChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1f6f6c_bb5d_4d7b_a5f0_c8cd11a38d42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDeviceAssociationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDeviceAssociationStatics {
    type Vtable = IUserDeviceAssociationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e491e14_f85a_4c07_8da9_7fe3d0542343);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserPicker(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserPicker {
    type Vtable = IUserPicker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d548008_f1e3_4a6c_8ddc_a9bb0f488aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPicker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserPickerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserPickerStatics {
    type Vtable = IUserPickerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde3290dc_7e73_4df6_a1ae_4d7eca82b40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPickerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserStatics {
    type Vtable = IUserStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: UserType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: UserType, status: UserAuthenticationStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nonroamableid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserStatics2 {
    type Vtable = IUserStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74a37e11_2eb5_4487_b0d5_2c6790e013e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserWatcher {
    type Vtable = IUserWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserWatcherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
pub struct KnownUserProperties {}
impl KnownUserProperties {
    pub fn DisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn FirstName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LastName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ProviderName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AccountName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GuestHost() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PrincipalName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DomainName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SessionInitiationProtocolUri() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AgeEnforcementRegion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IKnownUserPropertiesStatics<R, F: FnOnce(&IKnownUserPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKnownUserPropertiesStatics2<R, F: FnOnce(&IKnownUserPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownUserProperties {
    const NAME: &'static str = "Windows.System.KnownUserProperties";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LaunchFileStatus(pub i32);
impl LaunchFileStatus {
    pub const Success: LaunchFileStatus = LaunchFileStatus(0i32);
    pub const AppUnavailable: LaunchFileStatus = LaunchFileStatus(1i32);
    pub const DeniedByPolicy: LaunchFileStatus = LaunchFileStatus(2i32);
    pub const FileTypeNotSupported: LaunchFileStatus = LaunchFileStatus(3i32);
    pub const Unknown: LaunchFileStatus = LaunchFileStatus(4i32);
}
impl ::core::convert::From<i32> for LaunchFileStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LaunchFileStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LaunchFileStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchFileStatus;i4)");
}
impl ::windows::core::DefaultType for LaunchFileStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LaunchQuerySupportStatus(pub i32);
impl LaunchQuerySupportStatus {
    pub const Available: LaunchQuerySupportStatus = LaunchQuerySupportStatus(0i32);
    pub const AppNotInstalled: LaunchQuerySupportStatus = LaunchQuerySupportStatus(1i32);
    pub const AppUnavailable: LaunchQuerySupportStatus = LaunchQuerySupportStatus(2i32);
    pub const NotSupported: LaunchQuerySupportStatus = LaunchQuerySupportStatus(3i32);
    pub const Unknown: LaunchQuerySupportStatus = LaunchQuerySupportStatus(4i32);
}
impl ::core::convert::From<i32> for LaunchQuerySupportStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LaunchQuerySupportStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LaunchQuerySupportStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportStatus;i4)");
}
impl ::windows::core::DefaultType for LaunchQuerySupportStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LaunchQuerySupportType(pub i32);
impl LaunchQuerySupportType {
    pub const Uri: LaunchQuerySupportType = LaunchQuerySupportType(0i32);
    pub const UriForResults: LaunchQuerySupportType = LaunchQuerySupportType(1i32);
}
impl ::core::convert::From<i32> for LaunchQuerySupportType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LaunchQuerySupportType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LaunchQuerySupportType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportType;i4)");
}
impl ::windows::core::DefaultType for LaunchQuerySupportType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LaunchUriResult(pub ::windows::core::IInspectable);
impl LaunchUriResult {
    pub fn Status(&self) -> ::windows::core::Result<LaunchUriStatus> {
        let this = self;
        unsafe {
            let mut result__: LaunchUriStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LaunchUriStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Result(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchUriResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.LaunchUriResult;{ec27a8df-f6d5-45ca-913a-70a40c5c8221})");
}
unsafe impl ::windows::core::Interface for LaunchUriResult {
    type Vtable = ILaunchUriResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec27a8df_f6d5_45ca_913a_70a40c5c8221);
}
impl ::windows::core::RuntimeName for LaunchUriResult {
    const NAME: &'static str = "Windows.System.LaunchUriResult";
}
impl ::core::convert::From<LaunchUriResult> for ::windows::core::IUnknown {
    fn from(value: LaunchUriResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LaunchUriResult> for ::windows::core::IUnknown {
    fn from(value: &LaunchUriResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LaunchUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LaunchUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LaunchUriResult> for ::windows::core::IInspectable {
    fn from(value: LaunchUriResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LaunchUriResult> for ::windows::core::IInspectable {
    fn from(value: &LaunchUriResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LaunchUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LaunchUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LaunchUriResult {}
unsafe impl ::core::marker::Sync for LaunchUriResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LaunchUriStatus(pub i32);
impl LaunchUriStatus {
    pub const Success: LaunchUriStatus = LaunchUriStatus(0i32);
    pub const AppUnavailable: LaunchUriStatus = LaunchUriStatus(1i32);
    pub const ProtocolUnavailable: LaunchUriStatus = LaunchUriStatus(2i32);
    pub const Unknown: LaunchUriStatus = LaunchUriStatus(3i32);
}
impl ::core::convert::From<i32> for LaunchUriStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LaunchUriStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LaunchUriStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchUriStatus;i4)");
}
impl ::windows::core::DefaultType for LaunchUriStatus {
    type DefaultType = Self;
}
pub struct Launcher {}
impl Launcher {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>>(file: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>>(uri: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>>(uri: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn LaunchUriForResultsWithDataAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>, Param2: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(uri: Param0, options: Param1, inputdata: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn LaunchUriWithDataAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>, Param2: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(uri: Param0, options: Param1, inputdata: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), launchquerysupporttype, &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportWithPackageFamilyNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uri: Param0, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), uri.into_param().abi(), launchquerysupporttype, packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::StorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportWithPackageFamilyNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::StorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, packagefamilyname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), file.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scheme: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), scheme.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersWithLaunchUriTypeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scheme: Param0, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), scheme.into_param().abi(), launchquerysupporttype, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindFileHandlersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(extension: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), extension.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::IStorageFolder>>(folder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), folder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, FolderLauncherOptions>>(folder: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), folder.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportWithPackageFamilyNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uri: Param0, packagefamilyname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAppUriHandlersAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(user: Param0, uri: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, LauncherOptions>>(user: Param0, uri: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn LaunchUriWithDataForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, LauncherOptions>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(user: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, LauncherOptions>>(user: Param0, uri: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn LaunchUriForResultsWithDataForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, LauncherOptions>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(user: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(path: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, FolderLauncherOptions>>(path: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), path.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, path: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, FolderLauncherOptions>>(user: Param0, path: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ILauncherStatics<R, F: FnOnce(&ILauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics2<R, F: FnOnce(&ILauncherStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics3<R, F: FnOnce(&ILauncherStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics4<R, F: FnOnce(&ILauncherStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics5<R, F: FnOnce(&ILauncherStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for Launcher {
    const NAME: &'static str = "Windows.System.Launcher";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LauncherOptions(pub ::windows::core::IInspectable);
impl LauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LauncherOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TreatAsUntrusted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetTreatAsUntrusted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DisplayApplicationPicker(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDisplayApplicationPicker(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn UI(&self) -> ::windows::core::Result<LauncherUIOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LauncherUIOptions>(result__)
        }
    }
    pub fn PreferredApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPreferredApplicationPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PreferredApplicationDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPreferredApplicationDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContentType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetApplicationPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn SetNeighboringFilesQuery<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::Search::StorageFileQueryResult>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__: super::UI::ViewManagement::ViewSizePreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IgnoreAppUriHandlers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnoreAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LimitPickerToCurrentAppAndAppUriHandlers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetLimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.LauncherOptions;{bafa21d8-b071-4cd8-853e-341203e557d3})");
}
unsafe impl ::windows::core::Interface for LauncherOptions {
    type Vtable = ILauncherOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafa21d8_b071_4cd8_853e_341203e557d3);
}
impl ::windows::core::RuntimeName for LauncherOptions {
    const NAME: &'static str = "Windows.System.LauncherOptions";
}
impl ::core::convert::From<LauncherOptions> for ::windows::core::IUnknown {
    fn from(value: LauncherOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LauncherOptions> for ::windows::core::IUnknown {
    fn from(value: &LauncherOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LauncherOptions> for ::windows::core::IInspectable {
    fn from(value: LauncherOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LauncherOptions> for ::windows::core::IInspectable {
    fn from(value: &LauncherOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<LauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: LauncherOptions) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: &LauncherOptions) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILauncherViewOptions> for LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ILauncherViewOptions> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILauncherViewOptions> for &LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ILauncherViewOptions> {
        ::core::convert::TryInto::<ILauncherViewOptions>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LauncherOptions {}
unsafe impl ::core::marker::Sync for LauncherOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LauncherUIOptions(pub ::windows::core::IInspectable);
impl LauncherUIOptions {
    #[cfg(feature = "Foundation")]
    pub fn InvocationPoint(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetInvocationPoint<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::Point>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectionRect(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::Rect>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSelectionRect<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::Rect>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn PreferredPlacement(&self) -> ::windows::core::Result<super::UI::Popups::Placement> {
        let this = self;
        unsafe {
            let mut result__: super::UI::Popups::Placement = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::Popups::Placement>(result__)
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn SetPreferredPlacement(&self, value: super::UI::Popups::Placement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LauncherUIOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.LauncherUIOptions;{1b25da6e-8aa6-41e9-8251-4165f5985f49})");
}
unsafe impl ::windows::core::Interface for LauncherUIOptions {
    type Vtable = ILauncherUIOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b25da6e_8aa6_41e9_8251_4165f5985f49);
}
impl ::windows::core::RuntimeName for LauncherUIOptions {
    const NAME: &'static str = "Windows.System.LauncherUIOptions";
}
impl ::core::convert::From<LauncherUIOptions> for ::windows::core::IUnknown {
    fn from(value: LauncherUIOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LauncherUIOptions> for ::windows::core::IUnknown {
    fn from(value: &LauncherUIOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LauncherUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LauncherUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LauncherUIOptions> for ::windows::core::IInspectable {
    fn from(value: LauncherUIOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LauncherUIOptions> for ::windows::core::IInspectable {
    fn from(value: &LauncherUIOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LauncherUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LauncherUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LauncherUIOptions {}
unsafe impl ::core::marker::Sync for LauncherUIOptions {}
pub struct MemoryManager {}
impl MemoryManager {
    pub fn AppMemoryUsage() -> ::windows::core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    pub fn AppMemoryUsageLimit() -> ::windows::core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    pub fn AppMemoryUsageLevel() -> ::windows::core::Result<AppMemoryUsageLevel> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: AppMemoryUsageLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppMemoryUsageLevel>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageDecreased<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageDecreased<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageLimitChanging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageLimitChanging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn GetAppMemoryReport() -> ::windows::core::Result<AppMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppMemoryReport>(result__)
        })
    }
    pub fn GetProcessMemoryReport() -> ::windows::core::Result<ProcessMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessMemoryReport>(result__)
        })
    }
    pub fn TrySetAppMemoryUsageLimit(value: u64) -> ::windows::core::Result<bool> {
        Self::IMemoryManagerStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ExpectedAppMemoryUsageLimit() -> ::windows::core::Result<u64> {
        Self::IMemoryManagerStatics4(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    pub fn IMemoryManagerStatics<R, F: FnOnce(&IMemoryManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMemoryManagerStatics2<R, F: FnOnce(&IMemoryManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMemoryManagerStatics3<R, F: FnOnce(&IMemoryManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMemoryManagerStatics4<R, F: FnOnce(&IMemoryManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MemoryManager {
    const NAME: &'static str = "Windows.System.MemoryManager";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PowerState(pub i32);
impl PowerState {
    pub const ConnectedStandby: PowerState = PowerState(0i32);
    pub const SleepS3: PowerState = PowerState(1i32);
}
impl ::core::convert::From<i32> for PowerState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PowerState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PowerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.PowerState;i4)");
}
impl ::windows::core::DefaultType for PowerState {
    type DefaultType = Self;
}
pub struct ProcessLauncher {}
impl ProcessLauncher {
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(filename: Param0, args: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), filename.into_param().abi(), args.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<ProcessLauncherResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProcessLauncherOptions>>(filename: Param0, args: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), filename.into_param().abi(), args.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<ProcessLauncherResult>>(result__)
        })
    }
    pub fn IProcessLauncherStatics<R, F: FnOnce(&IProcessLauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProcessLauncher, IProcessLauncherStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ProcessLauncher {
    const NAME: &'static str = "Windows.System.ProcessLauncher";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessLauncherOptions(pub ::windows::core::IInspectable);
impl ProcessLauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProcessLauncherOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardInput(&self) -> ::windows::core::Result<super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardInput<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::Streams::IInputStream>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardOutput(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardOutput<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::Streams::IOutputStream>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardError(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardError<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::Streams::IOutputStream>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn WorkingDirectory(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetWorkingDirectory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessLauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherOptions;{3080b9cf-f444-4a83-beaf-a549a0f3229c})");
}
unsafe impl ::windows::core::Interface for ProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3080b9cf_f444_4a83_beaf_a549a0f3229c);
}
impl ::windows::core::RuntimeName for ProcessLauncherOptions {
    const NAME: &'static str = "Windows.System.ProcessLauncherOptions";
}
impl ::core::convert::From<ProcessLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: ProcessLauncherOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: &ProcessLauncherOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: ProcessLauncherOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: &ProcessLauncherOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessLauncherOptions {}
unsafe impl ::core::marker::Sync for ProcessLauncherOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessLauncherResult(pub ::windows::core::IInspectable);
impl ProcessLauncherResult {
    pub fn ExitCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessLauncherResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherResult;{544c8934-86d8-4991-8e75-ece8a43b6b6d})");
}
unsafe impl ::windows::core::Interface for ProcessLauncherResult {
    type Vtable = IProcessLauncherResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x544c8934_86d8_4991_8e75_ece8a43b6b6d);
}
impl ::windows::core::RuntimeName for ProcessLauncherResult {
    const NAME: &'static str = "Windows.System.ProcessLauncherResult";
}
impl ::core::convert::From<ProcessLauncherResult> for ::windows::core::IUnknown {
    fn from(value: ProcessLauncherResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessLauncherResult> for ::windows::core::IUnknown {
    fn from(value: &ProcessLauncherResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessLauncherResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessLauncherResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessLauncherResult> for ::windows::core::IInspectable {
    fn from(value: ProcessLauncherResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessLauncherResult> for ::windows::core::IInspectable {
    fn from(value: &ProcessLauncherResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessLauncherResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessLauncherResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessLauncherResult {}
unsafe impl ::core::marker::Sync for ProcessLauncherResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessMemoryReport(pub ::windows::core::IInspectable);
impl ProcessMemoryReport {
    pub fn PrivateWorkingSetUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn TotalWorkingSetUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessMemoryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessMemoryReport;{087305a8-9b70-4782-8741-3a982b6ce5e4})");
}
unsafe impl ::windows::core::Interface for ProcessMemoryReport {
    type Vtable = IProcessMemoryReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x087305a8_9b70_4782_8741_3a982b6ce5e4);
}
impl ::windows::core::RuntimeName for ProcessMemoryReport {
    const NAME: &'static str = "Windows.System.ProcessMemoryReport";
}
impl ::core::convert::From<ProcessMemoryReport> for ::windows::core::IUnknown {
    fn from(value: ProcessMemoryReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessMemoryReport> for ::windows::core::IUnknown {
    fn from(value: &ProcessMemoryReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessMemoryReport> for ::windows::core::IInspectable {
    fn from(value: ProcessMemoryReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessMemoryReport> for ::windows::core::IInspectable {
    fn from(value: &ProcessMemoryReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessMemoryReport {}
unsafe impl ::core::marker::Sync for ProcessMemoryReport {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProcessorArchitecture(pub i32);
impl ProcessorArchitecture {
    pub const X86: ProcessorArchitecture = ProcessorArchitecture(0i32);
    pub const Arm: ProcessorArchitecture = ProcessorArchitecture(5i32);
    pub const X64: ProcessorArchitecture = ProcessorArchitecture(9i32);
    pub const Neutral: ProcessorArchitecture = ProcessorArchitecture(11i32);
    pub const Arm64: ProcessorArchitecture = ProcessorArchitecture(12i32);
    pub const X86OnArm64: ProcessorArchitecture = ProcessorArchitecture(14i32);
    pub const Unknown: ProcessorArchitecture = ProcessorArchitecture(65535i32);
}
impl ::core::convert::From<i32> for ProcessorArchitecture {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ProcessorArchitecture {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ProcessorArchitecture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.ProcessorArchitecture;i4)");
}
impl ::windows::core::DefaultType for ProcessorArchitecture {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtocolForResultsOperation(pub ::windows::core::IInspectable);
impl ProtocolForResultsOperation {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ProtocolForResultsOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProtocolForResultsOperation;{d581293a-6de9-4d28-9378-f86782e182bb})");
}
unsafe impl ::windows::core::Interface for ProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd581293a_6de9_4d28_9378_f86782e182bb);
}
impl ::windows::core::RuntimeName for ProtocolForResultsOperation {
    const NAME: &'static str = "Windows.System.ProtocolForResultsOperation";
}
impl ::core::convert::From<ProtocolForResultsOperation> for ::windows::core::IUnknown {
    fn from(value: ProtocolForResultsOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtocolForResultsOperation> for ::windows::core::IUnknown {
    fn from(value: &ProtocolForResultsOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtocolForResultsOperation> for ::windows::core::IInspectable {
    fn from(value: ProtocolForResultsOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtocolForResultsOperation> for ::windows::core::IInspectable {
    fn from(value: &ProtocolForResultsOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtocolForResultsOperation {}
unsafe impl ::core::marker::Sync for ProtocolForResultsOperation {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteLaunchUriStatus(pub i32);
impl RemoteLaunchUriStatus {
    pub const Unknown: RemoteLaunchUriStatus = RemoteLaunchUriStatus(0i32);
    pub const Success: RemoteLaunchUriStatus = RemoteLaunchUriStatus(1i32);
    pub const AppUnavailable: RemoteLaunchUriStatus = RemoteLaunchUriStatus(2i32);
    pub const ProtocolUnavailable: RemoteLaunchUriStatus = RemoteLaunchUriStatus(3i32);
    pub const RemoteSystemUnavailable: RemoteLaunchUriStatus = RemoteLaunchUriStatus(4i32);
    pub const ValueSetTooLarge: RemoteLaunchUriStatus = RemoteLaunchUriStatus(5i32);
    pub const DeniedByLocalSystem: RemoteLaunchUriStatus = RemoteLaunchUriStatus(6i32);
    pub const DeniedByRemoteSystem: RemoteLaunchUriStatus = RemoteLaunchUriStatus(7i32);
}
impl ::core::convert::From<i32> for RemoteLaunchUriStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RemoteLaunchUriStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RemoteLaunchUriStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteLaunchUriStatus;i4)");
}
impl ::windows::core::DefaultType for RemoteLaunchUriStatus {
    type DefaultType = Self;
}
pub struct RemoteLauncher {}
impl RemoteLauncher {
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriAsync<'a, Param0: ::windows::core::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(remotesystemconnectionrequest: Param0, uri: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, RemoteLauncherOptions>>(remotesystemconnectionrequest: Param0, uri: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithDataAsync<'a, Param0: ::windows::core::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, RemoteLauncherOptions>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(remotesystemconnectionrequest: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    pub fn IRemoteLauncherStatics<R, F: FnOnce(&IRemoteLauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RemoteLauncher, IRemoteLauncherStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for RemoteLauncher {
    const NAME: &'static str = "Windows.System.RemoteLauncher";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteLauncherOptions(pub ::windows::core::IInspectable);
impl RemoteLauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RemoteLauncherOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferredAppIds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteLauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteLauncherOptions;{9e3a2788-2891-4cdf-a2d6-9dff7d02e693})");
}
unsafe impl ::windows::core::Interface for RemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e3a2788_2891_4cdf_a2d6_9dff7d02e693);
}
impl ::windows::core::RuntimeName for RemoteLauncherOptions {
    const NAME: &'static str = "Windows.System.RemoteLauncherOptions";
}
impl ::core::convert::From<RemoteLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: RemoteLauncherOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: &RemoteLauncherOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RemoteLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RemoteLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: RemoteLauncherOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: &RemoteLauncherOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RemoteLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RemoteLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteLauncherOptions {}
unsafe impl ::core::marker::Sync for RemoteLauncherOptions {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ShutdownKind(pub i32);
impl ShutdownKind {
    pub const Shutdown: ShutdownKind = ShutdownKind(0i32);
    pub const Restart: ShutdownKind = ShutdownKind(1i32);
}
impl ::core::convert::From<i32> for ShutdownKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ShutdownKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ShutdownKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.ShutdownKind;i4)");
}
impl ::windows::core::DefaultType for ShutdownKind {
    type DefaultType = Self;
}
pub struct ShutdownManager {}
impl ShutdownManager {
    #[cfg(feature = "Foundation")]
    pub fn BeginShutdown<'a, Param1: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(shutdownkind: ShutdownKind, timeout: Param1) -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), shutdownkind, timeout.into_param().abi()).ok() })
    }
    pub fn CancelShutdown() -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn IsPowerStateSupported(powerstate: PowerState) -> ::windows::core::Result<bool> {
        Self::IShutdownManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), powerstate, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn EnterPowerState(powerstate: PowerState) -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), powerstate).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn EnterPowerStateWithTimeSpan<'a, Param1: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(powerstate: PowerState, wakeupafter: Param1) -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), powerstate, wakeupafter.into_param().abi()).ok() })
    }
    pub fn IShutdownManagerStatics<R, F: FnOnce(&IShutdownManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ShutdownManager, IShutdownManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IShutdownManagerStatics2<R, F: FnOnce(&IShutdownManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ShutdownManager, IShutdownManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ShutdownManager {
    const NAME: &'static str = "Windows.System.ShutdownManager";
}
pub struct TimeZoneSettings {}
impl TimeZoneSettings {
    pub fn CurrentTimeZoneDisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTimeZoneDisplayNames() -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    pub fn CanChangeTimeZone() -> ::windows::core::Result<bool> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ChangeTimeZoneByDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(timezonedisplayname: Param0) -> ::windows::core::Result<()> {
        Self::ITimeZoneSettingsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), timezonedisplayname.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn AutoUpdateTimeZoneAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(timeout: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>> {
        Self::ITimeZoneSettingsStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timeout.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>(result__)
        })
    }
    pub fn ITimeZoneSettingsStatics<R, F: FnOnce(&ITimeZoneSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITimeZoneSettingsStatics2<R, F: FnOnce(&ITimeZoneSettingsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for TimeZoneSettings {
    const NAME: &'static str = "Windows.System.TimeZoneSettings";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct User(pub ::windows::core::IInspectable);
impl User {
    pub fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AuthenticationStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: UserAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<UserType> {
        let this = self;
        unsafe {
            let mut result__: UserType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetPropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(&self, values: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), values.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPictureAsync(&self, desiredsize: UserPictureSize) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), desiredsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>>(result__)
        }
    }
    pub fn CreateWatcher() -> ::windows::core::Result<UserWatcher> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserWatcher>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsyncByType(r#type: UserType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsyncByTypeAndStatus(r#type: UserType, status: UserAuthenticationStatus) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), r#type, status, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    pub fn GetFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(nonroamableid: Param0) -> ::windows::core::Result<User> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), nonroamableid.into_param().abi(), &mut result__).from_abi::<User>(result__)
        })
    }
    pub fn GetDefault() -> ::windows::core::Result<User> {
        Self::IUserStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CheckUserAgeConsentGroupAsync(&self, consentgroup: UserAgeConsentGroup) -> ::windows::core::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>> {
        let this = &::windows::core::Interface::cast::<IUser2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), consentgroup, &mut result__).from_abi::<super::Foundation::IAsyncOperation<UserAgeConsentResult>>(result__)
        }
    }
    pub fn IUserStatics<R, F: FnOnce(&IUserStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<User, IUserStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserStatics2<R, F: FnOnce(&IUserStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<User, IUserStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for User {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.User;{df9a26c6-e746-4bcd-b5d4-120103c4209b})");
}
unsafe impl ::windows::core::Interface for User {
    type Vtable = IUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf9a26c6_e746_4bcd_b5d4_120103c4209b);
}
impl ::windows::core::RuntimeName for User {
    const NAME: &'static str = "Windows.System.User";
}
impl ::core::convert::From<User> for ::windows::core::IUnknown {
    fn from(value: User) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&User> for ::windows::core::IUnknown {
    fn from(value: &User) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for User {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a User {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<User> for ::windows::core::IInspectable {
    fn from(value: User) -> Self {
        value.0
    }
}
impl ::core::convert::From<&User> for ::windows::core::IInspectable {
    fn from(value: &User) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for User {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a User {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for User {}
unsafe impl ::core::marker::Sync for User {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserAgeConsentGroup(pub i32);
impl UserAgeConsentGroup {
    pub const Child: UserAgeConsentGroup = UserAgeConsentGroup(0i32);
    pub const Minor: UserAgeConsentGroup = UserAgeConsentGroup(1i32);
    pub const Adult: UserAgeConsentGroup = UserAgeConsentGroup(2i32);
}
impl ::core::convert::From<i32> for UserAgeConsentGroup {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserAgeConsentGroup {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserAgeConsentGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentGroup;i4)");
}
impl ::windows::core::DefaultType for UserAgeConsentGroup {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserAgeConsentResult(pub i32);
impl UserAgeConsentResult {
    pub const NotEnforced: UserAgeConsentResult = UserAgeConsentResult(0i32);
    pub const Included: UserAgeConsentResult = UserAgeConsentResult(1i32);
    pub const NotIncluded: UserAgeConsentResult = UserAgeConsentResult(2i32);
    pub const Unknown: UserAgeConsentResult = UserAgeConsentResult(3i32);
    pub const Ambiguous: UserAgeConsentResult = UserAgeConsentResult(4i32);
}
impl ::core::convert::From<i32> for UserAgeConsentResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserAgeConsentResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserAgeConsentResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentResult;i4)");
}
impl ::windows::core::DefaultType for UserAgeConsentResult {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserAuthenticationStatus(pub i32);
impl UserAuthenticationStatus {
    pub const Unauthenticated: UserAuthenticationStatus = UserAuthenticationStatus(0i32);
    pub const LocallyAuthenticated: UserAuthenticationStatus = UserAuthenticationStatus(1i32);
    pub const RemotelyAuthenticated: UserAuthenticationStatus = UserAuthenticationStatus(2i32);
}
impl ::core::convert::From<i32> for UserAuthenticationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserAuthenticationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserAuthenticationStatus;i4)");
}
impl ::windows::core::DefaultType for UserAuthenticationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserAuthenticationStatusChangeDeferral(pub ::windows::core::IInspectable);
impl UserAuthenticationStatusChangeDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UserAuthenticationStatusChangeDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangeDeferral;{88b59568-bb30-42fb-a270-e9902e40efa7})");
}
unsafe impl ::windows::core::Interface for UserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88b59568_bb30_42fb_a270_e9902e40efa7);
}
impl ::windows::core::RuntimeName for UserAuthenticationStatusChangeDeferral {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangeDeferral";
}
impl ::core::convert::From<UserAuthenticationStatusChangeDeferral> for ::windows::core::IUnknown {
    fn from(value: UserAuthenticationStatusChangeDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangeDeferral> for ::windows::core::IUnknown {
    fn from(value: &UserAuthenticationStatusChangeDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserAuthenticationStatusChangeDeferral> for ::windows::core::IInspectable {
    fn from(value: UserAuthenticationStatusChangeDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangeDeferral> for ::windows::core::IInspectable {
    fn from(value: &UserAuthenticationStatusChangeDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangeDeferral {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangeDeferral {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserAuthenticationStatusChangingEventArgs(pub ::windows::core::IInspectable);
impl UserAuthenticationStatusChangingEventArgs {
    pub fn GetDeferral(&self) -> ::windows::core::Result<UserAuthenticationStatusChangeDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatusChangeDeferral>(result__)
        }
    }
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    pub fn NewStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: UserAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
    pub fn CurrentStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: UserAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserAuthenticationStatusChangingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangingEventArgs;{8c030f28-a711-4c1e-ab48-04179c15938f})");
}
unsafe impl ::windows::core::Interface for UserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c030f28_a711_4c1e_ab48_04179c15938f);
}
impl ::windows::core::RuntimeName for UserAuthenticationStatusChangingEventArgs {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangingEventArgs";
}
impl ::core::convert::From<UserAuthenticationStatusChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserAuthenticationStatusChangingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserAuthenticationStatusChangingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserAuthenticationStatusChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserAuthenticationStatusChangingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserAuthenticationStatusChangingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangingEventArgs {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserChangedEventArgs(pub ::windows::core::IInspectable);
impl UserChangedEventArgs {
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ChangedPropertyKinds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>> {
        let this = &::windows::core::Interface::cast::<IUserChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserChangedEventArgs;{086459dc-18c6-48db-bc99-724fb9203ccc})");
}
unsafe impl ::windows::core::Interface for UserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x086459dc_18c6_48db_bc99_724fb9203ccc);
}
impl ::windows::core::RuntimeName for UserChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserChangedEventArgs";
}
impl ::core::convert::From<UserChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserChangedEventArgs {}
pub struct UserDeviceAssociation {}
impl UserDeviceAssociation {
    pub fn FindUserFromDeviceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<User> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<User>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn UserDeviceAssociationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserDeviceAssociationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IUserDeviceAssociationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn IUserDeviceAssociationStatics<R, F: FnOnce(&IUserDeviceAssociationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDeviceAssociation, IUserDeviceAssociationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for UserDeviceAssociation {
    const NAME: &'static str = "Windows.System.UserDeviceAssociation";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDeviceAssociationChangedEventArgs(pub ::windows::core::IInspectable);
impl UserDeviceAssociationChangedEventArgs {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn NewUser(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    pub fn OldUser(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDeviceAssociationChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserDeviceAssociationChangedEventArgs;{bd1f6f6c-bb5d-4d7b-a5f0-c8cd11a38d42})");
}
unsafe impl ::windows::core::Interface for UserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1f6f6c_bb5d_4d7b_a5f0_c8cd11a38d42);
}
impl ::windows::core::RuntimeName for UserDeviceAssociationChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserDeviceAssociationChangedEventArgs";
}
impl ::core::convert::From<UserDeviceAssociationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserDeviceAssociationChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDeviceAssociationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserDeviceAssociationChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDeviceAssociationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserDeviceAssociationChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDeviceAssociationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserDeviceAssociationChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDeviceAssociationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDeviceAssociationChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserPicker(pub ::windows::core::IInspectable);
impl UserPicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserPicker, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AllowGuestAccounts(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowGuestAccounts(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SuggestedSelectedUser(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    pub fn SetSuggestedSelectedUser<'a, Param0: ::windows::core::IntoParam<'a, User>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PickSingleUserAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<User>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<User>>(result__)
        }
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IUserPickerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IUserPickerStatics<R, F: FnOnce(&IUserPickerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserPicker, IUserPickerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for UserPicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserPicker;{7d548008-f1e3-4a6c-8ddc-a9bb0f488aed})");
}
unsafe impl ::windows::core::Interface for UserPicker {
    type Vtable = IUserPicker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d548008_f1e3_4a6c_8ddc_a9bb0f488aed);
}
impl ::windows::core::RuntimeName for UserPicker {
    const NAME: &'static str = "Windows.System.UserPicker";
}
impl ::core::convert::From<UserPicker> for ::windows::core::IUnknown {
    fn from(value: UserPicker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserPicker> for ::windows::core::IUnknown {
    fn from(value: &UserPicker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserPicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserPicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserPicker> for ::windows::core::IInspectable {
    fn from(value: UserPicker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserPicker> for ::windows::core::IInspectable {
    fn from(value: &UserPicker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserPicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserPicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserPicker {}
unsafe impl ::core::marker::Sync for UserPicker {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserPictureSize(pub i32);
impl UserPictureSize {
    pub const Size64x64: UserPictureSize = UserPictureSize(0i32);
    pub const Size208x208: UserPictureSize = UserPictureSize(1i32);
    pub const Size424x424: UserPictureSize = UserPictureSize(2i32);
    pub const Size1080x1080: UserPictureSize = UserPictureSize(3i32);
}
impl ::core::convert::From<i32> for UserPictureSize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserPictureSize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserPictureSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserPictureSize;i4)");
}
impl ::windows::core::DefaultType for UserPictureSize {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserType(pub i32);
impl UserType {
    pub const LocalUser: UserType = UserType(0i32);
    pub const RemoteUser: UserType = UserType(1i32);
    pub const LocalGuest: UserType = UserType(2i32);
    pub const RemoteGuest: UserType = UserType(3i32);
    pub const SystemManaged: UserType = UserType(4i32);
}
impl ::core::convert::From<i32> for UserType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserType;i4)");
}
impl ::windows::core::DefaultType for UserType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserWatcher(pub ::windows::core::IInspectable);
impl UserWatcher {
    pub fn Status(&self) -> ::windows::core::Result<UserWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: UserWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UserWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserWatcher;{155eb23b-242a-45e0-a2e9-3171fc6a7fbb})");
}
unsafe impl ::windows::core::Interface for UserWatcher {
    type Vtable = IUserWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fbb);
}
impl ::windows::core::RuntimeName for UserWatcher {
    const NAME: &'static str = "Windows.System.UserWatcher";
}
impl ::core::convert::From<UserWatcher> for ::windows::core::IUnknown {
    fn from(value: UserWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserWatcher> for ::windows::core::IUnknown {
    fn from(value: &UserWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserWatcher> for ::windows::core::IInspectable {
    fn from(value: UserWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserWatcher> for ::windows::core::IInspectable {
    fn from(value: &UserWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserWatcher {}
unsafe impl ::core::marker::Sync for UserWatcher {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserWatcherStatus(pub i32);
impl UserWatcherStatus {
    pub const Created: UserWatcherStatus = UserWatcherStatus(0i32);
    pub const Started: UserWatcherStatus = UserWatcherStatus(1i32);
    pub const EnumerationCompleted: UserWatcherStatus = UserWatcherStatus(2i32);
    pub const Stopping: UserWatcherStatus = UserWatcherStatus(3i32);
    pub const Stopped: UserWatcherStatus = UserWatcherStatus(4i32);
    pub const Aborted: UserWatcherStatus = UserWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for UserWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherStatus;i4)");
}
impl ::windows::core::DefaultType for UserWatcherStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserWatcherUpdateKind(pub i32);
impl UserWatcherUpdateKind {
    pub const Properties: UserWatcherUpdateKind = UserWatcherUpdateKind(0i32);
    pub const Picture: UserWatcherUpdateKind = UserWatcherUpdateKind(1i32);
}
impl ::core::convert::From<i32> for UserWatcherUpdateKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserWatcherUpdateKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserWatcherUpdateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherUpdateKind;i4)");
}
impl ::windows::core::DefaultType for UserWatcherUpdateKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: VirtualKey = VirtualKey(0i32);
    pub const LeftButton: VirtualKey = VirtualKey(1i32);
    pub const RightButton: VirtualKey = VirtualKey(2i32);
    pub const Cancel: VirtualKey = VirtualKey(3i32);
    pub const MiddleButton: VirtualKey = VirtualKey(4i32);
    pub const XButton1: VirtualKey = VirtualKey(5i32);
    pub const XButton2: VirtualKey = VirtualKey(6i32);
    pub const Back: VirtualKey = VirtualKey(8i32);
    pub const Tab: VirtualKey = VirtualKey(9i32);
    pub const Clear: VirtualKey = VirtualKey(12i32);
    pub const Enter: VirtualKey = VirtualKey(13i32);
    pub const Shift: VirtualKey = VirtualKey(16i32);
    pub const Control: VirtualKey = VirtualKey(17i32);
    pub const Menu: VirtualKey = VirtualKey(18i32);
    pub const Pause: VirtualKey = VirtualKey(19i32);
    pub const CapitalLock: VirtualKey = VirtualKey(20i32);
    pub const Kana: VirtualKey = VirtualKey(21i32);
    pub const Hangul: VirtualKey = VirtualKey(21i32);
    pub const ImeOn: VirtualKey = VirtualKey(22i32);
    pub const Junja: VirtualKey = VirtualKey(23i32);
    pub const Final: VirtualKey = VirtualKey(24i32);
    pub const Hanja: VirtualKey = VirtualKey(25i32);
    pub const Kanji: VirtualKey = VirtualKey(25i32);
    pub const ImeOff: VirtualKey = VirtualKey(26i32);
    pub const Escape: VirtualKey = VirtualKey(27i32);
    pub const Convert: VirtualKey = VirtualKey(28i32);
    pub const NonConvert: VirtualKey = VirtualKey(29i32);
    pub const Accept: VirtualKey = VirtualKey(30i32);
    pub const ModeChange: VirtualKey = VirtualKey(31i32);
    pub const Space: VirtualKey = VirtualKey(32i32);
    pub const PageUp: VirtualKey = VirtualKey(33i32);
    pub const PageDown: VirtualKey = VirtualKey(34i32);
    pub const End: VirtualKey = VirtualKey(35i32);
    pub const Home: VirtualKey = VirtualKey(36i32);
    pub const Left: VirtualKey = VirtualKey(37i32);
    pub const Up: VirtualKey = VirtualKey(38i32);
    pub const Right: VirtualKey = VirtualKey(39i32);
    pub const Down: VirtualKey = VirtualKey(40i32);
    pub const Select: VirtualKey = VirtualKey(41i32);
    pub const Print: VirtualKey = VirtualKey(42i32);
    pub const Execute: VirtualKey = VirtualKey(43i32);
    pub const Snapshot: VirtualKey = VirtualKey(44i32);
    pub const Insert: VirtualKey = VirtualKey(45i32);
    pub const Delete: VirtualKey = VirtualKey(46i32);
    pub const Help: VirtualKey = VirtualKey(47i32);
    pub const Number0: VirtualKey = VirtualKey(48i32);
    pub const Number1: VirtualKey = VirtualKey(49i32);
    pub const Number2: VirtualKey = VirtualKey(50i32);
    pub const Number3: VirtualKey = VirtualKey(51i32);
    pub const Number4: VirtualKey = VirtualKey(52i32);
    pub const Number5: VirtualKey = VirtualKey(53i32);
    pub const Number6: VirtualKey = VirtualKey(54i32);
    pub const Number7: VirtualKey = VirtualKey(55i32);
    pub const Number8: VirtualKey = VirtualKey(56i32);
    pub const Number9: VirtualKey = VirtualKey(57i32);
    pub const A: VirtualKey = VirtualKey(65i32);
    pub const B: VirtualKey = VirtualKey(66i32);
    pub const C: VirtualKey = VirtualKey(67i32);
    pub const D: VirtualKey = VirtualKey(68i32);
    pub const E: VirtualKey = VirtualKey(69i32);
    pub const F: VirtualKey = VirtualKey(70i32);
    pub const G: VirtualKey = VirtualKey(71i32);
    pub const H: VirtualKey = VirtualKey(72i32);
    pub const I: VirtualKey = VirtualKey(73i32);
    pub const J: VirtualKey = VirtualKey(74i32);
    pub const K: VirtualKey = VirtualKey(75i32);
    pub const L: VirtualKey = VirtualKey(76i32);
    pub const M: VirtualKey = VirtualKey(77i32);
    pub const N: VirtualKey = VirtualKey(78i32);
    pub const O: VirtualKey = VirtualKey(79i32);
    pub const P: VirtualKey = VirtualKey(80i32);
    pub const Q: VirtualKey = VirtualKey(81i32);
    pub const R: VirtualKey = VirtualKey(82i32);
    pub const S: VirtualKey = VirtualKey(83i32);
    pub const T: VirtualKey = VirtualKey(84i32);
    pub const U: VirtualKey = VirtualKey(85i32);
    pub const V: VirtualKey = VirtualKey(86i32);
    pub const W: VirtualKey = VirtualKey(87i32);
    pub const X: VirtualKey = VirtualKey(88i32);
    pub const Y: VirtualKey = VirtualKey(89i32);
    pub const Z: VirtualKey = VirtualKey(90i32);
    pub const LeftWindows: VirtualKey = VirtualKey(91i32);
    pub const RightWindows: VirtualKey = VirtualKey(92i32);
    pub const Application: VirtualKey = VirtualKey(93i32);
    pub const Sleep: VirtualKey = VirtualKey(95i32);
    pub const NumberPad0: VirtualKey = VirtualKey(96i32);
    pub const NumberPad1: VirtualKey = VirtualKey(97i32);
    pub const NumberPad2: VirtualKey = VirtualKey(98i32);
    pub const NumberPad3: VirtualKey = VirtualKey(99i32);
    pub const NumberPad4: VirtualKey = VirtualKey(100i32);
    pub const NumberPad5: VirtualKey = VirtualKey(101i32);
    pub const NumberPad6: VirtualKey = VirtualKey(102i32);
    pub const NumberPad7: VirtualKey = VirtualKey(103i32);
    pub const NumberPad8: VirtualKey = VirtualKey(104i32);
    pub const NumberPad9: VirtualKey = VirtualKey(105i32);
    pub const Multiply: VirtualKey = VirtualKey(106i32);
    pub const Add: VirtualKey = VirtualKey(107i32);
    pub const Separator: VirtualKey = VirtualKey(108i32);
    pub const Subtract: VirtualKey = VirtualKey(109i32);
    pub const Decimal: VirtualKey = VirtualKey(110i32);
    pub const Divide: VirtualKey = VirtualKey(111i32);
    pub const F1: VirtualKey = VirtualKey(112i32);
    pub const F2: VirtualKey = VirtualKey(113i32);
    pub const F3: VirtualKey = VirtualKey(114i32);
    pub const F4: VirtualKey = VirtualKey(115i32);
    pub const F5: VirtualKey = VirtualKey(116i32);
    pub const F6: VirtualKey = VirtualKey(117i32);
    pub const F7: VirtualKey = VirtualKey(118i32);
    pub const F8: VirtualKey = VirtualKey(119i32);
    pub const F9: VirtualKey = VirtualKey(120i32);
    pub const F10: VirtualKey = VirtualKey(121i32);
    pub const F11: VirtualKey = VirtualKey(122i32);
    pub const F12: VirtualKey = VirtualKey(123i32);
    pub const F13: VirtualKey = VirtualKey(124i32);
    pub const F14: VirtualKey = VirtualKey(125i32);
    pub const F15: VirtualKey = VirtualKey(126i32);
    pub const F16: VirtualKey = VirtualKey(127i32);
    pub const F17: VirtualKey = VirtualKey(128i32);
    pub const F18: VirtualKey = VirtualKey(129i32);
    pub const F19: VirtualKey = VirtualKey(130i32);
    pub const F20: VirtualKey = VirtualKey(131i32);
    pub const F21: VirtualKey = VirtualKey(132i32);
    pub const F22: VirtualKey = VirtualKey(133i32);
    pub const F23: VirtualKey = VirtualKey(134i32);
    pub const F24: VirtualKey = VirtualKey(135i32);
    pub const NavigationView: VirtualKey = VirtualKey(136i32);
    pub const NavigationMenu: VirtualKey = VirtualKey(137i32);
    pub const NavigationUp: VirtualKey = VirtualKey(138i32);
    pub const NavigationDown: VirtualKey = VirtualKey(139i32);
    pub const NavigationLeft: VirtualKey = VirtualKey(140i32);
    pub const NavigationRight: VirtualKey = VirtualKey(141i32);
    pub const NavigationAccept: VirtualKey = VirtualKey(142i32);
    pub const NavigationCancel: VirtualKey = VirtualKey(143i32);
    pub const NumberKeyLock: VirtualKey = VirtualKey(144i32);
    pub const Scroll: VirtualKey = VirtualKey(145i32);
    pub const LeftShift: VirtualKey = VirtualKey(160i32);
    pub const RightShift: VirtualKey = VirtualKey(161i32);
    pub const LeftControl: VirtualKey = VirtualKey(162i32);
    pub const RightControl: VirtualKey = VirtualKey(163i32);
    pub const LeftMenu: VirtualKey = VirtualKey(164i32);
    pub const RightMenu: VirtualKey = VirtualKey(165i32);
    pub const GoBack: VirtualKey = VirtualKey(166i32);
    pub const GoForward: VirtualKey = VirtualKey(167i32);
    pub const Refresh: VirtualKey = VirtualKey(168i32);
    pub const Stop: VirtualKey = VirtualKey(169i32);
    pub const Search: VirtualKey = VirtualKey(170i32);
    pub const Favorites: VirtualKey = VirtualKey(171i32);
    pub const GoHome: VirtualKey = VirtualKey(172i32);
    pub const GamepadA: VirtualKey = VirtualKey(195i32);
    pub const GamepadB: VirtualKey = VirtualKey(196i32);
    pub const GamepadX: VirtualKey = VirtualKey(197i32);
    pub const GamepadY: VirtualKey = VirtualKey(198i32);
    pub const GamepadRightShoulder: VirtualKey = VirtualKey(199i32);
    pub const GamepadLeftShoulder: VirtualKey = VirtualKey(200i32);
    pub const GamepadLeftTrigger: VirtualKey = VirtualKey(201i32);
    pub const GamepadRightTrigger: VirtualKey = VirtualKey(202i32);
    pub const GamepadDPadUp: VirtualKey = VirtualKey(203i32);
    pub const GamepadDPadDown: VirtualKey = VirtualKey(204i32);
    pub const GamepadDPadLeft: VirtualKey = VirtualKey(205i32);
    pub const GamepadDPadRight: VirtualKey = VirtualKey(206i32);
    pub const GamepadMenu: VirtualKey = VirtualKey(207i32);
    pub const GamepadView: VirtualKey = VirtualKey(208i32);
    pub const GamepadLeftThumbstickButton: VirtualKey = VirtualKey(209i32);
    pub const GamepadRightThumbstickButton: VirtualKey = VirtualKey(210i32);
    pub const GamepadLeftThumbstickUp: VirtualKey = VirtualKey(211i32);
    pub const GamepadLeftThumbstickDown: VirtualKey = VirtualKey(212i32);
    pub const GamepadLeftThumbstickRight: VirtualKey = VirtualKey(213i32);
    pub const GamepadLeftThumbstickLeft: VirtualKey = VirtualKey(214i32);
    pub const GamepadRightThumbstickUp: VirtualKey = VirtualKey(215i32);
    pub const GamepadRightThumbstickDown: VirtualKey = VirtualKey(216i32);
    pub const GamepadRightThumbstickRight: VirtualKey = VirtualKey(217i32);
    pub const GamepadRightThumbstickLeft: VirtualKey = VirtualKey(218i32);
}
impl ::core::convert::From<i32> for VirtualKey {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VirtualKey {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VirtualKey {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKey;i4)");
}
impl ::windows::core::DefaultType for VirtualKey {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VirtualKeyModifiers(pub u32);
impl VirtualKeyModifiers {
    pub const None: VirtualKeyModifiers = VirtualKeyModifiers(0u32);
    pub const Control: VirtualKeyModifiers = VirtualKeyModifiers(1u32);
    pub const Menu: VirtualKeyModifiers = VirtualKeyModifiers(2u32);
    pub const Shift: VirtualKeyModifiers = VirtualKeyModifiers(4u32);
    pub const Windows: VirtualKeyModifiers = VirtualKeyModifiers(8u32);
}
impl ::core::convert::From<u32> for VirtualKeyModifiers {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VirtualKeyModifiers {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VirtualKeyModifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKeyModifiers;u4)");
}
impl ::windows::core::DefaultType for VirtualKeyModifiers {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for VirtualKeyModifiers {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for VirtualKeyModifiers {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for VirtualKeyModifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for VirtualKeyModifiers {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for VirtualKeyModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
