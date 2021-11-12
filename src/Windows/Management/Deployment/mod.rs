#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Management_Deployment_Preview")]
pub mod Preview;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AddPackageByAppInstallerOptions(pub u32);
impl AddPackageByAppInstallerOptions {
    pub const None: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(0u32);
    pub const InstallAllResources: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(32u32);
    pub const ForceTargetAppShutdown: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(64u32);
    pub const RequiredContentGroupOnly: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(256u32);
    pub const LimitToExistingPackages: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(512u32);
}
impl ::core::convert::From<u32> for AddPackageByAppInstallerOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AddPackageByAppInstallerOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AddPackageByAppInstallerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.AddPackageByAppInstallerOptions;u4)");
}
impl ::windows::core::DefaultType for AddPackageByAppInstallerOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AddPackageByAppInstallerOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AddPackageByAppInstallerOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AddPackageOptions(pub ::windows::core::IInspectable);
impl AddPackageOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AddPackageOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    pub fn TargetVolume(&self) -> ::windows::core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    pub fn SetTargetVolume<'a, Param0: ::windows::core::IntoParam<'a, PackageVolume>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RelatedPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExternalLocationUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StubPackageOption(&self) -> ::windows::core::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__: StubPackageOption = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StubPackageOption>(result__)
        }
    }
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DeveloperMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForceTargetAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RequiredContentGroupOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RetainFilesOnFailure(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRetainFilesOnFailure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AddPackageOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AddPackageOptions;{05cee018-f68f-422b-95a4-66679ec77fc0})");
}
unsafe impl ::windows::core::Interface for AddPackageOptions {
    type Vtable = IAddPackageOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05cee018_f68f_422b_95a4_66679ec77fc0);
}
impl ::windows::core::RuntimeName for AddPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AddPackageOptions";
}
impl ::core::convert::From<AddPackageOptions> for ::windows::core::IUnknown {
    fn from(value: AddPackageOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AddPackageOptions> for ::windows::core::IUnknown {
    fn from(value: &AddPackageOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AddPackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AddPackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AddPackageOptions> for ::windows::core::IInspectable {
    fn from(value: AddPackageOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AddPackageOptions> for ::windows::core::IInspectable {
    fn from(value: &AddPackageOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AddPackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AddPackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AddPackageOptions {}
unsafe impl ::core::marker::Sync for AddPackageOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppInstallerManager(pub ::windows::core::IInspectable);
impl AppInstallerManager {
    pub fn SetAutoUpdateSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, AutoUpdateSettingsOptions>>(&self, packagefamilyname: Param0, appinstallerinfo: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), appinstallerinfo.into_param().abi()).ok() }
    }
    pub fn ClearAutoUpdateSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PauseAutoUpdatesUntil<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, packagefamilyname: Param0, datetime: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), datetime.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallerManager>(result__)
        })
    }
    pub fn GetForSystem() -> ::windows::core::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallerManager>(result__)
        })
    }
    pub fn IAppInstallerManagerStatics<R, F: FnOnce(&IAppInstallerManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInstallerManager, IAppInstallerManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallerManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AppInstallerManager;{e7ee21c3-2103-53ee-9b18-68afeab0033d})");
}
unsafe impl ::windows::core::Interface for AppInstallerManager {
    type Vtable = IAppInstallerManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7ee21c3_2103_53ee_9b18_68afeab0033d);
}
impl ::windows::core::RuntimeName for AppInstallerManager {
    const NAME: &'static str = "Windows.Management.Deployment.AppInstallerManager";
}
impl ::core::convert::From<AppInstallerManager> for ::windows::core::IUnknown {
    fn from(value: AppInstallerManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppInstallerManager> for ::windows::core::IUnknown {
    fn from(value: &AppInstallerManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstallerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppInstallerManager> for ::windows::core::IInspectable {
    fn from(value: AppInstallerManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppInstallerManager> for ::windows::core::IInspectable {
    fn from(value: &AppInstallerManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstallerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppInstallerManager {}
unsafe impl ::core::marker::Sync for AppInstallerManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AutoUpdateSettingsOptions(pub ::windows::core::IInspectable);
impl AutoUpdateSettingsOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutoUpdateSettingsOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Version(&self) -> ::windows::core::Result<super::super::ApplicationModel::PackageVersion> {
        let this = self;
        unsafe {
            let mut result__: super::super::ApplicationModel::PackageVersion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::PackageVersion>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::PackageVersion>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AppInstallerUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetAppInstallerUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn OnLaunch(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetOnLaunch(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HoursBetweenUpdateChecks(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetHoursBetweenUpdateChecks(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ShowPrompt(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShowPrompt(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn UpdateBlocksActivation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetUpdateBlocksActivation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AutomaticBackgroundTask(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutomaticBackgroundTask(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsAutoRepairEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAutoRepairEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn UpdateUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RepairUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn CreateFromAppInstallerInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::AppInstallerInfo>>(appinstallerinfo: Param0) -> ::windows::core::Result<AutoUpdateSettingsOptions> {
        Self::IAutoUpdateSettingsOptionsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appinstallerinfo.into_param().abi(), &mut result__).from_abi::<AutoUpdateSettingsOptions>(result__)
        })
    }
    pub fn IAutoUpdateSettingsOptionsStatics<R, F: FnOnce(&IAutoUpdateSettingsOptionsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AutoUpdateSettingsOptions, IAutoUpdateSettingsOptionsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AutoUpdateSettingsOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AutoUpdateSettingsOptions;{67491d87-35e1-512a-8968-1ae88d1be6d3})");
}
unsafe impl ::windows::core::Interface for AutoUpdateSettingsOptions {
    type Vtable = IAutoUpdateSettingsOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67491d87_35e1_512a_8968_1ae88d1be6d3);
}
impl ::windows::core::RuntimeName for AutoUpdateSettingsOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AutoUpdateSettingsOptions";
}
impl ::core::convert::From<AutoUpdateSettingsOptions> for ::windows::core::IUnknown {
    fn from(value: AutoUpdateSettingsOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutoUpdateSettingsOptions> for ::windows::core::IUnknown {
    fn from(value: &AutoUpdateSettingsOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutoUpdateSettingsOptions> for ::windows::core::IInspectable {
    fn from(value: AutoUpdateSettingsOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutoUpdateSettingsOptions> for ::windows::core::IInspectable {
    fn from(value: &AutoUpdateSettingsOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AutoUpdateSettingsOptions {}
unsafe impl ::core::marker::Sync for AutoUpdateSettingsOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CreateSharedPackageContainerOptions(pub ::windows::core::IInspectable);
impl CreateSharedPackageContainerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CreateSharedPackageContainerOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Members(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>(result__)
        }
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CreateCollisionOption(&self) -> ::windows::core::Result<SharedPackageContainerCreationCollisionOptions> {
        let this = self;
        unsafe {
            let mut result__: SharedPackageContainerCreationCollisionOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerCreationCollisionOptions>(result__)
        }
    }
    pub fn SetCreateCollisionOption(&self, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CreateSharedPackageContainerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.CreateSharedPackageContainerOptions;{c2ab6ece-f664-5c8e-a4b3-2a33276d3dde})");
}
unsafe impl ::windows::core::Interface for CreateSharedPackageContainerOptions {
    type Vtable = ICreateSharedPackageContainerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2ab6ece_f664_5c8e_a4b3_2a33276d3dde);
}
impl ::windows::core::RuntimeName for CreateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerOptions";
}
impl ::core::convert::From<CreateSharedPackageContainerOptions> for ::windows::core::IUnknown {
    fn from(value: CreateSharedPackageContainerOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CreateSharedPackageContainerOptions> for ::windows::core::IUnknown {
    fn from(value: &CreateSharedPackageContainerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CreateSharedPackageContainerOptions> for ::windows::core::IInspectable {
    fn from(value: CreateSharedPackageContainerOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CreateSharedPackageContainerOptions> for ::windows::core::IInspectable {
    fn from(value: &CreateSharedPackageContainerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CreateSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for CreateSharedPackageContainerOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CreateSharedPackageContainerResult(pub ::windows::core::IInspectable);
impl CreateSharedPackageContainerResult {
    pub fn Container(&self) -> ::windows::core::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainer>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SharedPackageContainerOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerOperationStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CreateSharedPackageContainerResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.CreateSharedPackageContainerResult;{ce8810bf-151c-5707-b936-497e564afc7a})");
}
unsafe impl ::windows::core::Interface for CreateSharedPackageContainerResult {
    type Vtable = ICreateSharedPackageContainerResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce8810bf_151c_5707_b936_497e564afc7a);
}
impl ::windows::core::RuntimeName for CreateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerResult";
}
impl ::core::convert::From<CreateSharedPackageContainerResult> for ::windows::core::IUnknown {
    fn from(value: CreateSharedPackageContainerResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CreateSharedPackageContainerResult> for ::windows::core::IUnknown {
    fn from(value: &CreateSharedPackageContainerResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CreateSharedPackageContainerResult> for ::windows::core::IInspectable {
    fn from(value: CreateSharedPackageContainerResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CreateSharedPackageContainerResult> for ::windows::core::IInspectable {
    fn from(value: &CreateSharedPackageContainerResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CreateSharedPackageContainerResult {}
unsafe impl ::core::marker::Sync for CreateSharedPackageContainerResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeleteSharedPackageContainerOptions(pub ::windows::core::IInspectable);
impl DeleteSharedPackageContainerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeleteSharedPackageContainerOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllUsers(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllUsers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DeleteSharedPackageContainerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeleteSharedPackageContainerOptions;{9d81865f-986e-5138-8b5d-384d8e66ed6c})");
}
unsafe impl ::windows::core::Interface for DeleteSharedPackageContainerOptions {
    type Vtable = IDeleteSharedPackageContainerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d81865f_986e_5138_8b5d_384d8e66ed6c);
}
impl ::windows::core::RuntimeName for DeleteSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerOptions";
}
impl ::core::convert::From<DeleteSharedPackageContainerOptions> for ::windows::core::IUnknown {
    fn from(value: DeleteSharedPackageContainerOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeleteSharedPackageContainerOptions> for ::windows::core::IUnknown {
    fn from(value: &DeleteSharedPackageContainerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeleteSharedPackageContainerOptions> for ::windows::core::IInspectable {
    fn from(value: DeleteSharedPackageContainerOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeleteSharedPackageContainerOptions> for ::windows::core::IInspectable {
    fn from(value: &DeleteSharedPackageContainerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeleteSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for DeleteSharedPackageContainerOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeleteSharedPackageContainerResult(pub ::windows::core::IInspectable);
impl DeleteSharedPackageContainerResult {
    pub fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SharedPackageContainerOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerOperationStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeleteSharedPackageContainerResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeleteSharedPackageContainerResult;{35398884-5736-517b-85bc-e598c81ab284})");
}
unsafe impl ::windows::core::Interface for DeleteSharedPackageContainerResult {
    type Vtable = IDeleteSharedPackageContainerResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35398884_5736_517b_85bc_e598c81ab284);
}
impl ::windows::core::RuntimeName for DeleteSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerResult";
}
impl ::core::convert::From<DeleteSharedPackageContainerResult> for ::windows::core::IUnknown {
    fn from(value: DeleteSharedPackageContainerResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeleteSharedPackageContainerResult> for ::windows::core::IUnknown {
    fn from(value: &DeleteSharedPackageContainerResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeleteSharedPackageContainerResult> for ::windows::core::IInspectable {
    fn from(value: DeleteSharedPackageContainerResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeleteSharedPackageContainerResult> for ::windows::core::IInspectable {
    fn from(value: &DeleteSharedPackageContainerResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeleteSharedPackageContainerResult {}
unsafe impl ::core::marker::Sync for DeleteSharedPackageContainerResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeploymentOptions(pub u32);
impl DeploymentOptions {
    pub const None: DeploymentOptions = DeploymentOptions(0u32);
    pub const ForceApplicationShutdown: DeploymentOptions = DeploymentOptions(1u32);
    pub const DevelopmentMode: DeploymentOptions = DeploymentOptions(2u32);
    pub const InstallAllResources: DeploymentOptions = DeploymentOptions(32u32);
    pub const ForceTargetApplicationShutdown: DeploymentOptions = DeploymentOptions(64u32);
    pub const RequiredContentGroupOnly: DeploymentOptions = DeploymentOptions(256u32);
    pub const ForceUpdateFromAnyVersion: DeploymentOptions = DeploymentOptions(262144u32);
    pub const RetainFilesOnFailure: DeploymentOptions = DeploymentOptions(2097152u32);
    pub const StageInPlace: DeploymentOptions = DeploymentOptions(4194304u32);
}
impl ::core::convert::From<u32> for DeploymentOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DeploymentOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeploymentOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentOptions;u4)");
}
impl ::windows::core::DefaultType for DeploymentOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DeploymentOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DeploymentOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DeploymentOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DeploymentOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DeploymentOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DeploymentProgress {
    pub state: DeploymentProgressState,
    pub percentage: u32,
}
impl DeploymentProgress {}
impl ::core::default::Default for DeploymentProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DeploymentProgress {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DeploymentProgress").field("state", &self.state).field("percentage", &self.percentage).finish()
    }
}
impl ::core::cmp::PartialEq for DeploymentProgress {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.percentage == other.percentage
    }
}
impl ::core::cmp::Eq for DeploymentProgress {}
unsafe impl ::windows::core::Abi for DeploymentProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeploymentProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Management.Deployment.DeploymentProgress;enum(Windows.Management.Deployment.DeploymentProgressState;i4);u4)");
}
impl ::windows::core::DefaultType for DeploymentProgress {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeploymentProgressState(pub i32);
impl DeploymentProgressState {
    pub const Queued: DeploymentProgressState = DeploymentProgressState(0i32);
    pub const Processing: DeploymentProgressState = DeploymentProgressState(1i32);
}
impl ::core::convert::From<i32> for DeploymentProgressState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DeploymentProgressState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeploymentProgressState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentProgressState;i4)");
}
impl ::windows::core::DefaultType for DeploymentProgressState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeploymentResult(pub ::windows::core::IInspectable);
impl DeploymentResult {
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn IsRegistered(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeploymentResult2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeploymentResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeploymentResult;{2563b9ae-b77d-4c1f-8a7b-20e6ad515ef3})");
}
unsafe impl ::windows::core::Interface for DeploymentResult {
    type Vtable = IDeploymentResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2563b9ae_b77d_4c1f_8a7b_20e6ad515ef3);
}
impl ::windows::core::RuntimeName for DeploymentResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeploymentResult";
}
impl ::core::convert::From<DeploymentResult> for ::windows::core::IUnknown {
    fn from(value: DeploymentResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeploymentResult> for ::windows::core::IUnknown {
    fn from(value: &DeploymentResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeploymentResult> for ::windows::core::IInspectable {
    fn from(value: DeploymentResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeploymentResult> for ::windows::core::IInspectable {
    fn from(value: &DeploymentResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeploymentResult {}
unsafe impl ::core::marker::Sync for DeploymentResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FindSharedPackageContainerOptions(pub ::windows::core::IInspectable);
impl FindSharedPackageContainerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FindSharedPackageContainerOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
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
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FindSharedPackageContainerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.FindSharedPackageContainerOptions;{b40fc8fe-8384-54cc-817d-ae09d3b6a606})");
}
unsafe impl ::windows::core::Interface for FindSharedPackageContainerOptions {
    type Vtable = IFindSharedPackageContainerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb40fc8fe_8384_54cc_817d_ae09d3b6a606);
}
impl ::windows::core::RuntimeName for FindSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.FindSharedPackageContainerOptions";
}
impl ::core::convert::From<FindSharedPackageContainerOptions> for ::windows::core::IUnknown {
    fn from(value: FindSharedPackageContainerOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FindSharedPackageContainerOptions> for ::windows::core::IUnknown {
    fn from(value: &FindSharedPackageContainerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FindSharedPackageContainerOptions> for ::windows::core::IInspectable {
    fn from(value: FindSharedPackageContainerOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FindSharedPackageContainerOptions> for ::windows::core::IInspectable {
    fn from(value: &FindSharedPackageContainerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FindSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for FindSharedPackageContainerOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAddPackageOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAddPackageOptions {
    type Vtable = IAddPackageOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05cee018_f68f_422b_95a4_66679ec77fc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StubPackageOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StubPackageOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppInstallerManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppInstallerManager {
    type Vtable = IAppInstallerManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7ee21c3_2103_53ee_9b18_68afeab0033d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appinstallerinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, datetime: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppInstallerManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppInstallerManagerStatics {
    type Vtable = IAppInstallerManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc95a6ed5_fc59_5336_9b2e_2b07c5e61434);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManagerStatics_abi(
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
pub struct IAutoUpdateSettingsOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutoUpdateSettingsOptions {
    type Vtable = IAutoUpdateSettingsOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67491d87_35e1_512a_8968_1ae88d1be6d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::ApplicationModel::PackageVersion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::ApplicationModel::PackageVersion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptionsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutoUpdateSettingsOptionsStatics {
    type Vtable = IAutoUpdateSettingsOptionsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x887b337d_0c05_54d0_bd49_3bb7a2c084cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptionsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appinstallerinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateSharedPackageContainerOptions {
    type Vtable = ICreateSharedPackageContainerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2ab6ece_f664_5c8e_a4b3_2a33276d3dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SharedPackageContainerCreationCollisionOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateSharedPackageContainerResult {
    type Vtable = ICreateSharedPackageContainerResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce8810bf_151c_5707_b936_497e564afc7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeleteSharedPackageContainerOptions {
    type Vtable = IDeleteSharedPackageContainerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d81865f_986e_5138_8b5d_384d8e66ed6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerOptions_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeleteSharedPackageContainerResult {
    type Vtable = IDeleteSharedPackageContainerResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35398884_5736_517b_85bc_e598c81ab284);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeploymentResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeploymentResult {
    type Vtable = IDeploymentResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2563b9ae_b77d_4c1f_8a7b_20e6ad515ef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeploymentResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeploymentResult2 {
    type Vtable = IDeploymentResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc0e715c_5a01_4bd7_bcf1_381c8c82e04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult2_abi(
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
pub struct IFindSharedPackageContainerOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFindSharedPackageContainerOptions {
    type Vtable = IFindSharedPackageContainerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb40fc8fe_8384_54cc_817d_ae09d3b6a606);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindSharedPackageContainerOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageAllUserProvisioningOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageAllUserProvisioningOptions {
    type Vtable = IPackageAllUserProvisioningOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda35aa22_1de0_5d3e_99ff_d24f3118bf5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageAllUserProvisioningOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager {
    type Vtable = IPackageManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a7d4b65_5e8f_4fc7_a2e5_7f6925cb8b53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifesturi: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagestate: PackageState) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager10(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager10 {
    type Vtable = IPackageManager10_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7d7d07e_2e66_4093_aed5_e093ed87b3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager10_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mainpackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager2 {
    type Vtable = IPackageManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7aad08d_0840_46f2_b5d8_cad47693a095);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, removaloptions: RemovalOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mainpackagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dependencypackagefullnames: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager3 {
    type Vtable = IPackageManager3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaad9948_36f1_41a7_9188_bc263e0dcb72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagestorepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, status: PackageStatus) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifesturi: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, volumename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, volume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, volume: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, status: PackageStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagevolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagevolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager4 {
    type Vtable = IPackageManager4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c719963_bab6_46bf_8ff7_da4719230ae6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager4_abi(
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
pub struct IPackageManager5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager5 {
    type Vtable = IPackageManager5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x711f3117_1afd_4313_978c_9bb6e1b864a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, externalpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, externalpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mainpackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dependencypackagefamilynames: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager6 {
    type Vtable = IPackageManager6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0847e909_53cd_4e4f_832e_57d180f6e447);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appinstallerfileuri: ::windows::core::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appinstallerfileuri: ::windows::core::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, options: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, packageuristoinstall: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, options: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, packageuristoinstall: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager7(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager7 {
    type Vtable = IPackageManager7_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf28654f4_2ba7_4b80_88d6_be15f9a23fba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager7_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, packageuristoinstall: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager8(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager8 {
    type Vtable = IPackageManager8_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8575330_1298_4ee2_80ee_7f659c5d2782);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager8_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager9(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManager9 {
    type Vtable = IPackageManager9_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aa79035_cc71_4b2e_80a6_c7041d8579a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager9_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageuri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifesturi: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullnames: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usestub: PackageStubPreference) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut PackageStubPreference) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManagerDebugSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageManagerDebugSettings {
    type Vtable = IPackageManagerDebugSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a611683_a988_4fcf_8f0f_ce175898e8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManagerDebugSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, package: ::windows::core::RawPtr, contentgroupname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, package: ::windows::core::RawPtr, contentgroupname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageUserInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageUserInformation {
    type Vtable = IPackageUserInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6383423_fa09_4cbc_9055_15ca275e2e7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUserInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PackageInstallState) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageVolume(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageVolume {
    type Vtable = IPackageVolume_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2672c3_1a40_4450_9739_2ace2e898853);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagetypes: PackageTypes, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagetypes: PackageTypes, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageVolume2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPackageVolume2 {
    type Vtable = IPackageVolume2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46abcf2e_9dd4_47a2_ab8c_c6408349bcd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRegisterPackageOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRegisterPackageOptions {
    type Vtable = IRegisterPackageOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x677112a7_50d4_496c_8415_0602b4c6d3bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisterPackageOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISharedPackageContainer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISharedPackageContainer {
    type Vtable = ISharedPackageContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x177f1aa9_151e_5ef7_b1d9_2fba0b4b0d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISharedPackageContainerManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISharedPackageContainerManager {
    type Vtable = ISharedPackageContainerManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe353068_1ef7_5ac8_ab3f_0b9f612f0274);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISharedPackageContainerManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISharedPackageContainerManagerStatics {
    type Vtable = ISharedPackageContainerManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ef56348_838a_5f55_a89e_1198a2c627e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usersid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISharedPackageContainerMember(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISharedPackageContainerMember {
    type Vtable = ISharedPackageContainerMember_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe0d0438_43c9_5426_b89c_f79bf85ddff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMember_abi(
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
pub struct ISharedPackageContainerMemberFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISharedPackageContainerMemberFactory {
    type Vtable = ISharedPackageContainerMemberFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49b0ceeb_498f_5a62_b738_b3ca0d436704);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMemberFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStagePackageOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStagePackageOptions {
    type Vtable = IStagePackageOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b110c9c_b95d_4c56_bd36_6d656800d06b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStagePackageOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StubPackageOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StubPackageOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUpdateSharedPackageContainerOptions {
    type Vtable = IUpdateSharedPackageContainerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80672e83_7194_59f9_b5b9_daa5375f130a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerOptions_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUpdateSharedPackageContainerResult {
    type Vtable = IUpdateSharedPackageContainerResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa407df7_c72d_5458_aea3_4645b6a8ee99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PackageAllUserProvisioningOptions(pub ::windows::core::IInspectable);
impl PackageAllUserProvisioningOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PackageAllUserProvisioningOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProjectionOrderPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PackageAllUserProvisioningOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageAllUserProvisioningOptions;{da35aa22-1de0-5d3e-99ff-d24f3118bf5e})");
}
unsafe impl ::windows::core::Interface for PackageAllUserProvisioningOptions {
    type Vtable = IPackageAllUserProvisioningOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda35aa22_1de0_5d3e_99ff_d24f3118bf5e);
}
impl ::windows::core::RuntimeName for PackageAllUserProvisioningOptions {
    const NAME: &'static str = "Windows.Management.Deployment.PackageAllUserProvisioningOptions";
}
impl ::core::convert::From<PackageAllUserProvisioningOptions> for ::windows::core::IUnknown {
    fn from(value: PackageAllUserProvisioningOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PackageAllUserProvisioningOptions> for ::windows::core::IUnknown {
    fn from(value: &PackageAllUserProvisioningOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PackageAllUserProvisioningOptions> for ::windows::core::IInspectable {
    fn from(value: PackageAllUserProvisioningOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PackageAllUserProvisioningOptions> for ::windows::core::IInspectable {
    fn from(value: &PackageAllUserProvisioningOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PackageAllUserProvisioningOptions {}
unsafe impl ::core::marker::Sync for PackageAllUserProvisioningOptions {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageInstallState(pub i32);
impl PackageInstallState {
    pub const NotInstalled: PackageInstallState = PackageInstallState(0i32);
    pub const Staged: PackageInstallState = PackageInstallState(1i32);
    pub const Installed: PackageInstallState = PackageInstallState(2i32);
    pub const Paused: PackageInstallState = PackageInstallState(6i32);
}
impl ::core::convert::From<i32> for PackageInstallState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageInstallState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageInstallState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageInstallState;i4)");
}
impl ::windows::core::DefaultType for PackageInstallState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PackageManager(pub ::windows::core::IInspectable);
impl PackageManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PackageManager, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AddPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn UpdatePackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StagePackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RegisterPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, manifesturi: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagename: Param0, packagepublisher: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdNamePublisher<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagename: Param1, packagepublisher: Param2) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindUsers<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageUserInformation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<PackageUserInformation>>(result__)
        }
    }
    pub fn SetPackageState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0, packagestate: PackageState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), packagestate).ok() }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackageByPackageFullName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0) -> ::windows::core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::ApplicationModel::Package>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CleanupPackageForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagename: Param0, usersecurityid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), packagename.into_param().abi(), usersecurityid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagefamilyname: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackageByUserSecurityIdPackageFullName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagefullname: Param1) -> ::windows::core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::ApplicationModel::Package>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0, removaloptions: RemovalOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), removaloptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StagePackageWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RegisterPackageByFullNameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, mainpackagefullname: Param0, dependencypackagefullnames: Param1, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mainpackagefullname.into_param().abi(), dependencypackagefullnames.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisherWithPackageTypes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagename: Param0, packagepublisher: Param1, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagename: Param1, packagepublisher: Param2, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagename.into_param().abi(), packagepublisher.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagefamilyname: Param1, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefamilyname.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StageUserDataAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AddPackageVolumeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagestorepath: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PackageVolume>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagestorepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PackageVolume>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AddPackageToVolumeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Param3: ::windows::core::IntoParam<'a, PackageVolume>>(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn ClearPackageStatus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0, status: PackageStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), status).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RegisterPackageWithAppDataVolumeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Param3: ::windows::core::IntoParam<'a, PackageVolume>>(
        &self,
        manifesturi: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        appdatavolume: Param3,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, appdatavolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn FindPackageVolumeByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, volumename: Param0) -> ::windows::core::Result<PackageVolume> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), volumename.into_param().abi(), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindPackageVolumes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageVolume>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<PackageVolume>>(result__)
        }
    }
    pub fn GetDefaultPackageVolume(&self) -> ::windows::core::Result<PackageVolume> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MovePackageToVolumeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, PackageVolume>>(&self, packagefullname: Param0, deploymentoptions: DeploymentOptions, targetvolume: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageVolumeAsync<'a, Param0: ::windows::core::IntoParam<'a, PackageVolume>>(&self, volume: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), volume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn SetDefaultPackageVolume<'a, Param0: ::windows::core::IntoParam<'a, PackageVolume>>(&self, volume: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), volume.into_param().abi()).ok() }
    }
    pub fn SetPackageStatus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0, status: PackageStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), status).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPackageVolumeOfflineAsync<'a, Param0: ::windows::core::IntoParam<'a, PackageVolume>>(&self, packagevolume: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), packagevolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPackageVolumeOnlineAsync<'a, Param0: ::windows::core::IntoParam<'a, PackageVolume>>(&self, packagevolume: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), packagevolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StagePackageToVolumeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Param3: ::windows::core::IntoParam<'a, PackageVolume>>(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StageUserDataWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetPackageVolumesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageVolume>>> {
        let this = &::windows::core::Interface::cast::<IPackageManager4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageVolume>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AddPackageToVolumeAndOptionalPackagesAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::core::IntoParam<'a, PackageVolume>,
        Param4: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        externalpackageuris: Param5,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), externalpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StagePackageToVolumeAndOptionalPackagesAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::core::IntoParam<'a, PackageVolume>,
        Param4: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        externalpackageuris: Param5,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), externalpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RegisterPackageByFamilyNameAndOptionalPackagesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param3: ::windows::core::IntoParam<'a, PackageVolume>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(
        &self,
        mainpackagefamilyname: Param0,
        dependencypackagefamilynames: Param1,
        deploymentoptions: DeploymentOptions,
        appdatavolume: Param3,
        optionalpackagefamilynames: Param4,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mainpackagefamilyname.into_param().abi(), dependencypackagefamilynames.into_param().abi(), deploymentoptions, appdatavolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn DebugSettings(&self) -> ::windows::core::Result<PackageManagerDebugSettings> {
        let this = &::windows::core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageManagerDebugSettings>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProvisionPackageForAllUsersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AddPackageByAppInstallerFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, PackageVolume>>(&self, appinstallerfileuri: Param0, options: AddPackageByAppInstallerOptions, targetvolume: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appinstallerfileuri.into_param().abi(), options, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAddPackageByAppInstallerFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, PackageVolume>>(&self, appinstallerfileuri: Param0, options: AddPackageByAppInstallerOptions, targetvolume: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), appinstallerfileuri.into_param().abi(), options, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AddPackageToVolumeAndRelatedSetAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::core::IntoParam<'a, PackageVolume>,
        Param4: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param6: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        options: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        packageuristoinstall: Param5,
        relatedpackageuris: Param6,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), options, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), packageuristoinstall.into_param().abi(), relatedpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StagePackageToVolumeAndRelatedSetAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::core::IntoParam<'a, PackageVolume>,
        Param4: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param6: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        options: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        packageuristoinstall: Param5,
        relatedpackageuris: Param6,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), options, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), packageuristoinstall.into_param().abi(), relatedpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestAddPackageAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::core::IntoParam<'a, PackageVolume>,
        Param4: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        relatedpackageuris: Param5,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), relatedpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestAddPackageAndRelatedSetAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::core::IntoParam<'a, PackageVolume>,
        Param4: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param6: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        relatedpackageuris: Param5,
        packageuristoinstall: Param6,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), relatedpackageuris.into_param().abi(), packageuristoinstall.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeprovisionPackageForAllUsersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager8>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindProvisionedPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AddPackageByUriAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, AddPackageOptions>>(&self, packageuri: Param0, options: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StagePackageByUriAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, StagePackageOptions>>(&self, packageuri: Param0, options: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), packageuri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RegisterPackageByUriAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, RegisterPackageOptions>>(&self, manifesturi: Param0, options: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), manifesturi.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RegisterPackagesByFullNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, RegisterPackageOptions>>(&self, packagefullnames: Param0, options: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), packagefullnames.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn SetPackageStubPreference<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0, usestub: PackageStubPreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPackageManager9>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), usestub).ok() }
    }
    pub fn GetPackageStubPreference<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::core::Result<PackageStubPreference> {
        let this = &::windows::core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: PackageStubPreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<PackageStubPreference>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProvisionPackageForAllUsersWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PackageAllUserProvisioningOptions>>(&self, mainpackagefamilyname: Param0, options: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageManager10>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mainpackagefamilyname.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PackageManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageManager;{9a7d4b65-5e8f-4fc7-a2e5-7f6925cb8b53})");
}
unsafe impl ::windows::core::Interface for PackageManager {
    type Vtable = IPackageManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a7d4b65_5e8f_4fc7_a2e5_7f6925cb8b53);
}
impl ::windows::core::RuntimeName for PackageManager {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManager";
}
impl ::core::convert::From<PackageManager> for ::windows::core::IUnknown {
    fn from(value: PackageManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PackageManager> for ::windows::core::IUnknown {
    fn from(value: &PackageManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PackageManager> for ::windows::core::IInspectable {
    fn from(value: PackageManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PackageManager> for ::windows::core::IInspectable {
    fn from(value: &PackageManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PackageManager {}
unsafe impl ::core::marker::Sync for PackageManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PackageManagerDebugSettings(pub ::windows::core::IInspectable);
impl PackageManagerDebugSettings {
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn SetContentGroupStateAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Package>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, package: Param0, contentgroupname: Param1, state: super::super::ApplicationModel::PackageContentGroupState) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), package.into_param().abi(), contentgroupname.into_param().abi(), state, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn SetContentGroupStateWithPercentageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Package>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, package: Param0, contentgroupname: Param1, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), package.into_param().abi(), contentgroupname.into_param().abi(), state, completionpercentage, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PackageManagerDebugSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageManagerDebugSettings;{1a611683-a988-4fcf-8f0f-ce175898e8eb})");
}
unsafe impl ::windows::core::Interface for PackageManagerDebugSettings {
    type Vtable = IPackageManagerDebugSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a611683_a988_4fcf_8f0f_ce175898e8eb);
}
impl ::windows::core::RuntimeName for PackageManagerDebugSettings {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManagerDebugSettings";
}
impl ::core::convert::From<PackageManagerDebugSettings> for ::windows::core::IUnknown {
    fn from(value: PackageManagerDebugSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PackageManagerDebugSettings> for ::windows::core::IUnknown {
    fn from(value: &PackageManagerDebugSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageManagerDebugSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageManagerDebugSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PackageManagerDebugSettings> for ::windows::core::IInspectable {
    fn from(value: PackageManagerDebugSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PackageManagerDebugSettings> for ::windows::core::IInspectable {
    fn from(value: &PackageManagerDebugSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageManagerDebugSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageManagerDebugSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PackageManagerDebugSettings {}
unsafe impl ::core::marker::Sync for PackageManagerDebugSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageState(pub i32);
impl PackageState {
    pub const Normal: PackageState = PackageState(0i32);
    pub const LicenseInvalid: PackageState = PackageState(1i32);
    pub const Modified: PackageState = PackageState(2i32);
    pub const Tampered: PackageState = PackageState(3i32);
}
impl ::core::convert::From<i32> for PackageState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageState;i4)");
}
impl ::windows::core::DefaultType for PackageState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageStatus(pub u32);
impl PackageStatus {
    pub const OK: PackageStatus = PackageStatus(0u32);
    pub const LicenseIssue: PackageStatus = PackageStatus(1u32);
    pub const Modified: PackageStatus = PackageStatus(2u32);
    pub const Tampered: PackageStatus = PackageStatus(4u32);
    pub const Disabled: PackageStatus = PackageStatus(8u32);
}
impl ::core::convert::From<u32> for PackageStatus {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStatus;u4)");
}
impl ::windows::core::DefaultType for PackageStatus {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PackageStatus {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PackageStatus {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PackageStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PackageStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PackageStatus {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageStubPreference(pub i32);
impl PackageStubPreference {
    pub const Full: PackageStubPreference = PackageStubPreference(0i32);
    pub const Stub: PackageStubPreference = PackageStubPreference(1i32);
}
impl ::core::convert::From<i32> for PackageStubPreference {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageStubPreference {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageStubPreference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStubPreference;i4)");
}
impl ::windows::core::DefaultType for PackageStubPreference {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageTypes(pub u32);
impl PackageTypes {
    pub const None: PackageTypes = PackageTypes(0u32);
    pub const Main: PackageTypes = PackageTypes(1u32);
    pub const Framework: PackageTypes = PackageTypes(2u32);
    pub const Resource: PackageTypes = PackageTypes(4u32);
    pub const Bundle: PackageTypes = PackageTypes(8u32);
    pub const Xap: PackageTypes = PackageTypes(16u32);
    pub const Optional: PackageTypes = PackageTypes(32u32);
    pub const All: PackageTypes = PackageTypes(4294967295u32);
}
impl ::core::convert::From<u32> for PackageTypes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageTypes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageTypes;u4)");
}
impl ::windows::core::DefaultType for PackageTypes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PackageTypes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PackageTypes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PackageTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PackageTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PackageTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PackageUserInformation(pub ::windows::core::IInspectable);
impl PackageUserInformation {
    pub fn UserSecurityId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InstallState(&self) -> ::windows::core::Result<PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__: PackageInstallState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageInstallState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PackageUserInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageUserInformation;{f6383423-fa09-4cbc-9055-15ca275e2e7e})");
}
unsafe impl ::windows::core::Interface for PackageUserInformation {
    type Vtable = IPackageUserInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6383423_fa09_4cbc_9055_15ca275e2e7e);
}
impl ::windows::core::RuntimeName for PackageUserInformation {
    const NAME: &'static str = "Windows.Management.Deployment.PackageUserInformation";
}
impl ::core::convert::From<PackageUserInformation> for ::windows::core::IUnknown {
    fn from(value: PackageUserInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PackageUserInformation> for ::windows::core::IUnknown {
    fn from(value: &PackageUserInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageUserInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageUserInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PackageUserInformation> for ::windows::core::IInspectable {
    fn from(value: PackageUserInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PackageUserInformation> for ::windows::core::IInspectable {
    fn from(value: &PackageUserInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageUserInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageUserInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PackageUserInformation {}
unsafe impl ::core::marker::Sync for PackageUserInformation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PackageVolume(pub ::windows::core::IInspectable);
impl PackageVolume {
    pub fn IsOffline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsSystemVolume(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn MountPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PackageStorePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SupportsHardLinks(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagename: Param0, packagepublisher: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisherWithPackagesTypes<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagetypes: PackageTypes, packagename: Param1, packagepublisher: Param2) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), packagetypes, packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagetypes: PackageTypes, packagefamilyname: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), packagetypes, packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackageByPackageFullName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefullname: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdNamePublisher<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagename: Param1, packagepublisher: Param2) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagefamilyname: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes, packagename: Param2, packagepublisher: Param3) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagetypes, packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes, packagefamilyname: Param2) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagetypes, packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackageByUserSecurityIdPackageFullName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, usersecurityid: Param0, packagefullname: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    pub fn IsFullTrustPackageSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsAppxInstallSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetAvailableSpaceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u64>> {
        let this = &::windows::core::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u64>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PackageVolume {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageVolume;{cf2672c3-1a40-4450-9739-2ace2e898853})");
}
unsafe impl ::windows::core::Interface for PackageVolume {
    type Vtable = IPackageVolume_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2672c3_1a40_4450_9739_2ace2e898853);
}
impl ::windows::core::RuntimeName for PackageVolume {
    const NAME: &'static str = "Windows.Management.Deployment.PackageVolume";
}
impl ::core::convert::From<PackageVolume> for ::windows::core::IUnknown {
    fn from(value: PackageVolume) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PackageVolume> for ::windows::core::IUnknown {
    fn from(value: &PackageVolume) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PackageVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PackageVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PackageVolume> for ::windows::core::IInspectable {
    fn from(value: PackageVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PackageVolume> for ::windows::core::IInspectable {
    fn from(value: &PackageVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PackageVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PackageVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PackageVolume {}
unsafe impl ::core::marker::Sync for PackageVolume {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RegisterPackageOptions(pub ::windows::core::IInspectable);
impl RegisterPackageOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RegisterPackageOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    pub fn AppDataVolume(&self) -> ::windows::core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    pub fn SetAppDataVolume<'a, Param0: ::windows::core::IntoParam<'a, PackageVolume>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExternalLocationUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DeveloperMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForceTargetAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RegisterPackageOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.RegisterPackageOptions;{677112a7-50d4-496c-8415-0602b4c6d3bf})");
}
unsafe impl ::windows::core::Interface for RegisterPackageOptions {
    type Vtable = IRegisterPackageOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x677112a7_50d4_496c_8415_0602b4c6d3bf);
}
impl ::windows::core::RuntimeName for RegisterPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.RegisterPackageOptions";
}
impl ::core::convert::From<RegisterPackageOptions> for ::windows::core::IUnknown {
    fn from(value: RegisterPackageOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RegisterPackageOptions> for ::windows::core::IUnknown {
    fn from(value: &RegisterPackageOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RegisterPackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RegisterPackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RegisterPackageOptions> for ::windows::core::IInspectable {
    fn from(value: RegisterPackageOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RegisterPackageOptions> for ::windows::core::IInspectable {
    fn from(value: &RegisterPackageOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RegisterPackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RegisterPackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RegisterPackageOptions {}
unsafe impl ::core::marker::Sync for RegisterPackageOptions {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemovalOptions(pub u32);
impl RemovalOptions {
    pub const None: RemovalOptions = RemovalOptions(0u32);
    pub const PreserveApplicationData: RemovalOptions = RemovalOptions(4096u32);
    pub const PreserveRoamableApplicationData: RemovalOptions = RemovalOptions(128u32);
    pub const RemoveForAllUsers: RemovalOptions = RemovalOptions(524288u32);
}
impl ::core::convert::From<u32> for RemovalOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RemovalOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RemovalOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.RemovalOptions;u4)");
}
impl ::windows::core::DefaultType for RemovalOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for RemovalOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for RemovalOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for RemovalOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for RemovalOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for RemovalOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SharedPackageContainer(pub ::windows::core::IInspectable);
impl SharedPackageContainer {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMembers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>(result__)
        }
    }
    pub fn RemovePackageFamily<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, UpdateSharedPackageContainerOptions>>(&self, packagefamilyname: Param0, options: Param1) -> ::windows::core::Result<UpdateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<UpdateSharedPackageContainerResult>(result__)
        }
    }
    pub fn ResetData(&self) -> ::windows::core::Result<UpdateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UpdateSharedPackageContainerResult>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SharedPackageContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainer;{177f1aa9-151e-5ef7-b1d9-2fba0b4b0d17})");
}
unsafe impl ::windows::core::Interface for SharedPackageContainer {
    type Vtable = ISharedPackageContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x177f1aa9_151e_5ef7_b1d9_2fba0b4b0d17);
}
impl ::windows::core::RuntimeName for SharedPackageContainer {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainer";
}
impl ::core::convert::From<SharedPackageContainer> for ::windows::core::IUnknown {
    fn from(value: SharedPackageContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SharedPackageContainer> for ::windows::core::IUnknown {
    fn from(value: &SharedPackageContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SharedPackageContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SharedPackageContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SharedPackageContainer> for ::windows::core::IInspectable {
    fn from(value: SharedPackageContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SharedPackageContainer> for ::windows::core::IInspectable {
    fn from(value: &SharedPackageContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SharedPackageContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SharedPackageContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SharedPackageContainer {}
unsafe impl ::core::marker::Sync for SharedPackageContainer {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct SharedPackageContainerContract(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SharedPackageContainerCreationCollisionOptions(pub i32);
impl SharedPackageContainerCreationCollisionOptions {
    pub const FailIfExists: SharedPackageContainerCreationCollisionOptions = SharedPackageContainerCreationCollisionOptions(0i32);
    pub const MergeWithExisting: SharedPackageContainerCreationCollisionOptions = SharedPackageContainerCreationCollisionOptions(1i32);
    pub const ReplaceExisting: SharedPackageContainerCreationCollisionOptions = SharedPackageContainerCreationCollisionOptions(2i32);
}
impl ::core::convert::From<i32> for SharedPackageContainerCreationCollisionOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SharedPackageContainerCreationCollisionOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SharedPackageContainerCreationCollisionOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerCreationCollisionOptions;i4)");
}
impl ::windows::core::DefaultType for SharedPackageContainerCreationCollisionOptions {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SharedPackageContainerManager(pub ::windows::core::IInspectable);
impl SharedPackageContainerManager {
    pub fn CreateContainer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, CreateSharedPackageContainerOptions>>(&self, name: Param0, options: Param1) -> ::windows::core::Result<CreateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<CreateSharedPackageContainerResult>(result__)
        }
    }
    pub fn DeleteContainer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, DeleteSharedPackageContainerOptions>>(&self, id: Param0, options: Param1) -> ::windows::core::Result<DeleteSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<DeleteSharedPackageContainerResult>(result__)
        }
    }
    pub fn GetContainer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<SharedPackageContainer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContainers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SharedPackageContainer>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContainersWithOptions<'a, Param0: ::windows::core::IntoParam<'a, FindSharedPackageContainerOptions>>(&self, options: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SharedPackageContainer>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerManager>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(usersid: Param0) -> ::windows::core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), usersid.into_param().abi(), &mut result__).from_abi::<SharedPackageContainerManager>(result__)
        })
    }
    pub fn GetForProvisioning() -> ::windows::core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerManager>(result__)
        })
    }
    pub fn ISharedPackageContainerManagerStatics<R, F: FnOnce(&ISharedPackageContainerManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SharedPackageContainerManager, ISharedPackageContainerManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SharedPackageContainerManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainerManager;{be353068-1ef7-5ac8-ab3f-0b9f612f0274})");
}
unsafe impl ::windows::core::Interface for SharedPackageContainerManager {
    type Vtable = ISharedPackageContainerManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe353068_1ef7_5ac8_ab3f_0b9f612f0274);
}
impl ::windows::core::RuntimeName for SharedPackageContainerManager {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerManager";
}
impl ::core::convert::From<SharedPackageContainerManager> for ::windows::core::IUnknown {
    fn from(value: SharedPackageContainerManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SharedPackageContainerManager> for ::windows::core::IUnknown {
    fn from(value: &SharedPackageContainerManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SharedPackageContainerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SharedPackageContainerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SharedPackageContainerManager> for ::windows::core::IInspectable {
    fn from(value: SharedPackageContainerManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SharedPackageContainerManager> for ::windows::core::IInspectable {
    fn from(value: &SharedPackageContainerManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SharedPackageContainerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SharedPackageContainerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SharedPackageContainerManager {}
unsafe impl ::core::marker::Sync for SharedPackageContainerManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SharedPackageContainerMember(pub ::windows::core::IInspectable);
impl SharedPackageContainerMember {
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagefamilyname: Param0) -> ::windows::core::Result<SharedPackageContainerMember> {
        Self::ISharedPackageContainerMemberFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<SharedPackageContainerMember>(result__)
        })
    }
    pub fn ISharedPackageContainerMemberFactory<R, F: FnOnce(&ISharedPackageContainerMemberFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SharedPackageContainerMember, ISharedPackageContainerMemberFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SharedPackageContainerMember {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainerMember;{fe0d0438-43c9-5426-b89c-f79bf85ddff4})");
}
unsafe impl ::windows::core::Interface for SharedPackageContainerMember {
    type Vtable = ISharedPackageContainerMember_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe0d0438_43c9_5426_b89c_f79bf85ddff4);
}
impl ::windows::core::RuntimeName for SharedPackageContainerMember {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerMember";
}
impl ::core::convert::From<SharedPackageContainerMember> for ::windows::core::IUnknown {
    fn from(value: SharedPackageContainerMember) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SharedPackageContainerMember> for ::windows::core::IUnknown {
    fn from(value: &SharedPackageContainerMember) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SharedPackageContainerMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SharedPackageContainerMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SharedPackageContainerMember> for ::windows::core::IInspectable {
    fn from(value: SharedPackageContainerMember) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SharedPackageContainerMember> for ::windows::core::IInspectable {
    fn from(value: &SharedPackageContainerMember) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SharedPackageContainerMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SharedPackageContainerMember {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SharedPackageContainerMember {}
unsafe impl ::core::marker::Sync for SharedPackageContainerMember {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SharedPackageContainerOperationStatus(pub i32);
impl SharedPackageContainerOperationStatus {
    pub const Success: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(0i32);
    pub const BlockedByPolicy: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(1i32);
    pub const AlreadyExists: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(2i32);
    pub const PackageFamilyExistsInAnotherContainer: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(3i32);
    pub const NotFound: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(4i32);
    pub const UnknownFailure: SharedPackageContainerOperationStatus = SharedPackageContainerOperationStatus(5i32);
}
impl ::core::convert::From<i32> for SharedPackageContainerOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SharedPackageContainerOperationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SharedPackageContainerOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerOperationStatus;i4)");
}
impl ::windows::core::DefaultType for SharedPackageContainerOperationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StagePackageOptions(pub ::windows::core::IInspectable);
impl StagePackageOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StagePackageOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    pub fn TargetVolume(&self) -> ::windows::core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    pub fn SetTargetVolume<'a, Param0: ::windows::core::IntoParam<'a, PackageVolume>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RelatedPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExternalLocationUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StubPackageOption(&self) -> ::windows::core::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__: StubPackageOption = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StubPackageOption>(result__)
        }
    }
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DeveloperMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RequiredContentGroupOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for StagePackageOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.StagePackageOptions;{0b110c9c-b95d-4c56-bd36-6d656800d06b})");
}
unsafe impl ::windows::core::Interface for StagePackageOptions {
    type Vtable = IStagePackageOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b110c9c_b95d_4c56_bd36_6d656800d06b);
}
impl ::windows::core::RuntimeName for StagePackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.StagePackageOptions";
}
impl ::core::convert::From<StagePackageOptions> for ::windows::core::IUnknown {
    fn from(value: StagePackageOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StagePackageOptions> for ::windows::core::IUnknown {
    fn from(value: &StagePackageOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StagePackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StagePackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StagePackageOptions> for ::windows::core::IInspectable {
    fn from(value: StagePackageOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StagePackageOptions> for ::windows::core::IInspectable {
    fn from(value: &StagePackageOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StagePackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StagePackageOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StagePackageOptions {}
unsafe impl ::core::marker::Sync for StagePackageOptions {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StubPackageOption(pub i32);
impl StubPackageOption {
    pub const Default: StubPackageOption = StubPackageOption(0i32);
    pub const InstallFull: StubPackageOption = StubPackageOption(1i32);
    pub const InstallStub: StubPackageOption = StubPackageOption(2i32);
    pub const UsePreference: StubPackageOption = StubPackageOption(3i32);
}
impl ::core::convert::From<i32> for StubPackageOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StubPackageOption {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StubPackageOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.StubPackageOption;i4)");
}
impl ::windows::core::DefaultType for StubPackageOption {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UpdateSharedPackageContainerOptions(pub ::windows::core::IInspectable);
impl UpdateSharedPackageContainerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UpdateSharedPackageContainerOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RequirePackagesPresent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequirePackagesPresent(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UpdateSharedPackageContainerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.UpdateSharedPackageContainerOptions;{80672e83-7194-59f9-b5b9-daa5375f130a})");
}
unsafe impl ::windows::core::Interface for UpdateSharedPackageContainerOptions {
    type Vtable = IUpdateSharedPackageContainerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80672e83_7194_59f9_b5b9_daa5375f130a);
}
impl ::windows::core::RuntimeName for UpdateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerOptions";
}
impl ::core::convert::From<UpdateSharedPackageContainerOptions> for ::windows::core::IUnknown {
    fn from(value: UpdateSharedPackageContainerOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UpdateSharedPackageContainerOptions> for ::windows::core::IUnknown {
    fn from(value: &UpdateSharedPackageContainerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UpdateSharedPackageContainerOptions> for ::windows::core::IInspectable {
    fn from(value: UpdateSharedPackageContainerOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UpdateSharedPackageContainerOptions> for ::windows::core::IInspectable {
    fn from(value: &UpdateSharedPackageContainerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UpdateSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for UpdateSharedPackageContainerOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UpdateSharedPackageContainerResult(pub ::windows::core::IInspectable);
impl UpdateSharedPackageContainerResult {
    pub fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SharedPackageContainerOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerOperationStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UpdateSharedPackageContainerResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.UpdateSharedPackageContainerResult;{aa407df7-c72d-5458-aea3-4645b6a8ee99})");
}
unsafe impl ::windows::core::Interface for UpdateSharedPackageContainerResult {
    type Vtable = IUpdateSharedPackageContainerResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa407df7_c72d_5458_aea3_4645b6a8ee99);
}
impl ::windows::core::RuntimeName for UpdateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerResult";
}
impl ::core::convert::From<UpdateSharedPackageContainerResult> for ::windows::core::IUnknown {
    fn from(value: UpdateSharedPackageContainerResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UpdateSharedPackageContainerResult> for ::windows::core::IUnknown {
    fn from(value: &UpdateSharedPackageContainerResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UpdateSharedPackageContainerResult> for ::windows::core::IInspectable {
    fn from(value: UpdateSharedPackageContainerResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UpdateSharedPackageContainerResult> for ::windows::core::IInspectable {
    fn from(value: &UpdateSharedPackageContainerResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UpdateSharedPackageContainerResult {}
unsafe impl ::core::marker::Sync for UpdateSharedPackageContainerResult {}
