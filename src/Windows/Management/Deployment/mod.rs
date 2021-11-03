#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Management_Deployment_Preview")]
pub mod Preview;
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AddPackageByAppInstallerOptions(pub u32);
impl AddPackageByAppInstallerOptions {
    pub const None: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(0u32);
    pub const InstallAllResources: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(32u32);
    pub const ForceTargetAppShutdown: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(64u32);
    pub const RequiredContentGroupOnly: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(256u32);
    pub const LimitToExistingPackages: AddPackageByAppInstallerOptions = AddPackageByAppInstallerOptions(512u32);
}
impl ::std::convert::From<u32> for AddPackageByAppInstallerOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AddPackageByAppInstallerOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AddPackageByAppInstallerOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.AddPackageByAppInstallerOptions;u4)");
}
impl ::windows::runtime::DefaultType for AddPackageByAppInstallerOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for AddPackageByAppInstallerOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for AddPackageByAppInstallerOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AddPackageOptions(pub ::windows::runtime::IInspectable);
impl AddPackageOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AddPackageOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn DependencyPackageUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn TargetVolume(&self) -> ::windows::runtime::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetTargetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn OptionalPackageUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RelatedPackageUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn ExternalLocationUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn SetExternalLocationUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn StubPackageOption(&self) -> ::windows::runtime::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__: StubPackageOption = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<StubPackageOption>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn DeveloperMode(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceAppShutdown(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceTargetAppShutdown(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn InstallAllResources(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn RequiredContentGroupOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn RetainFilesOnFailure(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetRetainFilesOnFailure(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn StageInPlace(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetStageInPlace(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn AllowUnsigned(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AddPackageOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AddPackageOptions;{05cee018-f68f-422b-95a4-66679ec77fc0})");
}
unsafe impl ::windows::runtime::Interface for AddPackageOptions {
    type Vtable = IAddPackageOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(97443864, 63119, 16939, [149, 164, 102, 103, 158, 199, 127, 192]);
}
impl ::windows::runtime::RuntimeName for AddPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AddPackageOptions";
}
impl ::std::convert::From<AddPackageOptions> for ::windows::runtime::IUnknown {
    fn from(value: AddPackageOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AddPackageOptions> for ::windows::runtime::IUnknown {
    fn from(value: &AddPackageOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AddPackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AddPackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AddPackageOptions> for ::windows::runtime::IInspectable {
    fn from(value: AddPackageOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AddPackageOptions> for ::windows::runtime::IInspectable {
    fn from(value: &AddPackageOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AddPackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AddPackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AddPackageOptions {}
unsafe impl ::std::marker::Sync for AddPackageOptions {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppInstallerManager(pub ::windows::runtime::IInspectable);
impl AppInstallerManager {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetAutoUpdateSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, AutoUpdateSettingsOptions>>(&self, packagefamilyname: Param0, appinstallerinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), appinstallerinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ClearAutoUpdateSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn PauseAutoUpdatesUntil<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, packagefamilyname: Param0, datetime: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), datetime.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallerManager>(result__)
        })
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn GetForSystem() -> ::windows::runtime::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallerManager>(result__)
        })
    }
    pub fn IAppInstallerManagerStatics<R, F: FnOnce(&IAppInstallerManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppInstallerManager, IAppInstallerManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppInstallerManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AppInstallerManager;{e7ee21c3-2103-53ee-9b18-68afeab0033d})");
}
unsafe impl ::windows::runtime::Interface for AppInstallerManager {
    type Vtable = IAppInstallerManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3891143107, 8451, 21486, [155, 24, 104, 175, 234, 176, 3, 61]);
}
impl ::windows::runtime::RuntimeName for AppInstallerManager {
    const NAME: &'static str = "Windows.Management.Deployment.AppInstallerManager";
}
impl ::std::convert::From<AppInstallerManager> for ::windows::runtime::IUnknown {
    fn from(value: AppInstallerManager) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppInstallerManager> for ::windows::runtime::IUnknown {
    fn from(value: &AppInstallerManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppInstallerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppInstallerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppInstallerManager> for ::windows::runtime::IInspectable {
    fn from(value: AppInstallerManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppInstallerManager> for ::windows::runtime::IInspectable {
    fn from(value: &AppInstallerManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppInstallerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppInstallerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppInstallerManager {}
unsafe impl ::std::marker::Sync for AppInstallerManager {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AutoUpdateSettingsOptions(pub ::windows::runtime::IInspectable);
impl AutoUpdateSettingsOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AutoUpdateSettingsOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "ApplicationModel")]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`*"]
    pub fn Version(&self) -> ::windows::runtime::Result<super::super::ApplicationModel::PackageVersion> {
        let this = self;
        unsafe {
            let mut result__: super::super::ApplicationModel::PackageVersion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::PackageVersion>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`*"]
    pub fn SetVersion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::PackageVersion>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn AppInstallerUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn SetAppInstallerUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn OnLaunch(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetOnLaunch(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn HoursBetweenUpdateChecks(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetHoursBetweenUpdateChecks(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ShowPrompt(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetShowPrompt(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn UpdateBlocksActivation(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetUpdateBlocksActivation(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn AutomaticBackgroundTask(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetAutomaticBackgroundTask(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn IsAutoRepairEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetIsAutoRepairEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdateUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RepairUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn DependencyPackageUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn OptionalPackageUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`*"]
    pub fn CreateFromAppInstallerInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::AppInstallerInfo>>(appinstallerinfo: Param0) -> ::windows::runtime::Result<AutoUpdateSettingsOptions> {
        Self::IAutoUpdateSettingsOptionsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), appinstallerinfo.into_param().abi(), &mut result__).from_abi::<AutoUpdateSettingsOptions>(result__)
        })
    }
    pub fn IAutoUpdateSettingsOptionsStatics<R, F: FnOnce(&IAutoUpdateSettingsOptionsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AutoUpdateSettingsOptions, IAutoUpdateSettingsOptionsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AutoUpdateSettingsOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AutoUpdateSettingsOptions;{67491d87-35e1-512a-8968-1ae88d1be6d3})");
}
unsafe impl ::windows::runtime::Interface for AutoUpdateSettingsOptions {
    type Vtable = IAutoUpdateSettingsOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1732844935, 13793, 20778, [137, 104, 26, 232, 141, 27, 230, 211]);
}
impl ::windows::runtime::RuntimeName for AutoUpdateSettingsOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AutoUpdateSettingsOptions";
}
impl ::std::convert::From<AutoUpdateSettingsOptions> for ::windows::runtime::IUnknown {
    fn from(value: AutoUpdateSettingsOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AutoUpdateSettingsOptions> for ::windows::runtime::IUnknown {
    fn from(value: &AutoUpdateSettingsOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AutoUpdateSettingsOptions> for ::windows::runtime::IInspectable {
    fn from(value: AutoUpdateSettingsOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AutoUpdateSettingsOptions> for ::windows::runtime::IInspectable {
    fn from(value: &AutoUpdateSettingsOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AutoUpdateSettingsOptions {}
unsafe impl ::std::marker::Sync for AutoUpdateSettingsOptions {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CreateSharedPackageContainerOptions(pub ::windows::runtime::IInspectable);
impl CreateSharedPackageContainerOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CreateSharedPackageContainerOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn Members(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceAppShutdown(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn CreateCollisionOption(&self) -> ::windows::runtime::Result<SharedPackageContainerCreationCollisionOptions> {
        let this = self;
        unsafe {
            let mut result__: SharedPackageContainerCreationCollisionOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerCreationCollisionOptions>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetCreateCollisionOption(&self, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CreateSharedPackageContainerOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.CreateSharedPackageContainerOptions;{c2ab6ece-f664-5c8e-a4b3-2a33276d3dde})");
}
unsafe impl ::windows::runtime::Interface for CreateSharedPackageContainerOptions {
    type Vtable = ICreateSharedPackageContainerOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3266014926, 63076, 23694, [164, 179, 42, 51, 39, 109, 61, 222]);
}
impl ::windows::runtime::RuntimeName for CreateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerOptions";
}
impl ::std::convert::From<CreateSharedPackageContainerOptions> for ::windows::runtime::IUnknown {
    fn from(value: CreateSharedPackageContainerOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CreateSharedPackageContainerOptions> for ::windows::runtime::IUnknown {
    fn from(value: &CreateSharedPackageContainerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CreateSharedPackageContainerOptions> for ::windows::runtime::IInspectable {
    fn from(value: CreateSharedPackageContainerOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CreateSharedPackageContainerOptions> for ::windows::runtime::IInspectable {
    fn from(value: &CreateSharedPackageContainerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CreateSharedPackageContainerOptions {}
unsafe impl ::std::marker::Sync for CreateSharedPackageContainerOptions {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CreateSharedPackageContainerResult(pub ::windows::runtime::IInspectable);
impl CreateSharedPackageContainerResult {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn Container(&self) -> ::windows::runtime::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainer>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SharedPackageContainerOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CreateSharedPackageContainerResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.CreateSharedPackageContainerResult;{ce8810bf-151c-5707-b936-497e564afc7a})");
}
unsafe impl ::windows::runtime::Interface for CreateSharedPackageContainerResult {
    type Vtable = ICreateSharedPackageContainerResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3465023679, 5404, 22279, [185, 54, 73, 126, 86, 74, 252, 122]);
}
impl ::windows::runtime::RuntimeName for CreateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerResult";
}
impl ::std::convert::From<CreateSharedPackageContainerResult> for ::windows::runtime::IUnknown {
    fn from(value: CreateSharedPackageContainerResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CreateSharedPackageContainerResult> for ::windows::runtime::IUnknown {
    fn from(value: &CreateSharedPackageContainerResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CreateSharedPackageContainerResult> for ::windows::runtime::IInspectable {
    fn from(value: CreateSharedPackageContainerResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CreateSharedPackageContainerResult> for ::windows::runtime::IInspectable {
    fn from(value: &CreateSharedPackageContainerResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CreateSharedPackageContainerResult {}
unsafe impl ::std::marker::Sync for CreateSharedPackageContainerResult {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DeleteSharedPackageContainerOptions(pub ::windows::runtime::IInspectable);
impl DeleteSharedPackageContainerOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DeleteSharedPackageContainerOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceAppShutdown(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn AllUsers(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetAllUsers(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeleteSharedPackageContainerOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeleteSharedPackageContainerOptions;{9d81865f-986e-5138-8b5d-384d8e66ed6c})");
}
unsafe impl ::windows::runtime::Interface for DeleteSharedPackageContainerOptions {
    type Vtable = IDeleteSharedPackageContainerOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2642511455, 39022, 20792, [139, 93, 56, 77, 142, 102, 237, 108]);
}
impl ::windows::runtime::RuntimeName for DeleteSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerOptions";
}
impl ::std::convert::From<DeleteSharedPackageContainerOptions> for ::windows::runtime::IUnknown {
    fn from(value: DeleteSharedPackageContainerOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&DeleteSharedPackageContainerOptions> for ::windows::runtime::IUnknown {
    fn from(value: &DeleteSharedPackageContainerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<DeleteSharedPackageContainerOptions> for ::windows::runtime::IInspectable {
    fn from(value: DeleteSharedPackageContainerOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DeleteSharedPackageContainerOptions> for ::windows::runtime::IInspectable {
    fn from(value: &DeleteSharedPackageContainerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DeleteSharedPackageContainerOptions {}
unsafe impl ::std::marker::Sync for DeleteSharedPackageContainerOptions {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DeleteSharedPackageContainerResult(pub ::windows::runtime::IInspectable);
impl DeleteSharedPackageContainerResult {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SharedPackageContainerOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeleteSharedPackageContainerResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeleteSharedPackageContainerResult;{35398884-5736-517b-85bc-e598c81ab284})");
}
unsafe impl ::windows::runtime::Interface for DeleteSharedPackageContainerResult {
    type Vtable = IDeleteSharedPackageContainerResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(892962948, 22326, 20859, [133, 188, 229, 152, 200, 26, 178, 132]);
}
impl ::windows::runtime::RuntimeName for DeleteSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerResult";
}
impl ::std::convert::From<DeleteSharedPackageContainerResult> for ::windows::runtime::IUnknown {
    fn from(value: DeleteSharedPackageContainerResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&DeleteSharedPackageContainerResult> for ::windows::runtime::IUnknown {
    fn from(value: &DeleteSharedPackageContainerResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<DeleteSharedPackageContainerResult> for ::windows::runtime::IInspectable {
    fn from(value: DeleteSharedPackageContainerResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DeleteSharedPackageContainerResult> for ::windows::runtime::IInspectable {
    fn from(value: &DeleteSharedPackageContainerResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DeleteSharedPackageContainerResult {}
unsafe impl ::std::marker::Sync for DeleteSharedPackageContainerResult {}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for DeploymentOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeploymentOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeploymentOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentOptions;u4)");
}
impl ::windows::runtime::DefaultType for DeploymentOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DeploymentOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DeploymentOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DeploymentOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DeploymentOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DeploymentOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Management_Deployment`*"]
pub struct DeploymentProgress {
    pub state: DeploymentProgressState,
    pub percentage: u32,
}
impl DeploymentProgress {}
impl ::std::default::Default for DeploymentProgress {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DeploymentProgress {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DeploymentProgress").field("state", &self.state).field("percentage", &self.percentage).finish()
    }
}
impl ::std::cmp::PartialEq for DeploymentProgress {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.percentage == other.percentage
    }
}
impl ::std::cmp::Eq for DeploymentProgress {}
unsafe impl ::windows::runtime::Abi for DeploymentProgress {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeploymentProgress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Management.Deployment.DeploymentProgress;enum(Windows.Management.Deployment.DeploymentProgressState;i4);u4)");
}
impl ::windows::runtime::DefaultType for DeploymentProgress {
    type DefaultType = Self;
}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeploymentProgressState(pub i32);
impl DeploymentProgressState {
    pub const Queued: DeploymentProgressState = DeploymentProgressState(0i32);
    pub const Processing: DeploymentProgressState = DeploymentProgressState(1i32);
}
impl ::std::convert::From<i32> for DeploymentProgressState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeploymentProgressState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeploymentProgressState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentProgressState;i4)");
}
impl ::windows::runtime::DefaultType for DeploymentProgressState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DeploymentResult(pub ::windows::runtime::IInspectable);
impl DeploymentResult {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ErrorText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ExtendedErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn IsRegistered(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeploymentResult2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeploymentResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeploymentResult;{2563b9ae-b77d-4c1f-8a7b-20e6ad515ef3})");
}
unsafe impl ::windows::runtime::Interface for DeploymentResult {
    type Vtable = IDeploymentResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(627292590, 46973, 19487, [138, 123, 32, 230, 173, 81, 94, 243]);
}
impl ::windows::runtime::RuntimeName for DeploymentResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeploymentResult";
}
impl ::std::convert::From<DeploymentResult> for ::windows::runtime::IUnknown {
    fn from(value: DeploymentResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&DeploymentResult> for ::windows::runtime::IUnknown {
    fn from(value: &DeploymentResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeploymentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeploymentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<DeploymentResult> for ::windows::runtime::IInspectable {
    fn from(value: DeploymentResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DeploymentResult> for ::windows::runtime::IInspectable {
    fn from(value: &DeploymentResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeploymentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeploymentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DeploymentResult {}
unsafe impl ::std::marker::Sync for DeploymentResult {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FindSharedPackageContainerOptions(pub ::windows::runtime::IInspectable);
impl FindSharedPackageContainerOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FindSharedPackageContainerOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn PackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FindSharedPackageContainerOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.FindSharedPackageContainerOptions;{b40fc8fe-8384-54cc-817d-ae09d3b6a606})");
}
unsafe impl ::windows::runtime::Interface for FindSharedPackageContainerOptions {
    type Vtable = IFindSharedPackageContainerOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3020933374, 33668, 21708, [129, 125, 174, 9, 211, 182, 166, 6]);
}
impl ::windows::runtime::RuntimeName for FindSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.FindSharedPackageContainerOptions";
}
impl ::std::convert::From<FindSharedPackageContainerOptions> for ::windows::runtime::IUnknown {
    fn from(value: FindSharedPackageContainerOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&FindSharedPackageContainerOptions> for ::windows::runtime::IUnknown {
    fn from(value: &FindSharedPackageContainerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<FindSharedPackageContainerOptions> for ::windows::runtime::IInspectable {
    fn from(value: FindSharedPackageContainerOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FindSharedPackageContainerOptions> for ::windows::runtime::IInspectable {
    fn from(value: &FindSharedPackageContainerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FindSharedPackageContainerOptions {}
unsafe impl ::std::marker::Sync for FindSharedPackageContainerOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAddPackageOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAddPackageOptions {
    type Vtable = IAddPackageOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(97443864, 63119, 16939, [149, 164, 102, 103, 158, 199, 127, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut StubPackageOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: StubPackageOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppInstallerManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallerManager {
    type Vtable = IAppInstallerManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3891143107, 8451, 21486, [155, 24, 104, 175, 234, 176, 3, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, appinstallerinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, datetime: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppInstallerManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstallerManagerStatics {
    type Vtable = IAppInstallerManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3378147029, 64601, 21302, [155, 46, 43, 7, 197, 230, 20, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManagerStatics_abi(
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
pub struct IAutoUpdateSettingsOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutoUpdateSettingsOptions {
    type Vtable = IAutoUpdateSettingsOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1732844935, 13793, 20778, [137, 104, 26, 232, 141, 27, 230, 211]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::ApplicationModel::PackageVersion) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::ApplicationModel::PackageVersion) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptionsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutoUpdateSettingsOptionsStatics {
    type Vtable = IAutoUpdateSettingsOptionsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2289775485, 3077, 21712, [189, 73, 59, 183, 162, 192, 132, 203]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptionsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appinstallerinfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateSharedPackageContainerOptions {
    type Vtable = ICreateSharedPackageContainerOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3266014926, 63076, 23694, [164, 179, 42, 51, 39, 109, 61, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SharedPackageContainerCreationCollisionOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateSharedPackageContainerResult {
    type Vtable = ICreateSharedPackageContainerResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3465023679, 5404, 22279, [185, 54, 73, 126, 86, 74, 252, 122]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeleteSharedPackageContainerOptions {
    type Vtable = IDeleteSharedPackageContainerOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2642511455, 39022, 20792, [139, 93, 56, 77, 142, 102, 237, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerOptions_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeleteSharedPackageContainerResult {
    type Vtable = IDeleteSharedPackageContainerResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(892962948, 22326, 20859, [133, 188, 229, 152, 200, 26, 178, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeploymentResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeploymentResult {
    type Vtable = IDeploymentResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(627292590, 46973, 19487, [138, 123, 32, 230, 173, 81, 94, 243]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeploymentResult2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeploymentResult2 {
    type Vtable = IDeploymentResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4228804956, 23041, 19415, [188, 241, 56, 28, 140, 130, 224, 74]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult2_abi(
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
pub struct IFindSharedPackageContainerOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFindSharedPackageContainerOptions {
    type Vtable = IFindSharedPackageContainerOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3020933374, 33668, 21708, [129, 125, 174, 9, 211, 182, 166, 6]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindSharedPackageContainerOptions_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageAllUserProvisioningOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageAllUserProvisioningOptions {
    type Vtable = IPackageAllUserProvisioningOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3660950050, 7648, 23870, [153, 255, 210, 79, 49, 24, 191, 94]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageAllUserProvisioningOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager {
    type Vtable = IPackageManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2591902565, 24207, 20423, [162, 229, 127, 105, 37, 203, 139, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifesturi: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagestate: PackageState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager10(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager10 {
    type Vtable = IPackageManager10_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2815938686, 11878, 16531, [174, 213, 224, 147, 237, 135, 179, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager10_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mainpackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager2 {
    type Vtable = IPackageManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4155166861, 2112, 18162, [181, 216, 202, 212, 118, 147, 160, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, removaloptions: RemovalOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mainpackagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, dependencypackagefullnames: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagetypes: PackageTypes, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager3 {
    type Vtable = IPackageManager3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3668810056, 14065, 16807, [145, 136, 188, 38, 62, 13, 203, 114]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagestorepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, status: PackageStatus) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifesturi: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, volumename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, deploymentoptions: DeploymentOptions, targetvolume: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, volume: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, volume: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, status: PackageStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagevolume: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagevolume: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, deploymentoptions: DeploymentOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager4 {
    type Vtable = IPackageManager4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1014077795, 47798, 18111, [143, 247, 218, 71, 25, 35, 10, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager4_abi(
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
pub struct IPackageManager5(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager5 {
    type Vtable = IPackageManager5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1897869591, 6909, 17171, [151, 140, 155, 182, 225, 184, 100, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::runtime::RawPtr, optionalpackagefamilynames: ::windows::runtime::RawPtr, externalpackageuris: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::runtime::RawPtr, optionalpackagefamilynames: ::windows::runtime::RawPtr, externalpackageuris: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mainpackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, dependencypackagefamilynames: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows::runtime::RawPtr, optionalpackagefamilynames: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager6(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager6 {
    type Vtable = IPackageManager6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(138930441, 21453, 20047, [131, 46, 87, 209, 128, 246, 228, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appinstallerfileuri: ::windows::runtime::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appinstallerfileuri: ::windows::runtime::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, options: DeploymentOptions, targetvolume: ::windows::runtime::RawPtr, optionalpackagefamilynames: ::windows::runtime::RawPtr, packageuristoinstall: ::windows::runtime::RawPtr, relatedpackageuris: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, options: DeploymentOptions, targetvolume: ::windows::runtime::RawPtr, optionalpackagefamilynames: ::windows::runtime::RawPtr, packageuristoinstall: ::windows::runtime::RawPtr, relatedpackageuris: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::runtime::RawPtr, optionalpackagefamilynames: ::windows::runtime::RawPtr, relatedpackageuris: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager7(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager7 {
    type Vtable = IPackageManager7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4068889844, 11175, 19328, [136, 214, 190, 21, 249, 162, 63, 186]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager7_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, dependencypackageuris: ::windows::runtime::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::runtime::RawPtr, optionalpackagefamilynames: ::windows::runtime::RawPtr, relatedpackageuris: ::windows::runtime::RawPtr, packageuristoinstall: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager8(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager8 {
    type Vtable = IPackageManager8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3092730672, 4760, 20194, [128, 238, 127, 101, 156, 93, 39, 130]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManager9(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManager9 {
    type Vtable = IPackageManager9_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(447189045, 52337, 19246, [128, 166, 199, 4, 29, 133, 121, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager9_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageuri: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifesturi: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullnames: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, usestub: PackageStubPreference) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut PackageStubPreference) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageManagerDebugSettings(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageManagerDebugSettings {
    type Vtable = IPackageManagerDebugSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(442570371, 43400, 20431, [143, 15, 206, 23, 88, 152, 232, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManagerDebugSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: ::windows::runtime::RawPtr, contentgroupname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: ::windows::runtime::RawPtr, contentgroupname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageUserInformation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageUserInformation {
    type Vtable = IPackageUserInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4130878499, 64009, 19644, [144, 85, 21, 202, 39, 94, 46, 126]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUserInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PackageInstallState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageVolume(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageVolume {
    type Vtable = IPackageVolume_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3475403459, 6720, 17488, [151, 57, 42, 206, 46, 137, 136, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagetypes: PackageTypes, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagetypes: PackageTypes, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagetypes: PackageTypes, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagetypes: PackageTypes, packagename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagepublisher: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagetypes: PackageTypes, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersecurityid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageVolume2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageVolume2 {
    type Vtable = IPackageVolume2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1185664814, 40404, 18338, [171, 140, 198, 64, 131, 73, 188, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRegisterPackageOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRegisterPackageOptions {
    type Vtable = IRegisterPackageOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1735463591, 20692, 18796, [132, 21, 6, 2, 180, 198, 211, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisterPackageOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISharedPackageContainer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISharedPackageContainer {
    type Vtable = ISharedPackageContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(394205865, 5406, 24311, [177, 217, 47, 186, 11, 75, 13, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISharedPackageContainerManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISharedPackageContainerManager {
    type Vtable = ISharedPackageContainerManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3191156840, 7927, 23240, [171, 63, 11, 159, 97, 47, 2, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISharedPackageContainerManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISharedPackageContainerManagerStatics {
    type Vtable = ISharedPackageContainerManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(787833672, 33674, 24405, [168, 158, 17, 152, 162, 198, 39, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISharedPackageContainerMember(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISharedPackageContainerMember {
    type Vtable = ISharedPackageContainerMember_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4262265912, 17353, 21542, [184, 156, 247, 155, 248, 93, 223, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMember_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISharedPackageContainerMemberFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISharedPackageContainerMemberFactory {
    type Vtable = ISharedPackageContainerMemberFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1236324075, 18831, 23138, [183, 56, 179, 202, 13, 67, 103, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMemberFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStagePackageOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStagePackageOptions {
    type Vtable = IStagePackageOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(185666716, 47453, 19542, [189, 54, 109, 101, 104, 0, 208, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStagePackageOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut StubPackageOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: StubPackageOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUpdateSharedPackageContainerOptions {
    type Vtable = IUpdateSharedPackageContainerOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2154245763, 29076, 23033, [181, 185, 218, 165, 55, 95, 19, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerOptions_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUpdateSharedPackageContainerResult {
    type Vtable = IUpdateSharedPackageContainerResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2856353271, 50989, 21592, [174, 163, 70, 69, 182, 168, 238, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PackageAllUserProvisioningOptions(pub ::windows::runtime::IInspectable);
impl PackageAllUserProvisioningOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PackageAllUserProvisioningOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn ProjectionOrderPackageFamilyNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PackageAllUserProvisioningOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageAllUserProvisioningOptions;{da35aa22-1de0-5d3e-99ff-d24f3118bf5e})");
}
unsafe impl ::windows::runtime::Interface for PackageAllUserProvisioningOptions {
    type Vtable = IPackageAllUserProvisioningOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3660950050, 7648, 23870, [153, 255, 210, 79, 49, 24, 191, 94]);
}
impl ::windows::runtime::RuntimeName for PackageAllUserProvisioningOptions {
    const NAME: &'static str = "Windows.Management.Deployment.PackageAllUserProvisioningOptions";
}
impl ::std::convert::From<PackageAllUserProvisioningOptions> for ::windows::runtime::IUnknown {
    fn from(value: PackageAllUserProvisioningOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PackageAllUserProvisioningOptions> for ::windows::runtime::IUnknown {
    fn from(value: &PackageAllUserProvisioningOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PackageAllUserProvisioningOptions> for ::windows::runtime::IInspectable {
    fn from(value: PackageAllUserProvisioningOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PackageAllUserProvisioningOptions> for ::windows::runtime::IInspectable {
    fn from(value: &PackageAllUserProvisioningOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PackageAllUserProvisioningOptions {}
unsafe impl ::std::marker::Sync for PackageAllUserProvisioningOptions {}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageInstallState(pub i32);
impl PackageInstallState {
    pub const NotInstalled: PackageInstallState = PackageInstallState(0i32);
    pub const Staged: PackageInstallState = PackageInstallState(1i32);
    pub const Installed: PackageInstallState = PackageInstallState(2i32);
    pub const Paused: PackageInstallState = PackageInstallState(6i32);
}
impl ::std::convert::From<i32> for PackageInstallState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageInstallState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PackageInstallState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageInstallState;i4)");
}
impl ::windows::runtime::DefaultType for PackageInstallState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PackageManager(pub ::windows::runtime::IInspectable);
impl PackageManager {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PackageManager, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn AddPackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdatePackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn RemovePackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn StagePackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RegisterPackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, manifesturi: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagename: Param0, packagepublisher: Param1) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdNamePublisher<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagename: Param1, packagepublisher: Param2) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn FindUsers<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<PackageUserInformation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<PackageUserInformation>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetPackageState<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0, packagestate: PackageState) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), packagestate).ok() }
    }
    #[cfg(feature = "ApplicationModel")]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`*"]
    pub fn FindPackageByPackageFullName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0) -> ::windows::runtime::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::ApplicationModel::Package>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn CleanupPackageForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagename: Param0, usersecurityid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), packagename.into_param().abi(), usersecurityid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagefamilyname: Param1) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`*"]
    pub fn FindPackageByUserSecurityIdPackageFullName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagefullname: Param1) -> ::windows::runtime::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::ApplicationModel::Package>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn RemovePackageWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0, removaloptions: RemovalOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), removaloptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn StagePackageWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RegisterPackageByFullNameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, mainpackagefullname: Param0, dependencypackagefullnames: Param1, deploymentoptions: DeploymentOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), mainpackagefullname.into_param().abi(), dependencypackagefullnames.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByNamePublisherWithPackageTypes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagename: Param0, packagepublisher: Param1, packagetypes: PackageTypes) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        usersecurityid: Param0,
        packagename: Param1,
        packagepublisher: Param2,
        packagetypes: PackageTypes,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagename.into_param().abi(), packagepublisher.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0, packagetypes: PackageTypes) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagefamilyname: Param1, packagetypes: PackageTypes) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefamilyname.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn StageUserDataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn AddPackageVolumeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagestorepath: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PackageVolume>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packagestorepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PackageVolume>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn AddPackageToVolumeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Param3: ::windows::runtime::IntoParam<'a, PackageVolume>>(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ClearPackageStatus<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0, status: PackageStatus) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), status).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RegisterPackageWithAppDataVolumeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Param3: ::windows::runtime::IntoParam<'a, PackageVolume>>(
        &self,
        manifesturi: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        appdatavolume: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, appdatavolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn FindPackageVolumeByName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, volumename: Param0) -> ::windows::runtime::Result<PackageVolume> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), volumename.into_param().abi(), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn FindPackageVolumes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterable<PackageVolume>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<PackageVolume>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn GetDefaultPackageVolume(&self) -> ::windows::runtime::Result<PackageVolume> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn MovePackageToVolumeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, packagefullname: Param0, deploymentoptions: DeploymentOptions, targetvolume: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn RemovePackageVolumeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, volume: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), volume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetDefaultPackageVolume<'a, Param0: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, volume: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), volume.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetPackageStatus<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0, status: PackageStatus) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), status).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn SetPackageVolumeOfflineAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, packagevolume: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), packagevolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn SetPackageVolumeOnlineAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, packagevolume: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), packagevolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn StagePackageToVolumeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, Param3: ::windows::runtime::IntoParam<'a, PackageVolume>>(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn StageUserDataWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0, deploymentoptions: DeploymentOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), deploymentoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetPackageVolumesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageVolume>>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageVolume>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn AddPackageToVolumeAndOptionalPackagesAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::runtime::IntoParam<'a, PackageVolume>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        externalpackageuris: Param5,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), externalpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn StagePackageToVolumeAndOptionalPackagesAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::runtime::IntoParam<'a, PackageVolume>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        externalpackageuris: Param5,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), externalpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RegisterPackageByFamilyNameAndOptionalPackagesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param3: ::windows::runtime::IntoParam<'a, PackageVolume>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(
        &self,
        mainpackagefamilyname: Param0,
        dependencypackagefamilynames: Param1,
        deploymentoptions: DeploymentOptions,
        appdatavolume: Param3,
        optionalpackagefamilynames: Param4,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), mainpackagefamilyname.into_param().abi(), dependencypackagefamilynames.into_param().abi(), deploymentoptions, appdatavolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn DebugSettings(&self) -> ::windows::runtime::Result<PackageManagerDebugSettings> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PackageManagerDebugSettings>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn ProvisionPackageForAllUsersAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn AddPackageByAppInstallerFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, appinstallerfileuri: Param0, options: AddPackageByAppInstallerOptions, targetvolume: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), appinstallerfileuri.into_param().abi(), options, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn RequestAddPackageByAppInstallerFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, appinstallerfileuri: Param0, options: AddPackageByAppInstallerOptions, targetvolume: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), appinstallerfileuri.into_param().abi(), options, targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn AddPackageToVolumeAndRelatedSetAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::runtime::IntoParam<'a, PackageVolume>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        options: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        packageuristoinstall: Param5,
        relatedpackageuris: Param6,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), options, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), packageuristoinstall.into_param().abi(), relatedpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn StagePackageToVolumeAndRelatedSetAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::runtime::IntoParam<'a, PackageVolume>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        options: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        packageuristoinstall: Param5,
        relatedpackageuris: Param6,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), options, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), packageuristoinstall.into_param().abi(), relatedpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RequestAddPackageAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::runtime::IntoParam<'a, PackageVolume>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        relatedpackageuris: Param5,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), relatedpackageuris.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RequestAddPackageAndRelatedSetAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param3: ::windows::runtime::IntoParam<'a, PackageVolume>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    >(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        relatedpackageuris: Param5,
        packageuristoinstall: Param6,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager7>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), relatedpackageuris.into_param().abi(), packageuristoinstall.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn DeprovisionPackageForAllUsersAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager8>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindProvisionedPackages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn AddPackageByUriAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, AddPackageOptions>>(&self, packageuri: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn StagePackageByUriAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, StagePackageOptions>>(&self, packageuri: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), packageuri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn RegisterPackageByUriAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, RegisterPackageOptions>>(&self, manifesturi: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), manifesturi.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RegisterPackagesByFullNameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, RegisterPackageOptions>>(&self, packagefullnames: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), packagefullnames.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetPackageStubPreference<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0, usestub: PackageStubPreference) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager9>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), usestub).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn GetPackageStubPreference<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::runtime::Result<PackageStubPreference> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__: PackageStubPreference = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<PackageStubPreference>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn ProvisionPackageForAllUsersWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PackageAllUserProvisioningOptions>>(&self, mainpackagefamilyname: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::runtime::Interface::cast::<IPackageManager10>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), mainpackagefamilyname.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PackageManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageManager;{9a7d4b65-5e8f-4fc7-a2e5-7f6925cb8b53})");
}
unsafe impl ::windows::runtime::Interface for PackageManager {
    type Vtable = IPackageManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2591902565, 24207, 20423, [162, 229, 127, 105, 37, 203, 139, 83]);
}
impl ::windows::runtime::RuntimeName for PackageManager {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManager";
}
impl ::std::convert::From<PackageManager> for ::windows::runtime::IUnknown {
    fn from(value: PackageManager) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PackageManager> for ::windows::runtime::IUnknown {
    fn from(value: &PackageManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PackageManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PackageManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PackageManager> for ::windows::runtime::IInspectable {
    fn from(value: PackageManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PackageManager> for ::windows::runtime::IInspectable {
    fn from(value: &PackageManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PackageManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PackageManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PackageManager {}
unsafe impl ::std::marker::Sync for PackageManager {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PackageManagerDebugSettings(pub ::windows::runtime::IInspectable);
impl PackageManagerDebugSettings {
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation`*"]
    pub fn SetContentGroupStateAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::Package>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, package: Param0, contentgroupname: Param1, state: super::super::ApplicationModel::PackageContentGroupState) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), package.into_param().abi(), contentgroupname.into_param().abi(), state, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation`*"]
    pub fn SetContentGroupStateWithPercentageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::Package>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, package: Param0, contentgroupname: Param1, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), package.into_param().abi(), contentgroupname.into_param().abi(), state, completionpercentage, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PackageManagerDebugSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageManagerDebugSettings;{1a611683-a988-4fcf-8f0f-ce175898e8eb})");
}
unsafe impl ::windows::runtime::Interface for PackageManagerDebugSettings {
    type Vtable = IPackageManagerDebugSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(442570371, 43400, 20431, [143, 15, 206, 23, 88, 152, 232, 235]);
}
impl ::windows::runtime::RuntimeName for PackageManagerDebugSettings {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManagerDebugSettings";
}
impl ::std::convert::From<PackageManagerDebugSettings> for ::windows::runtime::IUnknown {
    fn from(value: PackageManagerDebugSettings) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PackageManagerDebugSettings> for ::windows::runtime::IUnknown {
    fn from(value: &PackageManagerDebugSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PackageManagerDebugSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PackageManagerDebugSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PackageManagerDebugSettings> for ::windows::runtime::IInspectable {
    fn from(value: PackageManagerDebugSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PackageManagerDebugSettings> for ::windows::runtime::IInspectable {
    fn from(value: &PackageManagerDebugSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PackageManagerDebugSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PackageManagerDebugSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PackageManagerDebugSettings {}
unsafe impl ::std::marker::Sync for PackageManagerDebugSettings {}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageState(pub i32);
impl PackageState {
    pub const Normal: PackageState = PackageState(0i32);
    pub const LicenseInvalid: PackageState = PackageState(1i32);
    pub const Modified: PackageState = PackageState(2i32);
    pub const Tampered: PackageState = PackageState(3i32);
}
impl ::std::convert::From<i32> for PackageState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PackageState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageState;i4)");
}
impl ::windows::runtime::DefaultType for PackageState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageStatus(pub u32);
impl PackageStatus {
    pub const OK: PackageStatus = PackageStatus(0u32);
    pub const LicenseIssue: PackageStatus = PackageStatus(1u32);
    pub const Modified: PackageStatus = PackageStatus(2u32);
    pub const Tampered: PackageStatus = PackageStatus(4u32);
    pub const Disabled: PackageStatus = PackageStatus(8u32);
}
impl ::std::convert::From<u32> for PackageStatus {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PackageStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStatus;u4)");
}
impl ::windows::runtime::DefaultType for PackageStatus {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PackageStatus {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PackageStatus {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PackageStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PackageStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PackageStatus {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageStubPreference(pub i32);
impl PackageStubPreference {
    pub const Full: PackageStubPreference = PackageStubPreference(0i32);
    pub const Stub: PackageStubPreference = PackageStubPreference(1i32);
}
impl ::std::convert::From<i32> for PackageStubPreference {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageStubPreference {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PackageStubPreference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStubPreference;i4)");
}
impl ::windows::runtime::DefaultType for PackageStubPreference {
    type DefaultType = Self;
}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for PackageTypes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageTypes {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PackageTypes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageTypes;u4)");
}
impl ::windows::runtime::DefaultType for PackageTypes {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PackageTypes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PackageTypes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PackageTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PackageTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PackageTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PackageUserInformation(pub ::windows::runtime::IInspectable);
impl PackageUserInformation {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn UserSecurityId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn InstallState(&self) -> ::windows::runtime::Result<PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__: PackageInstallState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PackageInstallState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PackageUserInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageUserInformation;{f6383423-fa09-4cbc-9055-15ca275e2e7e})");
}
unsafe impl ::windows::runtime::Interface for PackageUserInformation {
    type Vtable = IPackageUserInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4130878499, 64009, 19644, [144, 85, 21, 202, 39, 94, 46, 126]);
}
impl ::windows::runtime::RuntimeName for PackageUserInformation {
    const NAME: &'static str = "Windows.Management.Deployment.PackageUserInformation";
}
impl ::std::convert::From<PackageUserInformation> for ::windows::runtime::IUnknown {
    fn from(value: PackageUserInformation) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PackageUserInformation> for ::windows::runtime::IUnknown {
    fn from(value: &PackageUserInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PackageUserInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PackageUserInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PackageUserInformation> for ::windows::runtime::IInspectable {
    fn from(value: PackageUserInformation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PackageUserInformation> for ::windows::runtime::IInspectable {
    fn from(value: &PackageUserInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PackageUserInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PackageUserInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PackageUserInformation {}
unsafe impl ::std::marker::Sync for PackageUserInformation {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PackageVolume(pub ::windows::runtime::IInspectable);
impl PackageVolume {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn IsOffline(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn IsSystemVolume(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn MountPoint(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn PackageStorePath(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SupportsHardLinks(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagename: Param0, packagepublisher: Param1) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByNamePublisherWithPackagesTypes<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagetypes: PackageTypes, packagename: Param1, packagepublisher: Param2) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), packagetypes, packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagetypes: PackageTypes, packagefamilyname: Param1) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), packagetypes, packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackageByPackageFullName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagefullname: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdNamePublisher<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagename: Param1, packagepublisher: Param2) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagefamilyname: Param1) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagetypes, &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        usersecurityid: Param0,
        packagetypes: PackageTypes,
        packagename: Param2,
        packagepublisher: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagetypes, packagename.into_param().abi(), packagepublisher.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes, packagefamilyname: Param2) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagetypes, packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `ApplicationModel`, `Foundation_Collections`*"]
    pub fn FindPackageByUserSecurityIdPackageFullName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, usersecurityid: Param0, packagefullname: Param1) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), usersecurityid.into_param().abi(), packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn IsFullTrustPackageSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn IsAppxInstallSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn GetAvailableSpaceAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u64>> {
        let this = &::windows::runtime::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PackageVolume {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageVolume;{cf2672c3-1a40-4450-9739-2ace2e898853})");
}
unsafe impl ::windows::runtime::Interface for PackageVolume {
    type Vtable = IPackageVolume_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3475403459, 6720, 17488, [151, 57, 42, 206, 46, 137, 136, 83]);
}
impl ::windows::runtime::RuntimeName for PackageVolume {
    const NAME: &'static str = "Windows.Management.Deployment.PackageVolume";
}
impl ::std::convert::From<PackageVolume> for ::windows::runtime::IUnknown {
    fn from(value: PackageVolume) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PackageVolume> for ::windows::runtime::IUnknown {
    fn from(value: &PackageVolume) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PackageVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PackageVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PackageVolume> for ::windows::runtime::IInspectable {
    fn from(value: PackageVolume) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PackageVolume> for ::windows::runtime::IInspectable {
    fn from(value: &PackageVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PackageVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PackageVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PackageVolume {}
unsafe impl ::std::marker::Sync for PackageVolume {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RegisterPackageOptions(pub ::windows::runtime::IInspectable);
impl RegisterPackageOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RegisterPackageOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn DependencyPackageUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn AppDataVolume(&self) -> ::windows::runtime::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetAppDataVolume<'a, Param0: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn ExternalLocationUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn SetExternalLocationUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn DeveloperMode(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceAppShutdown(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceTargetAppShutdown(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn InstallAllResources(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn StageInPlace(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetStageInPlace(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn AllowUnsigned(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RegisterPackageOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.RegisterPackageOptions;{677112a7-50d4-496c-8415-0602b4c6d3bf})");
}
unsafe impl ::windows::runtime::Interface for RegisterPackageOptions {
    type Vtable = IRegisterPackageOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1735463591, 20692, 18796, [132, 21, 6, 2, 180, 198, 211, 191]);
}
impl ::windows::runtime::RuntimeName for RegisterPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.RegisterPackageOptions";
}
impl ::std::convert::From<RegisterPackageOptions> for ::windows::runtime::IUnknown {
    fn from(value: RegisterPackageOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&RegisterPackageOptions> for ::windows::runtime::IUnknown {
    fn from(value: &RegisterPackageOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RegisterPackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RegisterPackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<RegisterPackageOptions> for ::windows::runtime::IInspectable {
    fn from(value: RegisterPackageOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RegisterPackageOptions> for ::windows::runtime::IInspectable {
    fn from(value: &RegisterPackageOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RegisterPackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RegisterPackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RegisterPackageOptions {}
unsafe impl ::std::marker::Sync for RegisterPackageOptions {}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemovalOptions(pub u32);
impl RemovalOptions {
    pub const None: RemovalOptions = RemovalOptions(0u32);
    pub const PreserveApplicationData: RemovalOptions = RemovalOptions(4096u32);
    pub const PreserveRoamableApplicationData: RemovalOptions = RemovalOptions(128u32);
    pub const RemoveForAllUsers: RemovalOptions = RemovalOptions(524288u32);
}
impl ::std::convert::From<u32> for RemovalOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemovalOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemovalOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.RemovalOptions;u4)");
}
impl ::windows::runtime::DefaultType for RemovalOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RemovalOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RemovalOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RemovalOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RemovalOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RemovalOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SharedPackageContainer(pub ::windows::runtime::IInspectable);
impl SharedPackageContainer {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn GetMembers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn RemovePackageFamily<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, UpdateSharedPackageContainerOptions>>(&self, packagefamilyname: Param0, options: Param1) -> ::windows::runtime::Result<UpdateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<UpdateSharedPackageContainerResult>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ResetData(&self) -> ::windows::runtime::Result<UpdateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UpdateSharedPackageContainerResult>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SharedPackageContainer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainer;{177f1aa9-151e-5ef7-b1d9-2fba0b4b0d17})");
}
unsafe impl ::windows::runtime::Interface for SharedPackageContainer {
    type Vtable = ISharedPackageContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(394205865, 5406, 24311, [177, 217, 47, 186, 11, 75, 13, 23]);
}
impl ::windows::runtime::RuntimeName for SharedPackageContainer {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainer";
}
impl ::std::convert::From<SharedPackageContainer> for ::windows::runtime::IUnknown {
    fn from(value: SharedPackageContainer) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SharedPackageContainer> for ::windows::runtime::IUnknown {
    fn from(value: &SharedPackageContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SharedPackageContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SharedPackageContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SharedPackageContainer> for ::windows::runtime::IInspectable {
    fn from(value: SharedPackageContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SharedPackageContainer> for ::windows::runtime::IInspectable {
    fn from(value: &SharedPackageContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SharedPackageContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SharedPackageContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SharedPackageContainer {}
unsafe impl ::std::marker::Sync for SharedPackageContainer {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct SharedPackageContainerContract(pub u8);
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SharedPackageContainerCreationCollisionOptions(pub i32);
impl SharedPackageContainerCreationCollisionOptions {
    pub const FailIfExists: SharedPackageContainerCreationCollisionOptions = SharedPackageContainerCreationCollisionOptions(0i32);
    pub const MergeWithExisting: SharedPackageContainerCreationCollisionOptions = SharedPackageContainerCreationCollisionOptions(1i32);
    pub const ReplaceExisting: SharedPackageContainerCreationCollisionOptions = SharedPackageContainerCreationCollisionOptions(2i32);
}
impl ::std::convert::From<i32> for SharedPackageContainerCreationCollisionOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SharedPackageContainerCreationCollisionOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SharedPackageContainerCreationCollisionOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerCreationCollisionOptions;i4)");
}
impl ::windows::runtime::DefaultType for SharedPackageContainerCreationCollisionOptions {
    type DefaultType = Self;
}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SharedPackageContainerManager(pub ::windows::runtime::IInspectable);
impl SharedPackageContainerManager {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn CreateContainer<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, CreateSharedPackageContainerOptions>>(&self, name: Param0, options: Param1) -> ::windows::runtime::Result<CreateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<CreateSharedPackageContainerResult>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn DeleteContainer<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, DeleteSharedPackageContainerOptions>>(&self, id: Param0, options: Param1) -> ::windows::runtime::Result<DeleteSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), id.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<DeleteSharedPackageContainerResult>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn GetContainer<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, id: Param0) -> ::windows::runtime::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<SharedPackageContainer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn FindContainers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SharedPackageContainer>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn FindContainersWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, FindSharedPackageContainerOptions>>(&self, options: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SharedPackageContainer>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerManager>(result__)
        })
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(usersid: Param0) -> ::windows::runtime::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), usersid.into_param().abi(), &mut result__).from_abi::<SharedPackageContainerManager>(result__)
        })
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn GetForProvisioning() -> ::windows::runtime::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerManager>(result__)
        })
    }
    pub fn ISharedPackageContainerManagerStatics<R, F: FnOnce(&ISharedPackageContainerManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SharedPackageContainerManager, ISharedPackageContainerManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SharedPackageContainerManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainerManager;{be353068-1ef7-5ac8-ab3f-0b9f612f0274})");
}
unsafe impl ::windows::runtime::Interface for SharedPackageContainerManager {
    type Vtable = ISharedPackageContainerManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3191156840, 7927, 23240, [171, 63, 11, 159, 97, 47, 2, 116]);
}
impl ::windows::runtime::RuntimeName for SharedPackageContainerManager {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerManager";
}
impl ::std::convert::From<SharedPackageContainerManager> for ::windows::runtime::IUnknown {
    fn from(value: SharedPackageContainerManager) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SharedPackageContainerManager> for ::windows::runtime::IUnknown {
    fn from(value: &SharedPackageContainerManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SharedPackageContainerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SharedPackageContainerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SharedPackageContainerManager> for ::windows::runtime::IInspectable {
    fn from(value: SharedPackageContainerManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SharedPackageContainerManager> for ::windows::runtime::IInspectable {
    fn from(value: &SharedPackageContainerManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SharedPackageContainerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SharedPackageContainerManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SharedPackageContainerManager {}
unsafe impl ::std::marker::Sync for SharedPackageContainerManager {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SharedPackageContainerMember(pub ::windows::runtime::IInspectable);
impl SharedPackageContainerMember {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn PackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(packagefamilyname: Param0) -> ::windows::runtime::Result<SharedPackageContainerMember> {
        Self::ISharedPackageContainerMemberFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<SharedPackageContainerMember>(result__)
        })
    }
    pub fn ISharedPackageContainerMemberFactory<R, F: FnOnce(&ISharedPackageContainerMemberFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SharedPackageContainerMember, ISharedPackageContainerMemberFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SharedPackageContainerMember {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainerMember;{fe0d0438-43c9-5426-b89c-f79bf85ddff4})");
}
unsafe impl ::windows::runtime::Interface for SharedPackageContainerMember {
    type Vtable = ISharedPackageContainerMember_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4262265912, 17353, 21542, [184, 156, 247, 155, 248, 93, 223, 244]);
}
impl ::windows::runtime::RuntimeName for SharedPackageContainerMember {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerMember";
}
impl ::std::convert::From<SharedPackageContainerMember> for ::windows::runtime::IUnknown {
    fn from(value: SharedPackageContainerMember) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SharedPackageContainerMember> for ::windows::runtime::IUnknown {
    fn from(value: &SharedPackageContainerMember) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SharedPackageContainerMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SharedPackageContainerMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SharedPackageContainerMember> for ::windows::runtime::IInspectable {
    fn from(value: SharedPackageContainerMember) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SharedPackageContainerMember> for ::windows::runtime::IInspectable {
    fn from(value: &SharedPackageContainerMember) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SharedPackageContainerMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SharedPackageContainerMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SharedPackageContainerMember {}
unsafe impl ::std::marker::Sync for SharedPackageContainerMember {}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for SharedPackageContainerOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SharedPackageContainerOperationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SharedPackageContainerOperationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerOperationStatus;i4)");
}
impl ::windows::runtime::DefaultType for SharedPackageContainerOperationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct StagePackageOptions(pub ::windows::runtime::IInspectable);
impl StagePackageOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StagePackageOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn DependencyPackageUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn TargetVolume(&self) -> ::windows::runtime::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PackageVolume>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetTargetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, PackageVolume>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation_Collections`*"]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn OptionalPackageUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`, `Foundation_Collections`*"]
    pub fn RelatedPackageUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn ExternalLocationUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Deployment`, `Foundation`*"]
    pub fn SetExternalLocationUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn StubPackageOption(&self) -> ::windows::runtime::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__: StubPackageOption = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<StubPackageOption>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn DeveloperMode(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn InstallAllResources(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn RequiredContentGroupOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn StageInPlace(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetStageInPlace(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn AllowUnsigned(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StagePackageOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.StagePackageOptions;{0b110c9c-b95d-4c56-bd36-6d656800d06b})");
}
unsafe impl ::windows::runtime::Interface for StagePackageOptions {
    type Vtable = IStagePackageOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(185666716, 47453, 19542, [189, 54, 109, 101, 104, 0, 208, 107]);
}
impl ::windows::runtime::RuntimeName for StagePackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.StagePackageOptions";
}
impl ::std::convert::From<StagePackageOptions> for ::windows::runtime::IUnknown {
    fn from(value: StagePackageOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&StagePackageOptions> for ::windows::runtime::IUnknown {
    fn from(value: &StagePackageOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for StagePackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a StagePackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<StagePackageOptions> for ::windows::runtime::IInspectable {
    fn from(value: StagePackageOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StagePackageOptions> for ::windows::runtime::IInspectable {
    fn from(value: &StagePackageOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for StagePackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a StagePackageOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for StagePackageOptions {}
unsafe impl ::std::marker::Sync for StagePackageOptions {}
#[doc = "*Required features: `Management_Deployment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct StubPackageOption(pub i32);
impl StubPackageOption {
    pub const Default: StubPackageOption = StubPackageOption(0i32);
    pub const InstallFull: StubPackageOption = StubPackageOption(1i32);
    pub const InstallStub: StubPackageOption = StubPackageOption(2i32);
    pub const UsePreference: StubPackageOption = StubPackageOption(3i32);
}
impl ::std::convert::From<i32> for StubPackageOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for StubPackageOption {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for StubPackageOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.StubPackageOption;i4)");
}
impl ::windows::runtime::DefaultType for StubPackageOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UpdateSharedPackageContainerOptions(pub ::windows::runtime::IInspectable);
impl UpdateSharedPackageContainerOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UpdateSharedPackageContainerOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ForceAppShutdown(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn RequirePackagesPresent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn SetRequirePackagesPresent(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UpdateSharedPackageContainerOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.UpdateSharedPackageContainerOptions;{80672e83-7194-59f9-b5b9-daa5375f130a})");
}
unsafe impl ::windows::runtime::Interface for UpdateSharedPackageContainerOptions {
    type Vtable = IUpdateSharedPackageContainerOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2154245763, 29076, 23033, [181, 185, 218, 165, 55, 95, 19, 10]);
}
impl ::windows::runtime::RuntimeName for UpdateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerOptions";
}
impl ::std::convert::From<UpdateSharedPackageContainerOptions> for ::windows::runtime::IUnknown {
    fn from(value: UpdateSharedPackageContainerOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&UpdateSharedPackageContainerOptions> for ::windows::runtime::IUnknown {
    fn from(value: &UpdateSharedPackageContainerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<UpdateSharedPackageContainerOptions> for ::windows::runtime::IInspectable {
    fn from(value: UpdateSharedPackageContainerOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UpdateSharedPackageContainerOptions> for ::windows::runtime::IInspectable {
    fn from(value: &UpdateSharedPackageContainerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UpdateSharedPackageContainerOptions {}
unsafe impl ::std::marker::Sync for UpdateSharedPackageContainerOptions {}
#[doc = "*Required features: `Management_Deployment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UpdateSharedPackageContainerResult(pub ::windows::runtime::IInspectable);
impl UpdateSharedPackageContainerResult {
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SharedPackageContainerOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SharedPackageContainerOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UpdateSharedPackageContainerResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.UpdateSharedPackageContainerResult;{aa407df7-c72d-5458-aea3-4645b6a8ee99})");
}
unsafe impl ::windows::runtime::Interface for UpdateSharedPackageContainerResult {
    type Vtable = IUpdateSharedPackageContainerResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2856353271, 50989, 21592, [174, 163, 70, 69, 182, 168, 238, 153]);
}
impl ::windows::runtime::RuntimeName for UpdateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerResult";
}
impl ::std::convert::From<UpdateSharedPackageContainerResult> for ::windows::runtime::IUnknown {
    fn from(value: UpdateSharedPackageContainerResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&UpdateSharedPackageContainerResult> for ::windows::runtime::IUnknown {
    fn from(value: &UpdateSharedPackageContainerResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<UpdateSharedPackageContainerResult> for ::windows::runtime::IInspectable {
    fn from(value: UpdateSharedPackageContainerResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UpdateSharedPackageContainerResult> for ::windows::runtime::IInspectable {
    fn from(value: &UpdateSharedPackageContainerResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UpdateSharedPackageContainerResult {}
unsafe impl ::std::marker::Sync for UpdateSharedPackageContainerResult {}
