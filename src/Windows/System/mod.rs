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
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppActivationResult(pub ::windows::runtime::IInspectable);
impl AppActivationResult {
    #[doc = "*Required features: `System`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn AppResourceGroupInfo(&self) -> ::windows::runtime::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppActivationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppActivationResult;{6b528900-f46e-4eb0-aa6c-38af557cf9ed})");
}
unsafe impl ::windows::runtime::Interface for AppActivationResult {
    type Vtable = IAppActivationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1800571136, 62574, 20144, [170, 108, 56, 175, 85, 124, 249, 237]);
}
impl ::windows::runtime::RuntimeName for AppActivationResult {
    const NAME: &'static str = "Windows.System.AppActivationResult";
}
impl ::core::convert::From<AppActivationResult> for ::windows::runtime::IUnknown {
    fn from(value: AppActivationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppActivationResult> for ::windows::runtime::IUnknown {
    fn from(value: &AppActivationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppActivationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppActivationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppActivationResult> for ::windows::runtime::IInspectable {
    fn from(value: AppActivationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppActivationResult> for ::windows::runtime::IInspectable {
    fn from(value: &AppActivationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppActivationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppActivationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppActivationResult {}
unsafe impl ::core::marker::Sync for AppActivationResult {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppDiagnosticInfo(pub ::windows::runtime::IInspectable);
impl AppDiagnosticInfo {
    #[cfg(feature = "ApplicationModel")]
    #[doc = "*Required features: `System`, `ApplicationModel`*"]
    pub fn AppInfo(&self) -> ::windows::runtime::Result<super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ApplicationModel::AppInfo>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn RequestInfoAsync() -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn GetResourceGroups(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>> {
        let this = &::windows::runtime::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppResourceGroupInfo>>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn CreateResourceGroupWatcher(&self) -> ::windows::runtime::Result<AppResourceGroupInfoWatcher> {
        let this = &::windows::runtime::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfoWatcher>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn CreateWatcher() -> ::windows::runtime::Result<AppDiagnosticInfoWatcher> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDiagnosticInfoWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RequestAccessAsync() -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn RequestInfoForPackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(packagefamilyname: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn RequestInfoForAppAsync() -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn RequestInfoForAppUserModelId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appusermodelid: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), appusermodelid.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchAsync(&self) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<AppActivationResult>> {
        let this = &::windows::runtime::Interface::cast::<IAppDiagnosticInfo3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppActivationResult>>(result__)
        }
    }
    pub fn IAppDiagnosticInfoStatics<R, F: FnOnce(&IAppDiagnosticInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppDiagnosticInfoStatics2<R, F: FnOnce(&IAppDiagnosticInfoStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppDiagnosticInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfo;{e348a69a-8889-4ca3-be07-d5ffff5f0804})");
}
unsafe impl ::windows::runtime::Interface for AppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3813189274, 34953, 19619, [190, 7, 213, 255, 255, 95, 8, 4]);
}
impl ::windows::runtime::RuntimeName for AppDiagnosticInfo {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfo";
}
impl ::core::convert::From<AppDiagnosticInfo> for ::windows::runtime::IUnknown {
    fn from(value: AppDiagnosticInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppDiagnosticInfo> for ::windows::runtime::IUnknown {
    fn from(value: &AppDiagnosticInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppDiagnosticInfo> for ::windows::runtime::IInspectable {
    fn from(value: AppDiagnosticInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppDiagnosticInfo> for ::windows::runtime::IInspectable {
    fn from(value: &AppDiagnosticInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppDiagnosticInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfo {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfo {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppDiagnosticInfoWatcher(pub ::windows::runtime::IInspectable);
impl AppDiagnosticInfoWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<AppDiagnosticInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: AppDiagnosticInfoWatcherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDiagnosticInfoWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppDiagnosticInfoWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcher;{75575070-01d3-489a-9325-52f9cc6ede0a})");
}
unsafe impl ::windows::runtime::Interface for AppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1968656496, 467, 18586, [147, 37, 82, 249, 204, 110, 222, 10]);
}
impl ::windows::runtime::RuntimeName for AppDiagnosticInfoWatcher {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcher";
}
impl ::core::convert::From<AppDiagnosticInfoWatcher> for ::windows::runtime::IUnknown {
    fn from(value: AppDiagnosticInfoWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &AppDiagnosticInfoWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppDiagnosticInfoWatcher> for ::windows::runtime::IInspectable {
    fn from(value: AppDiagnosticInfoWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &AppDiagnosticInfoWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcher {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcher {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppDiagnosticInfoWatcherEventArgs(pub ::windows::runtime::IInspectable);
impl AppDiagnosticInfoWatcherEventArgs {
    #[doc = "*Required features: `System`*"]
    pub fn AppDiagnosticInfo(&self) -> ::windows::runtime::Result<AppDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDiagnosticInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppDiagnosticInfoWatcherEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcherEventArgs;{7017c716-e1da-4c65-99df-046dff5be71a})");
}
unsafe impl ::windows::runtime::Interface for AppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1880606486, 57818, 19557, [153, 223, 4, 109, 255, 91, 231, 26]);
}
impl ::windows::runtime::RuntimeName for AppDiagnosticInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcherEventArgs";
}
impl ::core::convert::From<AppDiagnosticInfoWatcherEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppDiagnosticInfoWatcherEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcherEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppDiagnosticInfoWatcherEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppDiagnosticInfoWatcherEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppDiagnosticInfoWatcherEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcherEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppDiagnosticInfoWatcherEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcherEventArgs {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for AppDiagnosticInfoWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppDiagnosticInfoWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.AppDiagnosticInfoWatcherStatus;i4)");
}
impl ::windows::runtime::DefaultType for AppDiagnosticInfoWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppExecutionStateChangeResult(pub ::windows::runtime::IInspectable);
impl AppExecutionStateChangeResult {
    #[doc = "*Required features: `System`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppExecutionStateChangeResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppExecutionStateChangeResult;{6f039bf0-f91b-4df8-ae77-3033ccb69114})");
}
unsafe impl ::windows::runtime::Interface for AppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1862507504, 63771, 19960, [174, 119, 48, 51, 204, 182, 145, 20]);
}
impl ::windows::runtime::RuntimeName for AppExecutionStateChangeResult {
    const NAME: &'static str = "Windows.System.AppExecutionStateChangeResult";
}
impl ::core::convert::From<AppExecutionStateChangeResult> for ::windows::runtime::IUnknown {
    fn from(value: AppExecutionStateChangeResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppExecutionStateChangeResult> for ::windows::runtime::IUnknown {
    fn from(value: &AppExecutionStateChangeResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppExecutionStateChangeResult> for ::windows::runtime::IInspectable {
    fn from(value: AppExecutionStateChangeResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppExecutionStateChangeResult> for ::windows::runtime::IInspectable {
    fn from(value: &AppExecutionStateChangeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppExecutionStateChangeResult {}
unsafe impl ::core::marker::Sync for AppExecutionStateChangeResult {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppMemoryReport(pub ::windows::runtime::IInspectable);
impl AppMemoryReport {
    #[doc = "*Required features: `System`*"]
    pub fn PrivateCommitUsage(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn PeakPrivateCommitUsage(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn TotalCommitUsage(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn TotalCommitLimit(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn ExpectedTotalCommitLimit(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<IAppMemoryReport2>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppMemoryReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryReport;{6d65339b-4d6f-45bc-9c5e-e49b3ff2758d})");
}
unsafe impl ::windows::runtime::Interface for AppMemoryReport {
    type Vtable = IAppMemoryReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1835348891, 19823, 17852, [156, 94, 228, 155, 63, 242, 117, 141]);
}
impl ::windows::runtime::RuntimeName for AppMemoryReport {
    const NAME: &'static str = "Windows.System.AppMemoryReport";
}
impl ::core::convert::From<AppMemoryReport> for ::windows::runtime::IUnknown {
    fn from(value: AppMemoryReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppMemoryReport> for ::windows::runtime::IUnknown {
    fn from(value: &AppMemoryReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppMemoryReport> for ::windows::runtime::IInspectable {
    fn from(value: AppMemoryReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppMemoryReport> for ::windows::runtime::IInspectable {
    fn from(value: &AppMemoryReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppMemoryReport {}
unsafe impl ::core::marker::Sync for AppMemoryReport {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for AppMemoryUsageLevel {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppMemoryUsageLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.AppMemoryUsageLevel;i4)");
}
impl ::windows::runtime::DefaultType for AppMemoryUsageLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppMemoryUsageLimitChangingEventArgs(pub ::windows::runtime::IInspectable);
impl AppMemoryUsageLimitChangingEventArgs {
    #[doc = "*Required features: `System`*"]
    pub fn OldLimit(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn NewLimit(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppMemoryUsageLimitChangingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryUsageLimitChangingEventArgs;{79f86664-feca-4da5-9e40-2bc63efdc979})");
}
unsafe impl ::windows::runtime::Interface for AppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2046322276, 65226, 19877, [158, 64, 43, 198, 62, 253, 201, 121]);
}
impl ::windows::runtime::RuntimeName for AppMemoryUsageLimitChangingEventArgs {
    const NAME: &'static str = "Windows.System.AppMemoryUsageLimitChangingEventArgs";
}
impl ::core::convert::From<AppMemoryUsageLimitChangingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppMemoryUsageLimitChangingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppMemoryUsageLimitChangingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppMemoryUsageLimitChangingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppMemoryUsageLimitChangingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppMemoryUsageLimitChangingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppMemoryUsageLimitChangingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppMemoryUsageLimitChangingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppMemoryUsageLimitChangingEventArgs {}
unsafe impl ::core::marker::Sync for AppMemoryUsageLimitChangingEventArgs {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupBackgroundTaskReport(pub ::windows::runtime::IInspectable);
impl AppResourceGroupBackgroundTaskReport {
    #[doc = "*Required features: `System`*"]
    pub fn TaskId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Trigger(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn EntryPoint(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupBackgroundTaskReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupBackgroundTaskReport;{2566e74e-b05d-40c2-9dc1-1a4f039ea120})");
}
unsafe impl ::windows::runtime::Interface for AppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(627500878, 45149, 16578, [157, 193, 26, 79, 3, 158, 161, 32]);
}
impl ::windows::runtime::RuntimeName for AppResourceGroupBackgroundTaskReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupBackgroundTaskReport";
}
impl ::core::convert::From<AppResourceGroupBackgroundTaskReport> for ::windows::runtime::IUnknown {
    fn from(value: AppResourceGroupBackgroundTaskReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupBackgroundTaskReport> for ::windows::runtime::IUnknown {
    fn from(value: &AppResourceGroupBackgroundTaskReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupBackgroundTaskReport> for ::windows::runtime::IInspectable {
    fn from(value: AppResourceGroupBackgroundTaskReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupBackgroundTaskReport> for ::windows::runtime::IInspectable {
    fn from(value: &AppResourceGroupBackgroundTaskReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupBackgroundTaskReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupBackgroundTaskReport {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for AppResourceGroupEnergyQuotaState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupEnergyQuotaState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupEnergyQuotaState;i4)");
}
impl ::windows::runtime::DefaultType for AppResourceGroupEnergyQuotaState {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for AppResourceGroupExecutionState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupExecutionState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupExecutionState;i4)");
}
impl ::windows::runtime::DefaultType for AppResourceGroupExecutionState {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupInfo(pub ::windows::runtime::IInspectable);
impl AppResourceGroupInfo {
    #[doc = "*Required features: `System`*"]
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn IsShared(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn GetBackgroundTaskReports(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetMemoryReport(&self) -> ::windows::runtime::Result<AppResourceGroupMemoryReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupMemoryReport>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    #[doc = "*Required features: `System`, `Foundation_Collections`, `System_Diagnostics`*"]
    pub fn GetProcessDiagnosticInfos(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetStateReport(&self) -> ::windows::runtime::Result<AppResourceGroupStateReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupStateReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn StartSuspendAsync(&self) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::runtime::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn StartResumeAsync(&self) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::runtime::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn StartTerminateAsync(&self) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::runtime::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfo;{b913f77a-e807-49f4-845e-7b8bdcfe8ee7})");
}
unsafe impl ::windows::runtime::Interface for AppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3105093498, 59399, 18932, [132, 94, 123, 139, 220, 254, 142, 231]);
}
impl ::windows::runtime::RuntimeName for AppResourceGroupInfo {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfo";
}
impl ::core::convert::From<AppResourceGroupInfo> for ::windows::runtime::IUnknown {
    fn from(value: AppResourceGroupInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupInfo> for ::windows::runtime::IUnknown {
    fn from(value: &AppResourceGroupInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppResourceGroupInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppResourceGroupInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupInfo> for ::windows::runtime::IInspectable {
    fn from(value: AppResourceGroupInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupInfo> for ::windows::runtime::IInspectable {
    fn from(value: &AppResourceGroupInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppResourceGroupInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppResourceGroupInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfo {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfo {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupInfoWatcher(pub ::windows::runtime::IInspectable);
impl AppResourceGroupInfoWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn ExecutionStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveExecutionStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<AppResourceGroupInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: AppResourceGroupInfoWatcherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfoWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupInfoWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcher;{d9b0a0fd-6e5a-4c72-8b17-09fec4a212bd})");
}
unsafe impl ::windows::runtime::Interface for AppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3652231421, 28250, 19570, [139, 23, 9, 254, 196, 162, 18, 189]);
}
impl ::windows::runtime::RuntimeName for AppResourceGroupInfoWatcher {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcher";
}
impl ::core::convert::From<AppResourceGroupInfoWatcher> for ::windows::runtime::IUnknown {
    fn from(value: AppResourceGroupInfoWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcher> for ::windows::runtime::IInspectable {
    fn from(value: AppResourceGroupInfoWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcher {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcher {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupInfoWatcherEventArgs(pub ::windows::runtime::IInspectable);
impl AppResourceGroupInfoWatcherEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn AppDiagnosticInfos(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn AppResourceGroupInfo(&self) -> ::windows::runtime::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupInfoWatcherEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherEventArgs;{7a787637-6302-4d2f-bf89-1c12d0b2a6b9})");
}
unsafe impl ::windows::runtime::Interface for AppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2054714935, 25346, 19759, [191, 137, 28, 18, 208, 178, 166, 185]);
}
impl ::windows::runtime::RuntimeName for AppResourceGroupInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherEventArgs";
}
impl ::core::convert::From<AppResourceGroupInfoWatcherEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppResourceGroupInfoWatcherEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcherEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcherEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppResourceGroupInfoWatcherEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcherEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherEventArgs {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupInfoWatcherExecutionStateChangedEventArgs(pub ::windows::runtime::IInspectable);
impl AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn AppDiagnosticInfos(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn AppResourceGroupInfo(&self) -> ::windows::runtime::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs;{1bdbedd7-fee6-4fd4-98dd-e92a2cc299f3})");
}
unsafe impl ::windows::runtime::Interface for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(467398103, 65254, 20436, [152, 221, 233, 42, 44, 194, 153, 243]);
}
impl ::windows::runtime::RuntimeName for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs";
}
impl ::core::convert::From<AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for AppResourceGroupInfoWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupInfoWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupInfoWatcherStatus;i4)");
}
impl ::windows::runtime::DefaultType for AppResourceGroupInfoWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupMemoryReport(pub ::windows::runtime::IInspectable);
impl AppResourceGroupMemoryReport {
    #[doc = "*Required features: `System`*"]
    pub fn CommitUsageLimit(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn CommitUsageLevel(&self) -> ::windows::runtime::Result<AppMemoryUsageLevel> {
        let this = self;
        unsafe {
            let mut result__: AppMemoryUsageLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppMemoryUsageLevel>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn PrivateCommitUsage(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn TotalCommitUsage(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupMemoryReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupMemoryReport;{2c8c06b1-7db1-4c51-a225-7fae2d49e431})");
}
unsafe impl ::windows::runtime::Interface for AppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747374257, 32177, 19537, [162, 37, 127, 174, 45, 73, 228, 49]);
}
impl ::windows::runtime::RuntimeName for AppResourceGroupMemoryReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupMemoryReport";
}
impl ::core::convert::From<AppResourceGroupMemoryReport> for ::windows::runtime::IUnknown {
    fn from(value: AppResourceGroupMemoryReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupMemoryReport> for ::windows::runtime::IUnknown {
    fn from(value: &AppResourceGroupMemoryReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupMemoryReport> for ::windows::runtime::IInspectable {
    fn from(value: AppResourceGroupMemoryReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupMemoryReport> for ::windows::runtime::IInspectable {
    fn from(value: &AppResourceGroupMemoryReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupMemoryReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupMemoryReport {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppResourceGroupStateReport(pub ::windows::runtime::IInspectable);
impl AppResourceGroupStateReport {
    #[doc = "*Required features: `System`*"]
    pub fn ExecutionState(&self) -> ::windows::runtime::Result<AppResourceGroupExecutionState> {
        let this = self;
        unsafe {
            let mut result__: AppResourceGroupExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn EnergyQuotaState(&self) -> ::windows::runtime::Result<AppResourceGroupEnergyQuotaState> {
        let this = self;
        unsafe {
            let mut result__: AppResourceGroupEnergyQuotaState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupEnergyQuotaState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppResourceGroupStateReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupStateReport;{52849f18-2f70-4236-ab40-d04db0c7b931})");
}
unsafe impl ::windows::runtime::Interface for AppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1384423192, 12144, 16950, [171, 64, 208, 77, 176, 199, 185, 49]);
}
impl ::windows::runtime::RuntimeName for AppResourceGroupStateReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupStateReport";
}
impl ::core::convert::From<AppResourceGroupStateReport> for ::windows::runtime::IUnknown {
    fn from(value: AppResourceGroupStateReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppResourceGroupStateReport> for ::windows::runtime::IUnknown {
    fn from(value: &AppResourceGroupStateReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppResourceGroupStateReport> for ::windows::runtime::IInspectable {
    fn from(value: AppResourceGroupStateReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppResourceGroupStateReport> for ::windows::runtime::IInspectable {
    fn from(value: &AppResourceGroupStateReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupStateReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupStateReport {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppUriHandlerHost(pub ::windows::runtime::IInspectable);
impl AppUriHandlerHost {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppUriHandlerHost, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<AppUriHandlerHost> {
        Self::IAppUriHandlerHostFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<AppUriHandlerHost>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IAppUriHandlerHostFactory<R, F: FnOnce(&IAppUriHandlerHostFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppUriHandlerHost, IAppUriHandlerHostFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppUriHandlerHost {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerHost;{5d50cac5-92d2-5409-b56f-7f73e10ea4c3})");
}
unsafe impl ::windows::runtime::Interface for AppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1565575877, 37586, 21513, [181, 111, 127, 115, 225, 14, 164, 195]);
}
impl ::windows::runtime::RuntimeName for AppUriHandlerHost {
    const NAME: &'static str = "Windows.System.AppUriHandlerHost";
}
impl ::core::convert::From<AppUriHandlerHost> for ::windows::runtime::IUnknown {
    fn from(value: AppUriHandlerHost) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppUriHandlerHost> for ::windows::runtime::IUnknown {
    fn from(value: &AppUriHandlerHost) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppUriHandlerHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppUriHandlerHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppUriHandlerHost> for ::windows::runtime::IInspectable {
    fn from(value: AppUriHandlerHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppUriHandlerHost> for ::windows::runtime::IInspectable {
    fn from(value: &AppUriHandlerHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppUriHandlerHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppUriHandlerHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerHost {}
unsafe impl ::core::marker::Sync for AppUriHandlerHost {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppUriHandlerRegistration(pub ::windows::runtime::IInspectable);
impl AppUriHandlerRegistration {
    #[doc = "*Required features: `System`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetAppAddedHostsAsync(&self) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetAppAddedHostsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IIterable<AppUriHandlerHost>>>(&self, hosts: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), hosts.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn GetAllHosts(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>> {
        let this = &::windows::runtime::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppUriHandlerHost>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn UpdateHosts<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IIterable<AppUriHandlerHost>>>(&self, hosts: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), hosts.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn PackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppUriHandlerRegistration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistration;{6f73aeb1-4569-5c3f-9ba0-99123eea32c3})");
}
unsafe impl ::windows::runtime::Interface for AppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1869852337, 17769, 23615, [155, 160, 153, 18, 62, 234, 50, 195]);
}
impl ::windows::runtime::RuntimeName for AppUriHandlerRegistration {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistration";
}
impl ::core::convert::From<AppUriHandlerRegistration> for ::windows::runtime::IUnknown {
    fn from(value: AppUriHandlerRegistration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppUriHandlerRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &AppUriHandlerRegistration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppUriHandlerRegistration> for ::windows::runtime::IInspectable {
    fn from(value: AppUriHandlerRegistration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppUriHandlerRegistration> for ::windows::runtime::IInspectable {
    fn from(value: &AppUriHandlerRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerRegistration {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistration {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppUriHandlerRegistrationManager(pub ::windows::runtime::IInspectable);
impl AppUriHandlerRegistrationManager {
    #[doc = "*Required features: `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn TryGetRegistration<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<AppUriHandlerRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistration>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, User>>(user: Param0) -> ::windows::runtime::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn PackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppUriHandlerRegistrationManager2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetForPackage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(packagefamilyname: Param0) -> ::windows::runtime::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetForPackageForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, User>>(packagefamilyname: Param0, user: Param1) -> ::windows::runtime::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), user.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    pub fn IAppUriHandlerRegistrationManagerStatics<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppUriHandlerRegistrationManagerStatics2<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppUriHandlerRegistrationManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistrationManager;{e62c9a52-ac94-5750-ac1b-6cfb6f250263})");
}
unsafe impl ::windows::runtime::Interface for AppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3861682770, 44180, 22352, [172, 27, 108, 251, 111, 37, 2, 99]);
}
impl ::windows::runtime::RuntimeName for AppUriHandlerRegistrationManager {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistrationManager";
}
impl ::core::convert::From<AppUriHandlerRegistrationManager> for ::windows::runtime::IUnknown {
    fn from(value: AppUriHandlerRegistrationManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppUriHandlerRegistrationManager> for ::windows::runtime::IUnknown {
    fn from(value: &AppUriHandlerRegistrationManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppUriHandlerRegistrationManager> for ::windows::runtime::IInspectable {
    fn from(value: AppUriHandlerRegistrationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppUriHandlerRegistrationManager> for ::windows::runtime::IInspectable {
    fn from(value: &AppUriHandlerRegistrationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerRegistrationManager {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistrationManager {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for AutoUpdateTimeZoneStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutoUpdateTimeZoneStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.AutoUpdateTimeZoneStatus;i4)");
}
impl ::windows::runtime::DefaultType for AutoUpdateTimeZoneStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
pub struct DateTimeSettings {}
impl DateTimeSettings {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn SetSystemDateTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::DateTime>>(utcdatetime: Param0) -> ::windows::runtime::Result<()> {
        Self::IDateTimeSettingsStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), utcdatetime.into_param().abi()).ok() })
    }
    pub fn IDateTimeSettingsStatics<R, F: FnOnce(&IDateTimeSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DateTimeSettings, IDateTimeSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for DateTimeSettings {
    const NAME: &'static str = "Windows.System.DateTimeSettings";
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for DiagnosticAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DiagnosticAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.DiagnosticAccessStatus;i4)");
}
impl ::windows::runtime::DefaultType for DiagnosticAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueue(pub ::windows::runtime::IInspectable);
impl DispatcherQueue {
    #[doc = "*Required features: `System`*"]
    pub fn CreateTimer(&self) -> ::windows::runtime::Result<DispatcherQueueTimer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueueTimer>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn TryEnqueue<'a, Param0: ::windows::runtime::IntoParam<'a, DispatcherQueueHandler>>(&self, callback: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), callback.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn TryEnqueueWithPriority<'a, Param1: ::windows::runtime::IntoParam<'a, DispatcherQueueHandler>>(&self, priority: DispatcherQueuePriority, callback: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), priority, callback.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn ShutdownStarting<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveShutdownStarting<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn ShutdownCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveShutdownCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetForCurrentThread() -> ::windows::runtime::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueue>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn HasThreadAccess(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDispatcherQueue2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IDispatcherQueueStatics<R, F: FnOnce(&IDispatcherQueueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueue;{603e88e4-a338-4ffe-a457-a5cfb9ceb899})");
}
unsafe impl ::windows::runtime::Interface for DispatcherQueue {
    type Vtable = IDispatcherQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1614711012, 41784, 20478, [164, 87, 165, 207, 185, 206, 184, 153]);
}
impl ::windows::runtime::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Windows.System.DispatcherQueue";
}
impl ::core::convert::From<DispatcherQueue> for ::windows::runtime::IUnknown {
    fn from(value: DispatcherQueue) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DispatcherQueue> for ::windows::runtime::IUnknown {
    fn from(value: &DispatcherQueue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DispatcherQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DispatcherQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DispatcherQueue> for ::windows::runtime::IInspectable {
    fn from(value: DispatcherQueue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DispatcherQueue> for ::windows::runtime::IInspectable {
    fn from(value: &DispatcherQueue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DispatcherQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DispatcherQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DispatcherQueue {}
unsafe impl ::core::marker::Sync for DispatcherQueue {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueueController(pub ::windows::runtime::IInspectable);
impl DispatcherQueueController {
    #[doc = "*Required features: `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn ShutdownQueueAsync(&self) -> ::windows::runtime::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn CreateOnDedicatedThread() -> ::windows::runtime::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueueController>(result__)
        })
    }
    pub fn IDispatcherQueueControllerStatics<R, F: FnOnce(&IDispatcherQueueControllerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DispatcherQueueController, IDispatcherQueueControllerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueueController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueController;{22f34e66-50db-4e36-a98d-61c01b384d20})");
}
unsafe impl ::windows::runtime::Interface for DispatcherQueueController {
    type Vtable = IDispatcherQueueController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(586370662, 20699, 20022, [169, 141, 97, 192, 27, 56, 77, 32]);
}
impl ::windows::runtime::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Windows.System.DispatcherQueueController";
}
impl ::core::convert::From<DispatcherQueueController> for ::windows::runtime::IUnknown {
    fn from(value: DispatcherQueueController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DispatcherQueueController> for ::windows::runtime::IUnknown {
    fn from(value: &DispatcherQueueController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DispatcherQueueController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DispatcherQueueController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DispatcherQueueController> for ::windows::runtime::IInspectable {
    fn from(value: DispatcherQueueController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DispatcherQueueController> for ::windows::runtime::IInspectable {
    fn from(value: &DispatcherQueueController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DispatcherQueueController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DispatcherQueueController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueController {}
unsafe impl ::core::marker::Sync for DispatcherQueueController {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueueHandler(::windows::runtime::IUnknown);
impl DispatcherQueueHandler {
    pub fn new<F: FnMut() -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DispatcherQueueHandler_box::<F> {
            vtable: &DispatcherQueueHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Invoke(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({dfa2dc9c-1a2d-4917-98f2-939af1d6e0c8})");
}
unsafe impl ::windows::runtime::Interface for DispatcherQueueHandler {
    type Vtable = DispatcherQueueHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3751992476, 6701, 18711, [152, 242, 147, 154, 241, 214, 224, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatcherQueueHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DispatcherQueueHandler_box<F: FnMut() -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DispatcherQueueHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut() -> ::windows::runtime::Result<()> + 'static> DispatcherQueueHandler_box<F> {
    const VTABLE: DispatcherQueueHandler_abi = DispatcherQueueHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DispatcherQueueHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for DispatcherQueuePriority {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.DispatcherQueuePriority;i4)");
}
impl ::windows::runtime::DefaultType for DispatcherQueuePriority {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueueShutdownStartingEventArgs(pub ::windows::runtime::IInspectable);
impl DispatcherQueueShutdownStartingEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueShutdownStartingEventArgs;{c4724c4c-ff97-40c0-a226-cc0aaa545e89})");
}
unsafe impl ::windows::runtime::Interface for DispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3295824972, 65431, 16576, [162, 38, 204, 10, 170, 84, 94, 137]);
}
impl ::windows::runtime::RuntimeName for DispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Windows.System.DispatcherQueueShutdownStartingEventArgs";
}
impl ::core::convert::From<DispatcherQueueShutdownStartingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DispatcherQueueShutdownStartingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DispatcherQueueShutdownStartingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DispatcherQueueShutdownStartingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueShutdownStartingEventArgs {}
unsafe impl ::core::marker::Sync for DispatcherQueueShutdownStartingEventArgs {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatcherQueueTimer(pub ::windows::runtime::IInspectable);
impl DispatcherQueueTimer {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Interval(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn SetInterval<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn IsRunning(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn IsRepeating(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetIsRepeating(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Tick<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveTick<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueTimer;{5feabb1d-a31c-4727-b1ac-37454649d56a})");
}
unsafe impl ::windows::runtime::Interface for DispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1609218845, 41756, 18215, [177, 172, 55, 69, 70, 73, 213, 106]);
}
impl ::windows::runtime::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Windows.System.DispatcherQueueTimer";
}
impl ::core::convert::From<DispatcherQueueTimer> for ::windows::runtime::IUnknown {
    fn from(value: DispatcherQueueTimer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DispatcherQueueTimer> for ::windows::runtime::IUnknown {
    fn from(value: &DispatcherQueueTimer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DispatcherQueueTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DispatcherQueueTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DispatcherQueueTimer> for ::windows::runtime::IInspectable {
    fn from(value: DispatcherQueueTimer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DispatcherQueueTimer> for ::windows::runtime::IInspectable {
    fn from(value: &DispatcherQueueTimer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DispatcherQueueTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DispatcherQueueTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueTimer {}
unsafe impl ::core::marker::Sync for DispatcherQueueTimer {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FolderLauncherOptions(pub ::windows::runtime::IInspectable);
impl FolderLauncherOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FolderLauncherOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `System`, `Foundation_Collections`, `Storage`*"]
    pub fn ItemsToSelect(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<super::Storage::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `System`, `UI_ViewManagement`*"]
    pub fn DesiredRemainingView(&self) -> ::windows::runtime::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows::runtime::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__: super::UI::ViewManagement::ViewSizePreference = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `System`, `UI_ViewManagement`*"]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FolderLauncherOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.FolderLauncherOptions;{bb91c27d-6b87-432a-bd04-776c6f5fb2ab})");
}
unsafe impl ::windows::runtime::Interface for FolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3146891901, 27527, 17194, [189, 4, 119, 108, 111, 95, 178, 171]);
}
impl ::windows::runtime::RuntimeName for FolderLauncherOptions {
    const NAME: &'static str = "Windows.System.FolderLauncherOptions";
}
impl ::core::convert::From<FolderLauncherOptions> for ::windows::runtime::IUnknown {
    fn from(value: FolderLauncherOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FolderLauncherOptions> for ::windows::runtime::IUnknown {
    fn from(value: &FolderLauncherOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FolderLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FolderLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FolderLauncherOptions> for ::windows::runtime::IInspectable {
    fn from(value: FolderLauncherOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FolderLauncherOptions> for ::windows::runtime::IInspectable {
    fn from(value: &FolderLauncherOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FolderLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FolderLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<FolderLauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderLauncherOptions) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderLauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderLauncherOptions) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILauncherViewOptions> for FolderLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILauncherViewOptions> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILauncherViewOptions> for &FolderLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILauncherViewOptions> {
        ::core::convert::TryInto::<ILauncherViewOptions>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for FolderLauncherOptions {}
unsafe impl ::core::marker::Sync for FolderLauncherOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppActivationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppActivationResult {
    type Vtable = IAppActivationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1800571136, 62574, 20144, [170, 108, 56, 175, 85, 124, 249, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3813189274, 34953, 19619, [190, 7, 213, 255, 255, 95, 8, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppDiagnosticInfo2 {
    type Vtable = IAppDiagnosticInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3745971159, 6426, 17516, [148, 115, 143, 188, 35, 116, 163, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppDiagnosticInfo3 {
    type Vtable = IAppDiagnosticInfo3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3365258813, 56673, 19557, [186, 189, 129, 161, 11, 79, 152, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppDiagnosticInfoStatics {
    type Vtable = IAppDiagnosticInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3462997439, 4298, 16584, [169, 202, 197, 201, 101, 1, 134, 110]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppDiagnosticInfoStatics2 {
    type Vtable = IAppDiagnosticInfoStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(95570822, 4096, 19600, [187, 159, 114, 53, 7, 28, 80, 254]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appusermodelid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1968656496, 467, 18586, [147, 37, 82, 249, 204, 110, 222, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcherEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1880606486, 57818, 19557, [153, 223, 4, 109, 255, 91, 231, 26]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcherEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppExecutionStateChangeResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1862507504, 63771, 19960, [174, 119, 48, 51, 204, 182, 145, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExecutionStateChangeResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppMemoryReport(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppMemoryReport {
    type Vtable = IAppMemoryReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1835348891, 19823, 17852, [156, 94, 228, 155, 63, 242, 117, 141]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppMemoryReport2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppMemoryReport2 {
    type Vtable = IAppMemoryReport2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602172728, 20919, 17116, [183, 237, 121, 186, 70, 210, 136, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppMemoryUsageLimitChangingEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2046322276, 65226, 19877, [158, 64, 43, 198, 62, 253, 201, 121]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryUsageLimitChangingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupBackgroundTaskReport(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(627500878, 45149, 16578, [157, 193, 26, 79, 3, 158, 161, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupBackgroundTaskReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3105093498, 59399, 18932, [132, 94, 123, 139, 220, 254, 142, 231]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_Diagnostics")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppResourceGroupInfo2 {
    type Vtable = IAppResourceGroupInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4003144557, 54021, 19819, [146, 247, 106, 253, 173, 114, 222, 220]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo2_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3652231421, 28250, 19570, [139, 23, 9, 254, 196, 162, 18, 189]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2054714935, 25346, 19759, [191, 137, 28, 18, 208, 178, 166, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(467398103, 65254, 20436, [152, 221, 233, 42, 44, 194, 153, 243]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupMemoryReport(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747374257, 32177, 19537, [162, 37, 127, 174, 45, 73, 228, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupMemoryReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppMemoryUsageLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppResourceGroupStateReport(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1384423192, 12144, 16950, [171, 64, 208, 77, 176, 199, 185, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupStateReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppResourceGroupExecutionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerHost(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1565575877, 37586, 21513, [181, 111, 127, 115, 225, 14, 164, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerHost2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUriHandlerHost2 {
    type Vtable = IAppUriHandlerHost2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(973860501, 10724, 20927, [128, 149, 163, 192, 104, 227, 199, 42]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerHostFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUriHandlerHostFactory {
    type Vtable = IAppUriHandlerHostFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(628898966, 52740, 24472, [150, 187, 62, 189, 62, 146, 117, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHostFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1869852337, 17769, 23615, [155, 160, 153, 18, 62, 234, 50, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hosts: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUriHandlerRegistration2 {
    type Vtable = IAppUriHandlerRegistration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3578637463, 52025, 24351, [136, 62, 1, 133, 55, 48, 189, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hosts: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3861682770, 44180, 22352, [172, 27, 108, 251, 111, 37, 2, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUriHandlerRegistrationManager2 {
    type Vtable = IAppUriHandlerRegistrationManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3185560305, 46362, 24169, [174, 253, 112, 136, 217, 242, 177, 35]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUriHandlerRegistrationManagerStatics {
    type Vtable = IAppUriHandlerRegistrationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3587104159, 22313, 23414, [161, 212, 2, 133, 242, 149, 193, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppUriHandlerRegistrationManagerStatics2 {
    type Vtable = IAppUriHandlerRegistrationManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(351765369, 26768, 20608, [144, 167, 152, 130, 74, 127, 7, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeSettingsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDateTimeSettingsStatics {
    type Vtable = IDateTimeSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1562464465, 18414, 18603, [165, 43, 159, 25, 84, 39, 141, 130]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, utcdatetime: super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueue(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueue {
    type Vtable = IDispatcherQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1614711012, 41784, 20478, [164, 87, 165, 207, 185, 206, 184, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callback: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, priority: DispatcherQueuePriority, callback: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueue2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueue2 {
    type Vtable = IDispatcherQueue2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3357722183, 12527, 20590, [189, 30, 166, 71, 174, 102, 117, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueController(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueController {
    type Vtable = IDispatcherQueueController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(586370662, 20699, 20022, [169, 141, 97, 192, 27, 56, 77, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueControllerStatics {
    type Vtable = IDispatcherQueueControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(174889184, 20888, 18850, [163, 19, 63, 112, 209, 241, 60, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3295824972, 65431, 16576, [162, 38, 204, 10, 170, 84, 94, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueStatics {
    type Vtable = IDispatcherQueueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2842526679, 37745, 17687, [146, 69, 208, 130, 74, 193, 44, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1609218845, 41756, 18215, [177, 172, 55, 69, 70, 73, 213, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFolderLauncherOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3146891901, 27527, 17194, [189, 4, 119, 108, 111, 95, 178, 171]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderLauncherOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownUserPropertiesStatics {
    type Vtable = IKnownUserPropertiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2002096410, 28869, 18661, [182, 55, 91, 163, 68, 30, 78, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownUserPropertiesStatics2 {
    type Vtable = IKnownUserPropertiesStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1531250562, 63008, 22398, [177, 179, 221, 86, 100, 77, 121, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILaunchUriResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILaunchUriResult {
    type Vtable = ILaunchUriResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3962022111, 63189, 17866, [145, 58, 112, 164, 12, 92, 130, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchUriResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LaunchUriStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherOptions {
    type Vtable = ILauncherOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3136954840, 45169, 19672, [133, 62, 52, 18, 3, 229, 87, 211]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherOptions2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherOptions2 {
    type Vtable = ILauncherOptions2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1000378036, 28224, 19918, [161, 163, 47, 83, 149, 10, 251, 73]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Search"))] usize,
    #[cfg(feature = "Storage_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Search"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherOptions3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherOptions3 {
    type Vtable = ILauncherOptions3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4034332245, 19299, 20026, [145, 7, 78, 104, 120, 65, 146, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherOptions4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherOptions4 {
    type Vtable = ILauncherOptions4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4017082638, 59131, 18452, [164, 78, 87, 232, 185, 217, 160, 27]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherStatics {
    type Vtable = ILauncherStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(661737923, 40510, 17142, [145, 164, 93, 253, 235, 35, 36, 81]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherStatics2 {
    type Vtable = ILauncherStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1505374139, 9419, 19458, [164, 196, 130, 148, 86, 157, 84, 241]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, inputdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, inputdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheme: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheme: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, extension: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherStatics3 {
    type Vtable = ILauncherStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(591552936, 40371, 18051, [170, 66, 220, 111, 81, 211, 56, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, folder: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, folder: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherStatics4 {
    type Vtable = ILauncherStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3119284639, 46501, 16838, [179, 179, 221, 27, 49, 120, 188, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, inputdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, inputdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherStatics5(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherStatics5 {
    type Vtable = ILauncherStatics5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1529147268, 55445, 24554, [145, 83, 26, 196, 154, 237, 155, 169]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILauncherUIOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherUIOptions {
    type Vtable = ILauncherUIOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(455465582, 35494, 16873, [130, 81, 65, 101, 245, 152, 95, 73]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherUIOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::UI::Popups::Placement) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::UI::Popups::Placement) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `System`*"]
pub struct ILauncherViewOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILauncherViewOptions {
    type Vtable = ILauncherViewOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2325424625, 31911, 18910, [155, 211, 60, 91, 113, 132, 246, 22]);
}
impl ILauncherViewOptions {
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `System`, `UI_ViewManagement`*"]
    pub fn DesiredRemainingView(&self) -> ::windows::runtime::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__: super::UI::ViewManagement::ViewSizePreference = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `System`, `UI_ViewManagement`*"]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILauncherViewOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{8a9b29f1-7ca7-49de-9bd3-3c5b7184f616}");
}
impl ::core::convert::From<ILauncherViewOptions> for ::windows::runtime::IUnknown {
    fn from(value: ILauncherViewOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILauncherViewOptions> for ::windows::runtime::IUnknown {
    fn from(value: &ILauncherViewOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILauncherViewOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILauncherViewOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILauncherViewOptions> for ::windows::runtime::IInspectable {
    fn from(value: ILauncherViewOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILauncherViewOptions> for ::windows::runtime::IInspectable {
    fn from(value: &ILauncherViewOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILauncherViewOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILauncherViewOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherViewOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_ViewManagement")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))] usize,
    #[cfg(feature = "UI_ViewManagement")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMemoryManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMemoryManagerStatics {
    type Vtable = IMemoryManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1550591900, 55242, 18297, [145, 136, 64, 87, 33, 156, 230, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppMemoryUsageLevel) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMemoryManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMemoryManagerStatics2 {
    type Vtable = IMemoryManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1861104927, 28002, 16959, [148, 121, 176, 31, 156, 159, 118, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics2_abi(
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
#[doc(hidden)]
pub struct IMemoryManagerStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMemoryManagerStatics3 {
    type Vtable = IMemoryManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(345725390, 37549, 20021, [137, 235, 80, 223, 180, 192, 217, 28]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMemoryManagerStatics4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMemoryManagerStatics4 {
    type Vtable = IMemoryManagerStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3316205608, 59470, 18566, [138, 13, 68, 179, 25, 14, 59, 114]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessLauncherOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(813742543, 62532, 19075, [190, 175, 165, 73, 160, 243, 34, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessLauncherResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessLauncherResult {
    type Vtable = IProcessLauncherResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1414302004, 34520, 18833, [142, 117, 236, 232, 164, 59, 107, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessLauncherStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessLauncherStatics {
    type Vtable = IProcessLauncherStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(866871015, 11534, 17547, [166, 160, 193, 60, 56, 54, 208, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessMemoryReport(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessMemoryReport {
    type Vtable = IProcessMemoryReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(141755816, 39792, 18306, [135, 65, 58, 152, 43, 108, 229, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtocolForResultsOperation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3582011706, 28137, 19752, [147, 120, 248, 103, 130, 225, 130, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteLauncherOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2654611336, 10385, 19679, [162, 214, 157, 255, 125, 2, 230, 147]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteLauncherStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteLauncherStatics {
    type Vtable = IRemoteLauncherStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3621485203, 41740, 18615, [159, 33, 5, 16, 38, 164, 229, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotesystemconnectionrequest: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotesystemconnectionrequest: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotesystemconnectionrequest: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, inputdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShutdownManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShutdownManagerStatics {
    type Vtable = IShutdownManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1927432173, 56667, 19820, [177, 208, 197, 122, 123, 187, 95, 148]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShutdownManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShutdownManagerStatics2 {
    type Vtable = IShutdownManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(258580527, 39988, 17351, [168, 195, 112, 179, 10, 127, 117, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, powerstate: PowerState, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, powerstate: PowerState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITimeZoneSettingsStatics {
    type Vtable = ITimeZoneSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2604346346, 41217, 16814, [159, 189, 2, 135, 40, 186, 183, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timezonedisplayname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITimeZoneSettingsStatics2 {
    type Vtable = ITimeZoneSettingsStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1432096184, 14760, 18938, [180, 246, 162, 199, 252, 40, 66, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeout: super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUser(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUser {
    type Vtable = IUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3751421638, 59206, 19405, [181, 212, 18, 1, 3, 196, 32, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserAuthenticationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, values: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredsize: UserPictureSize, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUser2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUser2 {
    type Vtable = IUser2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2562348584, 42723, 20878, [137, 217, 211, 178, 177, 153, 26, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, consentgroup: UserAgeConsentGroup, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangeDeferral(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2293601640, 47920, 17147, [162, 112, 233, 144, 46, 64, 239, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangeDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangingEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2349010728, 42769, 19486, [171, 72, 4, 23, 156, 21, 147, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserAuthenticationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserAuthenticationStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(140794332, 6342, 18651, [188, 153, 114, 79, 185, 32, 60, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserChangedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserChangedEventArgs2 {
    type Vtable = IUserChangedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1798097732, 28417, 22028, [151, 173, 252, 127, 50, 236, 88, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDeviceAssociationChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3172953964, 47965, 19835, [165, 240, 200, 205, 17, 163, 141, 66]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDeviceAssociationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDeviceAssociationStatics {
    type Vtable = IUserDeviceAssociationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2118721044, 63578, 19463, [141, 169, 127, 227, 208, 84, 35, 67]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserPicker(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserPicker {
    type Vtable = IUserPicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2102689800, 61923, 19052, [141, 220, 169, 187, 15, 72, 138, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPicker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserPickerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserPickerStatics {
    type Vtable = IUserPickerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3727855836, 32371, 19958, [161, 174, 77, 126, 202, 130, 180, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPickerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserStatics {
    type Vtable = IUserStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(358527547, 9258, 17888, [162, 233, 49, 113, 252, 106, 127, 221]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: UserType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: UserType, status: UserAuthenticationStatus, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nonroamableid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserStatics2 {
    type Vtable = IUserStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1956871697, 11957, 17543, [176, 213, 44, 103, 144, 224, 19, 233]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserWatcher {
    type Vtable = IUserWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(358527547, 9258, 17888, [162, 233, 49, 113, 252, 106, 127, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `System`*"]
pub struct KnownUserProperties {}
impl KnownUserProperties {
    #[doc = "*Required features: `System`*"]
    pub fn DisplayName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn FirstName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn LastName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn ProviderName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn AccountName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn GuestHost() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn PrincipalName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn DomainName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn SessionInitiationProtocolUri() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn AgeEnforcementRegion() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownUserPropertiesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownUserPropertiesStatics<R, F: FnOnce(&IKnownUserPropertiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKnownUserPropertiesStatics2<R, F: FnOnce(&IKnownUserPropertiesStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownUserProperties {
    const NAME: &'static str = "Windows.System.KnownUserProperties";
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for LaunchFileStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LaunchFileStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.LaunchFileStatus;i4)");
}
impl ::windows::runtime::DefaultType for LaunchFileStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for LaunchQuerySupportStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LaunchQuerySupportStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportStatus;i4)");
}
impl ::windows::runtime::DefaultType for LaunchQuerySupportStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for LaunchQuerySupportType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LaunchQuerySupportType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportType;i4)");
}
impl ::windows::runtime::DefaultType for LaunchQuerySupportType {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LaunchUriResult(pub ::windows::runtime::IInspectable);
impl LaunchUriResult {
    #[doc = "*Required features: `System`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<LaunchUriStatus> {
        let this = self;
        unsafe {
            let mut result__: LaunchUriStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LaunchUriStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn Result(&self) -> ::windows::runtime::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LaunchUriResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.LaunchUriResult;{ec27a8df-f6d5-45ca-913a-70a40c5c8221})");
}
unsafe impl ::windows::runtime::Interface for LaunchUriResult {
    type Vtable = ILaunchUriResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3962022111, 63189, 17866, [145, 58, 112, 164, 12, 92, 130, 33]);
}
impl ::windows::runtime::RuntimeName for LaunchUriResult {
    const NAME: &'static str = "Windows.System.LaunchUriResult";
}
impl ::core::convert::From<LaunchUriResult> for ::windows::runtime::IUnknown {
    fn from(value: LaunchUriResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LaunchUriResult> for ::windows::runtime::IUnknown {
    fn from(value: &LaunchUriResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LaunchUriResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LaunchUriResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LaunchUriResult> for ::windows::runtime::IInspectable {
    fn from(value: LaunchUriResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LaunchUriResult> for ::windows::runtime::IInspectable {
    fn from(value: &LaunchUriResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LaunchUriResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LaunchUriResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LaunchUriResult {}
unsafe impl ::core::marker::Sync for LaunchUriResult {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for LaunchUriStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LaunchUriStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.LaunchUriStatus;i4)");
}
impl ::windows::runtime::DefaultType for LaunchUriStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
pub struct Launcher {}
impl Launcher {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System`, `Foundation`, `Storage`*"]
    pub fn LaunchFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::IStorageFile>>(file: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System`, `Foundation`, `Storage`*"]
    pub fn LaunchFileWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, LauncherOptions>>(file: Param0, options: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchUriAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchUriWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, LauncherOptions>>(uri: Param0, options: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchUriForResultsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, LauncherOptions>>(uri: Param0, options: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn LaunchUriForResultsWithDataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, LauncherOptions>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::ValueSet>>(uri: Param0, options: Param1, inputdata: Param2) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn LaunchUriWithDataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, LauncherOptions>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::ValueSet>>(uri: Param0, options: Param1, inputdata: Param2) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn QueryUriSupportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(uri: Param0, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), launchquerysupporttype, &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn QueryUriSupportWithPackageFamilyNameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(uri: Param0, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: Param2) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), uri.into_param().abi(), launchquerysupporttype, packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System`, `Foundation`, `Storage`*"]
    pub fn QueryFileSupportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::StorageFile>>(file: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System`, `Foundation`, `Storage`*"]
    pub fn QueryFileSupportWithPackageFamilyNameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::StorageFile>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(file: Param0, packagefamilyname: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), file.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `ApplicationModel`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindUriSchemeHandlersAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(scheme: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), scheme.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `ApplicationModel`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindUriSchemeHandlersWithLaunchUriTypeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(scheme: Param0, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), scheme.into_param().abi(), launchquerysupporttype, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `ApplicationModel`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindFileHandlersAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(extension: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), extension.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System`, `Foundation`, `Storage`*"]
    pub fn LaunchFolderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::IStorageFolder>>(folder: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), folder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `System`, `Foundation`, `Storage`*"]
    pub fn LaunchFolderWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, FolderLauncherOptions>>(folder: Param0, options: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), folder.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn QueryAppUriSupportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn QueryAppUriSupportWithPackageFamilyNameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(uri: Param0, packagefamilyname: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `ApplicationModel`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAppUriHandlersAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchUriForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, User>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(user: Param0, uri: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchUriWithOptionsForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, User>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, LauncherOptions>>(user: Param0, uri: Param1, options: Param2) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn LaunchUriWithDataForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, User>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, LauncherOptions>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::ValueSet>>(user: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchUriForResultsForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, User>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, LauncherOptions>>(user: Param0, uri: Param1, options: Param2) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn LaunchUriForResultsWithDataForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, User>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, LauncherOptions>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::ValueSet>>(user: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchFolderPathAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(path: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchFolderPathWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, FolderLauncherOptions>>(path: Param0, options: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), path.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchFolderPathForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, path: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn LaunchFolderPathWithOptionsForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, FolderLauncherOptions>>(user: Param0, path: Param1, options: Param2) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ILauncherStatics<R, F: FnOnce(&ILauncherStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Launcher, ILauncherStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics2<R, F: FnOnce(&ILauncherStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Launcher, ILauncherStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics3<R, F: FnOnce(&ILauncherStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Launcher, ILauncherStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics4<R, F: FnOnce(&ILauncherStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Launcher, ILauncherStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics5<R, F: FnOnce(&ILauncherStatics5) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Launcher, ILauncherStatics5> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for Launcher {
    const NAME: &'static str = "Windows.System.Launcher";
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LauncherOptions(pub ::windows::runtime::IInspectable);
impl LauncherOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LauncherOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `System`*"]
    pub fn TreatAsUntrusted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetTreatAsUntrusted(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn DisplayApplicationPicker(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetDisplayApplicationPicker(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn UI(&self) -> ::windows::runtime::Result<LauncherUIOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LauncherUIOptions>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn PreferredApplicationPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetPreferredApplicationPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn PreferredApplicationDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetPreferredApplicationDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn FallbackUri(&self) -> ::windows::runtime::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn SetFallbackUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetContentType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn TargetApplicationPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetTargetApplicationPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `System`, `Storage_Search`*"]
    pub fn NeighboringFilesQuery(&self) -> ::windows::runtime::Result<super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::runtime::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `System`, `Storage_Search`*"]
    pub fn SetNeighboringFilesQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::Search::StorageFileQueryResult>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `System`, `UI_ViewManagement`*"]
    pub fn DesiredRemainingView(&self) -> ::windows::runtime::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows::runtime::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__: super::UI::ViewManagement::ViewSizePreference = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `System`, `UI_ViewManagement`*"]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn IgnoreAppUriHandlers(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetIgnoreAppUriHandlers(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn LimitPickerToCurrentAppAndAppUriHandlers(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetLimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LauncherOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.LauncherOptions;{bafa21d8-b071-4cd8-853e-341203e557d3})");
}
unsafe impl ::windows::runtime::Interface for LauncherOptions {
    type Vtable = ILauncherOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3136954840, 45169, 19672, [133, 62, 52, 18, 3, 229, 87, 211]);
}
impl ::windows::runtime::RuntimeName for LauncherOptions {
    const NAME: &'static str = "Windows.System.LauncherOptions";
}
impl ::core::convert::From<LauncherOptions> for ::windows::runtime::IUnknown {
    fn from(value: LauncherOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LauncherOptions> for ::windows::runtime::IUnknown {
    fn from(value: &LauncherOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LauncherOptions> for ::windows::runtime::IInspectable {
    fn from(value: LauncherOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LauncherOptions> for ::windows::runtime::IInspectable {
    fn from(value: &LauncherOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<LauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LauncherOptions) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LauncherOptions> for ILauncherViewOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LauncherOptions) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILauncherViewOptions> for LauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILauncherViewOptions> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILauncherViewOptions> for &LauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILauncherViewOptions> {
        ::core::convert::TryInto::<ILauncherViewOptions>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for LauncherOptions {}
unsafe impl ::core::marker::Sync for LauncherOptions {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LauncherUIOptions(pub ::windows::runtime::IInspectable);
impl LauncherUIOptions {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn InvocationPoint(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn SetInvocationPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::Point>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn SelectionRect(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::Rect>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn SetSelectionRect<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::Rect>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `System`, `UI_Popups`*"]
    pub fn PreferredPlacement(&self) -> ::windows::runtime::Result<super::UI::Popups::Placement> {
        let this = self;
        unsafe {
            let mut result__: super::UI::Popups::Placement = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::Popups::Placement>(result__)
        }
    }
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `System`, `UI_Popups`*"]
    pub fn SetPreferredPlacement(&self, value: super::UI::Popups::Placement) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LauncherUIOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.LauncherUIOptions;{1b25da6e-8aa6-41e9-8251-4165f5985f49})");
}
unsafe impl ::windows::runtime::Interface for LauncherUIOptions {
    type Vtable = ILauncherUIOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(455465582, 35494, 16873, [130, 81, 65, 101, 245, 152, 95, 73]);
}
impl ::windows::runtime::RuntimeName for LauncherUIOptions {
    const NAME: &'static str = "Windows.System.LauncherUIOptions";
}
impl ::core::convert::From<LauncherUIOptions> for ::windows::runtime::IUnknown {
    fn from(value: LauncherUIOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LauncherUIOptions> for ::windows::runtime::IUnknown {
    fn from(value: &LauncherUIOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LauncherUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LauncherUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LauncherUIOptions> for ::windows::runtime::IInspectable {
    fn from(value: LauncherUIOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LauncherUIOptions> for ::windows::runtime::IInspectable {
    fn from(value: &LauncherUIOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LauncherUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LauncherUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LauncherUIOptions {}
unsafe impl ::core::marker::Sync for LauncherUIOptions {}
#[doc = "*Required features: `System`*"]
pub struct MemoryManager {}
impl MemoryManager {
    #[doc = "*Required features: `System`*"]
    pub fn AppMemoryUsage() -> ::windows::runtime::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn AppMemoryUsageLimit() -> ::windows::runtime::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn AppMemoryUsageLevel() -> ::windows::runtime::Result<AppMemoryUsageLevel> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: AppMemoryUsageLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppMemoryUsageLevel>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn AppMemoryUsageIncreased<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveAppMemoryUsageIncreased<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn AppMemoryUsageDecreased<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveAppMemoryUsageDecreased<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn AppMemoryUsageLimitChanging<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveAppMemoryUsageLimitChanging<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetAppMemoryReport() -> ::windows::runtime::Result<AppMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppMemoryReport>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetProcessMemoryReport() -> ::windows::runtime::Result<ProcessMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessMemoryReport>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn TrySetAppMemoryUsageLimit(value: u64) -> ::windows::runtime::Result<bool> {
        Self::IMemoryManagerStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn ExpectedAppMemoryUsageLimit() -> ::windows::runtime::Result<u64> {
        Self::IMemoryManagerStatics4(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    pub fn IMemoryManagerStatics<R, F: FnOnce(&IMemoryManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MemoryManager, IMemoryManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMemoryManagerStatics2<R, F: FnOnce(&IMemoryManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MemoryManager, IMemoryManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMemoryManagerStatics3<R, F: FnOnce(&IMemoryManagerStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MemoryManager, IMemoryManagerStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMemoryManagerStatics4<R, F: FnOnce(&IMemoryManagerStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MemoryManager, IMemoryManagerStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MemoryManager {
    const NAME: &'static str = "Windows.System.MemoryManager";
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for PowerState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PowerState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.PowerState;i4)");
}
impl ::windows::runtime::DefaultType for PowerState {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
pub struct ProcessLauncher {}
impl ProcessLauncher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RunToCompletionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(filename: Param0, args: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), filename.into_param().abi(), args.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<ProcessLauncherResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RunToCompletionAsyncWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProcessLauncherOptions>>(filename: Param0, args: Param1, options: Param2) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), filename.into_param().abi(), args.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<ProcessLauncherResult>>(result__)
        })
    }
    pub fn IProcessLauncherStatics<R, F: FnOnce(&IProcessLauncherStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProcessLauncher, IProcessLauncherStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ProcessLauncher {
    const NAME: &'static str = "Windows.System.ProcessLauncher";
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessLauncherOptions(pub ::windows::runtime::IInspectable);
impl ProcessLauncherOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProcessLauncherOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System`, `Storage_Streams`*"]
    pub fn StandardInput(&self) -> ::windows::runtime::Result<super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System`, `Storage_Streams`*"]
    pub fn SetStandardInput<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::Streams::IInputStream>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System`, `Storage_Streams`*"]
    pub fn StandardOutput(&self) -> ::windows::runtime::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System`, `Storage_Streams`*"]
    pub fn SetStandardOutput<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::Streams::IOutputStream>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System`, `Storage_Streams`*"]
    pub fn StandardError(&self) -> ::windows::runtime::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System`, `Storage_Streams`*"]
    pub fn SetStandardError<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::Streams::IOutputStream>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn WorkingDirectory(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetWorkingDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessLauncherOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherOptions;{3080b9cf-f444-4a83-beaf-a549a0f3229c})");
}
unsafe impl ::windows::runtime::Interface for ProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(813742543, 62532, 19075, [190, 175, 165, 73, 160, 243, 34, 156]);
}
impl ::windows::runtime::RuntimeName for ProcessLauncherOptions {
    const NAME: &'static str = "Windows.System.ProcessLauncherOptions";
}
impl ::core::convert::From<ProcessLauncherOptions> for ::windows::runtime::IUnknown {
    fn from(value: ProcessLauncherOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessLauncherOptions> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessLauncherOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProcessLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessLauncherOptions> for ::windows::runtime::IInspectable {
    fn from(value: ProcessLauncherOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessLauncherOptions> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessLauncherOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessLauncherOptions {}
unsafe impl ::core::marker::Sync for ProcessLauncherOptions {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessLauncherResult(pub ::windows::runtime::IInspectable);
impl ProcessLauncherResult {
    #[doc = "*Required features: `System`*"]
    pub fn ExitCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessLauncherResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherResult;{544c8934-86d8-4991-8e75-ece8a43b6b6d})");
}
unsafe impl ::windows::runtime::Interface for ProcessLauncherResult {
    type Vtable = IProcessLauncherResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1414302004, 34520, 18833, [142, 117, 236, 232, 164, 59, 107, 109]);
}
impl ::windows::runtime::RuntimeName for ProcessLauncherResult {
    const NAME: &'static str = "Windows.System.ProcessLauncherResult";
}
impl ::core::convert::From<ProcessLauncherResult> for ::windows::runtime::IUnknown {
    fn from(value: ProcessLauncherResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessLauncherResult> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessLauncherResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessLauncherResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProcessLauncherResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessLauncherResult> for ::windows::runtime::IInspectable {
    fn from(value: ProcessLauncherResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessLauncherResult> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessLauncherResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessLauncherResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessLauncherResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessLauncherResult {}
unsafe impl ::core::marker::Sync for ProcessLauncherResult {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessMemoryReport(pub ::windows::runtime::IInspectable);
impl ProcessMemoryReport {
    #[doc = "*Required features: `System`*"]
    pub fn PrivateWorkingSetUsage(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn TotalWorkingSetUsage(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessMemoryReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.ProcessMemoryReport;{087305a8-9b70-4782-8741-3a982b6ce5e4})");
}
unsafe impl ::windows::runtime::Interface for ProcessMemoryReport {
    type Vtable = IProcessMemoryReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(141755816, 39792, 18306, [135, 65, 58, 152, 43, 108, 229, 228]);
}
impl ::windows::runtime::RuntimeName for ProcessMemoryReport {
    const NAME: &'static str = "Windows.System.ProcessMemoryReport";
}
impl ::core::convert::From<ProcessMemoryReport> for ::windows::runtime::IUnknown {
    fn from(value: ProcessMemoryReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessMemoryReport> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessMemoryReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProcessMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessMemoryReport> for ::windows::runtime::IInspectable {
    fn from(value: ProcessMemoryReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessMemoryReport> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessMemoryReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessMemoryReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessMemoryReport {}
unsafe impl ::core::marker::Sync for ProcessMemoryReport {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for ProcessorArchitecture {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProcessorArchitecture {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.ProcessorArchitecture;i4)");
}
impl ::windows::runtime::DefaultType for ProcessorArchitecture {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtocolForResultsOperation(pub ::windows::runtime::IInspectable);
impl ProtocolForResultsOperation {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn ReportCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::ValueSet>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtocolForResultsOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.ProtocolForResultsOperation;{d581293a-6de9-4d28-9378-f86782e182bb})");
}
unsafe impl ::windows::runtime::Interface for ProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3582011706, 28137, 19752, [147, 120, 248, 103, 130, 225, 130, 187]);
}
impl ::windows::runtime::RuntimeName for ProtocolForResultsOperation {
    const NAME: &'static str = "Windows.System.ProtocolForResultsOperation";
}
impl ::core::convert::From<ProtocolForResultsOperation> for ::windows::runtime::IUnknown {
    fn from(value: ProtocolForResultsOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtocolForResultsOperation> for ::windows::runtime::IUnknown {
    fn from(value: &ProtocolForResultsOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtocolForResultsOperation> for ::windows::runtime::IInspectable {
    fn from(value: ProtocolForResultsOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtocolForResultsOperation> for ::windows::runtime::IInspectable {
    fn from(value: &ProtocolForResultsOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtocolForResultsOperation {}
unsafe impl ::core::marker::Sync for ProtocolForResultsOperation {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for RemoteLaunchUriStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteLaunchUriStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteLaunchUriStatus;i4)");
}
impl ::windows::runtime::DefaultType for RemoteLaunchUriStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
pub struct RemoteLauncher {}
impl RemoteLauncher {
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    #[doc = "*Required features: `System`, `Foundation`, `System_RemoteSystems`*"]
    pub fn LaunchUriAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(remotesystemconnectionrequest: Param0, uri: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    #[doc = "*Required features: `System`, `Foundation`, `System_RemoteSystems`*"]
    pub fn LaunchUriWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, RemoteLauncherOptions>>(remotesystemconnectionrequest: Param0, uri: Param1, options: Param2) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`, `System_RemoteSystems`*"]
    pub fn LaunchUriWithDataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, RemoteLauncherOptions>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::ValueSet>>(
        remotesystemconnectionrequest: Param0,
        uri: Param1,
        options: Param2,
        inputdata: Param3,
    ) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    pub fn IRemoteLauncherStatics<R, F: FnOnce(&IRemoteLauncherStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteLauncher, IRemoteLauncherStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for RemoteLauncher {
    const NAME: &'static str = "Windows.System.RemoteLauncher";
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteLauncherOptions(pub ::windows::runtime::IInspectable);
impl RemoteLauncherOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteLauncherOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn FallbackUri(&self) -> ::windows::runtime::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn SetFallbackUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn PreferredAppIds(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteLauncherOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteLauncherOptions;{9e3a2788-2891-4cdf-a2d6-9dff7d02e693})");
}
unsafe impl ::windows::runtime::Interface for RemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2654611336, 10385, 19679, [162, 214, 157, 255, 125, 2, 230, 147]);
}
impl ::windows::runtime::RuntimeName for RemoteLauncherOptions {
    const NAME: &'static str = "Windows.System.RemoteLauncherOptions";
}
impl ::core::convert::From<RemoteLauncherOptions> for ::windows::runtime::IUnknown {
    fn from(value: RemoteLauncherOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteLauncherOptions> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteLauncherOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteLauncherOptions> for ::windows::runtime::IInspectable {
    fn from(value: RemoteLauncherOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteLauncherOptions> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteLauncherOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteLauncherOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteLauncherOptions {}
unsafe impl ::core::marker::Sync for RemoteLauncherOptions {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for ShutdownKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ShutdownKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.ShutdownKind;i4)");
}
impl ::windows::runtime::DefaultType for ShutdownKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
pub struct ShutdownManager {}
impl ShutdownManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn BeginShutdown<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(shutdownkind: ShutdownKind, timeout: Param1) -> ::windows::runtime::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), shutdownkind, timeout.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `System`*"]
    pub fn CancelShutdown() -> ::windows::runtime::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `System`*"]
    pub fn IsPowerStateSupported(powerstate: PowerState) -> ::windows::runtime::Result<bool> {
        Self::IShutdownManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), powerstate, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn EnterPowerState(powerstate: PowerState) -> ::windows::runtime::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), powerstate).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn EnterPowerStateWithTimeSpan<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(powerstate: PowerState, wakeupafter: Param1) -> ::windows::runtime::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), powerstate, wakeupafter.into_param().abi()).ok() })
    }
    pub fn IShutdownManagerStatics<R, F: FnOnce(&IShutdownManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ShutdownManager, IShutdownManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IShutdownManagerStatics2<R, F: FnOnce(&IShutdownManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ShutdownManager, IShutdownManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ShutdownManager {
    const NAME: &'static str = "Windows.System.ShutdownManager";
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct SystemManagementContract(pub u8);
#[doc = "*Required features: `System`*"]
pub struct TimeZoneSettings {}
impl TimeZoneSettings {
    #[doc = "*Required features: `System`*"]
    pub fn CurrentTimeZoneDisplayName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn SupportedTimeZoneDisplayNames() -> ::windows::runtime::Result<super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn CanChangeTimeZone() -> ::windows::runtime::Result<bool> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn ChangeTimeZoneByDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(timezonedisplayname: Param0) -> ::windows::runtime::Result<()> {
        Self::ITimeZoneSettingsStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), timezonedisplayname.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn AutoUpdateTimeZoneAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(timeout: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>> {
        Self::ITimeZoneSettingsStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timeout.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>(result__)
        })
    }
    pub fn ITimeZoneSettingsStatics<R, F: FnOnce(&ITimeZoneSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITimeZoneSettingsStatics2<R, F: FnOnce(&ITimeZoneSettingsStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for TimeZoneSettings {
    const NAME: &'static str = "Windows.System.TimeZoneSettings";
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct User(pub ::windows::runtime::IInspectable);
impl User {
    #[doc = "*Required features: `System`*"]
    pub fn NonRoamableId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn AuthenticationStatus(&self) -> ::windows::runtime::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: UserAuthenticationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<UserType> {
        let this = self;
        unsafe {
            let mut result__: UserType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn GetPropertyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::runtime::IInspectable>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetPropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(&self, values: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), values.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `System`, `Foundation`, `Storage_Streams`*"]
    pub fn GetPictureAsync(&self, desiredsize: UserPictureSize) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), desiredsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn CreateWatcher() -> ::windows::runtime::Result<UserWatcher> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserWatcher>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsync() -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncByType(r#type: UserType) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncByTypeAndStatus(r#type: UserType, status: UserAuthenticationStatus) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), r#type, status, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetFromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(nonroamableid: Param0) -> ::windows::runtime::Result<User> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), nonroamableid.into_param().abi(), &mut result__).from_abi::<User>(result__)
        })
    }
    #[doc = "*Required features: `System`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<User> {
        Self::IUserStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn CheckUserAgeConsentGroupAsync(&self, consentgroup: UserAgeConsentGroup) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>> {
        let this = &::windows::runtime::Interface::cast::<IUser2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), consentgroup, &mut result__).from_abi::<super::Foundation::IAsyncOperation<UserAgeConsentResult>>(result__)
        }
    }
    pub fn IUserStatics<R, F: FnOnce(&IUserStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<User, IUserStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserStatics2<R, F: FnOnce(&IUserStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<User, IUserStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for User {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.User;{df9a26c6-e746-4bcd-b5d4-120103c4209b})");
}
unsafe impl ::windows::runtime::Interface for User {
    type Vtable = IUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3751421638, 59206, 19405, [181, 212, 18, 1, 3, 196, 32, 155]);
}
impl ::windows::runtime::RuntimeName for User {
    const NAME: &'static str = "Windows.System.User";
}
impl ::core::convert::From<User> for ::windows::runtime::IUnknown {
    fn from(value: User) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&User> for ::windows::runtime::IUnknown {
    fn from(value: &User) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for User {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a User {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<User> for ::windows::runtime::IInspectable {
    fn from(value: User) -> Self {
        value.0
    }
}
impl ::core::convert::From<&User> for ::windows::runtime::IInspectable {
    fn from(value: &User) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for User {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a User {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for User {}
unsafe impl ::core::marker::Sync for User {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for UserAgeConsentGroup {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserAgeConsentGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentGroup;i4)");
}
impl ::windows::runtime::DefaultType for UserAgeConsentGroup {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for UserAgeConsentResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserAgeConsentResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentResult;i4)");
}
impl ::windows::runtime::DefaultType for UserAgeConsentResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for UserAuthenticationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserAuthenticationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserAuthenticationStatus;i4)");
}
impl ::windows::runtime::DefaultType for UserAuthenticationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserAuthenticationStatusChangeDeferral(pub ::windows::runtime::IInspectable);
impl UserAuthenticationStatusChangeDeferral {
    #[doc = "*Required features: `System`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserAuthenticationStatusChangeDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangeDeferral;{88b59568-bb30-42fb-a270-e9902e40efa7})");
}
unsafe impl ::windows::runtime::Interface for UserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2293601640, 47920, 17147, [162, 112, 233, 144, 46, 64, 239, 167]);
}
impl ::windows::runtime::RuntimeName for UserAuthenticationStatusChangeDeferral {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangeDeferral";
}
impl ::core::convert::From<UserAuthenticationStatusChangeDeferral> for ::windows::runtime::IUnknown {
    fn from(value: UserAuthenticationStatusChangeDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangeDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &UserAuthenticationStatusChangeDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserAuthenticationStatusChangeDeferral> for ::windows::runtime::IInspectable {
    fn from(value: UserAuthenticationStatusChangeDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangeDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &UserAuthenticationStatusChangeDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangeDeferral {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangeDeferral {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserAuthenticationStatusChangingEventArgs(pub ::windows::runtime::IInspectable);
impl UserAuthenticationStatusChangingEventArgs {
    #[doc = "*Required features: `System`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<UserAuthenticationStatusChangeDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatusChangeDeferral>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn NewStatus(&self) -> ::windows::runtime::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: UserAuthenticationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn CurrentStatus(&self) -> ::windows::runtime::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: UserAuthenticationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserAuthenticationStatusChangingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangingEventArgs;{8c030f28-a711-4c1e-ab48-04179c15938f})");
}
unsafe impl ::windows::runtime::Interface for UserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2349010728, 42769, 19486, [171, 72, 4, 23, 156, 21, 147, 143]);
}
impl ::windows::runtime::RuntimeName for UserAuthenticationStatusChangingEventArgs {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangingEventArgs";
}
impl ::core::convert::From<UserAuthenticationStatusChangingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: UserAuthenticationStatusChangingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &UserAuthenticationStatusChangingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserAuthenticationStatusChangingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: UserAuthenticationStatusChangingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &UserAuthenticationStatusChangingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangingEventArgs {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangingEventArgs {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserChangedEventArgs(pub ::windows::runtime::IInspectable);
impl UserChangedEventArgs {
    #[doc = "*Required features: `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System`, `Foundation_Collections`*"]
    pub fn ChangedPropertyKinds(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>> {
        let this = &::windows::runtime::Interface::cast::<IUserChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserChangedEventArgs;{086459dc-18c6-48db-bc99-724fb9203ccc})");
}
unsafe impl ::windows::runtime::Interface for UserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(140794332, 6342, 18651, [188, 153, 114, 79, 185, 32, 60, 204]);
}
impl ::windows::runtime::RuntimeName for UserChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserChangedEventArgs";
}
impl ::core::convert::From<UserChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: UserChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &UserChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: UserChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &UserChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserChangedEventArgs {}
#[doc = "*Required features: `System`*"]
pub struct UserDeviceAssociation {}
impl UserDeviceAssociation {
    #[doc = "*Required features: `System`*"]
    pub fn FindUserFromDeviceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<User> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<User>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn UserDeviceAssociationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveUserDeviceAssociationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IUserDeviceAssociationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn IUserDeviceAssociationStatics<R, F: FnOnce(&IUserDeviceAssociationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDeviceAssociation, IUserDeviceAssociationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for UserDeviceAssociation {
    const NAME: &'static str = "Windows.System.UserDeviceAssociation";
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDeviceAssociationChangedEventArgs(pub ::windows::runtime::IInspectable);
impl UserDeviceAssociationChangedEventArgs {
    #[doc = "*Required features: `System`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn NewUser(&self) -> ::windows::runtime::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn OldUser(&self) -> ::windows::runtime::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDeviceAssociationChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserDeviceAssociationChangedEventArgs;{bd1f6f6c-bb5d-4d7b-a5f0-c8cd11a38d42})");
}
unsafe impl ::windows::runtime::Interface for UserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3172953964, 47965, 19835, [165, 240, 200, 205, 17, 163, 141, 66]);
}
impl ::windows::runtime::RuntimeName for UserDeviceAssociationChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserDeviceAssociationChangedEventArgs";
}
impl ::core::convert::From<UserDeviceAssociationChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: UserDeviceAssociationChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDeviceAssociationChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &UserDeviceAssociationChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDeviceAssociationChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: UserDeviceAssociationChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDeviceAssociationChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &UserDeviceAssociationChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDeviceAssociationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDeviceAssociationChangedEventArgs {}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserPicker(pub ::windows::runtime::IInspectable);
impl UserPicker {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserPicker, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `System`*"]
    pub fn AllowGuestAccounts(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetAllowGuestAccounts(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SuggestedSelectedUser(&self) -> ::windows::runtime::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn SetSuggestedSelectedUser<'a, Param0: ::windows::runtime::IntoParam<'a, User>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn PickSingleUserAsync(&self) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<User>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<User>>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IUserPickerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IUserPickerStatics<R, F: FnOnce(&IUserPickerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserPicker, IUserPickerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserPicker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserPicker;{7d548008-f1e3-4a6c-8ddc-a9bb0f488aed})");
}
unsafe impl ::windows::runtime::Interface for UserPicker {
    type Vtable = IUserPicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2102689800, 61923, 19052, [141, 220, 169, 187, 15, 72, 138, 237]);
}
impl ::windows::runtime::RuntimeName for UserPicker {
    const NAME: &'static str = "Windows.System.UserPicker";
}
impl ::core::convert::From<UserPicker> for ::windows::runtime::IUnknown {
    fn from(value: UserPicker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserPicker> for ::windows::runtime::IUnknown {
    fn from(value: &UserPicker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserPicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserPicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserPicker> for ::windows::runtime::IInspectable {
    fn from(value: UserPicker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserPicker> for ::windows::runtime::IInspectable {
    fn from(value: &UserPicker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserPicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserPicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserPicker {}
unsafe impl ::core::marker::Sync for UserPicker {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for UserPictureSize {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserPictureSize {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserPictureSize;i4)");
}
impl ::windows::runtime::DefaultType for UserPictureSize {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for UserType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserType;i4)");
}
impl ::windows::runtime::DefaultType for UserType {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserWatcher(pub ::windows::runtime::IInspectable);
impl UserWatcher {
    #[doc = "*Required features: `System`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<UserWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: UserWatcherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Updated<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn AuthenticationStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveAuthenticationStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn AuthenticationStatusChanging<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveAuthenticationStatusChanging<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.UserWatcher;{155eb23b-242a-45e0-a2e9-3171fc6a7fbb})");
}
unsafe impl ::windows::runtime::Interface for UserWatcher {
    type Vtable = IUserWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(358527547, 9258, 17888, [162, 233, 49, 113, 252, 106, 127, 187]);
}
impl ::windows::runtime::RuntimeName for UserWatcher {
    const NAME: &'static str = "Windows.System.UserWatcher";
}
impl ::core::convert::From<UserWatcher> for ::windows::runtime::IUnknown {
    fn from(value: UserWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &UserWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserWatcher> for ::windows::runtime::IInspectable {
    fn from(value: UserWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &UserWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserWatcher {}
unsafe impl ::core::marker::Sync for UserWatcher {}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for UserWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherStatus;i4)");
}
impl ::windows::runtime::DefaultType for UserWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for UserWatcherUpdateKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserWatcherUpdateKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherUpdateKind;i4)");
}
impl ::windows::runtime::DefaultType for UserWatcherUpdateKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for VirtualKey {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VirtualKey {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKey;i4)");
}
impl ::windows::runtime::DefaultType for VirtualKey {
    type DefaultType = Self;
}
#[doc = "*Required features: `System`*"]
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
unsafe impl ::windows::runtime::Abi for VirtualKeyModifiers {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VirtualKeyModifiers {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKeyModifiers;u4)");
}
impl ::windows::runtime::DefaultType for VirtualKeyModifiers {
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
