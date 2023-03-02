#[cfg(feature = "Management_Deployment_Preview")]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAddPackageOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAddPackageOptions {
    type Vtable = IAddPackageOptions_Vtbl;
}
impl ::core::clone::Clone for IAddPackageOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAddPackageOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05cee018_f68f_422b_95a4_66679ec77fc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    pub TargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RelatedPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RelatedPackageUris: usize,
    #[cfg(feature = "Foundation")]
    pub ExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExternalLocationUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExternalLocationUri: usize,
    pub StubPackageOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StubPackageOption) -> ::windows::core::HRESULT,
    pub SetStubPackageOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StubPackageOption) -> ::windows::core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ForceTargetAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceTargetAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RequiredContentGroupOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRequiredContentGroupOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RetainFilesOnFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRetainFilesOnFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAddPackageOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAddPackageOptions2 {
    type Vtable = IAddPackageOptions2_Vtbl;
}
impl ::core::clone::Clone for IAddPackageOptions2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAddPackageOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee515828_bf33_40f7_84af_1b6fad2919d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ExpectedDigests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExpectedDigests: usize,
    pub LimitToExistingPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetLimitToExistingPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallerManager {
    type Vtable = IAppInstallerManager_Vtbl;
}
impl ::core::clone::Clone for IAppInstallerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppInstallerManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7ee21c3_2103_53ee_9b18_68afeab0033d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetAutoUpdateSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, appinstallerinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearAutoUpdateSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PauseAutoUpdatesUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, datetime: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseAutoUpdatesUntil: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallerManagerStatics {
    type Vtable = IAppInstallerManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IAppInstallerManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppInstallerManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc95a6ed5_fc59_5336_9b2e_2b07c5e61434);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutoUpdateSettingsOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutoUpdateSettingsOptions {
    type Vtable = IAutoUpdateSettingsOptions_Vtbl;
}
impl ::core::clone::Clone for IAutoUpdateSettingsOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAutoUpdateSettingsOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67491d87_35e1_512a_8968_1ae88d1be6d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::PackageVersion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Version: usize,
    #[cfg(feature = "ApplicationModel")]
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::ApplicationModel::PackageVersion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SetVersion: usize,
    #[cfg(feature = "Foundation")]
    pub AppInstallerUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppInstallerUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetAppInstallerUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAppInstallerUri: usize,
    pub OnLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetOnLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub HoursBetweenUpdateChecks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetHoursBetweenUpdateChecks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ShowPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub UpdateBlocksActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetUpdateBlocksActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AutomaticBackgroundTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutomaticBackgroundTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsAutoRepairEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAutoRepairEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RepairUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RepairUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageUris: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutoUpdateSettingsOptionsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutoUpdateSettingsOptionsStatics {
    type Vtable = IAutoUpdateSettingsOptionsStatics_Vtbl;
}
impl ::core::clone::Clone for IAutoUpdateSettingsOptionsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAutoUpdateSettingsOptionsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x887b337d_0c05_54d0_bd49_3bb7a2c084cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptionsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub CreateFromAppInstallerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appinstallerinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    CreateFromAppInstallerInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateSharedPackageContainerOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateSharedPackageContainerOptions {
    type Vtable = ICreateSharedPackageContainerOptions_Vtbl;
}
impl ::core::clone::Clone for ICreateSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateSharedPackageContainerOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2ab6ece_f664_5c8e_a4b3_2a33276d3dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Members: usize,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CreateCollisionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerCreationCollisionOptions) -> ::windows::core::HRESULT,
    pub SetCreateCollisionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateSharedPackageContainerResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateSharedPackageContainerResult {
    type Vtable = ICreateSharedPackageContainerResult_Vtbl;
}
impl ::core::clone::Clone for ICreateSharedPackageContainerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateSharedPackageContainerResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce8810bf_151c_5707_b936_497e564afc7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeleteSharedPackageContainerOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeleteSharedPackageContainerOptions {
    type Vtable = IDeleteSharedPackageContainerOptions_Vtbl;
}
impl ::core::clone::Clone for IDeleteSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeleteSharedPackageContainerOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d81865f_986e_5138_8b5d_384d8e66ed6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeleteSharedPackageContainerResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeleteSharedPackageContainerResult {
    type Vtable = IDeleteSharedPackageContainerResult_Vtbl;
}
impl ::core::clone::Clone for IDeleteSharedPackageContainerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeleteSharedPackageContainerResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35398884_5736_517b_85bc_e598c81ab284);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
}
impl ::core::clone::Clone for IDeploymentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeploymentResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2563b9ae_b77d_4c1f_8a7b_20e6ad515ef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeploymentResult2 {
    type Vtable = IDeploymentResult2_Vtbl;
}
impl ::core::clone::Clone for IDeploymentResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeploymentResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc0e715c_5a01_4bd7_bcf1_381c8c82e04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsRegistered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFindSharedPackageContainerOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFindSharedPackageContainerOptions {
    type Vtable = IFindSharedPackageContainerOptions_Vtbl;
}
impl ::core::clone::Clone for IFindSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFindSharedPackageContainerOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb40fc8fe_8384_54cc_817d_ae09d3b6a606);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindSharedPackageContainerOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageAllUserProvisioningOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageAllUserProvisioningOptions {
    type Vtable = IPackageAllUserProvisioningOptions_Vtbl;
}
impl ::core::clone::Clone for IPackageAllUserProvisioningOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageAllUserProvisioningOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda35aa22_1de0_5d3e_99ff_d24f3118bf5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageAllUserProvisioningOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProjectionOrderPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProjectionOrderPackageFamilyNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager {
    type Vtable = IPackageManager_Vtbl;
}
impl ::core::clone::Clone for IPackageManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a7d4b65_5e8f_4fc7_a2e5_7f6925cb8b53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddPackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdatePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdatePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackageAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackages: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityId: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagepublisher: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagepublisher: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdNamePublisher: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUsers: usize,
    pub SetPackageState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagestate: PackageState) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackageByPackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackageByPackageFullName: usize,
    #[cfg(feature = "Foundation")]
    pub CleanupPackageForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CleanupPackageForUserAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByPackageFamilyName: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdPackageFamilyName: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackageByUserSecurityIdPackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackageByUserSecurityIdPackageFullName: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager10(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager10 {
    type Vtable = IPackageManager10_Vtbl;
}
impl ::core::clone::Clone for IPackageManager10 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager10 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7d7d07e_2e66_4093_aed5_e093ed87b3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager10_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ProvisionPackageForAllUsersWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainpackagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProvisionPackageForAllUsersWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager2 {
    type Vtable = IPackageManager2_Vtbl;
}
impl ::core::clone::Clone for IPackageManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7aad08d_0840_46f2_b5d8_cad47693a095);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RemovePackageWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, removaloptions: RemovalOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageWithOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageWithOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackageByFullNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainpackagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, dependencypackagefullnames: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackageByFullNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisherWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagepublisher: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisherWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagepublisher: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByPackageFamilyNameWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes: usize,
    #[cfg(feature = "Foundation")]
    pub StageUserDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StageUserDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager3 {
    type Vtable = IPackageManager3_Vtbl;
}
impl ::core::clone::Clone for IPackageManager3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaad9948_36f1_41a7_9188_bc263e0dcb72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AddPackageVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagestorepath: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageVolumeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AddPackageToVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddPackageToVolumeAsync: usize,
    pub ClearPackageStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, status: PackageStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackageWithAppDataVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, appdatavolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackageWithAppDataVolumeAsync: usize,
    pub FindPackageVolumeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindPackageVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindPackageVolumes: usize,
    pub GetDefaultPackageVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MovePackageToVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MovePackageToVolumeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageVolumeAsync: usize,
    pub SetDefaultPackageVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPackageStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, status: PackageStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPackageVolumeOfflineAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagevolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPackageVolumeOfflineAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetPackageVolumeOnlineAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagevolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPackageVolumeOnlineAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageToVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageToVolumeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StageUserDataWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StageUserDataWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager4 {
    type Vtable = IPackageManager4_Vtbl;
}
impl ::core::clone::Clone for IPackageManager4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c719963_bab6_46bf_8ff7_da4719230ae6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPackageVolumesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPackageVolumesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager5 {
    type Vtable = IPackageManager5_Vtbl;
}
impl ::core::clone::Clone for IPackageManager5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x711f3117_1afd_4313_978c_9bb6e1b864a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddPackageToVolumeAndOptionalPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, externalpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddPackageToVolumeAndOptionalPackagesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageToVolumeAndOptionalPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, externalpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageToVolumeAndOptionalPackagesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackageByFamilyNameAndOptionalPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainpackagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, dependencypackagefamilynames: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, appdatavolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackageByFamilyNameAndOptionalPackagesAsync: usize,
    pub DebugSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager6 {
    type Vtable = IPackageManager6_Vtbl;
}
impl ::core::clone::Clone for IPackageManager6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0847e909_53cd_4e4f_832e_57d180f6e447);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager6_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ProvisionPackageForAllUsersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProvisionPackageForAllUsersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddPackageByAppInstallerFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appinstallerfileuri: *mut ::core::ffi::c_void, options: AddPackageByAppInstallerOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageByAppInstallerFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAddPackageByAppInstallerFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appinstallerfileuri: *mut ::core::ffi::c_void, options: AddPackageByAppInstallerOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAddPackageByAppInstallerFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AddPackageToVolumeAndRelatedSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, options: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, packageuristoinstall: *mut ::core::ffi::c_void, relatedpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddPackageToVolumeAndRelatedSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageToVolumeAndRelatedSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, options: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, packageuristoinstall: *mut ::core::ffi::c_void, relatedpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageToVolumeAndRelatedSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAddPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, relatedpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAddPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager7 {
    type Vtable = IPackageManager7_Vtbl;
}
impl ::core::clone::Clone for IPackageManager7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf28654f4_2ba7_4b80_88d6_be15f9a23fba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager7_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAddPackageAndRelatedSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, relatedpackageuris: *mut ::core::ffi::c_void, packageuristoinstall: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAddPackageAndRelatedSetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager8(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager8 {
    type Vtable = IPackageManager8_Vtbl;
}
impl ::core::clone::Clone for IPackageManager8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager8 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8575330_1298_4ee2_80ee_7f659c5d2782);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager8_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DeprovisionPackageForAllUsersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeprovisionPackageForAllUsersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager9(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManager9 {
    type Vtable = IPackageManager9_Vtbl;
}
impl ::core::clone::Clone for IPackageManager9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManager9 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aa79035_cc71_4b2e_80a6_c7041d8579a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager9_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindProvisionedPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindProvisionedPackages: usize,
    #[cfg(feature = "Foundation")]
    pub AddPackageByUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageByUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StagePackageByUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StagePackageByUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RegisterPackageByUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterPackageByUriAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackagesByFullNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullnames: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackagesByFullNameAsync: usize,
    pub SetPackageStubPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, usestub: PackageStubPreference) -> ::windows::core::HRESULT,
    pub GetPackageStubPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut PackageStubPreference) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManagerDebugSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageManagerDebugSettings {
    type Vtable = IPackageManagerDebugSettings_Vtbl;
}
impl ::core::clone::Clone for IPackageManagerDebugSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageManagerDebugSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a611683_a988_4fcf_8f0f_ce175898e8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManagerDebugSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub SetContentGroupStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, contentgroupname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    SetContentGroupStateAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub SetContentGroupStateWithPercentageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, contentgroupname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    SetContentGroupStateWithPercentageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUserInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageUserInformation {
    type Vtable = IPackageUserInformation_Vtbl;
}
impl ::core::clone::Clone for IPackageUserInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageUserInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6383423_fa09_4cbc_9055_15ca275e2e7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUserInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UserSecurityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageInstallState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageVolume(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageVolume {
    type Vtable = IPackageVolume_Vtbl;
}
impl ::core::clone::Clone for IPackageVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageVolume {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2672c3_1a40_4450_9739_2ace2e898853);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSystemVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MountPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PackageStorePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SupportsHardLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackages: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagepublisher: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByPackageFamilyName: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisherWithPackagesTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, packagename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagepublisher: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisherWithPackagesTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByPackageFamilyNameWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackageByPackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackageByPackageFullName: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityId: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagepublisher: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdNamePublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdPackageFamilyName: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagetypes: PackageTypes, packagename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagepublisher: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagetypes: PackageTypes, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackageByUserSecurityIdPackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, packagefullname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackageByUserSecurityIdPackageFullName: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageVolume2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPackageVolume2 {
    type Vtable = IPackageVolume2_Vtbl;
}
impl ::core::clone::Clone for IPackageVolume2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPackageVolume2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46abcf2e_9dd4_47a2_ab8c_c6408349bcd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsFullTrustPackageSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAppxInstallSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAvailableSpaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAvailableSpaceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRegisterPackageOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRegisterPackageOptions {
    type Vtable = IRegisterPackageOptions_Vtbl;
}
impl ::core::clone::Clone for IRegisterPackageOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRegisterPackageOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x677112a7_50d4_496c_8415_0602b4c6d3bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisterPackageOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    pub AppDataVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAppDataVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "Foundation")]
    pub ExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExternalLocationUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExternalLocationUri: usize,
    pub DeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ForceTargetAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceTargetAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRegisterPackageOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRegisterPackageOptions2 {
    type Vtable = IRegisterPackageOptions2_Vtbl;
}
impl ::core::clone::Clone for IRegisterPackageOptions2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRegisterPackageOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3dfa9743_86ff_4a11_bc93_434eb6be3a0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisterPackageOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ExpectedDigests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExpectedDigests: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISharedPackageContainer {
    type Vtable = ISharedPackageContainer_Vtbl;
}
impl ::core::clone::Clone for ISharedPackageContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISharedPackageContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x177f1aa9_151e_5ef7_b1d9_2fba0b4b0d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMembers: usize,
    pub RemovePackageFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainerManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISharedPackageContainerManager {
    type Vtable = ISharedPackageContainerManager_Vtbl;
}
impl ::core::clone::Clone for ISharedPackageContainerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISharedPackageContainerManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe353068_1ef7_5ac8_ab3f_0b9f612f0274);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContainers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContainers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContainersWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContainersWithOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainerManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISharedPackageContainerManagerStatics {
    type Vtable = ISharedPackageContainerManagerStatics_Vtbl;
}
impl ::core::clone::Clone for ISharedPackageContainerManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISharedPackageContainerManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ef56348_838a_5f55_a89e_1198a2c627e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForProvisioning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainerMember(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISharedPackageContainerMember {
    type Vtable = ISharedPackageContainerMember_Vtbl;
}
impl ::core::clone::Clone for ISharedPackageContainerMember {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISharedPackageContainerMember {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe0d0438_43c9_5426_b89c_f79bf85ddff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMember_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainerMemberFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISharedPackageContainerMemberFactory {
    type Vtable = ISharedPackageContainerMemberFactory_Vtbl;
}
impl ::core::clone::Clone for ISharedPackageContainerMemberFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISharedPackageContainerMemberFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49b0ceeb_498f_5a62_b738_b3ca0d436704);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMemberFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStagePackageOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStagePackageOptions {
    type Vtable = IStagePackageOptions_Vtbl;
}
impl ::core::clone::Clone for IStagePackageOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStagePackageOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b110c9c_b95d_4c56_bd36_6d656800d06b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStagePackageOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    pub TargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RelatedPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RelatedPackageUris: usize,
    #[cfg(feature = "Foundation")]
    pub ExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExternalLocationUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExternalLocationUri: usize,
    pub StubPackageOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StubPackageOption) -> ::windows::core::HRESULT,
    pub SetStubPackageOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StubPackageOption) -> ::windows::core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RequiredContentGroupOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRequiredContentGroupOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStagePackageOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStagePackageOptions2 {
    type Vtable = IStagePackageOptions2_Vtbl;
}
impl ::core::clone::Clone for IStagePackageOptions2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStagePackageOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x990c4ccc_6226_4192_ba92_79875fce0d9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStagePackageOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ExpectedDigests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExpectedDigests: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUpdateSharedPackageContainerOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUpdateSharedPackageContainerOptions {
    type Vtable = IUpdateSharedPackageContainerOptions_Vtbl;
}
impl ::core::clone::Clone for IUpdateSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUpdateSharedPackageContainerOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80672e83_7194_59f9_b5b9_daa5375f130a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RequirePackagesPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRequirePackagesPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUpdateSharedPackageContainerResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUpdateSharedPackageContainerResult {
    type Vtable = IUpdateSharedPackageContainerResult_Vtbl;
}
impl ::core::clone::Clone for IUpdateSharedPackageContainerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUpdateSharedPackageContainerResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa407df7_c72d_5458_aea3_4645b6a8ee99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct AddPackageOptions(::windows::core::IUnknown);
impl AddPackageOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AddPackageOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).DependencyPackageUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TargetVolume(&self) -> ::windows::core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PackageVolume>();
            (::windows::core::Interface::vtable(this).TargetVolume)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTargetVolume(&self, value: &PackageVolume) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetVolume)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).OptionalPackageFamilyNames)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).OptionalPackageUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RelatedPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).RelatedPackageUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ExternalLocationUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExternalLocationUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExternalLocationUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StubPackageOption(&self) -> ::windows::core::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StubPackageOption>();
            (::windows::core::Interface::vtable(this).StubPackageOption)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStubPackageOption)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeveloperMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DeveloperMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDeveloperMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceAppShutdown)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceAppShutdown)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceTargetAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceTargetAppShutdown)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceTargetAppShutdown)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).InstallAllResources)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInstallAllResources)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequiredContentGroupOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).RequiredContentGroupOnly)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequiredContentGroupOnly)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RetainFilesOnFailure(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).RetainFilesOnFailure)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRetainFilesOnFailure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRetainFilesOnFailure)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).StageInPlace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStageInPlace)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllowUnsigned)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowUnsigned)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DeferRegistrationWhenPackagesAreInUse)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDeferRegistrationWhenPackagesAreInUse)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExpectedDigests(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<super::super::Foundation::Uri, ::windows::core::HSTRING>> {
        let this = &::windows::core::ComInterface::cast::<IAddPackageOptions2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMap<super::super::Foundation::Uri, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).ExpectedDigests)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LimitToExistingPackages(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAddPackageOptions2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).LimitToExistingPackages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLimitToExistingPackages(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAddPackageOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLimitToExistingPackages)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for AddPackageOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPackageOptions {}
impl ::core::fmt::Debug for AddPackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPackageOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AddPackageOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AddPackageOptions;{05cee018-f68f-422b-95a4-66679ec77fc0})");
}
impl ::core::clone::Clone for AddPackageOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AddPackageOptions {
    type Vtable = IAddPackageOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AddPackageOptions {
    const IID: ::windows::core::GUID = <IAddPackageOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AddPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AddPackageOptions";
}
::windows::imp::interface_hierarchy!(AddPackageOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AddPackageOptions {}
unsafe impl ::core::marker::Sync for AddPackageOptions {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct AppInstallerManager(::windows::core::IUnknown);
impl AppInstallerManager {
    pub fn SetAutoUpdateSettings(&self, packagefamilyname: &::windows::core::HSTRING, appinstallerinfo: &AutoUpdateSettingsOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAutoUpdateSettings)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), ::core::mem::transmute_copy(appinstallerinfo)).ok() }
    }
    pub fn ClearAutoUpdateSettings(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearAutoUpdateSettings)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseAutoUpdatesUntil(&self, packagefamilyname: &::windows::core::HSTRING, datetime: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PauseAutoUpdatesUntil)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), datetime).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppInstallerManager>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForSystem() -> ::windows::core::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppInstallerManager>();
            (::windows::core::Interface::vtable(this).GetForSystem)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppInstallerManagerStatics<R, F: FnOnce(&IAppInstallerManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppInstallerManager, IAppInstallerManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AppInstallerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallerManager {}
impl ::core::fmt::Debug for AppInstallerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallerManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppInstallerManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AppInstallerManager;{e7ee21c3-2103-53ee-9b18-68afeab0033d})");
}
impl ::core::clone::Clone for AppInstallerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppInstallerManager {
    type Vtable = IAppInstallerManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppInstallerManager {
    const IID: ::windows::core::GUID = <IAppInstallerManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallerManager {
    const NAME: &'static str = "Windows.Management.Deployment.AppInstallerManager";
}
::windows::imp::interface_hierarchy!(AppInstallerManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallerManager {}
unsafe impl ::core::marker::Sync for AppInstallerManager {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct AutoUpdateSettingsOptions(::windows::core::IUnknown);
impl AutoUpdateSettingsOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AutoUpdateSettingsOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Version(&self) -> ::windows::core::Result<super::super::ApplicationModel::PackageVersion> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::ApplicationModel::PackageVersion>();
            (::windows::core::Interface::vtable(this).Version)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn SetVersion(&self, value: super::super::ApplicationModel::PackageVersion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVersion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppInstallerUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).AppInstallerUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAppInstallerUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAppInstallerUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn OnLaunch(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).OnLaunch)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOnLaunch(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOnLaunch)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoursBetweenUpdateChecks(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).HoursBetweenUpdateChecks)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHoursBetweenUpdateChecks(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHoursBetweenUpdateChecks)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowPrompt(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ShowPrompt)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShowPrompt(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShowPrompt)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateBlocksActivation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).UpdateBlocksActivation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUpdateBlocksActivation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUpdateBlocksActivation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticBackgroundTask(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AutomaticBackgroundTask)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutomaticBackgroundTask(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAutomaticBackgroundTask)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoRepairEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAutoRepairEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAutoRepairEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAutoRepairEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).UpdateUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RepairUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).RepairUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).DependencyPackageUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).OptionalPackageUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn CreateFromAppInstallerInfo(appinstallerinfo: &super::super::ApplicationModel::AppInstallerInfo) -> ::windows::core::Result<AutoUpdateSettingsOptions> {
        Self::IAutoUpdateSettingsOptionsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AutoUpdateSettingsOptions>();
            (::windows::core::Interface::vtable(this).CreateFromAppInstallerInfo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(appinstallerinfo), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAutoUpdateSettingsOptionsStatics<R, F: FnOnce(&IAutoUpdateSettingsOptionsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AutoUpdateSettingsOptions, IAutoUpdateSettingsOptionsStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AutoUpdateSettingsOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutoUpdateSettingsOptions {}
impl ::core::fmt::Debug for AutoUpdateSettingsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateSettingsOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AutoUpdateSettingsOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AutoUpdateSettingsOptions;{67491d87-35e1-512a-8968-1ae88d1be6d3})");
}
impl ::core::clone::Clone for AutoUpdateSettingsOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AutoUpdateSettingsOptions {
    type Vtable = IAutoUpdateSettingsOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AutoUpdateSettingsOptions {
    const IID: ::windows::core::GUID = <IAutoUpdateSettingsOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AutoUpdateSettingsOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AutoUpdateSettingsOptions";
}
::windows::imp::interface_hierarchy!(AutoUpdateSettingsOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AutoUpdateSettingsOptions {}
unsafe impl ::core::marker::Sync for AutoUpdateSettingsOptions {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct CreateSharedPackageContainerOptions(::windows::core::IUnknown);
impl CreateSharedPackageContainerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CreateSharedPackageContainerOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Members(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>();
            (::windows::core::Interface::vtable(this).Members)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceAppShutdown)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceAppShutdown)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateCollisionOption(&self) -> ::windows::core::Result<SharedPackageContainerCreationCollisionOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainerCreationCollisionOptions>();
            (::windows::core::Interface::vtable(this).CreateCollisionOption)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCreateCollisionOption(&self, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCreateCollisionOption)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for CreateSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateSharedPackageContainerOptions {}
impl ::core::fmt::Debug for CreateSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateSharedPackageContainerOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CreateSharedPackageContainerOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.CreateSharedPackageContainerOptions;{c2ab6ece-f664-5c8e-a4b3-2a33276d3dde})");
}
impl ::core::clone::Clone for CreateSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CreateSharedPackageContainerOptions {
    type Vtable = ICreateSharedPackageContainerOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CreateSharedPackageContainerOptions {
    const IID: ::windows::core::GUID = <ICreateSharedPackageContainerOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CreateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerOptions";
}
::windows::imp::interface_hierarchy!(CreateSharedPackageContainerOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CreateSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for CreateSharedPackageContainerOptions {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct CreateSharedPackageContainerResult(::windows::core::IUnknown);
impl CreateSharedPackageContainerResult {
    pub fn Container(&self) -> ::windows::core::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainer>();
            (::windows::core::Interface::vtable(this).Container)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainerOperationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CreateSharedPackageContainerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateSharedPackageContainerResult {}
impl ::core::fmt::Debug for CreateSharedPackageContainerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateSharedPackageContainerResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CreateSharedPackageContainerResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.CreateSharedPackageContainerResult;{ce8810bf-151c-5707-b936-497e564afc7a})");
}
impl ::core::clone::Clone for CreateSharedPackageContainerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CreateSharedPackageContainerResult {
    type Vtable = ICreateSharedPackageContainerResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CreateSharedPackageContainerResult {
    const IID: ::windows::core::GUID = <ICreateSharedPackageContainerResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CreateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerResult";
}
::windows::imp::interface_hierarchy!(CreateSharedPackageContainerResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CreateSharedPackageContainerResult {}
unsafe impl ::core::marker::Sync for CreateSharedPackageContainerResult {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct DeleteSharedPackageContainerOptions(::windows::core::IUnknown);
impl DeleteSharedPackageContainerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DeleteSharedPackageContainerOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceAppShutdown)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceAppShutdown)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllUsers(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllUsers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllUsers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllUsers)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for DeleteSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeleteSharedPackageContainerOptions {}
impl ::core::fmt::Debug for DeleteSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSharedPackageContainerOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeleteSharedPackageContainerOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeleteSharedPackageContainerOptions;{9d81865f-986e-5138-8b5d-384d8e66ed6c})");
}
impl ::core::clone::Clone for DeleteSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DeleteSharedPackageContainerOptions {
    type Vtable = IDeleteSharedPackageContainerOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DeleteSharedPackageContainerOptions {
    const IID: ::windows::core::GUID = <IDeleteSharedPackageContainerOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DeleteSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerOptions";
}
::windows::imp::interface_hierarchy!(DeleteSharedPackageContainerOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DeleteSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for DeleteSharedPackageContainerOptions {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct DeleteSharedPackageContainerResult(::windows::core::IUnknown);
impl DeleteSharedPackageContainerResult {
    pub fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainerOperationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeleteSharedPackageContainerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeleteSharedPackageContainerResult {}
impl ::core::fmt::Debug for DeleteSharedPackageContainerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSharedPackageContainerResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeleteSharedPackageContainerResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeleteSharedPackageContainerResult;{35398884-5736-517b-85bc-e598c81ab284})");
}
impl ::core::clone::Clone for DeleteSharedPackageContainerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DeleteSharedPackageContainerResult {
    type Vtable = IDeleteSharedPackageContainerResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DeleteSharedPackageContainerResult {
    const IID: ::windows::core::GUID = <IDeleteSharedPackageContainerResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DeleteSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerResult";
}
::windows::imp::interface_hierarchy!(DeleteSharedPackageContainerResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DeleteSharedPackageContainerResult {}
unsafe impl ::core::marker::Sync for DeleteSharedPackageContainerResult {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct DeploymentResult(::windows::core::IUnknown);
impl DeploymentResult {
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ActivityId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRegistered(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IDeploymentResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRegistered)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeploymentResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeploymentResult {}
impl ::core::fmt::Debug for DeploymentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeploymentResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeploymentResult;{2563b9ae-b77d-4c1f-8a7b-20e6ad515ef3})");
}
impl ::core::clone::Clone for DeploymentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DeploymentResult {
    const IID: ::windows::core::GUID = <IDeploymentResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DeploymentResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeploymentResult";
}
::windows::imp::interface_hierarchy!(DeploymentResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DeploymentResult {}
unsafe impl ::core::marker::Sync for DeploymentResult {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct FindSharedPackageContainerOptions(::windows::core::IUnknown);
impl FindSharedPackageContainerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<FindSharedPackageContainerOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPackageFamilyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for FindSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindSharedPackageContainerOptions {}
impl ::core::fmt::Debug for FindSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindSharedPackageContainerOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FindSharedPackageContainerOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.FindSharedPackageContainerOptions;{b40fc8fe-8384-54cc-817d-ae09d3b6a606})");
}
impl ::core::clone::Clone for FindSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FindSharedPackageContainerOptions {
    type Vtable = IFindSharedPackageContainerOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FindSharedPackageContainerOptions {
    const IID: ::windows::core::GUID = <IFindSharedPackageContainerOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FindSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.FindSharedPackageContainerOptions";
}
::windows::imp::interface_hierarchy!(FindSharedPackageContainerOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for FindSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for FindSharedPackageContainerOptions {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageAllUserProvisioningOptions(::windows::core::IUnknown);
impl PackageAllUserProvisioningOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PackageAllUserProvisioningOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).OptionalPackageFamilyNames)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProjectionOrderPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).ProjectionOrderPackageFamilyNames)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PackageAllUserProvisioningOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageAllUserProvisioningOptions {}
impl ::core::fmt::Debug for PackageAllUserProvisioningOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageAllUserProvisioningOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PackageAllUserProvisioningOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageAllUserProvisioningOptions;{da35aa22-1de0-5d3e-99ff-d24f3118bf5e})");
}
impl ::core::clone::Clone for PackageAllUserProvisioningOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PackageAllUserProvisioningOptions {
    type Vtable = IPackageAllUserProvisioningOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PackageAllUserProvisioningOptions {
    const IID: ::windows::core::GUID = <IPackageAllUserProvisioningOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PackageAllUserProvisioningOptions {
    const NAME: &'static str = "Windows.Management.Deployment.PackageAllUserProvisioningOptions";
}
::windows::imp::interface_hierarchy!(PackageAllUserProvisioningOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageAllUserProvisioningOptions {}
unsafe impl ::core::marker::Sync for PackageAllUserProvisioningOptions {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageManager(::windows::core::IUnknown);
impl PackageManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PackageManager, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddPackageAsync<P0>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).AddPackageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdatePackageAsync<P0>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).UpdatePackageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageAsync(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RemovePackageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StagePackageAsync<P0>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).StagePackageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterPackageAsync<P0>(&self, manifesturi: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RegisterPackageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(manifesturi), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityId(&self, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisher(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByNamePublisher)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdNamePublisher(&self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisher)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindUsers(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageUserInformation>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<PackageUserInformation>>();
            (::windows::core::Interface::vtable(this).FindUsers)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPackageState(&self, packagefullname: &::windows::core::HSTRING, packagestate: PackageState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPackageState)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), packagestate).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackageByPackageFullName(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::ApplicationModel::Package>();
            (::windows::core::Interface::vtable(this).FindPackageByPackageFullName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CleanupPackageForUserAsync(&self, packagename: &::windows::core::HSTRING, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).CleanupPackageForUserAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(usersecurityid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByPackageFamilyName(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByPackageFamilyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName(&self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackageByUserSecurityIdPackageFullName(&self, usersecurityid: &::windows::core::HSTRING, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::ApplicationModel::Package>();
            (::windows::core::Interface::vtable(this).FindPackageByUserSecurityIdPackageFullName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), ::core::mem::transmute_copy(packagefullname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProvisionPackageForAllUsersWithOptionsAsync(&self, mainpackagefamilyname: &::windows::core::HSTRING, options: &PackageAllUserProvisioningOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager10>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).ProvisionPackageForAllUsersWithOptionsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(mainpackagefamilyname), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageWithOptionsAsync(&self, packagefullname: &::windows::core::HSTRING, removaloptions: RemovalOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RemovePackageWithOptionsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), removaloptions, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StagePackageWithOptionsAsync<P0>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).StagePackageWithOptionsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterPackageByFullNameAsync<P0>(&self, mainpackagefullname: &::windows::core::HSTRING, dependencypackagefullnames: P0, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RegisterPackageByFullNameAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(mainpackagefullname), dependencypackagefullnames.try_into_param()?.abi(), deploymentoptions, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesWithPackageTypes)(::windows::core::Interface::as_raw(this), packagetypes, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdWithPackageTypes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), packagetypes, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisherWithPackageTypes(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByNamePublisherWithPackageTypes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), packagetypes, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), packagetypes, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packagefamilyname: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByPackageFamilyNameWithPackageTypes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), packagetypes, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), ::core::mem::transmute_copy(packagefamilyname), packagetypes, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StageUserDataAsync(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).StageUserDataAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddPackageVolumeAsync(&self, packagestorepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PackageVolume>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<PackageVolume>>();
            (::windows::core::Interface::vtable(this).AddPackageVolumeAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagestorepath), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddPackageToVolumeAsync<P0>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions, targetvolume: &PackageVolume) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).AddPackageToVolumeAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, ::core::mem::transmute_copy(targetvolume), &mut result__).from_abi(result__)
        }
    }
    pub fn ClearPackageStatus(&self, packagefullname: &::windows::core::HSTRING, status: PackageStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ClearPackageStatus)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), status).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterPackageWithAppDataVolumeAsync<P0>(&self, manifesturi: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions, appdatavolume: &PackageVolume) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RegisterPackageWithAppDataVolumeAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(manifesturi), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, ::core::mem::transmute_copy(appdatavolume), &mut result__).from_abi(result__)
        }
    }
    pub fn FindPackageVolumeByName(&self, volumename: &::windows::core::HSTRING) -> ::windows::core::Result<PackageVolume> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PackageVolume>();
            (::windows::core::Interface::vtable(this).FindPackageVolumeByName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(volumename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindPackageVolumes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageVolume>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<PackageVolume>>();
            (::windows::core::Interface::vtable(this).FindPackageVolumes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefaultPackageVolume(&self) -> ::windows::core::Result<PackageVolume> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PackageVolume>();
            (::windows::core::Interface::vtable(this).GetDefaultPackageVolume)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MovePackageToVolumeAsync(&self, packagefullname: &::windows::core::HSTRING, deploymentoptions: DeploymentOptions, targetvolume: &PackageVolume) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).MovePackageToVolumeAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), deploymentoptions, ::core::mem::transmute_copy(targetvolume), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageVolumeAsync(&self, volume: &PackageVolume) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RemovePackageVolumeAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(volume), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDefaultPackageVolume(&self, volume: &PackageVolume) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultPackageVolume)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(volume)).ok() }
    }
    pub fn SetPackageStatus(&self, packagefullname: &::windows::core::HSTRING, status: PackageStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPackageStatus)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), status).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPackageVolumeOfflineAsync(&self, packagevolume: &PackageVolume) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).SetPackageVolumeOfflineAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagevolume), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPackageVolumeOnlineAsync(&self, packagevolume: &PackageVolume) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).SetPackageVolumeOnlineAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagevolume), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StagePackageToVolumeAsync<P0>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions, targetvolume: &PackageVolume) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).StagePackageToVolumeAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, ::core::mem::transmute_copy(targetvolume), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StageUserDataWithOptionsAsync(&self, packagefullname: &::windows::core::HSTRING, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).StageUserDataWithOptionsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), deploymentoptions, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPackageVolumesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageVolume>>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageVolume>>>();
            (::windows::core::Interface::vtable(this).GetPackageVolumesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddPackageToVolumeAndOptionalPackagesAsync<P0, P1, P2>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions, targetvolume: &PackageVolume, optionalpackagefamilynames: P1, externalpackageuris: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        P2: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).AddPackageToVolumeAndOptionalPackagesAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, ::core::mem::transmute_copy(targetvolume), optionalpackagefamilynames.try_into_param()?.abi(), externalpackageuris.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StagePackageToVolumeAndOptionalPackagesAsync<P0, P1, P2>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions, targetvolume: &PackageVolume, optionalpackagefamilynames: P1, externalpackageuris: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        P2: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).StagePackageToVolumeAndOptionalPackagesAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, ::core::mem::transmute_copy(targetvolume), optionalpackagefamilynames.try_into_param()?.abi(), externalpackageuris.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterPackageByFamilyNameAndOptionalPackagesAsync<P0, P1>(&self, mainpackagefamilyname: &::windows::core::HSTRING, dependencypackagefamilynames: P0, deploymentoptions: DeploymentOptions, appdatavolume: &PackageVolume, optionalpackagefamilynames: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RegisterPackageByFamilyNameAndOptionalPackagesAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(mainpackagefamilyname), dependencypackagefamilynames.try_into_param()?.abi(), deploymentoptions, ::core::mem::transmute_copy(appdatavolume), optionalpackagefamilynames.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn DebugSettings(&self) -> ::windows::core::Result<PackageManagerDebugSettings> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PackageManagerDebugSettings>();
            (::windows::core::Interface::vtable(this).DebugSettings)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProvisionPackageForAllUsersAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).ProvisionPackageForAllUsersAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddPackageByAppInstallerFileAsync(&self, appinstallerfileuri: &super::super::Foundation::Uri, options: AddPackageByAppInstallerOptions, targetvolume: &PackageVolume) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).AddPackageByAppInstallerFileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(appinstallerfileuri), options, ::core::mem::transmute_copy(targetvolume), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAddPackageByAppInstallerFileAsync(&self, appinstallerfileuri: &super::super::Foundation::Uri, options: AddPackageByAppInstallerOptions, targetvolume: &PackageVolume) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RequestAddPackageByAppInstallerFileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(appinstallerfileuri), options, ::core::mem::transmute_copy(targetvolume), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddPackageToVolumeAndRelatedSetAsync<P0, P1, P2, P3>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, options: DeploymentOptions, targetvolume: &PackageVolume, optionalpackagefamilynames: P1, packageuristoinstall: P2, relatedpackageuris: P3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        P2: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        P3: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).AddPackageToVolumeAndRelatedSetAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), options, ::core::mem::transmute_copy(targetvolume), optionalpackagefamilynames.try_into_param()?.abi(), packageuristoinstall.try_into_param()?.abi(), relatedpackageuris.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StagePackageToVolumeAndRelatedSetAsync<P0, P1, P2, P3>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, options: DeploymentOptions, targetvolume: &PackageVolume, optionalpackagefamilynames: P1, packageuristoinstall: P2, relatedpackageuris: P3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        P2: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        P3: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).StagePackageToVolumeAndRelatedSetAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), options, ::core::mem::transmute_copy(targetvolume), optionalpackagefamilynames.try_into_param()?.abi(), packageuristoinstall.try_into_param()?.abi(), relatedpackageuris.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestAddPackageAsync<P0, P1, P2>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions, targetvolume: &PackageVolume, optionalpackagefamilynames: P1, relatedpackageuris: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        P2: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RequestAddPackageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, ::core::mem::transmute_copy(targetvolume), optionalpackagefamilynames.try_into_param()?.abi(), relatedpackageuris.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestAddPackageAndRelatedSetAsync<P0, P1, P2, P3>(&self, packageuri: &super::super::Foundation::Uri, dependencypackageuris: P0, deploymentoptions: DeploymentOptions, targetvolume: &PackageVolume, optionalpackagefamilynames: P1, relatedpackageuris: P2, packageuristoinstall: P3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        P2: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        P3: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager7>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RequestAddPackageAndRelatedSetAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), dependencypackageuris.try_into_param()?.abi(), deploymentoptions, ::core::mem::transmute_copy(targetvolume), optionalpackagefamilynames.try_into_param()?.abi(), relatedpackageuris.try_into_param()?.abi(), packageuristoinstall.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeprovisionPackageForAllUsersAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager8>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).DeprovisionPackageForAllUsersAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindProvisionedPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindProvisionedPackages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddPackageByUriAsync(&self, packageuri: &super::super::Foundation::Uri, options: &AddPackageOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).AddPackageByUriAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StagePackageByUriAsync(&self, packageuri: &super::super::Foundation::Uri, options: &StagePackageOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).StagePackageByUriAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packageuri), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterPackageByUriAsync(&self, manifesturi: &super::super::Foundation::Uri, options: &RegisterPackageOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RegisterPackageByUriAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(manifesturi), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterPackagesByFullNameAsync<P0>(&self, packagefullnames: P0, options: &RegisterPackageOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>();
            (::windows::core::Interface::vtable(this).RegisterPackagesByFullNameAsync)(::windows::core::Interface::as_raw(this), packagefullnames.try_into_param()?.abi(), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPackageStubPreference(&self, packagefamilyname: &::windows::core::HSTRING, usestub: PackageStubPreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager9>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPackageStubPreference)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), usestub).ok() }
    }
    pub fn GetPackageStubPreference(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<PackageStubPreference> {
        let this = &::windows::core::ComInterface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PackageStubPreference>();
            (::windows::core::Interface::vtable(this).GetPackageStubPreference)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PackageManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageManager {}
impl ::core::fmt::Debug for PackageManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PackageManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageManager;{9a7d4b65-5e8f-4fc7-a2e5-7f6925cb8b53})");
}
impl ::core::clone::Clone for PackageManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PackageManager {
    type Vtable = IPackageManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PackageManager {
    const IID: ::windows::core::GUID = <IPackageManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PackageManager {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManager";
}
::windows::imp::interface_hierarchy!(PackageManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageManager {}
unsafe impl ::core::marker::Sync for PackageManager {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageManagerDebugSettings(::windows::core::IUnknown);
impl PackageManagerDebugSettings {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn SetContentGroupStateAsync(&self, package: &super::super::ApplicationModel::Package, contentgroupname: &::windows::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SetContentGroupStateAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(package), ::core::mem::transmute_copy(contentgroupname), state, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn SetContentGroupStateWithPercentageAsync(&self, package: &super::super::ApplicationModel::Package, contentgroupname: &::windows::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SetContentGroupStateWithPercentageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(package), ::core::mem::transmute_copy(contentgroupname), state, completionpercentage, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PackageManagerDebugSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageManagerDebugSettings {}
impl ::core::fmt::Debug for PackageManagerDebugSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageManagerDebugSettings").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PackageManagerDebugSettings {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageManagerDebugSettings;{1a611683-a988-4fcf-8f0f-ce175898e8eb})");
}
impl ::core::clone::Clone for PackageManagerDebugSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PackageManagerDebugSettings {
    type Vtable = IPackageManagerDebugSettings_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PackageManagerDebugSettings {
    const IID: ::windows::core::GUID = <IPackageManagerDebugSettings as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PackageManagerDebugSettings {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManagerDebugSettings";
}
::windows::imp::interface_hierarchy!(PackageManagerDebugSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageManagerDebugSettings {}
unsafe impl ::core::marker::Sync for PackageManagerDebugSettings {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageUserInformation(::windows::core::IUnknown);
impl PackageUserInformation {
    pub fn UserSecurityId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).UserSecurityId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstallState(&self) -> ::windows::core::Result<PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PackageInstallState>();
            (::windows::core::Interface::vtable(this).InstallState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PackageUserInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageUserInformation {}
impl ::core::fmt::Debug for PackageUserInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUserInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PackageUserInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageUserInformation;{f6383423-fa09-4cbc-9055-15ca275e2e7e})");
}
impl ::core::clone::Clone for PackageUserInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PackageUserInformation {
    type Vtable = IPackageUserInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PackageUserInformation {
    const IID: ::windows::core::GUID = <IPackageUserInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PackageUserInformation {
    const NAME: &'static str = "Windows.Management.Deployment.PackageUserInformation";
}
::windows::imp::interface_hierarchy!(PackageUserInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageUserInformation {}
unsafe impl ::core::marker::Sync for PackageUserInformation {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageVolume(::windows::core::IUnknown);
impl PackageVolume {
    pub fn IsOffline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsOffline)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSystemVolume(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSystemVolume)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MountPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).MountPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PackageStorePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PackageStorePath)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SupportsHardLinks(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).SupportsHardLinks)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisher(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByNamePublisher)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByPackageFamilyName(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByPackageFamilyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesWithPackageTypes)(::windows::core::Interface::as_raw(this), packagetypes, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByNamePublisherWithPackagesTypes(&self, packagetypes: PackageTypes, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByNamePublisherWithPackagesTypes)(::windows::core::Interface::as_raw(this), packagetypes, ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packagetypes: PackageTypes, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByPackageFamilyNameWithPackageTypes)(::windows::core::Interface::as_raw(this), packagetypes, ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackageByPackageFullName(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackageByPackageFullName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefullname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityId(&self, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdNamePublisher(&self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisher)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName(&self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdWithPackageTypes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), packagetypes, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), packagetypes, ::core::mem::transmute_copy(packagename), ::core::mem::transmute_copy(packagepublisher), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), packagetypes, ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindPackageByUserSecurityIdPackageFullName(&self, usersecurityid: &::windows::core::HSTRING, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>();
            (::windows::core::Interface::vtable(this).FindPackageByUserSecurityIdPackageFullName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersecurityid), ::core::mem::transmute_copy(packagefullname), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFullTrustPackageSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsFullTrustPackageSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsAppxInstallSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAppxInstallSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAvailableSpaceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u64>> {
        let this = &::windows::core::ComInterface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<u64>>();
            (::windows::core::Interface::vtable(this).GetAvailableSpaceAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PackageVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageVolume {}
impl ::core::fmt::Debug for PackageVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageVolume").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PackageVolume {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageVolume;{cf2672c3-1a40-4450-9739-2ace2e898853})");
}
impl ::core::clone::Clone for PackageVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PackageVolume {
    type Vtable = IPackageVolume_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PackageVolume {
    const IID: ::windows::core::GUID = <IPackageVolume as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PackageVolume {
    const NAME: &'static str = "Windows.Management.Deployment.PackageVolume";
}
::windows::imp::interface_hierarchy!(PackageVolume, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageVolume {}
unsafe impl ::core::marker::Sync for PackageVolume {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct RegisterPackageOptions(::windows::core::IUnknown);
impl RegisterPackageOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<RegisterPackageOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).DependencyPackageUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDataVolume(&self) -> ::windows::core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PackageVolume>();
            (::windows::core::Interface::vtable(this).AppDataVolume)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppDataVolume(&self, value: &PackageVolume) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAppDataVolume)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).OptionalPackageFamilyNames)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ExternalLocationUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExternalLocationUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExternalLocationUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DeveloperMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DeveloperMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDeveloperMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceAppShutdown)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceAppShutdown)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceTargetAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceTargetAppShutdown)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceTargetAppShutdown)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).InstallAllResources)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInstallAllResources)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).StageInPlace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStageInPlace)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllowUnsigned)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowUnsigned)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DeferRegistrationWhenPackagesAreInUse)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDeferRegistrationWhenPackagesAreInUse)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExpectedDigests(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<super::super::Foundation::Uri, ::windows::core::HSTRING>> {
        let this = &::windows::core::ComInterface::cast::<IRegisterPackageOptions2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMap<super::super::Foundation::Uri, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).ExpectedDigests)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RegisterPackageOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RegisterPackageOptions {}
impl ::core::fmt::Debug for RegisterPackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RegisterPackageOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for RegisterPackageOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.RegisterPackageOptions;{677112a7-50d4-496c-8415-0602b4c6d3bf})");
}
impl ::core::clone::Clone for RegisterPackageOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RegisterPackageOptions {
    type Vtable = IRegisterPackageOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RegisterPackageOptions {
    const IID: ::windows::core::GUID = <IRegisterPackageOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RegisterPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.RegisterPackageOptions";
}
::windows::imp::interface_hierarchy!(RegisterPackageOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RegisterPackageOptions {}
unsafe impl ::core::marker::Sync for RegisterPackageOptions {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct SharedPackageContainer(::windows::core::IUnknown);
impl SharedPackageContainer {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMembers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>();
            (::windows::core::Interface::vtable(this).GetMembers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemovePackageFamily(&self, packagefamilyname: &::windows::core::HSTRING, options: &UpdateSharedPackageContainerOptions) -> ::windows::core::Result<UpdateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UpdateSharedPackageContainerResult>();
            (::windows::core::Interface::vtable(this).RemovePackageFamily)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    pub fn ResetData(&self) -> ::windows::core::Result<UpdateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UpdateSharedPackageContainerResult>();
            (::windows::core::Interface::vtable(this).ResetData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SharedPackageContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SharedPackageContainer {}
impl ::core::fmt::Debug for SharedPackageContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SharedPackageContainer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainer;{177f1aa9-151e-5ef7-b1d9-2fba0b4b0d17})");
}
impl ::core::clone::Clone for SharedPackageContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SharedPackageContainer {
    type Vtable = ISharedPackageContainer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SharedPackageContainer {
    const IID: ::windows::core::GUID = <ISharedPackageContainer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SharedPackageContainer {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainer";
}
::windows::imp::interface_hierarchy!(SharedPackageContainer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SharedPackageContainer {}
unsafe impl ::core::marker::Sync for SharedPackageContainer {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct SharedPackageContainerManager(::windows::core::IUnknown);
impl SharedPackageContainerManager {
    pub fn CreateContainer(&self, name: &::windows::core::HSTRING, options: &CreateSharedPackageContainerOptions) -> ::windows::core::Result<CreateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CreateSharedPackageContainerResult>();
            (::windows::core::Interface::vtable(this).CreateContainer)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    pub fn DeleteContainer(&self, id: &::windows::core::HSTRING, options: &DeleteSharedPackageContainerOptions) -> ::windows::core::Result<DeleteSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DeleteSharedPackageContainerResult>();
            (::windows::core::Interface::vtable(this).DeleteContainer)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    pub fn GetContainer(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainer>();
            (::windows::core::Interface::vtable(this).GetContainer)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContainers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<SharedPackageContainer>>();
            (::windows::core::Interface::vtable(this).FindContainers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContainersWithOptions(&self, options: &FindSharedPackageContainerOptions) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<SharedPackageContainer>>();
            (::windows::core::Interface::vtable(this).FindContainersWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainerManager>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForUser(usersid: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainerManager>();
            (::windows::core::Interface::vtable(this).GetForUser)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(usersid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForProvisioning() -> ::windows::core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainerManager>();
            (::windows::core::Interface::vtable(this).GetForProvisioning)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISharedPackageContainerManagerStatics<R, F: FnOnce(&ISharedPackageContainerManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SharedPackageContainerManager, ISharedPackageContainerManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SharedPackageContainerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SharedPackageContainerManager {}
impl ::core::fmt::Debug for SharedPackageContainerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SharedPackageContainerManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainerManager;{be353068-1ef7-5ac8-ab3f-0b9f612f0274})");
}
impl ::core::clone::Clone for SharedPackageContainerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SharedPackageContainerManager {
    type Vtable = ISharedPackageContainerManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SharedPackageContainerManager {
    const IID: ::windows::core::GUID = <ISharedPackageContainerManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SharedPackageContainerManager {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerManager";
}
::windows::imp::interface_hierarchy!(SharedPackageContainerManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SharedPackageContainerManager {}
unsafe impl ::core::marker::Sync for SharedPackageContainerManager {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct SharedPackageContainerMember(::windows::core::IUnknown);
impl SharedPackageContainerMember {
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstance(packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainerMember> {
        Self::ISharedPackageContainerMemberFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainerMember>();
            (::windows::core::Interface::vtable(this).CreateInstance)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISharedPackageContainerMemberFactory<R, F: FnOnce(&ISharedPackageContainerMemberFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SharedPackageContainerMember, ISharedPackageContainerMemberFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SharedPackageContainerMember {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SharedPackageContainerMember {}
impl ::core::fmt::Debug for SharedPackageContainerMember {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerMember").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SharedPackageContainerMember {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainerMember;{fe0d0438-43c9-5426-b89c-f79bf85ddff4})");
}
impl ::core::clone::Clone for SharedPackageContainerMember {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SharedPackageContainerMember {
    type Vtable = ISharedPackageContainerMember_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SharedPackageContainerMember {
    const IID: ::windows::core::GUID = <ISharedPackageContainerMember as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SharedPackageContainerMember {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerMember";
}
::windows::imp::interface_hierarchy!(SharedPackageContainerMember, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SharedPackageContainerMember {}
unsafe impl ::core::marker::Sync for SharedPackageContainerMember {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct StagePackageOptions(::windows::core::IUnknown);
impl StagePackageOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StagePackageOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).DependencyPackageUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TargetVolume(&self) -> ::windows::core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PackageVolume>();
            (::windows::core::Interface::vtable(this).TargetVolume)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTargetVolume(&self, value: &PackageVolume) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetVolume)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).OptionalPackageFamilyNames)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).OptionalPackageUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RelatedPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).RelatedPackageUris)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ExternalLocationUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExternalLocationUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExternalLocationUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StubPackageOption(&self) -> ::windows::core::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StubPackageOption>();
            (::windows::core::Interface::vtable(this).StubPackageOption)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStubPackageOption)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeveloperMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DeveloperMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDeveloperMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).InstallAllResources)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInstallAllResources)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequiredContentGroupOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).RequiredContentGroupOnly)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequiredContentGroupOnly)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).StageInPlace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStageInPlace)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllowUnsigned)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowUnsigned)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExpectedDigests(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<super::super::Foundation::Uri, ::windows::core::HSTRING>> {
        let this = &::windows::core::ComInterface::cast::<IStagePackageOptions2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMap<super::super::Foundation::Uri, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).ExpectedDigests)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StagePackageOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StagePackageOptions {}
impl ::core::fmt::Debug for StagePackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StagePackageOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StagePackageOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.StagePackageOptions;{0b110c9c-b95d-4c56-bd36-6d656800d06b})");
}
impl ::core::clone::Clone for StagePackageOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StagePackageOptions {
    type Vtable = IStagePackageOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StagePackageOptions {
    const IID: ::windows::core::GUID = <IStagePackageOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StagePackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.StagePackageOptions";
}
::windows::imp::interface_hierarchy!(StagePackageOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StagePackageOptions {}
unsafe impl ::core::marker::Sync for StagePackageOptions {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct UpdateSharedPackageContainerOptions(::windows::core::IUnknown);
impl UpdateSharedPackageContainerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UpdateSharedPackageContainerOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ForceAppShutdown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ForceAppShutdown)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForceAppShutdown)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequirePackagesPresent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).RequirePackagesPresent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequirePackagesPresent(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequirePackagesPresent)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for UpdateSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UpdateSharedPackageContainerOptions {}
impl ::core::fmt::Debug for UpdateSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateSharedPackageContainerOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for UpdateSharedPackageContainerOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.UpdateSharedPackageContainerOptions;{80672e83-7194-59f9-b5b9-daa5375f130a})");
}
impl ::core::clone::Clone for UpdateSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UpdateSharedPackageContainerOptions {
    type Vtable = IUpdateSharedPackageContainerOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UpdateSharedPackageContainerOptions {
    const IID: ::windows::core::GUID = <IUpdateSharedPackageContainerOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UpdateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerOptions";
}
::windows::imp::interface_hierarchy!(UpdateSharedPackageContainerOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UpdateSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for UpdateSharedPackageContainerOptions {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct UpdateSharedPackageContainerResult(::windows::core::IUnknown);
impl UpdateSharedPackageContainerResult {
    pub fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SharedPackageContainerOperationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for UpdateSharedPackageContainerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UpdateSharedPackageContainerResult {}
impl ::core::fmt::Debug for UpdateSharedPackageContainerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateSharedPackageContainerResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for UpdateSharedPackageContainerResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.UpdateSharedPackageContainerResult;{aa407df7-c72d-5458-aea3-4645b6a8ee99})");
}
impl ::core::clone::Clone for UpdateSharedPackageContainerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UpdateSharedPackageContainerResult {
    type Vtable = IUpdateSharedPackageContainerResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UpdateSharedPackageContainerResult {
    const IID: ::windows::core::GUID = <IUpdateSharedPackageContainerResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UpdateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerResult";
}
::windows::imp::interface_hierarchy!(UpdateSharedPackageContainerResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UpdateSharedPackageContainerResult {}
unsafe impl ::core::marker::Sync for UpdateSharedPackageContainerResult {}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AddPackageByAppInstallerOptions(pub u32);
impl AddPackageByAppInstallerOptions {
    pub const None: Self = Self(0u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetAppShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const LimitToExistingPackages: Self = Self(512u32);
}
impl ::core::marker::Copy for AddPackageByAppInstallerOptions {}
impl ::core::clone::Clone for AddPackageByAppInstallerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AddPackageByAppInstallerOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AddPackageByAppInstallerOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AddPackageByAppInstallerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPackageByAppInstallerOptions").field(&self.0).finish()
    }
}
impl AddPackageByAppInstallerOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AddPackageByAppInstallerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AddPackageByAppInstallerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for AddPackageByAppInstallerOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.AddPackageByAppInstallerOptions;u4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeploymentOptions(pub u32);
impl DeploymentOptions {
    pub const None: Self = Self(0u32);
    pub const ForceApplicationShutdown: Self = Self(1u32);
    pub const DevelopmentMode: Self = Self(2u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetApplicationShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const ForceUpdateFromAnyVersion: Self = Self(262144u32);
    pub const RetainFilesOnFailure: Self = Self(2097152u32);
    pub const StageInPlace: Self = Self(4194304u32);
}
impl ::core::marker::Copy for DeploymentOptions {}
impl ::core::clone::Clone for DeploymentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeploymentOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DeploymentOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DeploymentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentOptions").field(&self.0).finish()
    }
}
impl DeploymentOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DeploymentOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DeploymentOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DeploymentOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DeploymentOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DeploymentOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for DeploymentOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentOptions;u4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeploymentProgressState(pub i32);
impl DeploymentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
}
impl ::core::marker::Copy for DeploymentProgressState {}
impl ::core::clone::Clone for DeploymentProgressState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeploymentProgressState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DeploymentProgressState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DeploymentProgressState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentProgressState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeploymentProgressState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentProgressState;i4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageInstallState(pub i32);
impl PackageInstallState {
    pub const NotInstalled: Self = Self(0i32);
    pub const Staged: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
    pub const Paused: Self = Self(6i32);
}
impl ::core::marker::Copy for PackageInstallState {}
impl ::core::clone::Clone for PackageInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageInstallState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PackageInstallState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PackageInstallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageInstallState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PackageInstallState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageInstallState;i4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageState(pub i32);
impl PackageState {
    pub const Normal: Self = Self(0i32);
    pub const LicenseInvalid: Self = Self(1i32);
    pub const Modified: Self = Self(2i32);
    pub const Tampered: Self = Self(3i32);
}
impl ::core::marker::Copy for PackageState {}
impl ::core::clone::Clone for PackageState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PackageState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PackageState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PackageState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageState;i4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageStatus(pub u32);
impl PackageStatus {
    pub const OK: Self = Self(0u32);
    pub const LicenseIssue: Self = Self(1u32);
    pub const Modified: Self = Self(2u32);
    pub const Tampered: Self = Self(4u32);
    pub const Disabled: Self = Self(8u32);
}
impl ::core::marker::Copy for PackageStatus {}
impl ::core::clone::Clone for PackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PackageStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStatus").field(&self.0).finish()
    }
}
impl PackageStatus {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PackageStatus {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PackageStatus {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PackageStatus {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PackageStatus {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PackageStatus {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for PackageStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStatus;u4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageStubPreference(pub i32);
impl PackageStubPreference {
    pub const Full: Self = Self(0i32);
    pub const Stub: Self = Self(1i32);
}
impl ::core::marker::Copy for PackageStubPreference {}
impl ::core::clone::Clone for PackageStubPreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageStubPreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PackageStubPreference {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PackageStubPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStubPreference").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PackageStubPreference {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStubPreference;i4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageTypes(pub u32);
impl PackageTypes {
    pub const None: Self = Self(0u32);
    pub const Main: Self = Self(1u32);
    pub const Framework: Self = Self(2u32);
    pub const Resource: Self = Self(4u32);
    pub const Bundle: Self = Self(8u32);
    pub const Xap: Self = Self(16u32);
    pub const Optional: Self = Self(32u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PackageTypes {}
impl ::core::clone::Clone for PackageTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PackageTypes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PackageTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageTypes").field(&self.0).finish()
    }
}
impl PackageTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PackageTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PackageTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PackageTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PackageTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PackageTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for PackageTypes {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageTypes;u4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemovalOptions(pub u32);
impl RemovalOptions {
    pub const None: Self = Self(0u32);
    pub const PreserveApplicationData: Self = Self(4096u32);
    pub const PreserveRoamableApplicationData: Self = Self(128u32);
    pub const RemoveForAllUsers: Self = Self(524288u32);
}
impl ::core::marker::Copy for RemovalOptions {}
impl ::core::clone::Clone for RemovalOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemovalOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RemovalOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RemovalOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemovalOptions").field(&self.0).finish()
    }
}
impl RemovalOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for RemovalOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RemovalOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RemovalOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RemovalOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RemovalOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for RemovalOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.RemovalOptions;u4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SharedPackageContainerCreationCollisionOptions(pub i32);
impl SharedPackageContainerCreationCollisionOptions {
    pub const FailIfExists: Self = Self(0i32);
    pub const MergeWithExisting: Self = Self(1i32);
    pub const ReplaceExisting: Self = Self(2i32);
}
impl ::core::marker::Copy for SharedPackageContainerCreationCollisionOptions {}
impl ::core::clone::Clone for SharedPackageContainerCreationCollisionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SharedPackageContainerCreationCollisionOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SharedPackageContainerCreationCollisionOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SharedPackageContainerCreationCollisionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerCreationCollisionOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SharedPackageContainerCreationCollisionOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerCreationCollisionOptions;i4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SharedPackageContainerOperationStatus(pub i32);
impl SharedPackageContainerOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const BlockedByPolicy: Self = Self(1i32);
    pub const AlreadyExists: Self = Self(2i32);
    pub const PackageFamilyExistsInAnotherContainer: Self = Self(3i32);
    pub const NotFound: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for SharedPackageContainerOperationStatus {}
impl ::core::clone::Clone for SharedPackageContainerOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SharedPackageContainerOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SharedPackageContainerOperationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SharedPackageContainerOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerOperationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SharedPackageContainerOperationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerOperationStatus;i4)");
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StubPackageOption(pub i32);
impl StubPackageOption {
    pub const Default: Self = Self(0i32);
    pub const InstallFull: Self = Self(1i32);
    pub const InstallStub: Self = Self(2i32);
    pub const UsePreference: Self = Self(3i32);
}
impl ::core::marker::Copy for StubPackageOption {}
impl ::core::clone::Clone for StubPackageOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StubPackageOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StubPackageOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StubPackageOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StubPackageOption").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StubPackageOption {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.StubPackageOption;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Management_Deployment\"`*"]
pub struct DeploymentProgress {
    pub state: DeploymentProgressState,
    pub percentage: u32,
}
impl ::core::marker::Copy for DeploymentProgress {}
impl ::core::clone::Clone for DeploymentProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DeploymentProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DeploymentProgress").field("state", &self.state).field("percentage", &self.percentage).finish()
    }
}
impl ::windows::core::TypeKind for DeploymentProgress {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for DeploymentProgress {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Management.Deployment.DeploymentProgress;enum(Windows.Management.Deployment.DeploymentProgressState;i4);u4)");
}
impl ::core::cmp::PartialEq for DeploymentProgress {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.percentage == other.percentage
    }
}
impl ::core::cmp::Eq for DeploymentProgress {}
impl ::core::default::Default for DeploymentProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
