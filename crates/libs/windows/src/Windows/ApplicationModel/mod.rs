#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_Activation")]
pub mod Activation;
#[cfg(feature = "ApplicationModel_AppExtensions")]
pub mod AppExtensions;
#[cfg(feature = "ApplicationModel_AppService")]
pub mod AppService;
#[cfg(feature = "ApplicationModel_Appointments")]
pub mod Appointments;
#[cfg(feature = "ApplicationModel_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls")]
pub mod Calls;
#[cfg(feature = "ApplicationModel_Chat")]
pub mod Chat;
#[cfg(feature = "ApplicationModel_CommunicationBlocking")]
pub mod CommunicationBlocking;
#[cfg(feature = "ApplicationModel_Contacts")]
pub mod Contacts;
#[cfg(feature = "ApplicationModel_ConversationalAgent")]
pub mod ConversationalAgent;
#[cfg(feature = "ApplicationModel_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_DataTransfer")]
pub mod DataTransfer;
#[cfg(feature = "ApplicationModel_Email")]
pub mod Email;
#[cfg(feature = "ApplicationModel_ExtendedExecution")]
pub mod ExtendedExecution;
#[cfg(feature = "ApplicationModel_Holographic")]
pub mod Holographic;
#[cfg(feature = "ApplicationModel_LockScreen")]
pub mod LockScreen;
#[cfg(feature = "ApplicationModel_Payments")]
pub mod Payments;
#[cfg(feature = "ApplicationModel_Preview")]
pub mod Preview;
#[cfg(feature = "ApplicationModel_Resources")]
pub mod Resources;
#[cfg(feature = "ApplicationModel_Search")]
pub mod Search;
#[cfg(feature = "ApplicationModel_SocialInfo")]
pub mod SocialInfo;
#[cfg(feature = "ApplicationModel_Store")]
pub mod Store;
#[cfg(feature = "ApplicationModel_UserActivities")]
pub mod UserActivities;
#[cfg(feature = "ApplicationModel_UserDataAccounts")]
pub mod UserDataAccounts;
#[cfg(feature = "ApplicationModel_UserDataTasks")]
pub mod UserDataTasks;
#[cfg(feature = "ApplicationModel_VoiceCommands")]
pub mod VoiceCommands;
#[cfg(feature = "ApplicationModel_Wallet")]
pub mod Wallet;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AddResourcePackageOptions(pub u32);
impl AddResourcePackageOptions {
    pub const None: Self = Self(0u32);
    pub const ForceTargetAppShutdown: Self = Self(1u32);
    pub const ApplyUpdateIfAvailable: Self = Self(2u32);
}
impl ::core::marker::Copy for AddResourcePackageOptions {}
impl ::core::clone::Clone for AddResourcePackageOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AddResourcePackageOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AddResourcePackageOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for AddResourcePackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddResourcePackageOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AddResourcePackageOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AddResourcePackageOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AddResourcePackageOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AddResourcePackageOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AddResourcePackageOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for AddResourcePackageOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AddResourcePackageOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppDisplayInfo(::windows::core::IUnknown);
impl AppDisplayInfo {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetLogo<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Size>>(&self, size: Param0) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetLogo)(::core::mem::transmute_copy(this), size.into_param().abi(), &mut result__).from_abi::<super::Storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
}
impl ::core::clone::Clone for AppDisplayInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDisplayInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDisplayInfo {}
impl ::core::fmt::Debug for AppDisplayInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDisplayInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDisplayInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppDisplayInfo;{1aeb1103-e4d4-41aa-a4f6-c4a276e79eac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppDisplayInfo {
    type Vtable = IAppDisplayInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAppDisplayInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppDisplayInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppDisplayInfo";
}
impl ::core::convert::From<AppDisplayInfo> for ::windows::core::IUnknown {
    fn from(value: AppDisplayInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDisplayInfo> for ::windows::core::IUnknown {
    fn from(value: &AppDisplayInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppDisplayInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppDisplayInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppDisplayInfo> for ::windows::core::IInspectable {
    fn from(value: AppDisplayInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDisplayInfo> for ::windows::core::IInspectable {
    fn from(value: &AppDisplayInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppDisplayInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppDisplayInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppDisplayInfo {}
unsafe impl ::core::marker::Sync for AppDisplayInfo {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppExecutionContext(pub i32);
impl AppExecutionContext {
    pub const Unknown: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Guest: Self = Self(2i32);
}
impl ::core::marker::Copy for AppExecutionContext {}
impl ::core::clone::Clone for AppExecutionContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppExecutionContext {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppExecutionContext {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppExecutionContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExecutionContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppExecutionContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppExecutionContext;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppInfo(::windows::core::IUnknown);
impl AppInfo {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppUserModelId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn DisplayInfo(&self) -> ::windows::core::Result<AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDisplayInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = &::windows::core::Interface::cast::<IAppInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ExecutionContext(&self) -> ::windows::core::Result<AppExecutionContext> {
        let this = &::windows::core::Interface::cast::<IAppInfo3>(self)?;
        unsafe {
            let mut result__: AppExecutionContext = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecutionContext)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppExecutionContext>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn SupportedFileExtensions(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IAppInfo4>(self)?;
        unsafe {
            let mut result__: ::windows::core::Array<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedFileExtensions)(::core::mem::transmute_copy(this), ::windows::core::Array::<::windows::core::HSTRING>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Current() -> ::windows::core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Current)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn GetFromAppUserModelId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appusermodelid: Param0) -> ::windows::core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFromAppUserModelId)(::core::mem::transmute_copy(this), appusermodelid.into_param().abi(), &mut result__).from_abi::<AppInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetFromAppUserModelIdForUser<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, appusermodelid: Param1) -> ::windows::core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFromAppUserModelIdForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), appusermodelid.into_param().abi(), &mut result__).from_abi::<AppInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppInfoStatics<R, F: FnOnce(&IAppInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInfo, IAppInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInfo {}
impl ::core::fmt::Debug for AppInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppInfo;{cf7f59b3-6a09-4de8-a6c0-5792d56880d1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppInfo {
    type Vtable = IAppInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAppInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppInfo";
}
impl ::core::convert::From<AppInfo> for ::windows::core::IUnknown {
    fn from(value: AppInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInfo> for ::windows::core::IUnknown {
    fn from(value: &AppInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInfo> for ::windows::core::IInspectable {
    fn from(value: AppInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInfo> for ::windows::core::IInspectable {
    fn from(value: &AppInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInfo {}
unsafe impl ::core::marker::Sync for AppInfo {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppInstallerInfo(::windows::core::IUnknown);
impl AppInstallerInfo {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn OnLaunch(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OnLaunch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn HoursBetweenUpdateChecks(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoursBetweenUpdateChecks)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ShowPrompt(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowPrompt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn UpdateBlocksActivation(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateBlocksActivation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn AutomaticBackgroundTask(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutomaticBackgroundTask)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsAutoRepairEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAutoRepairEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Version(&self) -> ::windows::core::Result<PackageVersion> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: PackageVersion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Version)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageVersion>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastChecked(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastChecked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PausedUntil(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PausedUntil)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateUris)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RepairUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RepairUris)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DependencyPackageUris)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OptionalPackageUris)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn PolicySource(&self) -> ::windows::core::Result<AppInstallerPolicySource> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__: AppInstallerPolicySource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PolicySource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallerPolicySource>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallerInfo {}
impl ::core::fmt::Debug for AppInstallerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallerInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppInstallerInfo;{29ab2ac0-d4f6-42a3-adcd-d6583c659508})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppInstallerInfo {
    type Vtable = IAppInstallerInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAppInstallerInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppInstallerInfo";
}
impl ::core::convert::From<AppInstallerInfo> for ::windows::core::IUnknown {
    fn from(value: AppInstallerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallerInfo> for ::windows::core::IUnknown {
    fn from(value: &AppInstallerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstallerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallerInfo> for ::windows::core::IInspectable {
    fn from(value: AppInstallerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallerInfo> for ::windows::core::IInspectable {
    fn from(value: &AppInstallerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstallerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallerInfo {}
unsafe impl ::core::marker::Sync for AppInstallerInfo {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppInstallerPolicySource(pub i32);
impl AppInstallerPolicySource {
    pub const Default: Self = Self(0i32);
    pub const System: Self = Self(1i32);
}
impl ::core::marker::Copy for AppInstallerPolicySource {}
impl ::core::clone::Clone for AppInstallerPolicySource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallerPolicySource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppInstallerPolicySource {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallerPolicySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallerPolicySource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallerPolicySource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppInstallerPolicySource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppInstance(::windows::core::IUnknown);
impl AppInstance {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Key(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Key)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsCurrentInstance(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentInstance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn RedirectActivationTo(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RedirectActivationTo)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn RecommendedInstance() -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecommendedInstance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstance>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn GetActivatedEventArgs() -> ::windows::core::Result<Activation::IActivatedEventArgs> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetActivatedEventArgs)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Activation::IActivatedEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn FindOrRegisterInstanceForKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(key: Param0) -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindOrRegisterInstanceForKey)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<AppInstance>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Unregister() -> ::windows::core::Result<()> {
        Self::IAppInstanceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Unregister)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInstances() -> ::windows::core::Result<super::Foundation::Collections::IVector<AppInstance>> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInstances)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<AppInstance>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppInstanceStatics<R, F: FnOnce(&IAppInstanceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInstance, IAppInstanceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstance {}
impl ::core::fmt::Debug for AppInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppInstance;{675f2b47-f25f-4532-9fd6-3633e0634d01})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppInstance {
    type Vtable = IAppInstance_Vtbl;
    const IID: ::windows::core::GUID = <IAppInstance as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstance {
    const NAME: &'static str = "Windows.ApplicationModel.AppInstance";
}
impl ::core::convert::From<AppInstance> for ::windows::core::IUnknown {
    fn from(value: AppInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstance> for ::windows::core::IUnknown {
    fn from(value: &AppInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstance> for ::windows::core::IInspectable {
    fn from(value: AppInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstance> for ::windows::core::IInspectable {
    fn from(value: &AppInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstance {}
unsafe impl ::core::marker::Sync for AppInstance {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct CameraApplicationManager {}
impl CameraApplicationManager {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ShowInstalledApplicationsUI() -> ::windows::core::Result<()> {
        Self::ICameraApplicationManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ShowInstalledApplicationsUI)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ICameraApplicationManagerStatics<R, F: FnOnce(&ICameraApplicationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CameraApplicationManager, ICameraApplicationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CameraApplicationManager {
    const NAME: &'static str = "Windows.ApplicationModel.CameraApplicationManager";
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct DesignMode {}
impl DesignMode {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn DesignModeEnabled() -> ::windows::core::Result<bool> {
        Self::IDesignModeStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesignModeEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn DesignMode2Enabled() -> ::windows::core::Result<bool> {
        Self::IDesignModeStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesignMode2Enabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDesignModeStatics<R, F: FnOnce(&IDesignModeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DesignMode, IDesignModeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IDesignModeStatics2<R, F: FnOnce(&IDesignModeStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DesignMode, IDesignModeStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for DesignMode {
    const NAME: &'static str = "Windows.ApplicationModel.DesignMode";
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct EnteredBackgroundEventArgs(::windows::core::IUnknown);
impl EnteredBackgroundEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for EnteredBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnteredBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnteredBackgroundEventArgs {}
impl ::core::fmt::Debug for EnteredBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnteredBackgroundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnteredBackgroundEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.EnteredBackgroundEventArgs;{f722dcc2-9827-403d-aaed-ecca9ac17398})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EnteredBackgroundEventArgs {
    type Vtable = IEnteredBackgroundEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IEnteredBackgroundEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.EnteredBackgroundEventArgs";
}
impl ::core::convert::From<EnteredBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnteredBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnteredBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnteredBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<EnteredBackgroundEventArgs> for IEnteredBackgroundEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: EnteredBackgroundEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&EnteredBackgroundEventArgs> for IEnteredBackgroundEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &EnteredBackgroundEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IEnteredBackgroundEventArgs> for EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IEnteredBackgroundEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IEnteredBackgroundEventArgs> for &EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IEnteredBackgroundEventArgs> {
        ::core::convert::TryInto::<IEnteredBackgroundEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for EnteredBackgroundEventArgs {}
unsafe impl ::core::marker::Sync for EnteredBackgroundEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FullTrustLaunchResult(pub i32);
impl FullTrustLaunchResult {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for FullTrustLaunchResult {}
impl ::core::clone::Clone for FullTrustLaunchResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FullTrustLaunchResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FullTrustLaunchResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for FullTrustLaunchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullTrustLaunchResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FullTrustLaunchResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.FullTrustLaunchResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct FullTrustProcessLaunchResult(::windows::core::IUnknown);
impl FullTrustProcessLaunchResult {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn LaunchResult(&self) -> ::windows::core::Result<FullTrustLaunchResult> {
        let this = self;
        unsafe {
            let mut result__: FullTrustLaunchResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchResult)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FullTrustLaunchResult>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for FullTrustProcessLaunchResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FullTrustProcessLaunchResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FullTrustProcessLaunchResult {}
impl ::core::fmt::Debug for FullTrustProcessLaunchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullTrustProcessLaunchResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FullTrustProcessLaunchResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.FullTrustProcessLaunchResult;{8917d888-edfb-515f-8e22-5ebceb69dfd9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FullTrustProcessLaunchResult {
    type Vtable = IFullTrustProcessLaunchResult_Vtbl;
    const IID: ::windows::core::GUID = <IFullTrustProcessLaunchResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FullTrustProcessLaunchResult {
    const NAME: &'static str = "Windows.ApplicationModel.FullTrustProcessLaunchResult";
}
impl ::core::convert::From<FullTrustProcessLaunchResult> for ::windows::core::IUnknown {
    fn from(value: FullTrustProcessLaunchResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullTrustProcessLaunchResult> for ::windows::core::IUnknown {
    fn from(value: &FullTrustProcessLaunchResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FullTrustProcessLaunchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FullTrustProcessLaunchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FullTrustProcessLaunchResult> for ::windows::core::IInspectable {
    fn from(value: FullTrustProcessLaunchResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullTrustProcessLaunchResult> for ::windows::core::IInspectable {
    fn from(value: &FullTrustProcessLaunchResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FullTrustProcessLaunchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FullTrustProcessLaunchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FullTrustProcessLaunchResult {}
unsafe impl ::core::marker::Sync for FullTrustProcessLaunchResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct FullTrustProcessLauncher {}
impl FullTrustProcessLauncher {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForCurrentAppAsync() -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFullTrustProcessForCurrentAppAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForCurrentAppWithParametersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(parametergroupid: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFullTrustProcessForCurrentAppWithParametersAsync)(::core::mem::transmute_copy(this), parametergroupid.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForAppAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(fulltrustpackagerelativeappid: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFullTrustProcessForAppAsync)(::core::mem::transmute_copy(this), fulltrustpackagerelativeappid.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForAppWithParametersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(fulltrustpackagerelativeappid: Param0, parametergroupid: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFullTrustProcessForAppWithParametersAsync)(::core::mem::transmute_copy(this), fulltrustpackagerelativeappid.into_param().abi(), parametergroupid.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForCurrentAppWithArgumentsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(commandline: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>> {
        Self::IFullTrustProcessLauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFullTrustProcessForCurrentAppWithArgumentsAsync)(::core::mem::transmute_copy(this), commandline.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForAppWithArgumentsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(fulltrustpackagerelativeappid: Param0, commandline: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>> {
        Self::IFullTrustProcessLauncherStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchFullTrustProcessForAppWithArgumentsAsync)(::core::mem::transmute_copy(this), fulltrustpackagerelativeappid.into_param().abi(), commandline.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFullTrustProcessLauncherStatics<R, F: FnOnce(&IFullTrustProcessLauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FullTrustProcessLauncher, IFullTrustProcessLauncherStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IFullTrustProcessLauncherStatics2<R, F: FnOnce(&IFullTrustProcessLauncherStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FullTrustProcessLauncher, IFullTrustProcessLauncherStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for FullTrustProcessLauncher {
    const NAME: &'static str = "Windows.ApplicationModel.FullTrustProcessLauncher";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDisplayInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDisplayInfo {
    type Vtable = IAppDisplayInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aeb1103_e4d4_41aa_a4f6_c4a276e79eac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDisplayInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetLogo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInfo {
    type Vtable = IAppInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf7f59b3_6a09_4de8_a6c0_5792d56880d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInfo2 {
    type Vtable = IAppInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe4b1f5a_2098_431b_bd25_b30878748d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInfo3 {
    type Vtable = IAppInfo3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09a78e46_93a4_46de_9397_0843b57115ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExecutionContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppExecutionContext) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInfo4 {
    type Vtable = IAppInfo4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f34bdeb_1609_4554_9f33_12e1e803e0d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SupportedFileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInfoStatics {
    type Vtable = IAppInfoStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf1f782a_e48b_4f0c_9b0b_79c3f8957dd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfoStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFromAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetFromAppUserModelIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFromAppUserModelIdForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallerInfo {
    type Vtable = IAppInstallerInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29ab2ac0_d4f6_42a3_adcd_d6583c659508);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallerInfo2 {
    type Vtable = IAppInstallerInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd20f1388_8256_597c_8511_c84ec50d5e2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OnLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HoursBetweenUpdateChecks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ShowPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub UpdateBlocksActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AutomaticBackgroundTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAutoRepairEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageVersion) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastChecked: usize,
    #[cfg(feature = "Foundation")]
    pub PausedUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PausedUntil: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RepairUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RepairUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageUris: usize,
    pub PolicySource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallerPolicySource) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstance(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstance {
    type Vtable = IAppInstance_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x675f2b47_f25f_4532_9fd6_3633e0634d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstance_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsCurrentInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RedirectActivationTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstanceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstanceStatics {
    type Vtable = IAppInstanceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d11e77f_9ea6_47af_a6ec_46784c5ba254);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RecommendedInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub GetActivatedEventArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    GetActivatedEventArgs: usize,
    pub FindOrRegisterInstanceForKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInstances: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraApplicationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraApplicationManagerStatics {
    type Vtable = ICameraApplicationManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9599ddce_9bd3_435c_8054_c1add50028fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraApplicationManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ShowInstalledApplicationsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignModeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesignModeStatics {
    type Vtable = IDesignModeStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c3893cc_f81a_4e7a_b857_76a80887e185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignModeStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DesignModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignModeStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesignModeStatics2 {
    type Vtable = IDesignModeStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80cf8137_b064_4858_bec8_3eba22357535);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignModeStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DesignMode2Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct IEnteredBackgroundEventArgs(::windows::core::IUnknown);
impl IEnteredBackgroundEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::convert::From<IEnteredBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: IEnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnteredBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IEnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEnteredBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: IEnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnteredBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IEnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IEnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IEnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnteredBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnteredBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnteredBackgroundEventArgs {}
impl ::core::fmt::Debug for IEnteredBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnteredBackgroundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IEnteredBackgroundEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f722dcc2-9827-403d-aaed-ecca9ac17398}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IEnteredBackgroundEventArgs {
    type Vtable = IEnteredBackgroundEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf722dcc2_9827_403d_aaed_ecca9ac17398);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnteredBackgroundEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullTrustProcessLaunchResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFullTrustProcessLaunchResult {
    type Vtable = IFullTrustProcessLaunchResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8917d888_edfb_515f_8e22_5ebceb69dfd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullTrustProcessLaunchResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LaunchResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FullTrustLaunchResult) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFullTrustProcessLauncherStatics {
    type Vtable = IFullTrustProcessLauncherStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd784837f_1100_3c6b_a455_f6262cc331b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullTrustProcessLauncherStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForCurrentAppAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForCurrentAppWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parametergroupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForCurrentAppWithParametersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForAppAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForAppWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, parametergroupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForAppWithParametersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFullTrustProcessLauncherStatics2 {
    type Vtable = IFullTrustProcessLauncherStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b8ed72f_b65c_56cf_a1a7_2bf77cbc6ea8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullTrustProcessLauncherStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForCurrentAppWithArgumentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandline: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForCurrentAppWithArgumentsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForAppWithArgumentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, commandline: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForAppWithArgumentsAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct ILeavingBackgroundEventArgs(::windows::core::IUnknown);
impl ILeavingBackgroundEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::convert::From<ILeavingBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILeavingBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILeavingBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILeavingBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILeavingBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILeavingBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILeavingBackgroundEventArgs {}
impl ::core::fmt::Debug for ILeavingBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILeavingBackgroundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILeavingBackgroundEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{39c6ec9a-ae6e-46f9-a07a-cfc23f88733e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILeavingBackgroundEventArgs {
    type Vtable = ILeavingBackgroundEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39c6ec9a_ae6e_46f9_a07a_cfc23f88733e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILeavingBackgroundEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimitedAccessFeatureRequestResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILimitedAccessFeatureRequestResult {
    type Vtable = ILimitedAccessFeatureRequestResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45156a6_1e24_5ddd_abb4_6188aba4d5bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimitedAccessFeatureRequestResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FeatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LimitedAccessFeatureStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EstimatedRemovalDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EstimatedRemovalDate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimitedAccessFeaturesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILimitedAccessFeaturesStatics {
    type Vtable = ILimitedAccessFeaturesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be612d4_302b_5fbf_a632_1a99e43e8925);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimitedAccessFeaturesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryUnlockFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, featureid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, attestation: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackage {
    type Vtable = IPackage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x163c792f_bd75_413c_bf23_b1fe7b95d825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub InstalledLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    InstalledLocation: usize,
    pub IsFramework: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Dependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Dependencies: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackage2 {
    type Vtable = IPackage2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6612fb6_7688_4ace_95fb_359538e7aa01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PublisherDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Logo: usize,
    pub IsResourcePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBundle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDevelopmentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackage3 {
    type Vtable = IPackage3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f738b61_f86a_4917_93d1_f1ee9d3b35d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstalledDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstalledDate: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntriesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntriesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackage4 {
    type Vtable = IPackage4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65aed1ae_b95b_450c_882b_6255187f397e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SignatureKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageSignatureKind) -> ::windows::core::HRESULT,
    pub IsOptional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VerifyContentIntegrityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VerifyContentIntegrityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackage5 {
    type Vtable = IPackage5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e842dd4_d9ac_45ed_9a1e_74ce056b2635);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetContentGroupsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetContentGroupsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContentGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContentGroupAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsWithPriorityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: ::windows::core::RawPtr, movetoheadofqueue: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsWithPriorityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetInUseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inuse: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInUseAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackage6 {
    type Vtable = IPackage6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b1ad942_12d7_4754_ae4e_638cbc0e3a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetAppInstallerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CheckUpdateAvailabilityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckUpdateAvailabilityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackage7 {
    type Vtable = IPackage7_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86ff8d31_a2e4_45e0_9732_283a6d88fde1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage7_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub MutableLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MutableLocation: usize,
    #[cfg(feature = "Storage")]
    pub EffectiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveLocation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage8(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackage8 {
    type Vtable = IPackage8_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c584f7b_ce2a_4be6_a093_77cfbb2a7ea1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage8_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub EffectiveExternalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub MachineExternalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MachineExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub UserExternalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    UserExternalLocation: usize,
    pub InstalledPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MutablePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EffectivePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EffectiveExternalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MachineExternalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserExternalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetLogoAsRandomAccessStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetLogoAsRandomAccessStreamReference: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntries: usize,
    pub IsStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageCatalog {
    type Vtable = IPackageCatalog_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x230a3751_9de3_4445_be74_91fb325abefe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PackageStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageStaging: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageStaging: usize,
    #[cfg(feature = "Foundation")]
    pub PackageInstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageInstalling: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageInstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageInstalling: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub PackageStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageCatalog2 {
    type Vtable = IPackageCatalog2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96a60c36_8ff7_4344_b6bf_ee64c2207ed2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PackageContentGroupStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageContentGroupStaging: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageContentGroupStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageContentGroupStaging: usize,
    #[cfg(feature = "Foundation")]
    pub AddOptionalPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddOptionalPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageCatalog3 {
    type Vtable = IPackageCatalog3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96dd5c88_8837_43f9_9015_033434ba14f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveOptionalPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackagefamilynames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveOptionalPackagesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageCatalog4 {
    type Vtable = IPackageCatalog4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc37c399b_44cc_4b7b_8baf_796c04ead3b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AddResourcePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcepackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AddResourcePackageOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddResourcePackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveResourcePackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcepackages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveResourcePackagesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogAddOptionalPackageResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageCatalogAddOptionalPackageResult {
    type Vtable = IPackageCatalogAddOptionalPackageResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bf10cd4_b4df_47b3_a963_e2fa832f7dd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogAddOptionalPackageResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogAddResourcePackageResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageCatalogAddResourcePackageResult {
    type Vtable = IPackageCatalogAddResourcePackageResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9636ce0d_3e17_493f_aa08_ccec6fdef699);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogAddResourcePackageResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogRemoveOptionalPackagesResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageCatalogRemoveOptionalPackagesResult {
    type Vtable = IPackageCatalogRemoveOptionalPackagesResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29d2f97b_d974_4e64_9359_22cadfd79828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogRemoveOptionalPackagesResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogRemoveResourcePackagesResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageCatalogRemoveResourcePackagesResult {
    type Vtable = IPackageCatalogRemoveResourcePackagesResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae719709_1a52_4321_87b3_e5a1a17981a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogRemoveResourcePackagesResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageCatalogStatics {
    type Vtable = IPackageCatalogStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa18c9696_e65b_4634_ba21_5e63eb7244a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OpenForCurrentPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OpenForCurrentUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageContentGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageContentGroup {
    type Vtable = IPackageContentGroup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f62695d_120a_4798_b5e1_5800dda8f2e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageContentGroup_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageContentGroupState) -> ::windows::core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageContentGroupStagingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageContentGroupStagingEventArgs {
    type Vtable = IPackageContentGroupStagingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d7bc27e_6f27_446c_986e_d4733d4d9113);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageContentGroupStagingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ContentGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsContentGroupRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageContentGroupStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageContentGroupStatics {
    type Vtable = IPackageContentGroupStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70ee7619_5f12_4b92_b9ea_6ccada13bc75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageContentGroupStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequiredGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageId {
    type Vtable = IPackageId_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1adb665e_37c7_4790_9980_dd7ae74e8bb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageId_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageVersion) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub Architecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::System::ProcessorArchitecture) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Architecture: usize,
    pub ResourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PublisherId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageIdWithMetadata(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageIdWithMetadata {
    type Vtable = IPackageIdWithMetadata_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40577a7c_0c9e_443d_9074_855f5ce0a08d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageIdWithMetadata_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageInstallingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageInstallingEventArgs {
    type Vtable = IPackageInstallingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97741eb7_ab7a_401a_8b61_eb0e7faff237);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStagingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageStagingEventArgs {
    type Vtable = IPackageStagingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1041682d_54e2_4f51_b828_9ef7046c210f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStagingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageStatics {
    type Vtable = IPackageStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e534bdf_2960_4878_97a4_9624deb72f2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageStatus {
    type Vtable = IPackageStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fe74f71_a365_4c09_a02d_046d525ea1da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatus_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub VerifyIsOK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub NotAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PackageOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DataOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Disabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub NeedsRemediation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub LicenseIssue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Tampered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DependencyIssue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Servicing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DeploymentInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatus2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageStatus2 {
    type Vtable = IPackageStatus2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf428fa93_7c56_4862_acfa_abaedcc0694d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatus2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPartiallyStaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageStatusChangedEventArgs {
    type Vtable = IPackageStatusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x437d714d_bd80_4a70_bc50_f6e796509575);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatusChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUninstallingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageUninstallingEventArgs {
    type Vtable = IPackageUninstallingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4443aa52_ab22_44cd_82bb_4ec9b827367a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUninstallingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUpdateAvailabilityResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageUpdateAvailabilityResult {
    type Vtable = IPackageUpdateAvailabilityResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x114e5009_199a_48a1_a079_313c45634a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUpdateAvailabilityResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Availability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageUpdateAvailability) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageUpdatingEventArgs {
    type Vtable = IPackageUpdatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd7b4228_fd74_443e_b114_23e677b0e86f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUpdatingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SourcePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TargetPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageWithMetadata(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageWithMetadata {
    type Vtable = IPackageWithMetadata_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95949780_1de9_40f2_b452_0de9f1910012);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageWithMetadata_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub InstallDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstallDate: usize,
    pub GetThumbnailToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Launch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Launch: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartupTask(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStartupTask {
    type Vtable = IStartupTask_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf75c23c8_b5f2_4f6c_88dd_36cb1d599d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTask_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestEnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestEnableAsync: usize,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StartupTaskState) -> ::windows::core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartupTaskStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStartupTaskStatics {
    type Vtable = IStartupTaskStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5b60bd_a148_41a7_b26e_e8b88a1e62f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetForCurrentPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetForCurrentPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct ISuspendingDeferral(::windows::core::IUnknown);
impl ISuspendingDeferral {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<ISuspendingDeferral> for ::windows::core::IUnknown {
    fn from(value: ISuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingDeferral> for ::windows::core::IUnknown {
    fn from(value: &ISuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISuspendingDeferral> for ::windows::core::IInspectable {
    fn from(value: ISuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingDeferral> for ::windows::core::IInspectable {
    fn from(value: &ISuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISuspendingDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISuspendingDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISuspendingDeferral {}
impl ::core::fmt::Debug for ISuspendingDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISuspendingDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISuspendingDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{59140509-8bc9-4eb4-b636-dabdc4f46f66}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISuspendingDeferral {
    type Vtable = ISuspendingDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59140509_8bc9_4eb4_b636_dabdc4f46f66);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuspendingDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct ISuspendingEventArgs(::windows::core::IUnknown);
impl ISuspendingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn SuspendingOperation(&self) -> ::windows::core::Result<SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SuspendingOperation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SuspendingOperation>(result__)
        }
    }
}
impl ::core::convert::From<ISuspendingEventArgs> for ::windows::core::IUnknown {
    fn from(value: ISuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ISuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISuspendingEventArgs> for ::windows::core::IInspectable {
    fn from(value: ISuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ISuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISuspendingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISuspendingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISuspendingEventArgs {}
impl ::core::fmt::Debug for ISuspendingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISuspendingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISuspendingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{96061c05-2dba-4d08-b0bd-2b30a131c6aa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISuspendingEventArgs {
    type Vtable = ISuspendingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96061c05_2dba_4d08_b0bd_2b30a131c6aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuspendingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SuspendingOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct ISuspendingOperation(::windows::core::IUnknown);
impl ISuspendingOperation {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SuspendingDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Deadline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::convert::From<ISuspendingOperation> for ::windows::core::IUnknown {
    fn from(value: ISuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingOperation> for ::windows::core::IUnknown {
    fn from(value: &ISuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISuspendingOperation> for ::windows::core::IInspectable {
    fn from(value: ISuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingOperation> for ::windows::core::IInspectable {
    fn from(value: &ISuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISuspendingOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISuspendingOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISuspendingOperation {}
impl ::core::fmt::Debug for ISuspendingOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISuspendingOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISuspendingOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9da4ca41-20e1-4e9b-9f65-a9f435340c3a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISuspendingOperation {
    type Vtable = ISuspendingOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9da4ca41_20e1_4e9b_9f65_a9f435340c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuspendingOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct LeavingBackgroundEventArgs(::windows::core::IUnknown);
impl LeavingBackgroundEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for LeavingBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LeavingBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LeavingBackgroundEventArgs {}
impl ::core::fmt::Debug for LeavingBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LeavingBackgroundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LeavingBackgroundEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.LeavingBackgroundEventArgs;{39c6ec9a-ae6e-46f9-a07a-cfc23f88733e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LeavingBackgroundEventArgs {
    type Vtable = ILeavingBackgroundEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILeavingBackgroundEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.LeavingBackgroundEventArgs";
}
impl ::core::convert::From<LeavingBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LeavingBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LeavingBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LeavingBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LeavingBackgroundEventArgs> for ILeavingBackgroundEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LeavingBackgroundEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LeavingBackgroundEventArgs> for ILeavingBackgroundEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LeavingBackgroundEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILeavingBackgroundEventArgs> for LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILeavingBackgroundEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILeavingBackgroundEventArgs> for &LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILeavingBackgroundEventArgs> {
        ::core::convert::TryInto::<ILeavingBackgroundEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LeavingBackgroundEventArgs {}
unsafe impl ::core::marker::Sync for LeavingBackgroundEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct LimitedAccessFeatureRequestResult(::windows::core::IUnknown);
impl LimitedAccessFeatureRequestResult {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn FeatureId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FeatureId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<LimitedAccessFeatureStatus> {
        let this = self;
        unsafe {
            let mut result__: LimitedAccessFeatureStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LimitedAccessFeatureStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EstimatedRemovalDate(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EstimatedRemovalDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::DateTime>>(result__)
        }
    }
}
impl ::core::clone::Clone for LimitedAccessFeatureRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LimitedAccessFeatureRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LimitedAccessFeatureRequestResult {}
impl ::core::fmt::Debug for LimitedAccessFeatureRequestResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LimitedAccessFeatureRequestResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LimitedAccessFeatureRequestResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.LimitedAccessFeatureRequestResult;{d45156a6-1e24-5ddd-abb4-6188aba4d5bf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LimitedAccessFeatureRequestResult {
    type Vtable = ILimitedAccessFeatureRequestResult_Vtbl;
    const IID: ::windows::core::GUID = <ILimitedAccessFeatureRequestResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LimitedAccessFeatureRequestResult {
    const NAME: &'static str = "Windows.ApplicationModel.LimitedAccessFeatureRequestResult";
}
impl ::core::convert::From<LimitedAccessFeatureRequestResult> for ::windows::core::IUnknown {
    fn from(value: LimitedAccessFeatureRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LimitedAccessFeatureRequestResult> for ::windows::core::IUnknown {
    fn from(value: &LimitedAccessFeatureRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LimitedAccessFeatureRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LimitedAccessFeatureRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LimitedAccessFeatureRequestResult> for ::windows::core::IInspectable {
    fn from(value: LimitedAccessFeatureRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LimitedAccessFeatureRequestResult> for ::windows::core::IInspectable {
    fn from(value: &LimitedAccessFeatureRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LimitedAccessFeatureRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LimitedAccessFeatureRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LimitedAccessFeatureRequestResult {}
unsafe impl ::core::marker::Sync for LimitedAccessFeatureRequestResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LimitedAccessFeatureStatus(pub i32);
impl LimitedAccessFeatureStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Available: Self = Self(1i32);
    pub const AvailableWithoutToken: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LimitedAccessFeatureStatus {}
impl ::core::clone::Clone for LimitedAccessFeatureStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LimitedAccessFeatureStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LimitedAccessFeatureStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LimitedAccessFeatureStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LimitedAccessFeatureStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LimitedAccessFeatureStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.LimitedAccessFeatureStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct LimitedAccessFeatures {}
impl LimitedAccessFeatures {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn TryUnlockFeature<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(featureid: Param0, token: Param1, attestation: Param2) -> ::windows::core::Result<LimitedAccessFeatureRequestResult> {
        Self::ILimitedAccessFeaturesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUnlockFeature)(::core::mem::transmute_copy(this), featureid.into_param().abi(), token.into_param().abi(), attestation.into_param().abi(), &mut result__).from_abi::<LimitedAccessFeatureRequestResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILimitedAccessFeaturesStatics<R, F: FnOnce(&ILimitedAccessFeaturesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LimitedAccessFeatures, ILimitedAccessFeaturesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for LimitedAccessFeatures {
    const NAME: &'static str = "Windows.ApplicationModel.LimitedAccessFeatures";
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct Package(::windows::core::IUnknown);
impl Package {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<PackageId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageId>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn InstalledLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstalledLocation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsFramework(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFramework)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Dependencies(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dependencies)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<Package>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn PublisherDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublisherDisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Logo(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Logo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsResourcePackage(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsResourcePackage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsBundle(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBundle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsDevelopmentMode(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDevelopmentMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PackageStatus> {
        let this = &::windows::core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstalledDate(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstalledDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"ApplicationModel_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub fn GetAppListEntriesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<Core::AppListEntry>>> {
        let this = &::windows::core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppListEntriesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<Core::AppListEntry>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn SignatureKind(&self) -> ::windows::core::Result<PackageSignatureKind> {
        let this = &::windows::core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__: PackageSignatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignatureKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageSignatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsOptional(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOptional)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VerifyContentIntegrityAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VerifyContentIntegrityAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetContentGroupsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>> {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContentGroupsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContentGroupAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageContentGroup>> {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContentGroupAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<PackageContentGroup>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StageContentGroupsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, names: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>> {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StageContentGroupsAsync)(::core::mem::transmute_copy(this), names.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StageContentGroupsWithPriorityAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, names: Param0, movetoheadofqueue: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>> {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StageContentGroupsWithPriorityAsync)(::core::mem::transmute_copy(this), names.into_param().abi(), movetoheadofqueue, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInUseAsync(&self, inuse: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetInUseAsync)(::core::mem::transmute_copy(this), inuse, &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn GetAppInstallerInfo(&self) -> ::windows::core::Result<AppInstallerInfo> {
        let this = &::windows::core::Interface::cast::<IPackage6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppInstallerInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallerInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckUpdateAvailabilityAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageUpdateAvailabilityResult>> {
        let this = &::windows::core::Interface::cast::<IPackage6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckUpdateAvailabilityAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<PackageUpdateAvailabilityResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn MutableLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MutableLocation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn EffectiveLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectiveLocation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn EffectiveExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectiveExternalLocation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn MachineExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MachineExternalLocation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn UserExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserExternalLocation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn InstalledPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstalledPath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn MutablePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MutablePath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn EffectivePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectivePath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn EffectiveExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectiveExternalPath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn MachineExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MachineExternalPath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn UserExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserExternalPath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetLogoAsRandomAccessStreamReference<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Size>>(&self, size: Param0) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetLogoAsRandomAccessStreamReference)(::core::mem::transmute_copy(this), size.into_param().abi(), &mut result__).from_abi::<super::Storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"ApplicationModel_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub fn GetAppListEntries(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Core::AppListEntry>> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppListEntries)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<Core::AppListEntry>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsStub(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStub)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Current() -> ::windows::core::Result<Package> {
        Self::IPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Current)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstallDate(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn GetThumbnailToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailToken)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Launch<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, parameters: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Launch)(::core::mem::transmute_copy(this), parameters.into_param().abi()).ok() }
    }
    #[doc(hidden)]
    pub fn IPackageStatics<R, F: FnOnce(&IPackageStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Package, IPackageStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Package {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Package {}
impl ::core::fmt::Debug for Package {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Package").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Package {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Package;{163c792f-bd75-413c-bf23-b1fe7b95d825})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Package {
    type Vtable = IPackage_Vtbl;
    const IID: ::windows::core::GUID = <IPackage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Package {
    const NAME: &'static str = "Windows.ApplicationModel.Package";
}
impl ::core::convert::From<Package> for ::windows::core::IUnknown {
    fn from(value: Package) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Package> for ::windows::core::IUnknown {
    fn from(value: &Package) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Package {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Package {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Package> for ::windows::core::IInspectable {
    fn from(value: Package) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Package> for ::windows::core::IInspectable {
    fn from(value: &Package) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Package {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Package {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Package {}
unsafe impl ::core::marker::Sync for Package {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalog(::windows::core::IUnknown);
impl PackageCatalog {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageStaging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<PackageCatalog, PackageStagingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageStaging)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageStaging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageStaging)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageInstalling<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<PackageCatalog, PackageInstallingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageInstalling)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageInstalling<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageInstalling)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<PackageCatalog, PackageUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageUpdating)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUpdating<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageUpdating)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUninstalling<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<PackageCatalog, PackageUninstallingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageUninstalling)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUninstalling<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageUninstalling)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<PackageCatalog, PackageStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageContentGroupStaging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<PackageCatalog, PackageContentGroupStagingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageContentGroupStaging)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageContentGroupStaging<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePackageContentGroupStaging)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddOptionalPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, optionalpackagefamilyname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogAddOptionalPackageResult>> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddOptionalPackageAsync)(::core::mem::transmute_copy(this), optionalpackagefamilyname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<PackageCatalogAddOptionalPackageResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveOptionalPackagesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, optionalpackagefamilynames: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveOptionalPackagesResult>> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveOptionalPackagesAsync)(::core::mem::transmute_copy(this), optionalpackagefamilynames.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<PackageCatalogRemoveOptionalPackagesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddResourcePackageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, resourcepackagefamilyname: Param0, resourceid: Param1, options: AddResourcePackageOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperationWithProgress<PackageCatalogAddResourcePackageResult, PackageInstallProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddResourcePackageAsync)(::core::mem::transmute_copy(this), resourcepackagefamilyname.into_param().abi(), resourceid.into_param().abi(), options, &mut result__).from_abi::<super::Foundation::IAsyncOperationWithProgress<PackageCatalogAddResourcePackageResult, PackageInstallProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveResourcePackagesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<Package>>>(&self, resourcepackages: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveResourcePackagesResult>> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveResourcePackagesAsync)(::core::mem::transmute_copy(this), resourcepackages.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<PackageCatalogRemoveResourcePackagesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn OpenForCurrentPackage() -> ::windows::core::Result<PackageCatalog> {
        Self::IPackageCatalogStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenForCurrentPackage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageCatalog>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn OpenForCurrentUser() -> ::windows::core::Result<PackageCatalog> {
        Self::IPackageCatalogStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenForCurrentUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageCatalog>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageCatalogStatics<R, F: FnOnce(&IPackageCatalogStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PackageCatalog, IPackageCatalogStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PackageCatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalog {}
impl ::core::fmt::Debug for PackageCatalog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalog").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageCatalog {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalog;{230a3751-9de3-4445-be74-91fb325abefe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageCatalog {
    type Vtable = IPackageCatalog_Vtbl;
    const IID: ::windows::core::GUID = <IPackageCatalog as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalog";
}
impl ::core::convert::From<PackageCatalog> for ::windows::core::IUnknown {
    fn from(value: PackageCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalog> for ::windows::core::IUnknown {
    fn from(value: &PackageCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageCatalog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageCatalog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalog> for ::windows::core::IInspectable {
    fn from(value: PackageCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalog> for ::windows::core::IInspectable {
    fn from(value: &PackageCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageCatalog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageCatalog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalogAddOptionalPackageResult(::windows::core::IUnknown);
impl PackageCatalogAddOptionalPackageResult {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageCatalogAddOptionalPackageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalogAddOptionalPackageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalogAddOptionalPackageResult {}
impl ::core::fmt::Debug for PackageCatalogAddOptionalPackageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalogAddOptionalPackageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageCatalogAddOptionalPackageResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult;{3bf10cd4-b4df-47b3-a963-e2fa832f7dd3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageCatalogAddOptionalPackageResult {
    type Vtable = IPackageCatalogAddOptionalPackageResult_Vtbl;
    const IID: ::windows::core::GUID = <IPackageCatalogAddOptionalPackageResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalogAddOptionalPackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult";
}
impl ::core::convert::From<PackageCatalogAddOptionalPackageResult> for ::windows::core::IUnknown {
    fn from(value: PackageCatalogAddOptionalPackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogAddOptionalPackageResult> for ::windows::core::IUnknown {
    fn from(value: &PackageCatalogAddOptionalPackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageCatalogAddOptionalPackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageCatalogAddOptionalPackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalogAddOptionalPackageResult> for ::windows::core::IInspectable {
    fn from(value: PackageCatalogAddOptionalPackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogAddOptionalPackageResult> for ::windows::core::IInspectable {
    fn from(value: &PackageCatalogAddOptionalPackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageCatalogAddOptionalPackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageCatalogAddOptionalPackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalogAddResourcePackageResult(::windows::core::IUnknown);
impl PackageCatalogAddResourcePackageResult {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsComplete)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageCatalogAddResourcePackageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalogAddResourcePackageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalogAddResourcePackageResult {}
impl ::core::fmt::Debug for PackageCatalogAddResourcePackageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalogAddResourcePackageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageCatalogAddResourcePackageResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalogAddResourcePackageResult;{9636ce0d-3e17-493f-aa08-ccec6fdef699})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageCatalogAddResourcePackageResult {
    type Vtable = IPackageCatalogAddResourcePackageResult_Vtbl;
    const IID: ::windows::core::GUID = <IPackageCatalogAddResourcePackageResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalogAddResourcePackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogAddResourcePackageResult";
}
impl ::core::convert::From<PackageCatalogAddResourcePackageResult> for ::windows::core::IUnknown {
    fn from(value: PackageCatalogAddResourcePackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogAddResourcePackageResult> for ::windows::core::IUnknown {
    fn from(value: &PackageCatalogAddResourcePackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageCatalogAddResourcePackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageCatalogAddResourcePackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalogAddResourcePackageResult> for ::windows::core::IInspectable {
    fn from(value: PackageCatalogAddResourcePackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogAddResourcePackageResult> for ::windows::core::IInspectable {
    fn from(value: &PackageCatalogAddResourcePackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageCatalogAddResourcePackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageCatalogAddResourcePackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageCatalogAddResourcePackageResult {}
unsafe impl ::core::marker::Sync for PackageCatalogAddResourcePackageResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalogRemoveOptionalPackagesResult(::windows::core::IUnknown);
impl PackageCatalogRemoveOptionalPackagesResult {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PackagesRemoved(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackagesRemoved)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<Package>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageCatalogRemoveOptionalPackagesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalogRemoveOptionalPackagesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalogRemoveOptionalPackagesResult {}
impl ::core::fmt::Debug for PackageCatalogRemoveOptionalPackagesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalogRemoveOptionalPackagesResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageCatalogRemoveOptionalPackagesResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult;{29d2f97b-d974-4e64-9359-22cadfd79828})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageCatalogRemoveOptionalPackagesResult {
    type Vtable = IPackageCatalogRemoveOptionalPackagesResult_Vtbl;
    const IID: ::windows::core::GUID = <IPackageCatalogRemoveOptionalPackagesResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalogRemoveOptionalPackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult";
}
impl ::core::convert::From<PackageCatalogRemoveOptionalPackagesResult> for ::windows::core::IUnknown {
    fn from(value: PackageCatalogRemoveOptionalPackagesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogRemoveOptionalPackagesResult> for ::windows::core::IUnknown {
    fn from(value: &PackageCatalogRemoveOptionalPackagesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageCatalogRemoveOptionalPackagesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageCatalogRemoveOptionalPackagesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalogRemoveOptionalPackagesResult> for ::windows::core::IInspectable {
    fn from(value: PackageCatalogRemoveOptionalPackagesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogRemoveOptionalPackagesResult> for ::windows::core::IInspectable {
    fn from(value: &PackageCatalogRemoveOptionalPackagesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageCatalogRemoveOptionalPackagesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageCatalogRemoveOptionalPackagesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalogRemoveResourcePackagesResult(::windows::core::IUnknown);
impl PackageCatalogRemoveResourcePackagesResult {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PackagesRemoved(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackagesRemoved)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<Package>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageCatalogRemoveResourcePackagesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalogRemoveResourcePackagesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalogRemoveResourcePackagesResult {}
impl ::core::fmt::Debug for PackageCatalogRemoveResourcePackagesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalogRemoveResourcePackagesResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageCatalogRemoveResourcePackagesResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult;{ae719709-1a52-4321-87b3-e5a1a17981a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageCatalogRemoveResourcePackagesResult {
    type Vtable = IPackageCatalogRemoveResourcePackagesResult_Vtbl;
    const IID: ::windows::core::GUID = <IPackageCatalogRemoveResourcePackagesResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalogRemoveResourcePackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult";
}
impl ::core::convert::From<PackageCatalogRemoveResourcePackagesResult> for ::windows::core::IUnknown {
    fn from(value: PackageCatalogRemoveResourcePackagesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogRemoveResourcePackagesResult> for ::windows::core::IUnknown {
    fn from(value: &PackageCatalogRemoveResourcePackagesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageCatalogRemoveResourcePackagesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageCatalogRemoveResourcePackagesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalogRemoveResourcePackagesResult> for ::windows::core::IInspectable {
    fn from(value: PackageCatalogRemoveResourcePackagesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogRemoveResourcePackagesResult> for ::windows::core::IInspectable {
    fn from(value: &PackageCatalogRemoveResourcePackagesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageCatalogRemoveResourcePackagesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageCatalogRemoveResourcePackagesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageCatalogRemoveResourcePackagesResult {}
unsafe impl ::core::marker::Sync for PackageCatalogRemoveResourcePackagesResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageContentGroup(::windows::core::IUnknown);
impl PackageContentGroup {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn State(&self) -> ::windows::core::Result<PackageContentGroupState> {
        let this = self;
        unsafe {
            let mut result__: PackageContentGroupState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageContentGroupState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn RequiredGroupName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPackageContentGroupStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequiredGroupName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageContentGroupStatics<R, F: FnOnce(&IPackageContentGroupStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PackageContentGroup, IPackageContentGroupStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PackageContentGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageContentGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageContentGroup {}
impl ::core::fmt::Debug for PackageContentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageContentGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageContentGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageContentGroup;{8f62695d-120a-4798-b5e1-5800dda8f2e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageContentGroup {
    type Vtable = IPackageContentGroup_Vtbl;
    const IID: ::windows::core::GUID = <IPackageContentGroup as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageContentGroup {
    const NAME: &'static str = "Windows.ApplicationModel.PackageContentGroup";
}
impl ::core::convert::From<PackageContentGroup> for ::windows::core::IUnknown {
    fn from(value: PackageContentGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageContentGroup> for ::windows::core::IUnknown {
    fn from(value: &PackageContentGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageContentGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageContentGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageContentGroup> for ::windows::core::IInspectable {
    fn from(value: PackageContentGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageContentGroup> for ::windows::core::IInspectable {
    fn from(value: &PackageContentGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageContentGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageContentGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageContentGroup {}
unsafe impl ::core::marker::Sync for PackageContentGroup {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageContentGroupStagingEventArgs(::windows::core::IUnknown);
impl PackageContentGroupStagingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Progress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsComplete)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ContentGroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentGroupName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsContentGroupRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsContentGroupRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageContentGroupStagingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageContentGroupStagingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageContentGroupStagingEventArgs {}
impl ::core::fmt::Debug for PackageContentGroupStagingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageContentGroupStagingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageContentGroupStagingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageContentGroupStagingEventArgs;{3d7bc27e-6f27-446c-986e-d4733d4d9113})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageContentGroupStagingEventArgs {
    type Vtable = IPackageContentGroupStagingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPackageContentGroupStagingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageContentGroupStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageContentGroupStagingEventArgs";
}
impl ::core::convert::From<PackageContentGroupStagingEventArgs> for ::windows::core::IUnknown {
    fn from(value: PackageContentGroupStagingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageContentGroupStagingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PackageContentGroupStagingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageContentGroupStagingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageContentGroupStagingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageContentGroupStagingEventArgs> for ::windows::core::IInspectable {
    fn from(value: PackageContentGroupStagingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageContentGroupStagingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PackageContentGroupStagingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageContentGroupStagingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageContentGroupStagingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageContentGroupStagingEventArgs {}
unsafe impl ::core::marker::Sync for PackageContentGroupStagingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageContentGroupState(pub i32);
impl PackageContentGroupState {
    pub const NotStaged: Self = Self(0i32);
    pub const Queued: Self = Self(1i32);
    pub const Staging: Self = Self(2i32);
    pub const Staged: Self = Self(3i32);
}
impl ::core::marker::Copy for PackageContentGroupState {}
impl ::core::clone::Clone for PackageContentGroupState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageContentGroupState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PackageContentGroupState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageContentGroupState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageContentGroupState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageContentGroupState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageContentGroupState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageId(::windows::core::IUnknown);
impl PackageId {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Version(&self) -> ::windows::core::Result<PackageVersion> {
        let this = self;
        unsafe {
            let mut result__: PackageVersion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Version)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageVersion>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn Architecture(&self) -> ::windows::core::Result<super::System::ProcessorArchitecture> {
        let this = self;
        unsafe {
            let mut result__: super::System::ProcessorArchitecture = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Architecture)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::System::ProcessorArchitecture>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ResourceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Publisher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn PublisherId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublisherId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FullName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackageIdWithMetadata>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackageIdWithMetadata>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Author)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageId {}
impl ::core::fmt::Debug for PackageId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageId;{1adb665e-37c7-4790-9980-dd7ae74e8bb2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageId {
    type Vtable = IPackageId_Vtbl;
    const IID: ::windows::core::GUID = <IPackageId as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageId {
    const NAME: &'static str = "Windows.ApplicationModel.PackageId";
}
impl ::core::convert::From<PackageId> for ::windows::core::IUnknown {
    fn from(value: PackageId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageId> for ::windows::core::IUnknown {
    fn from(value: &PackageId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageId> for ::windows::core::IInspectable {
    fn from(value: PackageId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageId> for ::windows::core::IInspectable {
    fn from(value: &PackageId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageId {}
unsafe impl ::core::marker::Sync for PackageId {}
#[repr(C)]
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct PackageInstallProgress {
    pub PercentComplete: u32,
}
impl ::core::marker::Copy for PackageInstallProgress {}
impl ::core::clone::Clone for PackageInstallProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PackageInstallProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PackageInstallProgress").field("PercentComplete", &self.PercentComplete).finish()
    }
}
unsafe impl ::windows::core::Abi for PackageInstallProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageInstallProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.PackageInstallProgress;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PackageInstallProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PackageInstallProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for PackageInstallProgress {}
impl ::core::default::Default for PackageInstallProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageInstallingEventArgs(::windows::core::IUnknown);
impl PackageInstallingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Progress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsComplete)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageInstallingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageInstallingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageInstallingEventArgs {}
impl ::core::fmt::Debug for PackageInstallingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageInstallingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageInstallingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageInstallingEventArgs;{97741eb7-ab7a-401a-8b61-eb0e7faff237})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageInstallingEventArgs {
    type Vtable = IPackageInstallingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPackageInstallingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageInstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageInstallingEventArgs";
}
impl ::core::convert::From<PackageInstallingEventArgs> for ::windows::core::IUnknown {
    fn from(value: PackageInstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageInstallingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PackageInstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageInstallingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageInstallingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageInstallingEventArgs> for ::windows::core::IInspectable {
    fn from(value: PackageInstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageInstallingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PackageInstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageInstallingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageInstallingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageInstallingEventArgs {}
unsafe impl ::core::marker::Sync for PackageInstallingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageSignatureKind(pub i32);
impl PackageSignatureKind {
    pub const None: Self = Self(0i32);
    pub const Developer: Self = Self(1i32);
    pub const Enterprise: Self = Self(2i32);
    pub const Store: Self = Self(3i32);
    pub const System: Self = Self(4i32);
}
impl ::core::marker::Copy for PackageSignatureKind {}
impl ::core::clone::Clone for PackageSignatureKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageSignatureKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PackageSignatureKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageSignatureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageSignatureKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageSignatureKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageSignatureKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageStagingEventArgs(::windows::core::IUnknown);
impl PackageStagingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Progress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsComplete)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageStagingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageStagingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageStagingEventArgs {}
impl ::core::fmt::Debug for PackageStagingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStagingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageStagingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageStagingEventArgs;{1041682d-54e2-4f51-b828-9ef7046c210f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageStagingEventArgs {
    type Vtable = IPackageStagingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPackageStagingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStagingEventArgs";
}
impl ::core::convert::From<PackageStagingEventArgs> for ::windows::core::IUnknown {
    fn from(value: PackageStagingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStagingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PackageStagingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageStagingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageStagingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageStagingEventArgs> for ::windows::core::IInspectable {
    fn from(value: PackageStagingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStagingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PackageStagingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageStagingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageStagingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageStagingEventArgs {}
unsafe impl ::core::marker::Sync for PackageStagingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageStatus(::windows::core::IUnknown);
impl PackageStatus {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn VerifyIsOK(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VerifyIsOK)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn NotAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NotAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn PackageOffline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageOffline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn DataOffline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataOffline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Disabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Disabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn NeedsRemediation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NeedsRemediation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn LicenseIssue(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LicenseIssue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Modified(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Modified)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Tampered(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tampered)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn DependencyIssue(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DependencyIssue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Servicing(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Servicing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn DeploymentInProgress(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeploymentInProgress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsPartiallyStaged(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackageStatus2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPartiallyStaged)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageStatus {}
impl ::core::fmt::Debug for PackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageStatus;{5fe74f71-a365-4c09-a02d-046d525ea1da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageStatus {
    type Vtable = IPackageStatus_Vtbl;
    const IID: ::windows::core::GUID = <IPackageStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageStatus {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStatus";
}
impl ::core::convert::From<PackageStatus> for ::windows::core::IUnknown {
    fn from(value: PackageStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStatus> for ::windows::core::IUnknown {
    fn from(value: &PackageStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageStatus> for ::windows::core::IInspectable {
    fn from(value: PackageStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStatus> for ::windows::core::IInspectable {
    fn from(value: &PackageStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageStatus {}
unsafe impl ::core::marker::Sync for PackageStatus {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageStatusChangedEventArgs(::windows::core::IUnknown);
impl PackageStatusChangedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageStatusChangedEventArgs {}
impl ::core::fmt::Debug for PackageStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageStatusChangedEventArgs;{437d714d-bd80-4a70-bc50-f6e796509575})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageStatusChangedEventArgs {
    type Vtable = IPackageStatusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPackageStatusChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStatusChangedEventArgs";
}
impl ::core::convert::From<PackageStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PackageStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PackageStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PackageStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PackageStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for PackageStatusChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageUninstallingEventArgs(::windows::core::IUnknown);
impl PackageUninstallingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Progress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsComplete)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageUninstallingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageUninstallingEventArgs {}
impl ::core::fmt::Debug for PackageUninstallingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUninstallingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageUninstallingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageUninstallingEventArgs;{4443aa52-ab22-44cd-82bb-4ec9b827367a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageUninstallingEventArgs {
    type Vtable = IPackageUninstallingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPackageUninstallingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUninstallingEventArgs";
}
impl ::core::convert::From<PackageUninstallingEventArgs> for ::windows::core::IUnknown {
    fn from(value: PackageUninstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUninstallingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PackageUninstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageUninstallingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageUninstallingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageUninstallingEventArgs> for ::windows::core::IInspectable {
    fn from(value: PackageUninstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUninstallingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PackageUninstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageUninstallingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageUninstallingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageUninstallingEventArgs {}
unsafe impl ::core::marker::Sync for PackageUninstallingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageUpdateAvailability(pub i32);
impl PackageUpdateAvailability {
    pub const Unknown: Self = Self(0i32);
    pub const NoUpdates: Self = Self(1i32);
    pub const Available: Self = Self(2i32);
    pub const Required: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl ::core::marker::Copy for PackageUpdateAvailability {}
impl ::core::clone::Clone for PackageUpdateAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageUpdateAvailability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PackageUpdateAvailability {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageUpdateAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUpdateAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageUpdateAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageUpdateAvailability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageUpdateAvailabilityResult(::windows::core::IUnknown);
impl PackageUpdateAvailabilityResult {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Availability(&self) -> ::windows::core::Result<PackageUpdateAvailability> {
        let this = self;
        unsafe {
            let mut result__: PackageUpdateAvailability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Availability)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageUpdateAvailability>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageUpdateAvailabilityResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageUpdateAvailabilityResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageUpdateAvailabilityResult {}
impl ::core::fmt::Debug for PackageUpdateAvailabilityResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUpdateAvailabilityResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageUpdateAvailabilityResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageUpdateAvailabilityResult;{114e5009-199a-48a1-a079-313c45634a71})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageUpdateAvailabilityResult {
    type Vtable = IPackageUpdateAvailabilityResult_Vtbl;
    const IID: ::windows::core::GUID = <IPackageUpdateAvailabilityResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageUpdateAvailabilityResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUpdateAvailabilityResult";
}
impl ::core::convert::From<PackageUpdateAvailabilityResult> for ::windows::core::IUnknown {
    fn from(value: PackageUpdateAvailabilityResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUpdateAvailabilityResult> for ::windows::core::IUnknown {
    fn from(value: &PackageUpdateAvailabilityResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageUpdateAvailabilityResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageUpdateAvailabilityResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageUpdateAvailabilityResult> for ::windows::core::IInspectable {
    fn from(value: PackageUpdateAvailabilityResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUpdateAvailabilityResult> for ::windows::core::IInspectable {
    fn from(value: &PackageUpdateAvailabilityResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageUpdateAvailabilityResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageUpdateAvailabilityResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageUpdateAvailabilityResult {}
unsafe impl ::core::marker::Sync for PackageUpdateAvailabilityResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageUpdatingEventArgs(::windows::core::IUnknown);
impl PackageUpdatingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn SourcePackage(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourcePackage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn TargetPackage(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetPackage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Package>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Progress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsComplete)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageUpdatingEventArgs {}
impl ::core::fmt::Debug for PackageUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUpdatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageUpdatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageUpdatingEventArgs;{cd7b4228-fd74-443e-b114-23e677b0e86f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PackageUpdatingEventArgs {
    type Vtable = IPackageUpdatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPackageUpdatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUpdatingEventArgs";
}
impl ::core::convert::From<PackageUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: PackageUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUpdatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PackageUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: PackageUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUpdatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PackageUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageUpdatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for PackageUpdatingEventArgs {}
#[repr(C)]
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct PackageVersion {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for PackageVersion {}
impl ::core::clone::Clone for PackageVersion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PackageVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PackageVersion").field("Major", &self.Major).field("Minor", &self.Minor).field("Build", &self.Build).field("Revision", &self.Revision).finish()
    }
}
unsafe impl ::windows::core::Abi for PackageVersion {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageVersion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.PackageVersion;u2;u2;u2;u2)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PackageVersion {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PackageVersion>()) == 0 }
    }
}
impl ::core::cmp::Eq for PackageVersion {}
impl ::core::default::Default for PackageVersion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct StartupTask(::windows::core::IUnknown);
impl StartupTask {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestEnableAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTaskState>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestEnableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StartupTaskState>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Disable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Disable)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn State(&self) -> ::windows::core::Result<StartupTaskState> {
        let this = self;
        unsafe {
            let mut result__: StartupTaskState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StartupTaskState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TaskId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetForCurrentPackageAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StartupTask>>> {
        Self::IStartupTaskStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentPackageAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StartupTask>>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(taskid: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTask>> {
        Self::IStartupTaskStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsync)(::core::mem::transmute_copy(this), taskid.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StartupTask>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStartupTaskStatics<R, F: FnOnce(&IStartupTaskStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StartupTask, IStartupTaskStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StartupTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StartupTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StartupTask {}
impl ::core::fmt::Debug for StartupTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StartupTask").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StartupTask {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.StartupTask;{f75c23c8-b5f2-4f6c-88dd-36cb1d599d17})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StartupTask {
    type Vtable = IStartupTask_Vtbl;
    const IID: ::windows::core::GUID = <IStartupTask as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StartupTask {
    const NAME: &'static str = "Windows.ApplicationModel.StartupTask";
}
impl ::core::convert::From<StartupTask> for ::windows::core::IUnknown {
    fn from(value: StartupTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTask> for ::windows::core::IUnknown {
    fn from(value: &StartupTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StartupTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StartupTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StartupTask> for ::windows::core::IInspectable {
    fn from(value: StartupTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTask> for ::windows::core::IInspectable {
    fn from(value: &StartupTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StartupTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StartupTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StartupTask {}
unsafe impl ::core::marker::Sync for StartupTask {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StartupTaskState(pub i32);
impl StartupTaskState {
    pub const Disabled: Self = Self(0i32);
    pub const DisabledByUser: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const EnabledByPolicy: Self = Self(4i32);
}
impl ::core::marker::Copy for StartupTaskState {}
impl ::core::clone::Clone for StartupTaskState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StartupTaskState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StartupTaskState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StartupTaskState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StartupTaskState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StartupTaskState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.StartupTaskState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct SuspendingDeferral(::windows::core::IUnknown);
impl SuspendingDeferral {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for SuspendingDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SuspendingDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SuspendingDeferral {}
impl ::core::fmt::Debug for SuspendingDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SuspendingDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SuspendingDeferral;{59140509-8bc9-4eb4-b636-dabdc4f46f66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SuspendingDeferral {
    type Vtable = ISuspendingDeferral_Vtbl;
    const IID: ::windows::core::GUID = <ISuspendingDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SuspendingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingDeferral";
}
impl ::core::convert::From<SuspendingDeferral> for ::windows::core::IUnknown {
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingDeferral> for ::windows::core::IUnknown {
    fn from(value: &SuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuspendingDeferral> for ::windows::core::IInspectable {
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingDeferral> for ::windows::core::IInspectable {
    fn from(value: &SuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SuspendingDeferral> for ISuspendingDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: SuspendingDeferral) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SuspendingDeferral> for ISuspendingDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingDeferral) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISuspendingDeferral> for SuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ISuspendingDeferral> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISuspendingDeferral> for &SuspendingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ISuspendingDeferral> {
        ::core::convert::TryInto::<ISuspendingDeferral>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SuspendingDeferral {}
unsafe impl ::core::marker::Sync for SuspendingDeferral {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct SuspendingEventArgs(::windows::core::IUnknown);
impl SuspendingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn SuspendingOperation(&self) -> ::windows::core::Result<SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SuspendingOperation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SuspendingOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for SuspendingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SuspendingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SuspendingEventArgs {}
impl ::core::fmt::Debug for SuspendingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SuspendingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SuspendingEventArgs;{96061c05-2dba-4d08-b0bd-2b30a131c6aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SuspendingEventArgs {
    type Vtable = ISuspendingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISuspendingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SuspendingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingEventArgs";
}
impl ::core::convert::From<SuspendingEventArgs> for ::windows::core::IUnknown {
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuspendingEventArgs> for ::windows::core::IInspectable {
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SuspendingEventArgs> for ISuspendingEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SuspendingEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SuspendingEventArgs> for ISuspendingEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISuspendingEventArgs> for SuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISuspendingEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISuspendingEventArgs> for &SuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISuspendingEventArgs> {
        ::core::convert::TryInto::<ISuspendingEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SuspendingEventArgs {}
unsafe impl ::core::marker::Sync for SuspendingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct SuspendingOperation(::windows::core::IUnknown);
impl SuspendingOperation {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SuspendingDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Deadline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for SuspendingOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SuspendingOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SuspendingOperation {}
impl ::core::fmt::Debug for SuspendingOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SuspendingOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SuspendingOperation;{9da4ca41-20e1-4e9b-9f65-a9f435340c3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SuspendingOperation {
    type Vtable = ISuspendingOperation_Vtbl;
    const IID: ::windows::core::GUID = <ISuspendingOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SuspendingOperation {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingOperation";
}
impl ::core::convert::From<SuspendingOperation> for ::windows::core::IUnknown {
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingOperation> for ::windows::core::IUnknown {
    fn from(value: &SuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuspendingOperation> for ::windows::core::IInspectable {
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingOperation> for ::windows::core::IInspectable {
    fn from(value: &SuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SuspendingOperation> for ISuspendingOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: SuspendingOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SuspendingOperation> for ISuspendingOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISuspendingOperation> for SuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ISuspendingOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISuspendingOperation> for &SuspendingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ISuspendingOperation> {
        ::core::convert::TryInto::<ISuspendingOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SuspendingOperation {}
unsafe impl ::core::marker::Sync for SuspendingOperation {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
