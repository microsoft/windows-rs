#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppActivationResult(::windows::core::IUnknown);
impl AppActivationResult {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppResourceGroupInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppActivationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppActivationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppActivationResult {}
impl ::core::fmt::Debug for AppActivationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppActivationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppActivationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppActivationResult;{6b528900-f46e-4eb0-aa6c-38af557cf9ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppActivationResult {
    type Vtable = IAppActivationResult_Vtbl;
    const IID: ::windows::core::GUID = <IAppActivationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppActivationResult {
    const NAME: &'static str = "Windows.System.AppActivationResult";
}
impl ::core::convert::From<AppActivationResult> for ::windows::core::IUnknown {
    fn from(value: AppActivationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppActivationResult> for ::windows::core::IUnknown {
    fn from(value: &AppActivationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppActivationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppActivationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppActivationResult> for ::windows::core::IInspectable {
    fn from(value: AppActivationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppActivationResult> for ::windows::core::IInspectable {
    fn from(value: &AppActivationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppActivationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppActivationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppActivationResult {}
unsafe impl ::core::marker::Sync for AppActivationResult {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppDiagnosticInfo(::windows::core::IUnknown);
impl AppDiagnosticInfo {
    #[doc = "*Required features: `\"System\"`, `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> ::windows::core::Result<super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ApplicationModel::AppInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResourceGroups(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>> {
        let this = &::windows::core::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetResourceGroups)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppResourceGroupInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CreateResourceGroupWatcher(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcher> {
        let this = &::windows::core::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateResourceGroupWatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfoWatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppActivationResult>> {
        let this = &::windows::core::Interface::cast::<IAppDiagnosticInfo3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppActivationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestInfoAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CreateWatcher() -> ::windows::core::Result<AppDiagnosticInfoWatcher> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDiagnosticInfoWatcher>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagefamilyname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestInfoForPackageAsync)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForAppAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestInfoForAppAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForAppUserModelId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appusermodelid: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestInfoForAppUserModelId)(::core::mem::transmute_copy(this), appusermodelid.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppDiagnosticInfoStatics<R, F: FnOnce(&IAppDiagnosticInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAppDiagnosticInfoStatics2<R, F: FnOnce(&IAppDiagnosticInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppDiagnosticInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDiagnosticInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDiagnosticInfo {}
impl ::core::fmt::Debug for AppDiagnosticInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfo;{e348a69a-8889-4ca3-be07-d5ffff5f0804})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAppDiagnosticInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppDiagnosticInfo {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfo";
}
impl ::core::convert::From<AppDiagnosticInfo> for ::windows::core::IUnknown {
    fn from(value: AppDiagnosticInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfo> for ::windows::core::IUnknown {
    fn from(value: &AppDiagnosticInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppDiagnosticInfo> for ::windows::core::IInspectable {
    fn from(value: AppDiagnosticInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfo> for ::windows::core::IInspectable {
    fn from(value: &AppDiagnosticInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppDiagnosticInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfo {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfo {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcher(::windows::core::IUnknown);
impl AppDiagnosticInfoWatcher {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Added)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Removed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemoved)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AppDiagnosticInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: AppDiagnosticInfoWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDiagnosticInfoWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AppDiagnosticInfoWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDiagnosticInfoWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDiagnosticInfoWatcher {}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfoWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcher;{75575070-01d3-489a-9325-52f9cc6ede0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IAppDiagnosticInfoWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppDiagnosticInfoWatcher {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcher";
}
impl ::core::convert::From<AppDiagnosticInfoWatcher> for ::windows::core::IUnknown {
    fn from(value: AppDiagnosticInfoWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcher> for ::windows::core::IUnknown {
    fn from(value: &AppDiagnosticInfoWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppDiagnosticInfoWatcher> for ::windows::core::IInspectable {
    fn from(value: AppDiagnosticInfoWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcher> for ::windows::core::IInspectable {
    fn from(value: &AppDiagnosticInfoWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcher {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcher {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcherEventArgs(::windows::core::IUnknown);
impl AppDiagnosticInfoWatcherEventArgs {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AppDiagnosticInfo(&self) -> ::windows::core::Result<AppDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppDiagnosticInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDiagnosticInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDiagnosticInfoWatcherEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDiagnosticInfoWatcherEventArgs {}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcherEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcherEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfoWatcherEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcherEventArgs;{7017c716-e1da-4c65-99df-046dff5be71a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppDiagnosticInfoWatcherEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppDiagnosticInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcherEventArgs";
}
impl ::core::convert::From<AppDiagnosticInfoWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppDiagnosticInfoWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppDiagnosticInfoWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppDiagnosticInfoWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppDiagnosticInfoWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppDiagnosticInfoWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcherEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppDiagnosticInfoWatcherStatus(pub i32);
impl AppDiagnosticInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppDiagnosticInfoWatcherStatus {}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppDiagnosticInfoWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppDiagnosticInfoWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDiagnosticInfoWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppDiagnosticInfoWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppExecutionStateChangeResult(::windows::core::IUnknown);
impl AppExecutionStateChangeResult {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for AppExecutionStateChangeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExecutionStateChangeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExecutionStateChangeResult {}
impl ::core::fmt::Debug for AppExecutionStateChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExecutionStateChangeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppExecutionStateChangeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppExecutionStateChangeResult;{6f039bf0-f91b-4df8-ae77-3033ccb69114})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_Vtbl;
    const IID: ::windows::core::GUID = <IAppExecutionStateChangeResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppExecutionStateChangeResult {
    const NAME: &'static str = "Windows.System.AppExecutionStateChangeResult";
}
impl ::core::convert::From<AppExecutionStateChangeResult> for ::windows::core::IUnknown {
    fn from(value: AppExecutionStateChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExecutionStateChangeResult> for ::windows::core::IUnknown {
    fn from(value: &AppExecutionStateChangeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppExecutionStateChangeResult> for ::windows::core::IInspectable {
    fn from(value: AppExecutionStateChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExecutionStateChangeResult> for ::windows::core::IInspectable {
    fn from(value: &AppExecutionStateChangeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppExecutionStateChangeResult {}
unsafe impl ::core::marker::Sync for AppExecutionStateChangeResult {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppMemoryReport(::windows::core::IUnknown);
impl AppMemoryReport {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrivateCommitUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn PeakPrivateCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PeakPrivateCommitUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TotalCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TotalCommitUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TotalCommitLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TotalCommitLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn ExpectedTotalCommitLimit(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IAppMemoryReport2>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExpectedTotalCommitLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppMemoryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppMemoryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppMemoryReport {}
impl ::core::fmt::Debug for AppMemoryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppMemoryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryReport;{6d65339b-4d6f-45bc-9c5e-e49b3ff2758d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppMemoryReport {
    type Vtable = IAppMemoryReport_Vtbl;
    const IID: ::windows::core::GUID = <IAppMemoryReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppMemoryReport {
    const NAME: &'static str = "Windows.System.AppMemoryReport";
}
impl ::core::convert::From<AppMemoryReport> for ::windows::core::IUnknown {
    fn from(value: AppMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppMemoryReport> for ::windows::core::IUnknown {
    fn from(value: &AppMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppMemoryReport> for ::windows::core::IInspectable {
    fn from(value: AppMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppMemoryReport> for ::windows::core::IInspectable {
    fn from(value: &AppMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppMemoryReport {}
unsafe impl ::core::marker::Sync for AppMemoryReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppMemoryUsageLevel(pub i32);
impl AppMemoryUsageLevel {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
    pub const OverLimit: Self = Self(3i32);
}
impl ::core::marker::Copy for AppMemoryUsageLevel {}
impl ::core::clone::Clone for AppMemoryUsageLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppMemoryUsageLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppMemoryUsageLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppMemoryUsageLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryUsageLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppMemoryUsageLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppMemoryUsageLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppMemoryUsageLimitChangingEventArgs(::windows::core::IUnknown);
impl AppMemoryUsageLimitChangingEventArgs {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn OldLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn NewLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppMemoryUsageLimitChangingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppMemoryUsageLimitChangingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppMemoryUsageLimitChangingEventArgs {}
impl ::core::fmt::Debug for AppMemoryUsageLimitChangingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryUsageLimitChangingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppMemoryUsageLimitChangingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryUsageLimitChangingEventArgs;{79f86664-feca-4da5-9e40-2bc63efdc979})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppMemoryUsageLimitChangingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppMemoryUsageLimitChangingEventArgs {
    const NAME: &'static str = "Windows.System.AppMemoryUsageLimitChangingEventArgs";
}
impl ::core::convert::From<AppMemoryUsageLimitChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppMemoryUsageLimitChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppMemoryUsageLimitChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppMemoryUsageLimitChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppMemoryUsageLimitChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppMemoryUsageLimitChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppMemoryUsageLimitChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppMemoryUsageLimitChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppMemoryUsageLimitChangingEventArgs {}
unsafe impl ::core::marker::Sync for AppMemoryUsageLimitChangingEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupBackgroundTaskReport(::windows::core::IUnknown);
impl AppResourceGroupBackgroundTaskReport {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TaskId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Trigger(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Trigger)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn EntryPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EntryPoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupBackgroundTaskReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupBackgroundTaskReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupBackgroundTaskReport {}
impl ::core::fmt::Debug for AppResourceGroupBackgroundTaskReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupBackgroundTaskReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupBackgroundTaskReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupBackgroundTaskReport;{2566e74e-b05d-40c2-9dc1-1a4f039ea120})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_Vtbl;
    const IID: ::windows::core::GUID = <IAppResourceGroupBackgroundTaskReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupBackgroundTaskReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupBackgroundTaskReport";
}
impl ::core::convert::From<AppResourceGroupBackgroundTaskReport> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupBackgroundTaskReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupBackgroundTaskReport> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupBackgroundTaskReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupBackgroundTaskReport> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupBackgroundTaskReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupBackgroundTaskReport> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupBackgroundTaskReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupBackgroundTaskReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupBackgroundTaskReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppResourceGroupEnergyQuotaState(pub i32);
impl AppResourceGroupEnergyQuotaState {
    pub const Unknown: Self = Self(0i32);
    pub const Over: Self = Self(1i32);
    pub const Under: Self = Self(2i32);
}
impl ::core::marker::Copy for AppResourceGroupEnergyQuotaState {}
impl ::core::clone::Clone for AppResourceGroupEnergyQuotaState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupEnergyQuotaState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppResourceGroupEnergyQuotaState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppResourceGroupEnergyQuotaState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupEnergyQuotaState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupEnergyQuotaState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupEnergyQuotaState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppResourceGroupExecutionState(pub i32);
impl AppResourceGroupExecutionState {
    pub const Unknown: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspending: Self = Self(2i32);
    pub const Suspended: Self = Self(3i32);
    pub const NotRunning: Self = Self(4i32);
}
impl ::core::marker::Copy for AppResourceGroupExecutionState {}
impl ::core::clone::Clone for AppResourceGroupExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupExecutionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppResourceGroupExecutionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppResourceGroupExecutionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupExecutionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupExecutionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupExecutionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupInfo(::windows::core::IUnknown);
impl AppResourceGroupInfo {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstanceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn IsShared(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsShared)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBackgroundTaskReports(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBackgroundTaskReports)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetMemoryReport(&self) -> ::windows::core::Result<AppResourceGroupMemoryReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMemoryReport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupMemoryReport>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`, `\"System_Diagnostics\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub fn GetProcessDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetProcessDiagnosticInfos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetStateReport(&self) -> ::windows::core::Result<AppResourceGroupStateReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStateReport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupStateReport>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartSuspendAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartSuspendAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartResumeAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartResumeAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTerminateAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTerminateAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfo {}
impl ::core::fmt::Debug for AppResourceGroupInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfo;{b913f77a-e807-49f4-845e-7b8bdcfe8ee7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAppResourceGroupInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupInfo {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfo";
}
impl ::core::convert::From<AppResourceGroupInfo> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfo> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupInfo> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfo> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfo {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfo {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcher(::windows::core::IUnknown);
impl AppResourceGroupInfoWatcher {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Added)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Removed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemoved)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecutionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecutionStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveExecutionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveExecutionStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: AppResourceGroupInfoWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfoWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfoWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfoWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfoWatcher {}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcher;{d9b0a0fd-6e5a-4c72-8b17-09fec4a212bd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IAppResourceGroupInfoWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupInfoWatcher {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcher";
}
impl ::core::convert::From<AppResourceGroupInfoWatcher> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupInfoWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcher> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcher> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupInfoWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcher> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcher {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcher {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherEventArgs(::windows::core::IUnknown);
impl AppResourceGroupInfoWatcherEventArgs {
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppDiagnosticInfos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppResourceGroupInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfoWatcherEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfoWatcherEventArgs {}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcherEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherEventArgs;{7a787637-6302-4d2f-bf89-1c12d0b2a6b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppResourceGroupInfoWatcherEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherEventArgs";
}
impl ::core::convert::From<AppResourceGroupInfoWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupInfoWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupInfoWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherExecutionStateChangedEventArgs(::windows::core::IUnknown);
impl AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppDiagnosticInfos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppResourceGroupInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherExecutionStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs;{1bdbedd7-fee6-4fd4-98dd-e92a2cc299f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs";
}
impl ::core::convert::From<AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppResourceGroupInfoWatcherStatus(pub i32);
impl AppResourceGroupInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppResourceGroupInfoWatcherStatus {}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupInfoWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppResourceGroupInfoWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupInfoWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupInfoWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupMemoryReport(::windows::core::IUnknown);
impl AppResourceGroupMemoryReport {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CommitUsageLimit(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CommitUsageLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CommitUsageLevel(&self) -> ::windows::core::Result<AppMemoryUsageLevel> {
        let this = self;
        unsafe {
            let mut result__: AppMemoryUsageLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CommitUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppMemoryUsageLevel>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrivateCommitUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TotalCommitUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TotalCommitUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupMemoryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupMemoryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupMemoryReport {}
impl ::core::fmt::Debug for AppResourceGroupMemoryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupMemoryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupMemoryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupMemoryReport;{2c8c06b1-7db1-4c51-a225-7fae2d49e431})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_Vtbl;
    const IID: ::windows::core::GUID = <IAppResourceGroupMemoryReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupMemoryReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupMemoryReport";
}
impl ::core::convert::From<AppResourceGroupMemoryReport> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupMemoryReport> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupMemoryReport> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupMemoryReport> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupMemoryReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupMemoryReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupStateReport(::windows::core::IUnknown);
impl AppResourceGroupStateReport {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn ExecutionState(&self) -> ::windows::core::Result<AppResourceGroupExecutionState> {
        let this = self;
        unsafe {
            let mut result__: AppResourceGroupExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecutionState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn EnergyQuotaState(&self) -> ::windows::core::Result<AppResourceGroupEnergyQuotaState> {
        let this = self;
        unsafe {
            let mut result__: AppResourceGroupEnergyQuotaState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnergyQuotaState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppResourceGroupEnergyQuotaState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupStateReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupStateReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupStateReport {}
impl ::core::fmt::Debug for AppResourceGroupStateReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupStateReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppResourceGroupStateReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupStateReport;{52849f18-2f70-4236-ab40-d04db0c7b931})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_Vtbl;
    const IID: ::windows::core::GUID = <IAppResourceGroupStateReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppResourceGroupStateReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupStateReport";
}
impl ::core::convert::From<AppResourceGroupStateReport> for ::windows::core::IUnknown {
    fn from(value: AppResourceGroupStateReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupStateReport> for ::windows::core::IUnknown {
    fn from(value: &AppResourceGroupStateReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupStateReport> for ::windows::core::IInspectable {
    fn from(value: AppResourceGroupStateReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupStateReport> for ::windows::core::IInspectable {
    fn from(value: &AppResourceGroupStateReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppResourceGroupStateReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupStateReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupStateReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppUriHandlerHost(::windows::core::IUnknown);
impl AppUriHandlerHost {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUriHandlerHost, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<AppUriHandlerHost> {
        Self::IAppUriHandlerHostFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<AppUriHandlerHost>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppUriHandlerHostFactory<R, F: FnOnce(&IAppUriHandlerHostFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUriHandlerHost, IAppUriHandlerHostFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppUriHandlerHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUriHandlerHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUriHandlerHost {}
impl ::core::fmt::Debug for AppUriHandlerHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUriHandlerHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppUriHandlerHost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerHost;{5d50cac5-92d2-5409-b56f-7f73e10ea4c3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_Vtbl;
    const IID: ::windows::core::GUID = <IAppUriHandlerHost as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppUriHandlerHost {
    const NAME: &'static str = "Windows.System.AppUriHandlerHost";
}
impl ::core::convert::From<AppUriHandlerHost> for ::windows::core::IUnknown {
    fn from(value: AppUriHandlerHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerHost> for ::windows::core::IUnknown {
    fn from(value: &AppUriHandlerHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppUriHandlerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppUriHandlerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppUriHandlerHost> for ::windows::core::IInspectable {
    fn from(value: AppUriHandlerHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerHost> for ::windows::core::IInspectable {
    fn from(value: &AppUriHandlerHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppUriHandlerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppUriHandlerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerHost {}
unsafe impl ::core::marker::Sync for AppUriHandlerHost {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppUriHandlerRegistration(::windows::core::IUnknown);
impl AppUriHandlerRegistration {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppAddedHostsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppAddedHostsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAppAddedHostsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<AppUriHandlerHost>>>(&self, hosts: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetAppAddedHostsAsync)(::core::mem::transmute_copy(this), hosts.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllHosts(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAllHosts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppUriHandlerHost>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateHosts<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<AppUriHandlerHost>>>(&self, hosts: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).UpdateHosts)(::core::mem::transmute_copy(this), hosts.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppUriHandlerRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUriHandlerRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUriHandlerRegistration {}
impl ::core::fmt::Debug for AppUriHandlerRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUriHandlerRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppUriHandlerRegistration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistration;{6f73aeb1-4569-5c3f-9ba0-99123eea32c3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_Vtbl;
    const IID: ::windows::core::GUID = <IAppUriHandlerRegistration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppUriHandlerRegistration {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistration";
}
impl ::core::convert::From<AppUriHandlerRegistration> for ::windows::core::IUnknown {
    fn from(value: AppUriHandlerRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerRegistration> for ::windows::core::IUnknown {
    fn from(value: &AppUriHandlerRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppUriHandlerRegistration> for ::windows::core::IInspectable {
    fn from(value: AppUriHandlerRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerRegistration> for ::windows::core::IInspectable {
    fn from(value: &AppUriHandlerRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppUriHandlerRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerRegistration {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistration {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppUriHandlerRegistrationManager(::windows::core::IUnknown);
impl AppUriHandlerRegistrationManager {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TryGetRegistration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<AppUriHandlerRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetRegistration)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistration>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppUriHandlerRegistrationManager2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, User>>(user: Param0) -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetForPackage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagefamilyname: Param0) -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForPackage)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetForPackageForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, User>>(packagefamilyname: Param0, user: Param1) -> ::windows::core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForPackageForUser)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), user.into_param().abi(), &mut result__).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppUriHandlerRegistrationManagerStatics<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAppUriHandlerRegistrationManagerStatics2<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppUriHandlerRegistrationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUriHandlerRegistrationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUriHandlerRegistrationManager {}
impl ::core::fmt::Debug for AppUriHandlerRegistrationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUriHandlerRegistrationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppUriHandlerRegistrationManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistrationManager;{e62c9a52-ac94-5750-ac1b-6cfb6f250263})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_Vtbl;
    const IID: ::windows::core::GUID = <IAppUriHandlerRegistrationManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppUriHandlerRegistrationManager {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistrationManager";
}
impl ::core::convert::From<AppUriHandlerRegistrationManager> for ::windows::core::IUnknown {
    fn from(value: AppUriHandlerRegistrationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerRegistrationManager> for ::windows::core::IUnknown {
    fn from(value: &AppUriHandlerRegistrationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppUriHandlerRegistrationManager> for ::windows::core::IInspectable {
    fn from(value: AppUriHandlerRegistrationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerRegistrationManager> for ::windows::core::IInspectable {
    fn from(value: &AppUriHandlerRegistrationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerRegistrationManager {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistrationManager {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoUpdateTimeZoneStatus(pub i32);
impl AutoUpdateTimeZoneStatus {
    pub const Attempted: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoUpdateTimeZoneStatus {}
impl ::core::clone::Clone for AutoUpdateTimeZoneStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoUpdateTimeZoneStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AutoUpdateTimeZoneStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoUpdateTimeZoneStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateTimeZoneStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutoUpdateTimeZoneStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.AutoUpdateTimeZoneStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
pub struct DateTimeSettings {}
impl DateTimeSettings {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemDateTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::DateTime>>(utcdatetime: Param0) -> ::windows::core::Result<()> {
        Self::IDateTimeSettingsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetSystemDateTime)(::core::mem::transmute_copy(this), utcdatetime.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IDateTimeSettingsStatics<R, F: FnOnce(&IDateTimeSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DateTimeSettings, IDateTimeSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for DateTimeSettings {
    const NAME: &'static str = "Windows.System.DateTimeSettings";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DiagnosticAccessStatus(pub i32);
impl DiagnosticAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
    pub const Allowed: Self = Self(3i32);
}
impl ::core::marker::Copy for DiagnosticAccessStatus {}
impl ::core::clone::Clone for DiagnosticAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DiagnosticAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DiagnosticAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DiagnosticAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DiagnosticAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.DiagnosticAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueue(::windows::core::IUnknown);
impl DispatcherQueue {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CreateTimer(&self) -> ::windows::core::Result<DispatcherQueueTimer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateTimer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueueTimer>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TryEnqueue<'a, Param0: ::windows::core::IntoParam<'a, DispatcherQueueHandler>>(&self, callback: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryEnqueue)(::core::mem::transmute_copy(this), callback.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TryEnqueueWithPriority<'a, Param1: ::windows::core::IntoParam<'a, DispatcherQueueHandler>>(&self, priority: DispatcherQueuePriority, callback: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryEnqueueWithPriority)(::core::mem::transmute_copy(this), priority, callback.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShutdownStarting<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShutdownStarting)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownStarting<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShutdownStarting)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShutdownCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShutdownCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShutdownCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn HasThreadAccess(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDispatcherQueue2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasThreadAccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetForCurrentThread() -> ::windows::core::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentThread)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueue>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDispatcherQueueStatics<R, F: FnOnce(&IDispatcherQueueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DispatcherQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueue {}
impl ::core::fmt::Debug for DispatcherQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueue;{603e88e4-a338-4ffe-a457-a5cfb9ceb899})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
    const IID: ::windows::core::GUID = <IDispatcherQueue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Windows.System.DispatcherQueue";
}
impl ::core::convert::From<DispatcherQueue> for ::windows::core::IUnknown {
    fn from(value: DispatcherQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueue> for ::windows::core::IUnknown {
    fn from(value: &DispatcherQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DispatcherQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DispatcherQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DispatcherQueue> for ::windows::core::IInspectable {
    fn from(value: DispatcherQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueue> for ::windows::core::IInspectable {
    fn from(value: &DispatcherQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DispatcherQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DispatcherQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DispatcherQueue {}
unsafe impl ::core::marker::Sync for DispatcherQueue {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueueController(::windows::core::IUnknown);
impl DispatcherQueueController {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShutdownQueueAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShutdownQueueAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CreateOnDedicatedThread() -> ::windows::core::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateOnDedicatedThread)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DispatcherQueueController>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDispatcherQueueControllerStatics<R, F: FnOnce(&IDispatcherQueueControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DispatcherQueueController, IDispatcherQueueControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DispatcherQueueController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueController {}
impl ::core::fmt::Debug for DispatcherQueueController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueController;{22f34e66-50db-4e36-a98d-61c01b384d20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
    const IID: ::windows::core::GUID = <IDispatcherQueueController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Windows.System.DispatcherQueueController";
}
impl ::core::convert::From<DispatcherQueueController> for ::windows::core::IUnknown {
    fn from(value: DispatcherQueueController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueController> for ::windows::core::IUnknown {
    fn from(value: &DispatcherQueueController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DispatcherQueueController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DispatcherQueueController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DispatcherQueueController> for ::windows::core::IInspectable {
    fn from(value: DispatcherQueueController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueController> for ::windows::core::IInspectable {
    fn from(value: &DispatcherQueueController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DispatcherQueueController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DispatcherQueueController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueController {}
unsafe impl ::core::marker::Sync for DispatcherQueueController {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueueHandler(pub ::windows::core::IUnknown);
impl DispatcherQueueHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DispatcherQueueHandlerBox::<F> { vtable: &DispatcherQueueHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[repr(C)]
struct DispatcherQueueHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DispatcherQueueHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DispatcherQueueHandlerBox<F> {
    const VTABLE: DispatcherQueueHandler_Vtbl = DispatcherQueueHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::clone::Clone for DispatcherQueueHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueHandler {}
impl ::core::fmt::Debug for DispatcherQueueHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DispatcherQueueHandler {
    type Vtable = DispatcherQueueHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfa2dc9c_1a2d_4917_98f2_939af1d6e0c8);
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{dfa2dc9c-1a2d-4917-98f2-939af1d6e0c8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatcherQueueHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(10i32);
}
impl ::core::marker::Copy for DispatcherQueuePriority {}
impl ::core::clone::Clone for DispatcherQueuePriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DispatcherQueuePriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DispatcherQueuePriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for DispatcherQueuePriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueuePriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.DispatcherQueuePriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueueShutdownStartingEventArgs(::windows::core::IUnknown);
impl DispatcherQueueShutdownStartingEventArgs {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for DispatcherQueueShutdownStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueShutdownStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueShutdownStartingEventArgs {}
impl ::core::fmt::Debug for DispatcherQueueShutdownStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueShutdownStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueShutdownStartingEventArgs;{c4724c4c-ff97-40c0-a226-cc0aaa545e89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDispatcherQueueShutdownStartingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Windows.System.DispatcherQueueShutdownStartingEventArgs";
}
impl ::core::convert::From<DispatcherQueueShutdownStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueShutdownStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DispatcherQueueShutdownStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueShutdownStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueShutdownStartingEventArgs {}
unsafe impl ::core::marker::Sync for DispatcherQueueShutdownStartingEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueueTimer(::windows::core::IUnknown);
impl DispatcherQueueTimer {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Interval(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Interval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInterval<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInterval)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn IsRunning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRunning)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn IsRepeating(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRepeating)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetIsRepeating(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsRepeating)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Tick<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tick)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTick<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTick)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DispatcherQueueTimer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueTimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueTimer {}
impl ::core::fmt::Debug for DispatcherQueueTimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueTimer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueTimer;{5feabb1d-a31c-4727-b1ac-37454649d56a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
    const IID: ::windows::core::GUID = <IDispatcherQueueTimer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Windows.System.DispatcherQueueTimer";
}
impl ::core::convert::From<DispatcherQueueTimer> for ::windows::core::IUnknown {
    fn from(value: DispatcherQueueTimer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueTimer> for ::windows::core::IUnknown {
    fn from(value: &DispatcherQueueTimer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DispatcherQueueTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DispatcherQueueTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DispatcherQueueTimer> for ::windows::core::IInspectable {
    fn from(value: DispatcherQueueTimer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueTimer> for ::windows::core::IInspectable {
    fn from(value: &DispatcherQueueTimer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DispatcherQueueTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DispatcherQueueTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueTimer {}
unsafe impl ::core::marker::Sync for DispatcherQueueTimer {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct FolderLauncherOptions(::windows::core::IUnknown);
impl FolderLauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FolderLauncherOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn ItemsToSelect(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemsToSelect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<super::Storage::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__: super::UI::ViewManagement::ViewSizePreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredRemainingView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredRemainingView)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for FolderLauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FolderLauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FolderLauncherOptions {}
impl ::core::fmt::Debug for FolderLauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderLauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FolderLauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.FolderLauncherOptions;{bb91c27d-6b87-432a-bd04-776c6f5fb2ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_Vtbl;
    const IID: ::windows::core::GUID = <IFolderLauncherOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FolderLauncherOptions {
    const NAME: &'static str = "Windows.System.FolderLauncherOptions";
}
impl ::core::convert::From<FolderLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: FolderLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: &FolderLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FolderLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: FolderLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: &FolderLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FolderLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppActivationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppActivationResult {
    type Vtable = IAppActivationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b528900_f46e_4eb0_aa6c_38af557cf9ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe348a69a_8889_4ca3_be07_d5ffff5f0804);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    AppInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfo2 {
    type Vtable = IAppDiagnosticInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf46fbd7_191a_446c_9473_8fbc2374a354);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetResourceGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetResourceGroups: usize,
    pub CreateResourceGroupWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfo3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfo3 {
    type Vtable = IAppDiagnosticInfo3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc895c63d_dd61_4c65_babd_81a10b4f9815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoStatics {
    type Vtable = IAppDiagnosticInfoStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce6925bf_10ca_40c8_a9ca_c5c96501866e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoStatics2 {
    type Vtable = IAppDiagnosticInfoStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05b24b86_1000_4c90_bb9f_7235071c50fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForPackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppUserModelId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75575070_01d3_489a_9325_52f9cc6ede0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcherEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7017c716_e1da_4c65_99df_046dff5be71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcherEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppDiagnosticInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExecutionStateChangeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f039bf0_f91b_4df8_ae77_3033ccb69114);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExecutionStateChangeResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppMemoryReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppMemoryReport {
    type Vtable = IAppMemoryReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d65339b_4d6f_45bc_9c5e_e49b3ff2758d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub PeakPrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalCommitLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppMemoryReport2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppMemoryReport2 {
    type Vtable = IAppMemoryReport2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f7f3738_51b7_42dc_b7ed_79ba46d28857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExpectedTotalCommitLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppMemoryUsageLimitChangingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f86664_feca_4da5_9e40_2bc63efdc979);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryUsageLimitChangingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OldLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub NewLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupBackgroundTaskReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2566e74e_b05d_40c2_9dc1_1a4f039ea120);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupBackgroundTaskReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Trigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb913f77a_e807_49f4_845e_7b8bdcfe8ee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IsShared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBackgroundTaskReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBackgroundTaskReports: usize,
    pub GetMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub GetProcessDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_Diagnostics")))]
    GetProcessDiagnosticInfos: usize,
    pub GetStateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfo2 {
    type Vtable = IAppResourceGroupInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee9b236d_d305_4d6b_92f7_6afdad72dedc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartSuspendAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartSuspendAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartResumeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartTerminateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTerminateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9b0a0fd_6e5a_4c72_8b17_09fec4a212bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    #[cfg(feature = "Foundation")]
    pub ExecutionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecutionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExecutionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExecutionStateChanged: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a787637_6302_4d2f_bf89_1c12d0b2a6b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bdbedd7_fee6_4fd4_98dd_e92a2cc299f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupMemoryReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c8c06b1_7db1_4c51_a225_7fae2d49e431);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupMemoryReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CommitUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub CommitUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupStateReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52849f18_2f70_4236_ab40_d04db0c7b931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupStateReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExecutionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupExecutionState) -> ::windows::core::HRESULT,
    pub EnergyQuotaState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerHost(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d50cac5_92d2_5409_b56f_7f73e10ea4c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerHost2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUriHandlerHost2 {
    type Vtable = IAppUriHandlerHost2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a0bee95_29e4_51bf_8095_a3c068e3c72a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerHostFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUriHandlerHostFactory {
    type Vtable = IAppUriHandlerHostFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x257c3c96_ce04_5f98_96bb_3ebd3e9275bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHostFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f73aeb1_4569_5c3f_9ba0_99123eea32c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppAddedHostsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hosts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAppAddedHostsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistration2 {
    type Vtable = IAppUriHandlerRegistration2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd54dac97_cb39_5f1f_883e_01853730bd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllHosts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllHosts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateHosts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hosts: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateHosts: usize,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe62c9a52_ac94_5750_ac1b_6cfb6f250263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryGetRegistration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManager2 {
    type Vtable = IAppUriHandlerRegistrationManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbddfcaf1_b51a_5e69_aefd_7088d9f2b123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManagerStatics {
    type Vtable = IAppUriHandlerRegistrationManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5cedd9f_5729_5b76_a1d4_0285f295c124);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUriHandlerRegistrationManagerStatics2 {
    type Vtable = IAppUriHandlerRegistrationManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14f78379_6890_5080_90a7_98824a7f079e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetForPackageForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDateTimeSettingsStatics {
    type Vtable = IDateTimeSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2150d1_47ee_48ab_a52b_9f1954278d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeSettingsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetSystemDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utcdatetime: super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemDateTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x603e88e4_a338_4ffe_a457_a5cfb9ceb899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryEnqueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: DispatcherQueuePriority, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ShutdownCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueue2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDispatcherQueue2 {
    type Vtable = IDispatcherQueue2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc822c647_30ef_506e_bd1e_a647ae6675ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HasThreadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f34e66_50db_4e36_a98d_61c01b384d20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownQueueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownQueueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDispatcherQueueControllerStatics {
    type Vtable = IDispatcherQueueControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a6c98e0_5198_49a2_a313_3f70d1f13c27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateOnDedicatedThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueShutdownStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4724c4c_ff97_40c0_a226_cc0aaa545e89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDispatcherQueueStatics {
    type Vtable = IDispatcherQueueStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa96d83d7_9371_4517_9245_d0824ac12c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueTimer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5feabb1d_a31c_4727_b1ac_37454649d56a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Interval: usize,
    #[cfg(feature = "Foundation")]
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInterval: usize,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRepeating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRepeating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Tick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTick: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFolderLauncherOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb91c27d_6b87_432a_bd04_776c6f5fb2ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderLauncherOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub ItemsToSelect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    ItemsToSelect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownUserPropertiesStatics {
    type Vtable = IKnownUserPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7755911a_70c5_48e5_b637_5ba3441e4ee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GuestHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PrincipalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SessionInitiationProtocolUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownUserPropertiesStatics2 {
    type Vtable = IKnownUserPropertiesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b450782_f620_577e_b1b3_dd56644d79b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AgeEnforcementRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILaunchUriResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILaunchUriResult {
    type Vtable = ILaunchUriResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec27a8df_f6d5_45ca_913a_70a40c5c8221);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchUriResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LaunchUriStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Result: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherOptions {
    type Vtable = ILauncherOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafa21d8_b071_4cd8_853e_341203e557d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TreatAsUntrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTreatAsUntrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DisplayApplicationPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDisplayApplicationPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub UI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherOptions2 {
    type Vtable = ILauncherOptions2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ba08eb4_6e40_4dce_a1a3_2f53950afb49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
    #[cfg(feature = "Storage_Search")]
    pub SetNeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    SetNeighboringFilesQuery: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherOptions3 {
    type Vtable = ILauncherOptions3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0770655_4b63_4e3a_9107_4e687841923a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherOptions4 {
    type Vtable = ILauncherOptions4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef6fd10e_e6fb_4814_a44e_57e8b9d9a01b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetLimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherStatics {
    type Vtable = ILauncherStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x277151c3_9e3e_42f6_91a4_5dfdeb232451);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherStatics2 {
    type Vtable = ILauncherStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59ba2fbb_24cb_4c02_a4c4_8294569d54f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersWithLaunchUriTypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersWithLaunchUriTypeAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindFileHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindFileHandlersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherStatics3 {
    type Vtable = ILauncherStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x234261a8_9db3_4683_aa42_dc6f51d33847);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherStatics4 {
    type Vtable = ILauncherStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9ec819f_b5a5_41c6_b3b3_dd1b3178bcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindAppUriHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindAppUriHandlersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherStatics5 {
    type Vtable = ILauncherStatics5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b24ef84_d895_5fea_9153_1ac49aed9ba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherUIOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILauncherUIOptions {
    type Vtable = ILauncherUIOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b25da6e_8aa6_41e9_8251_4165f5985f49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherUIOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub InvocationPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetInvocationPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectionRect: usize,
    #[cfg(feature = "UI_Popups")]
    pub PreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    PreferredPlacement: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetPreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetPreferredPlacement: usize,
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ILauncherViewOptions(::windows::core::IUnknown);
impl ILauncherViewOptions {
    #[doc = "*Required features: `\"System\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__: super::UI::ViewManagement::ViewSizePreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredRemainingView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredRemainingView)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::convert::From<ILauncherViewOptions> for ::windows::core::IUnknown {
    fn from(value: ILauncherViewOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILauncherViewOptions> for ::windows::core::IUnknown {
    fn from(value: &ILauncherViewOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILauncherViewOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILauncherViewOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILauncherViewOptions> for ::windows::core::IInspectable {
    fn from(value: ILauncherViewOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILauncherViewOptions> for ::windows::core::IInspectable {
    fn from(value: &ILauncherViewOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILauncherViewOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILauncherViewOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILauncherViewOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILauncherViewOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILauncherViewOptions {}
impl ::core::fmt::Debug for ILauncherViewOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILauncherViewOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILauncherViewOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8a9b29f1-7ca7-49de-9bd3-3c5b7184f616}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILauncherViewOptions {
    type Vtable = ILauncherViewOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9b29f1_7ca7_49de_9bd3_3c5b7184f616);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherViewOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub DesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    DesiredRemainingView: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub SetDesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    SetDesiredRemainingView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMemoryManagerStatics {
    type Vtable = IMemoryManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c6c279c_d7ca_4779_9188_4057219ce64c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppMemoryUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub AppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub AppMemoryUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageLimitChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageLimitChanging: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMemoryManagerStatics2 {
    type Vtable = IMemoryManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eee351f_6d62_423f_9479_b01f9c9f7669);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetAppMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetProcessMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMemoryManagerStatics3 {
    type Vtable = IMemoryManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x149b59ce_92ad_4e35_89eb_50dfb4c0d91c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TrySetAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMemoryManagerStatics4 {
    type Vtable = IMemoryManagerStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5a94828_e84e_4886_8a0d_44b3190e3b72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExpectedAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessLauncherOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3080b9cf_f444_4a83_beaf_a549a0f3229c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub StandardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardError: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardError: usize,
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessLauncherResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProcessLauncherResult {
    type Vtable = IProcessLauncherResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x544c8934_86d8_4991_8e75_ece8a43b6b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessLauncherStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProcessLauncherStatics {
    type Vtable = IProcessLauncherStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ab66e7_2d0e_448b_a6a0_c13c3836d09c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsyncWithOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessMemoryReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProcessMemoryReport {
    type Vtable = IProcessMemoryReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x087305a8_9b70_4782_8741_3a982b6ce5e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PrivateWorkingSetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalWorkingSetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtocolForResultsOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd581293a_6de9_4d28_9378_f86782e182bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteLauncherOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e3a2788_2891_4cdf_a2d6_9dff7d02e693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferredAppIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferredAppIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteLauncherStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteLauncherStatics {
    type Vtable = IRemoteLauncherStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7db7a93_a30c_48b7_9f21_051026a4e517);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_RemoteSystems")))]
    LaunchUriWithDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShutdownManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShutdownManagerStatics {
    type Vtable = IShutdownManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72e247ed_dd5b_4d6c_b1d0_c57a7bbb5f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub BeginShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginShutdown: usize,
    pub CancelShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShutdownManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShutdownManagerStatics2 {
    type Vtable = IShutdownManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f69a02f_9c34_43c7_a8c3_70b30a7f7504);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPowerStateSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState, result__: *mut bool) -> ::windows::core::HRESULT,
    pub EnterPowerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnterPowerStateWithTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterPowerStateWithTimeSpan: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimeZoneSettingsStatics {
    type Vtable = ITimeZoneSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b3b2bea_a101_41ae_9fbd_028728bab73d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CurrentTimeZoneDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTimeZoneDisplayNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTimeZoneDisplayNames: usize,
    pub CanChangeTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ChangeTimeZoneByDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timezonedisplayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimeZoneSettingsStatics2 {
    type Vtable = ITimeZoneSettingsStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x555c0db8_39a8_49fa_b4f6_a2c7fc2842ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AutoUpdateTimeZoneAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoUpdateTimeZoneAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUser {
    type Vtable = IUser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf9a26c6_e746_4bcd_b5d4_120103c4209b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NonRoamableId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AuthenticationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredsize: UserPictureSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPictureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUser2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUser2 {
    type Vtable = IUser2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98ba5628_a6e3_518e_89d9_d3b2b1991a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CheckUserAgeConsentGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, consentgroup: UserAgeConsentGroup, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckUserAgeConsentGroupAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangeDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88b59568_bb30_42fb_a270_e9902e40efa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangeDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c030f28_a711_4c1e_ab48_04179c15938f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NewStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT,
    pub CurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x086459dc_18c6_48db_bc99_724fb9203ccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserChangedEventArgs2 {
    type Vtable = IUserChangedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b2ccb44_6f01_560c_97ad_fc7f32ec581f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ChangedPropertyKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ChangedPropertyKinds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDeviceAssociationChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1f6f6c_bb5d_4d7b_a5f0_c8cd11a38d42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NewUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OldUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDeviceAssociationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDeviceAssociationStatics {
    type Vtable = IUserDeviceAssociationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e491e14_f85a_4c07_8da9_7fe3d0542343);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FindUserFromDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserDeviceAssociationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserDeviceAssociationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserPicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserPicker {
    type Vtable = IUserPicker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d548008_f1e3_4a6c_8ddc_a9bb0f488aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPicker_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllowGuestAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowGuestAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SuggestedSelectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetSuggestedSelectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PickSingleUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserPickerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserPickerStatics {
    type Vtable = IUserPickerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde3290dc_7e73_4df6_a1ae_4d7eca82b40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPickerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserStatics {
    type Vtable = IUserStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: UserType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByType: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByTypeAndStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: UserType, status: UserAuthenticationStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByTypeAndStatus: usize,
    pub GetFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonroamableid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserStatics2 {
    type Vtable = IUserStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74a37e11_2eb5_4487_b0d5_2c6790e013e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserWatcher {
    type Vtable = IUserWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserWatcher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
#[doc = "*Required features: `\"System\"`*"]
pub struct KnownUserProperties {}
impl KnownUserProperties {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn DisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn FirstName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FirstName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn LastName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn ProviderName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AccountName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GuestHost() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GuestHost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn PrincipalName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrincipalName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn DomainName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DomainName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SessionInitiationProtocolUri() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionInitiationProtocolUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AgeEnforcementRegion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownUserPropertiesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AgeEnforcementRegion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownUserPropertiesStatics<R, F: FnOnce(&IKnownUserPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKnownUserPropertiesStatics2<R, F: FnOnce(&IKnownUserPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownUserProperties {
    const NAME: &'static str = "Windows.System.KnownUserProperties";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LaunchFileStatus(pub i32);
impl LaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const DeniedByPolicy: Self = Self(2i32);
    pub const FileTypeNotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchFileStatus {}
impl ::core::clone::Clone for LaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LaunchFileStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchFileStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchFileStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchFileStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LaunchQuerySupportStatus(pub i32);
impl LaunchQuerySupportStatus {
    pub const Available: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchQuerySupportStatus {}
impl ::core::clone::Clone for LaunchQuerySupportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchQuerySupportStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LaunchQuerySupportStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchQuerySupportStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchQuerySupportStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchQuerySupportStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LaunchQuerySupportType(pub i32);
impl LaunchQuerySupportType {
    pub const Uri: Self = Self(0i32);
    pub const UriForResults: Self = Self(1i32);
}
impl ::core::marker::Copy for LaunchQuerySupportType {}
impl ::core::clone::Clone for LaunchQuerySupportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchQuerySupportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LaunchQuerySupportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchQuerySupportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchQuerySupportType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchQuerySupportType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LaunchUriResult(::windows::core::IUnknown);
impl LaunchUriResult {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<LaunchUriStatus> {
        let this = self;
        unsafe {
            let mut result__: LaunchUriStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LaunchUriStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Result(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for LaunchUriResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LaunchUriResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LaunchUriResult {}
impl ::core::fmt::Debug for LaunchUriResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchUriResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchUriResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.LaunchUriResult;{ec27a8df-f6d5-45ca-913a-70a40c5c8221})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LaunchUriResult {
    type Vtable = ILaunchUriResult_Vtbl;
    const IID: ::windows::core::GUID = <ILaunchUriResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LaunchUriResult {
    const NAME: &'static str = "Windows.System.LaunchUriResult";
}
impl ::core::convert::From<LaunchUriResult> for ::windows::core::IUnknown {
    fn from(value: LaunchUriResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchUriResult> for ::windows::core::IUnknown {
    fn from(value: &LaunchUriResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LaunchUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LaunchUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LaunchUriResult> for ::windows::core::IInspectable {
    fn from(value: LaunchUriResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchUriResult> for ::windows::core::IInspectable {
    fn from(value: &LaunchUriResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LaunchUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LaunchUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LaunchUriResult {}
unsafe impl ::core::marker::Sync for LaunchUriResult {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LaunchUriStatus(pub i32);
impl LaunchUriStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const ProtocolUnavailable: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LaunchUriStatus {}
impl ::core::clone::Clone for LaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchUriStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LaunchUriStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchUriStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchUriStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchUriStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchUriStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
pub struct Launcher {}
impl Launcher {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFileAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>>(file: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFileWithOptionsAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>>(uri: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriWithOptionsAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>>(uri: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriForResultsAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriForResultsWithDataAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>, Param2: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(uri: Param0, options: Param1, inputdata: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriForResultsWithDataAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriWithDataAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, LauncherOptions>, Param2: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(uri: Param0, options: Param1, inputdata: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriWithDataAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QueryUriSupportAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), launchquerysupporttype, &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportWithPackageFamilyNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uri: Param0, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QueryUriSupportWithPackageFamilyNameAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), launchquerysupporttype, packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::StorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QueryFileSupportAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportWithPackageFamilyNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::StorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, packagefamilyname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QueryFileSupportWithPackageFamilyNameAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scheme: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindUriSchemeHandlersAsync)(::core::mem::transmute_copy(this), scheme.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersWithLaunchUriTypeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scheme: Param0, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindUriSchemeHandlersWithLaunchUriTypeAsync)(::core::mem::transmute_copy(this), scheme.into_param().abi(), launchquerysupporttype, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindFileHandlersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(extension: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindFileHandlersAsync)(::core::mem::transmute_copy(this), extension.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::IStorageFolder>>(folder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFolderAsync)(::core::mem::transmute_copy(this), folder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, FolderLauncherOptions>>(folder: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFolderWithOptionsAsync)(::core::mem::transmute_copy(this), folder.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QueryAppUriSupportAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportWithPackageFamilyNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uri: Param0, packagefamilyname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QueryAppUriSupportWithPackageFamilyNameAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindAppUriHandlersAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAppUriHandlersAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(user: Param0, uri: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, LauncherOptions>>(user: Param0, uri: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriWithOptionsForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriWithDataForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, LauncherOptions>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(user: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriWithDataForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, LauncherOptions>>(user: Param0, uri: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriForResultsForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriForResultsWithDataForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, LauncherOptions>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(user: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriForResultsWithDataForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(path: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFolderPathAsync)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, FolderLauncherOptions>>(path: Param0, options: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFolderPathWithOptionsAsync)(::core::mem::transmute_copy(this), path.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, path: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFolderPathForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, FolderLauncherOptions>>(user: Param0, path: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFolderPathWithOptionsForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILauncherStatics<R, F: FnOnce(&ILauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ILauncherStatics2<R, F: FnOnce(&ILauncherStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ILauncherStatics3<R, F: FnOnce(&ILauncherStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ILauncherStatics4<R, F: FnOnce(&ILauncherStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ILauncherStatics5<R, F: FnOnce(&ILauncherStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Launcher, ILauncherStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for Launcher {
    const NAME: &'static str = "Windows.System.Launcher";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LauncherOptions(::windows::core::IUnknown);
impl LauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LauncherOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TreatAsUntrusted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TreatAsUntrusted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetTreatAsUntrusted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTreatAsUntrusted)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn DisplayApplicationPicker(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayApplicationPicker)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetDisplayApplicationPicker(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayApplicationPicker)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn UI(&self) -> ::windows::core::Result<LauncherUIOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UI)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LauncherUIOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn PreferredApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreferredApplicationPackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetPreferredApplicationPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferredApplicationPackageFamilyName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn PreferredApplicationDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreferredApplicationDisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetPreferredApplicationDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferredApplicationDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FallbackUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFallbackUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetContentType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentType)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TargetApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetApplicationPackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetTargetApplicationPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetApplicationPackageFamilyName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringFilesQuery)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn SetNeighboringFilesQuery<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::Search::StorageFileQueryResult>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNeighboringFilesQuery)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn IgnoreAppUriHandlers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IgnoreAppUriHandlers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetIgnoreAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIgnoreAppUriHandlers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn LimitPickerToCurrentAppAndAppUriHandlers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LimitPickerToCurrentAppAndAppUriHandlers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetLimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLimitPickerToCurrentAppAndAppUriHandlers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__: super::UI::ViewManagement::ViewSizePreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredRemainingView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredRemainingView)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for LauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LauncherOptions {}
impl ::core::fmt::Debug for LauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.LauncherOptions;{bafa21d8-b071-4cd8-853e-341203e557d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LauncherOptions {
    type Vtable = ILauncherOptions_Vtbl;
    const IID: ::windows::core::GUID = <ILauncherOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LauncherOptions {
    const NAME: &'static str = "Windows.System.LauncherOptions";
}
impl ::core::convert::From<LauncherOptions> for ::windows::core::IUnknown {
    fn from(value: LauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LauncherOptions> for ::windows::core::IUnknown {
    fn from(value: &LauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LauncherOptions> for ::windows::core::IInspectable {
    fn from(value: LauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LauncherOptions> for ::windows::core::IInspectable {
    fn from(value: &LauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LauncherUIOptions(::windows::core::IUnknown);
impl LauncherUIOptions {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InvocationPoint(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InvocationPoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::Point>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInvocationPoint<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::Point>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInvocationPoint)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectionRect(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectionRect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::Rect>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSelectionRect<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::Rect>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSelectionRect)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn PreferredPlacement(&self) -> ::windows::core::Result<super::UI::Popups::Placement> {
        let this = self;
        unsafe {
            let mut result__: super::UI::Popups::Placement = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreferredPlacement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UI::Popups::Placement>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetPreferredPlacement(&self, value: super::UI::Popups::Placement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferredPlacement)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for LauncherUIOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LauncherUIOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LauncherUIOptions {}
impl ::core::fmt::Debug for LauncherUIOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LauncherUIOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LauncherUIOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.LauncherUIOptions;{1b25da6e-8aa6-41e9-8251-4165f5985f49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LauncherUIOptions {
    type Vtable = ILauncherUIOptions_Vtbl;
    const IID: ::windows::core::GUID = <ILauncherUIOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LauncherUIOptions {
    const NAME: &'static str = "Windows.System.LauncherUIOptions";
}
impl ::core::convert::From<LauncherUIOptions> for ::windows::core::IUnknown {
    fn from(value: LauncherUIOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LauncherUIOptions> for ::windows::core::IUnknown {
    fn from(value: &LauncherUIOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LauncherUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LauncherUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LauncherUIOptions> for ::windows::core::IInspectable {
    fn from(value: LauncherUIOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LauncherUIOptions> for ::windows::core::IInspectable {
    fn from(value: &LauncherUIOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LauncherUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LauncherUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LauncherUIOptions {}
unsafe impl ::core::marker::Sync for LauncherUIOptions {}
#[doc = "*Required features: `\"System\"`*"]
pub struct MemoryManager {}
impl MemoryManager {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AppMemoryUsage() -> ::windows::core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppMemoryUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AppMemoryUsageLimit() -> ::windows::core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppMemoryUsageLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AppMemoryUsageLevel() -> ::windows::core::Result<AppMemoryUsageLevel> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: AppMemoryUsageLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppMemoryUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppMemoryUsageLevel>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppMemoryUsageIncreased)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveAppMemoryUsageIncreased)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageDecreased<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppMemoryUsageDecreased)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageDecreased<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveAppMemoryUsageDecreased)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageLimitChanging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppMemoryUsageLimitChanging)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageLimitChanging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveAppMemoryUsageLimitChanging)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetAppMemoryReport() -> ::windows::core::Result<AppMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppMemoryReport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppMemoryReport>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetProcessMemoryReport() -> ::windows::core::Result<ProcessMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetProcessMemoryReport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProcessMemoryReport>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TrySetAppMemoryUsageLimit(value: u64) -> ::windows::core::Result<bool> {
        Self::IMemoryManagerStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetAppMemoryUsageLimit)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn ExpectedAppMemoryUsageLimit() -> ::windows::core::Result<u64> {
        Self::IMemoryManagerStatics4(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExpectedAppMemoryUsageLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics<R, F: FnOnce(&IMemoryManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics2<R, F: FnOnce(&IMemoryManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics3<R, F: FnOnce(&IMemoryManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics4<R, F: FnOnce(&IMemoryManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MemoryManager, IMemoryManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MemoryManager {
    const NAME: &'static str = "Windows.System.MemoryManager";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PowerState(pub i32);
impl PowerState {
    pub const ConnectedStandby: Self = Self(0i32);
    pub const SleepS3: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerState {}
impl ::core::clone::Clone for PowerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PowerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.PowerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
pub struct ProcessLauncher {}
impl ProcessLauncher {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(filename: Param0, args: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RunToCompletionAsync)(::core::mem::transmute_copy(this), filename.into_param().abi(), args.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<ProcessLauncherResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsyncWithOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProcessLauncherOptions>>(filename: Param0, args: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RunToCompletionAsyncWithOptions)(::core::mem::transmute_copy(this), filename.into_param().abi(), args.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<ProcessLauncherResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProcessLauncherStatics<R, F: FnOnce(&IProcessLauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProcessLauncher, IProcessLauncherStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ProcessLauncher {
    const NAME: &'static str = "Windows.System.ProcessLauncher";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ProcessLauncherOptions(::windows::core::IUnknown);
impl ProcessLauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProcessLauncherOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"System\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardInput(&self) -> ::windows::core::Result<super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StandardInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardInput<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::Streams::IInputStream>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStandardInput)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardOutput(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StandardOutput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardOutput<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::Streams::IOutputStream>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStandardOutput)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardError(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StandardError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardError<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::Streams::IOutputStream>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStandardError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn WorkingDirectory(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WorkingDirectory)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetWorkingDirectory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWorkingDirectory)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ProcessLauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessLauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessLauncherOptions {}
impl ::core::fmt::Debug for ProcessLauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessLauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessLauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherOptions;{3080b9cf-f444-4a83-beaf-a549a0f3229c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_Vtbl;
    const IID: ::windows::core::GUID = <IProcessLauncherOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProcessLauncherOptions {
    const NAME: &'static str = "Windows.System.ProcessLauncherOptions";
}
impl ::core::convert::From<ProcessLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: ProcessLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: &ProcessLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: ProcessLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: &ProcessLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessLauncherOptions {}
unsafe impl ::core::marker::Sync for ProcessLauncherOptions {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ProcessLauncherResult(::windows::core::IUnknown);
impl ProcessLauncherResult {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn ExitCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessLauncherResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessLauncherResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessLauncherResult {}
impl ::core::fmt::Debug for ProcessLauncherResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessLauncherResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessLauncherResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherResult;{544c8934-86d8-4991-8e75-ece8a43b6b6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProcessLauncherResult {
    type Vtable = IProcessLauncherResult_Vtbl;
    const IID: ::windows::core::GUID = <IProcessLauncherResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProcessLauncherResult {
    const NAME: &'static str = "Windows.System.ProcessLauncherResult";
}
impl ::core::convert::From<ProcessLauncherResult> for ::windows::core::IUnknown {
    fn from(value: ProcessLauncherResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessLauncherResult> for ::windows::core::IUnknown {
    fn from(value: &ProcessLauncherResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessLauncherResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessLauncherResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessLauncherResult> for ::windows::core::IInspectable {
    fn from(value: ProcessLauncherResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessLauncherResult> for ::windows::core::IInspectable {
    fn from(value: &ProcessLauncherResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessLauncherResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessLauncherResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessLauncherResult {}
unsafe impl ::core::marker::Sync for ProcessLauncherResult {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ProcessMemoryReport(::windows::core::IUnknown);
impl ProcessMemoryReport {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn PrivateWorkingSetUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrivateWorkingSetUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn TotalWorkingSetUsage(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TotalWorkingSetUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessMemoryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessMemoryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessMemoryReport {}
impl ::core::fmt::Debug for ProcessMemoryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessMemoryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessMemoryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessMemoryReport;{087305a8-9b70-4782-8741-3a982b6ce5e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProcessMemoryReport {
    type Vtable = IProcessMemoryReport_Vtbl;
    const IID: ::windows::core::GUID = <IProcessMemoryReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProcessMemoryReport {
    const NAME: &'static str = "Windows.System.ProcessMemoryReport";
}
impl ::core::convert::From<ProcessMemoryReport> for ::windows::core::IUnknown {
    fn from(value: ProcessMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessMemoryReport> for ::windows::core::IUnknown {
    fn from(value: &ProcessMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessMemoryReport> for ::windows::core::IInspectable {
    fn from(value: ProcessMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessMemoryReport> for ::windows::core::IInspectable {
    fn from(value: &ProcessMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessMemoryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessMemoryReport {}
unsafe impl ::core::marker::Sync for ProcessMemoryReport {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProcessorArchitecture(pub i32);
impl ProcessorArchitecture {
    pub const X86: Self = Self(0i32);
    pub const Arm: Self = Self(5i32);
    pub const X64: Self = Self(9i32);
    pub const Neutral: Self = Self(11i32);
    pub const Arm64: Self = Self(12i32);
    pub const X86OnArm64: Self = Self(14i32);
    pub const Unknown: Self = Self(65535i32);
}
impl ::core::marker::Copy for ProcessorArchitecture {}
impl ::core::clone::Clone for ProcessorArchitecture {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProcessorArchitecture {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProcessorArchitecture {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProcessorArchitecture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessorArchitecture").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessorArchitecture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.ProcessorArchitecture;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ProtocolForResultsOperation(::windows::core::IUnknown);
impl ProtocolForResultsOperation {
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ProtocolForResultsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtocolForResultsOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolForResultsOperation {}
impl ::core::fmt::Debug for ProtocolForResultsOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtocolForResultsOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtocolForResultsOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.ProtocolForResultsOperation;{d581293a-6de9-4d28-9378-f86782e182bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_Vtbl;
    const IID: ::windows::core::GUID = <IProtocolForResultsOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtocolForResultsOperation {
    const NAME: &'static str = "Windows.System.ProtocolForResultsOperation";
}
impl ::core::convert::From<ProtocolForResultsOperation> for ::windows::core::IUnknown {
    fn from(value: ProtocolForResultsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsOperation> for ::windows::core::IUnknown {
    fn from(value: &ProtocolForResultsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtocolForResultsOperation> for ::windows::core::IInspectable {
    fn from(value: ProtocolForResultsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsOperation> for ::windows::core::IInspectable {
    fn from(value: &ProtocolForResultsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtocolForResultsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtocolForResultsOperation {}
unsafe impl ::core::marker::Sync for ProtocolForResultsOperation {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteLaunchUriStatus(pub i32);
impl RemoteLaunchUriStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const ProtocolUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const ValueSetTooLarge: Self = Self(5i32);
    pub const DeniedByLocalSystem: Self = Self(6i32);
    pub const DeniedByRemoteSystem: Self = Self(7i32);
}
impl ::core::marker::Copy for RemoteLaunchUriStatus {}
impl ::core::clone::Clone for RemoteLaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteLaunchUriStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteLaunchUriStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteLaunchUriStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteLaunchUriStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteLaunchUriStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteLaunchUriStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
pub struct RemoteLauncher {}
impl RemoteLauncher {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`, `\"System_RemoteSystems\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriAsync<'a, Param0: ::windows::core::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(remotesystemconnectionrequest: Param0, uri: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriAsync)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`, `\"System_RemoteSystems\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, RemoteLauncherOptions>>(remotesystemconnectionrequest: Param0, uri: Param1, options: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriWithOptionsAsync)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`, `\"System_RemoteSystems\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithDataAsync<'a, Param0: ::windows::core::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, RemoteLauncherOptions>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(remotesystemconnectionrequest: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchUriWithDataAsync)(::core::mem::transmute_copy(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteLauncherStatics<R, F: FnOnce(&IRemoteLauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RemoteLauncher, IRemoteLauncherStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for RemoteLauncher {
    const NAME: &'static str = "Windows.System.RemoteLauncher";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct RemoteLauncherOptions(::windows::core::IUnknown);
impl RemoteLauncherOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RemoteLauncherOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FallbackUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFallbackUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferredAppIds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreferredAppIds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteLauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteLauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteLauncherOptions {}
impl ::core::fmt::Debug for RemoteLauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteLauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteLauncherOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteLauncherOptions;{9e3a2788-2891-4cdf-a2d6-9dff7d02e693})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_Vtbl;
    const IID: ::windows::core::GUID = <IRemoteLauncherOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteLauncherOptions {
    const NAME: &'static str = "Windows.System.RemoteLauncherOptions";
}
impl ::core::convert::From<RemoteLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: RemoteLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteLauncherOptions> for ::windows::core::IUnknown {
    fn from(value: &RemoteLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RemoteLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RemoteLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: RemoteLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteLauncherOptions> for ::windows::core::IInspectable {
    fn from(value: &RemoteLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RemoteLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RemoteLauncherOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteLauncherOptions {}
unsafe impl ::core::marker::Sync for RemoteLauncherOptions {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ShutdownKind(pub i32);
impl ShutdownKind {
    pub const Shutdown: Self = Self(0i32);
    pub const Restart: Self = Self(1i32);
}
impl ::core::marker::Copy for ShutdownKind {}
impl ::core::clone::Clone for ShutdownKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShutdownKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ShutdownKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ShutdownKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShutdownKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShutdownKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.ShutdownKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
pub struct ShutdownManager {}
impl ShutdownManager {
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginShutdown<'a, Param1: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(shutdownkind: ShutdownKind, timeout: Param1) -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).BeginShutdown)(::core::mem::transmute_copy(this), shutdownkind, timeout.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CancelShutdown() -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).CancelShutdown)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn IsPowerStateSupported(powerstate: PowerState) -> ::windows::core::Result<bool> {
        Self::IShutdownManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPowerStateSupported)(::core::mem::transmute_copy(this), powerstate, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn EnterPowerState(powerstate: PowerState) -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).EnterPowerState)(::core::mem::transmute_copy(this), powerstate).ok() })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnterPowerStateWithTimeSpan<'a, Param1: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(powerstate: PowerState, wakeupafter: Param1) -> ::windows::core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).EnterPowerStateWithTimeSpan)(::core::mem::transmute_copy(this), powerstate, wakeupafter.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IShutdownManagerStatics<R, F: FnOnce(&IShutdownManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ShutdownManager, IShutdownManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IShutdownManagerStatics2<R, F: FnOnce(&IShutdownManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ShutdownManager, IShutdownManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ShutdownManager {
    const NAME: &'static str = "Windows.System.ShutdownManager";
}
#[doc = "*Required features: `\"System\"`*"]
pub struct TimeZoneSettings {}
impl TimeZoneSettings {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CurrentTimeZoneDisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentTimeZoneDisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTimeZoneDisplayNames() -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedTimeZoneDisplayNames)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CanChangeTimeZone() -> ::windows::core::Result<bool> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanChangeTimeZone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn ChangeTimeZoneByDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(timezonedisplayname: Param0) -> ::windows::core::Result<()> {
        Self::ITimeZoneSettingsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ChangeTimeZoneByDisplayName)(::core::mem::transmute_copy(this), timezonedisplayname.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoUpdateTimeZoneAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(timeout: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>> {
        Self::ITimeZoneSettingsStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoUpdateTimeZoneAsync)(::core::mem::transmute_copy(this), timeout.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimeZoneSettingsStatics<R, F: FnOnce(&ITimeZoneSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITimeZoneSettingsStatics2<R, F: FnOnce(&ITimeZoneSettingsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for TimeZoneSettings {
    const NAME: &'static str = "Windows.System.TimeZoneSettings";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct User(::windows::core::IUnknown);
impl User {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NonRoamableId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AuthenticationStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: UserAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<UserType> {
        let this = self;
        unsafe {
            let mut result__: UserType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserType>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPropertyAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(&self, values: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPropertiesAsync)(::core::mem::transmute_copy(this), values.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPictureAsync(&self, desiredsize: UserPictureSize) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPictureAsync)(::core::mem::transmute_copy(this), desiredsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckUserAgeConsentGroupAsync(&self, consentgroup: UserAgeConsentGroup) -> ::windows::core::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>> {
        let this = &::windows::core::Interface::cast::<IUser2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckUserAgeConsentGroupAsync)(::core::mem::transmute_copy(this), consentgroup, &mut result__).from_abi::<super::Foundation::IAsyncOperation<UserAgeConsentResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CreateWatcher() -> ::windows::core::Result<UserWatcher> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserWatcher>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsyncByType(r#type: UserType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllAsyncByType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsyncByTypeAndStatus(r#type: UserType, status: UserAuthenticationStatus) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllAsyncByTypeAndStatus)(::core::mem::transmute_copy(this), r#type, status, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(nonroamableid: Param0) -> ::windows::core::Result<User> {
        Self::IUserStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFromId)(::core::mem::transmute_copy(this), nonroamableid.into_param().abi(), &mut result__).from_abi::<User>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<User> {
        Self::IUserStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserStatics<R, F: FnOnce(&IUserStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<User, IUserStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IUserStatics2<R, F: FnOnce(&IUserStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<User, IUserStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for User {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for User {}
impl ::core::fmt::Debug for User {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("User").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for User {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.User;{df9a26c6-e746-4bcd-b5d4-120103c4209b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for User {
    type Vtable = IUser_Vtbl;
    const IID: ::windows::core::GUID = <IUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for User {
    const NAME: &'static str = "Windows.System.User";
}
impl ::core::convert::From<User> for ::windows::core::IUnknown {
    fn from(value: User) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&User> for ::windows::core::IUnknown {
    fn from(value: &User) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for User {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a User {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<User> for ::windows::core::IInspectable {
    fn from(value: User) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&User> for ::windows::core::IInspectable {
    fn from(value: &User) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for User {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a User {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for User {}
unsafe impl ::core::marker::Sync for User {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserAgeConsentGroup(pub i32);
impl UserAgeConsentGroup {
    pub const Child: Self = Self(0i32);
    pub const Minor: Self = Self(1i32);
    pub const Adult: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAgeConsentGroup {}
impl ::core::clone::Clone for UserAgeConsentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAgeConsentGroup {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserAgeConsentGroup {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserAgeConsentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAgeConsentGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAgeConsentGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentGroup;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserAgeConsentResult(pub i32);
impl UserAgeConsentResult {
    pub const NotEnforced: Self = Self(0i32);
    pub const Included: Self = Self(1i32);
    pub const NotIncluded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const Ambiguous: Self = Self(4i32);
}
impl ::core::marker::Copy for UserAgeConsentResult {}
impl ::core::clone::Clone for UserAgeConsentResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAgeConsentResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserAgeConsentResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserAgeConsentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAgeConsentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAgeConsentResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserAuthenticationStatus(pub i32);
impl UserAuthenticationStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const LocallyAuthenticated: Self = Self(1i32);
    pub const RemotelyAuthenticated: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAuthenticationStatus {}
impl ::core::clone::Clone for UserAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAuthenticationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserAuthenticationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserAuthenticationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserAuthenticationStatusChangeDeferral(::windows::core::IUnknown);
impl UserAuthenticationStatusChangeDeferral {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for UserAuthenticationStatusChangeDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserAuthenticationStatusChangeDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserAuthenticationStatusChangeDeferral {}
impl ::core::fmt::Debug for UserAuthenticationStatusChangeDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatusChangeDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAuthenticationStatusChangeDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangeDeferral;{88b59568-bb30-42fb-a270-e9902e40efa7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IUserAuthenticationStatusChangeDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserAuthenticationStatusChangeDeferral {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangeDeferral";
}
impl ::core::convert::From<UserAuthenticationStatusChangeDeferral> for ::windows::core::IUnknown {
    fn from(value: UserAuthenticationStatusChangeDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangeDeferral> for ::windows::core::IUnknown {
    fn from(value: &UserAuthenticationStatusChangeDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserAuthenticationStatusChangeDeferral> for ::windows::core::IInspectable {
    fn from(value: UserAuthenticationStatusChangeDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangeDeferral> for ::windows::core::IInspectable {
    fn from(value: &UserAuthenticationStatusChangeDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangeDeferral {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangeDeferral {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserAuthenticationStatusChangingEventArgs(::windows::core::IUnknown);
impl UserAuthenticationStatusChangingEventArgs {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<UserAuthenticationStatusChangeDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatusChangeDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn NewStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: UserAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn CurrentStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: UserAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for UserAuthenticationStatusChangingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserAuthenticationStatusChangingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserAuthenticationStatusChangingEventArgs {}
impl ::core::fmt::Debug for UserAuthenticationStatusChangingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatusChangingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserAuthenticationStatusChangingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangingEventArgs;{8c030f28-a711-4c1e-ab48-04179c15938f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUserAuthenticationStatusChangingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserAuthenticationStatusChangingEventArgs {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangingEventArgs";
}
impl ::core::convert::From<UserAuthenticationStatusChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserAuthenticationStatusChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserAuthenticationStatusChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserAuthenticationStatusChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserAuthenticationStatusChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserAuthenticationStatusChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangingEventArgs {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangingEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserChangedEventArgs(::windows::core::IUnknown);
impl UserChangedEventArgs {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn User(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ChangedPropertyKinds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>> {
        let this = &::windows::core::Interface::cast::<IUserChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangedPropertyKinds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserChangedEventArgs {}
impl ::core::fmt::Debug for UserChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserChangedEventArgs;{086459dc-18c6-48db-bc99-724fb9203ccc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUserChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserChangedEventArgs";
}
impl ::core::convert::From<UserChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserChangedEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
pub struct UserDeviceAssociation {}
impl UserDeviceAssociation {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn FindUserFromDeviceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<User> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindUserFromDeviceId)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<User>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserDeviceAssociationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserDeviceAssociationChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserDeviceAssociationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IUserDeviceAssociationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveUserDeviceAssociationChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IUserDeviceAssociationStatics<R, F: FnOnce(&IUserDeviceAssociationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDeviceAssociation, IUserDeviceAssociationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for UserDeviceAssociation {
    const NAME: &'static str = "Windows.System.UserDeviceAssociation";
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserDeviceAssociationChangedEventArgs(::windows::core::IUnknown);
impl UserDeviceAssociationChangedEventArgs {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn NewUser(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn OldUser(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDeviceAssociationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDeviceAssociationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDeviceAssociationChangedEventArgs {}
impl ::core::fmt::Debug for UserDeviceAssociationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDeviceAssociationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDeviceAssociationChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserDeviceAssociationChangedEventArgs;{bd1f6f6c-bb5d-4d7b-a5f0-c8cd11a38d42})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUserDeviceAssociationChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDeviceAssociationChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserDeviceAssociationChangedEventArgs";
}
impl ::core::convert::From<UserDeviceAssociationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserDeviceAssociationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDeviceAssociationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserDeviceAssociationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDeviceAssociationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserDeviceAssociationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDeviceAssociationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserDeviceAssociationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDeviceAssociationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDeviceAssociationChangedEventArgs {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserPicker(::windows::core::IUnknown);
impl UserPicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserPicker, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn AllowGuestAccounts(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowGuestAccounts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetAllowGuestAccounts(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowGuestAccounts)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SuggestedSelectedUser(&self) -> ::windows::core::Result<User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SuggestedSelectedUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<User>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn SetSuggestedSelectedUser<'a, Param0: ::windows::core::IntoParam<'a, User>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSuggestedSelectedUser)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleUserAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<User>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PickSingleUserAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<User>>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IUserPickerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserPickerStatics<R, F: FnOnce(&IUserPickerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserPicker, IUserPickerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserPicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserPicker {}
impl ::core::fmt::Debug for UserPicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPicker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserPicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserPicker;{7d548008-f1e3-4a6c-8ddc-a9bb0f488aed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserPicker {
    type Vtable = IUserPicker_Vtbl;
    const IID: ::windows::core::GUID = <IUserPicker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserPicker {
    const NAME: &'static str = "Windows.System.UserPicker";
}
impl ::core::convert::From<UserPicker> for ::windows::core::IUnknown {
    fn from(value: UserPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserPicker> for ::windows::core::IUnknown {
    fn from(value: &UserPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserPicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserPicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserPicker> for ::windows::core::IInspectable {
    fn from(value: UserPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserPicker> for ::windows::core::IInspectable {
    fn from(value: &UserPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserPicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserPicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserPicker {}
unsafe impl ::core::marker::Sync for UserPicker {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserPictureSize(pub i32);
impl UserPictureSize {
    pub const Size64x64: Self = Self(0i32);
    pub const Size208x208: Self = Self(1i32);
    pub const Size424x424: Self = Self(2i32);
    pub const Size1080x1080: Self = Self(3i32);
}
impl ::core::marker::Copy for UserPictureSize {}
impl ::core::clone::Clone for UserPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserPictureSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserPictureSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserPictureSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPictureSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserPictureSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserPictureSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserType(pub i32);
impl UserType {
    pub const LocalUser: Self = Self(0i32);
    pub const RemoteUser: Self = Self(1i32);
    pub const LocalGuest: Self = Self(2i32);
    pub const RemoteGuest: Self = Self(3i32);
    pub const SystemManaged: Self = Self(4i32);
}
impl ::core::marker::Copy for UserType {}
impl ::core::clone::Clone for UserType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserWatcher(::windows::core::IUnknown);
impl UserWatcher {
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<UserWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: UserWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Added)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Removed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemoved)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Updated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAuthenticationStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationStatusChanging)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAuthenticationStatusChanging)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserWatcher {}
impl ::core::fmt::Debug for UserWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserWatcher;{155eb23b-242a-45e0-a2e9-3171fc6a7fbb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserWatcher {
    type Vtable = IUserWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IUserWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserWatcher {
    const NAME: &'static str = "Windows.System.UserWatcher";
}
impl ::core::convert::From<UserWatcher> for ::windows::core::IUnknown {
    fn from(value: UserWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserWatcher> for ::windows::core::IUnknown {
    fn from(value: &UserWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserWatcher> for ::windows::core::IInspectable {
    fn from(value: UserWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserWatcher> for ::windows::core::IInspectable {
    fn from(value: &UserWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserWatcher {}
unsafe impl ::core::marker::Sync for UserWatcher {}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserWatcherStatus(pub i32);
impl UserWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for UserWatcherStatus {}
impl ::core::clone::Clone for UserWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserWatcherUpdateKind(pub i32);
impl UserWatcherUpdateKind {
    pub const Properties: Self = Self(0i32);
    pub const Picture: Self = Self(1i32);
}
impl ::core::marker::Copy for UserWatcherUpdateKind {}
impl ::core::clone::Clone for UserWatcherUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserWatcherUpdateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserWatcherUpdateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserWatcherUpdateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcherUpdateKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserWatcherUpdateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherUpdateKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: Self = Self(0i32);
    pub const LeftButton: Self = Self(1i32);
    pub const RightButton: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const MiddleButton: Self = Self(4i32);
    pub const XButton1: Self = Self(5i32);
    pub const XButton2: Self = Self(6i32);
    pub const Back: Self = Self(8i32);
    pub const Tab: Self = Self(9i32);
    pub const Clear: Self = Self(12i32);
    pub const Enter: Self = Self(13i32);
    pub const Shift: Self = Self(16i32);
    pub const Control: Self = Self(17i32);
    pub const Menu: Self = Self(18i32);
    pub const Pause: Self = Self(19i32);
    pub const CapitalLock: Self = Self(20i32);
    pub const Kana: Self = Self(21i32);
    pub const Hangul: Self = Self(21i32);
    pub const ImeOn: Self = Self(22i32);
    pub const Junja: Self = Self(23i32);
    pub const Final: Self = Self(24i32);
    pub const Hanja: Self = Self(25i32);
    pub const Kanji: Self = Self(25i32);
    pub const ImeOff: Self = Self(26i32);
    pub const Escape: Self = Self(27i32);
    pub const Convert: Self = Self(28i32);
    pub const NonConvert: Self = Self(29i32);
    pub const Accept: Self = Self(30i32);
    pub const ModeChange: Self = Self(31i32);
    pub const Space: Self = Self(32i32);
    pub const PageUp: Self = Self(33i32);
    pub const PageDown: Self = Self(34i32);
    pub const End: Self = Self(35i32);
    pub const Home: Self = Self(36i32);
    pub const Left: Self = Self(37i32);
    pub const Up: Self = Self(38i32);
    pub const Right: Self = Self(39i32);
    pub const Down: Self = Self(40i32);
    pub const Select: Self = Self(41i32);
    pub const Print: Self = Self(42i32);
    pub const Execute: Self = Self(43i32);
    pub const Snapshot: Self = Self(44i32);
    pub const Insert: Self = Self(45i32);
    pub const Delete: Self = Self(46i32);
    pub const Help: Self = Self(47i32);
    pub const Number0: Self = Self(48i32);
    pub const Number1: Self = Self(49i32);
    pub const Number2: Self = Self(50i32);
    pub const Number3: Self = Self(51i32);
    pub const Number4: Self = Self(52i32);
    pub const Number5: Self = Self(53i32);
    pub const Number6: Self = Self(54i32);
    pub const Number7: Self = Self(55i32);
    pub const Number8: Self = Self(56i32);
    pub const Number9: Self = Self(57i32);
    pub const A: Self = Self(65i32);
    pub const B: Self = Self(66i32);
    pub const C: Self = Self(67i32);
    pub const D: Self = Self(68i32);
    pub const E: Self = Self(69i32);
    pub const F: Self = Self(70i32);
    pub const G: Self = Self(71i32);
    pub const H: Self = Self(72i32);
    pub const I: Self = Self(73i32);
    pub const J: Self = Self(74i32);
    pub const K: Self = Self(75i32);
    pub const L: Self = Self(76i32);
    pub const M: Self = Self(77i32);
    pub const N: Self = Self(78i32);
    pub const O: Self = Self(79i32);
    pub const P: Self = Self(80i32);
    pub const Q: Self = Self(81i32);
    pub const R: Self = Self(82i32);
    pub const S: Self = Self(83i32);
    pub const T: Self = Self(84i32);
    pub const U: Self = Self(85i32);
    pub const V: Self = Self(86i32);
    pub const W: Self = Self(87i32);
    pub const X: Self = Self(88i32);
    pub const Y: Self = Self(89i32);
    pub const Z: Self = Self(90i32);
    pub const LeftWindows: Self = Self(91i32);
    pub const RightWindows: Self = Self(92i32);
    pub const Application: Self = Self(93i32);
    pub const Sleep: Self = Self(95i32);
    pub const NumberPad0: Self = Self(96i32);
    pub const NumberPad1: Self = Self(97i32);
    pub const NumberPad2: Self = Self(98i32);
    pub const NumberPad3: Self = Self(99i32);
    pub const NumberPad4: Self = Self(100i32);
    pub const NumberPad5: Self = Self(101i32);
    pub const NumberPad6: Self = Self(102i32);
    pub const NumberPad7: Self = Self(103i32);
    pub const NumberPad8: Self = Self(104i32);
    pub const NumberPad9: Self = Self(105i32);
    pub const Multiply: Self = Self(106i32);
    pub const Add: Self = Self(107i32);
    pub const Separator: Self = Self(108i32);
    pub const Subtract: Self = Self(109i32);
    pub const Decimal: Self = Self(110i32);
    pub const Divide: Self = Self(111i32);
    pub const F1: Self = Self(112i32);
    pub const F2: Self = Self(113i32);
    pub const F3: Self = Self(114i32);
    pub const F4: Self = Self(115i32);
    pub const F5: Self = Self(116i32);
    pub const F6: Self = Self(117i32);
    pub const F7: Self = Self(118i32);
    pub const F8: Self = Self(119i32);
    pub const F9: Self = Self(120i32);
    pub const F10: Self = Self(121i32);
    pub const F11: Self = Self(122i32);
    pub const F12: Self = Self(123i32);
    pub const F13: Self = Self(124i32);
    pub const F14: Self = Self(125i32);
    pub const F15: Self = Self(126i32);
    pub const F16: Self = Self(127i32);
    pub const F17: Self = Self(128i32);
    pub const F18: Self = Self(129i32);
    pub const F19: Self = Self(130i32);
    pub const F20: Self = Self(131i32);
    pub const F21: Self = Self(132i32);
    pub const F22: Self = Self(133i32);
    pub const F23: Self = Self(134i32);
    pub const F24: Self = Self(135i32);
    pub const NavigationView: Self = Self(136i32);
    pub const NavigationMenu: Self = Self(137i32);
    pub const NavigationUp: Self = Self(138i32);
    pub const NavigationDown: Self = Self(139i32);
    pub const NavigationLeft: Self = Self(140i32);
    pub const NavigationRight: Self = Self(141i32);
    pub const NavigationAccept: Self = Self(142i32);
    pub const NavigationCancel: Self = Self(143i32);
    pub const NumberKeyLock: Self = Self(144i32);
    pub const Scroll: Self = Self(145i32);
    pub const LeftShift: Self = Self(160i32);
    pub const RightShift: Self = Self(161i32);
    pub const LeftControl: Self = Self(162i32);
    pub const RightControl: Self = Self(163i32);
    pub const LeftMenu: Self = Self(164i32);
    pub const RightMenu: Self = Self(165i32);
    pub const GoBack: Self = Self(166i32);
    pub const GoForward: Self = Self(167i32);
    pub const Refresh: Self = Self(168i32);
    pub const Stop: Self = Self(169i32);
    pub const Search: Self = Self(170i32);
    pub const Favorites: Self = Self(171i32);
    pub const GoHome: Self = Self(172i32);
    pub const GamepadA: Self = Self(195i32);
    pub const GamepadB: Self = Self(196i32);
    pub const GamepadX: Self = Self(197i32);
    pub const GamepadY: Self = Self(198i32);
    pub const GamepadRightShoulder: Self = Self(199i32);
    pub const GamepadLeftShoulder: Self = Self(200i32);
    pub const GamepadLeftTrigger: Self = Self(201i32);
    pub const GamepadRightTrigger: Self = Self(202i32);
    pub const GamepadDPadUp: Self = Self(203i32);
    pub const GamepadDPadDown: Self = Self(204i32);
    pub const GamepadDPadLeft: Self = Self(205i32);
    pub const GamepadDPadRight: Self = Self(206i32);
    pub const GamepadMenu: Self = Self(207i32);
    pub const GamepadView: Self = Self(208i32);
    pub const GamepadLeftThumbstickButton: Self = Self(209i32);
    pub const GamepadRightThumbstickButton: Self = Self(210i32);
    pub const GamepadLeftThumbstickUp: Self = Self(211i32);
    pub const GamepadLeftThumbstickDown: Self = Self(212i32);
    pub const GamepadLeftThumbstickRight: Self = Self(213i32);
    pub const GamepadLeftThumbstickLeft: Self = Self(214i32);
    pub const GamepadRightThumbstickUp: Self = Self(215i32);
    pub const GamepadRightThumbstickDown: Self = Self(216i32);
    pub const GamepadRightThumbstickRight: Self = Self(217i32);
    pub const GamepadRightThumbstickLeft: Self = Self(218i32);
}
impl ::core::marker::Copy for VirtualKey {}
impl ::core::clone::Clone for VirtualKey {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VirtualKey {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VirtualKey {
    type Abi = Self;
}
impl ::core::fmt::Debug for VirtualKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualKey").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VirtualKey {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKey;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VirtualKeyModifiers(pub u32);
impl VirtualKeyModifiers {
    pub const None: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Menu: Self = Self(2u32);
    pub const Shift: Self = Self(4u32);
    pub const Windows: Self = Self(8u32);
}
impl ::core::marker::Copy for VirtualKeyModifiers {}
impl ::core::clone::Clone for VirtualKeyModifiers {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VirtualKeyModifiers {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VirtualKeyModifiers {
    type Abi = Self;
}
impl ::core::fmt::Debug for VirtualKeyModifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualKeyModifiers").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VirtualKeyModifiers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VirtualKeyModifiers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VirtualKeyModifiers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VirtualKeyModifiers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VirtualKeyModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for VirtualKeyModifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKeyModifiers;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
