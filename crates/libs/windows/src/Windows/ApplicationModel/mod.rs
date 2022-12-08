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
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDisplayInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppDisplayInfo {
    type Vtable = IAppDisplayInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppDisplayInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aeb1103_e4d4_41aa_a4f6_c4a276e79eac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDisplayInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetLogo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInfo {
    type Vtable = IAppInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf7f59b3_6a09_4de8_a6c0_5792d56880d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInfo2 {
    type Vtable = IAppInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe4b1f5a_2098_431b_bd25_b30878748d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInfo3 {
    type Vtable = IAppInfo3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInfo3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09a78e46_93a4_46de_9397_0843b57115ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExecutionContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppExecutionContext) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInfo4 {
    type Vtable = IAppInfo4_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInfo4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f34bdeb_1609_4554_9f33_12e1e803e0d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SupportedFileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInfoStatics {
    type Vtable = IAppInfoStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInfoStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf1f782a_e48b_4f0c_9b0b_79c3f8957dd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFromAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetFromAppUserModelIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, appusermodelid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFromAppUserModelIdForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallerInfo {
    type Vtable = IAppInstallerInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallerInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29ab2ac0_d4f6_42a3_adcd_d6583c659508);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallerInfo2 {
    type Vtable = IAppInstallerInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallerInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd20f1388_8256_597c_8511_c84ec50d5e2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
    pub PausedUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PausedUntil: usize,
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
    pub PolicySource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallerPolicySource) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstance(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstance {
    type Vtable = IAppInstance_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstance {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x675f2b47_f25f_4532_9fd6_3633e0634d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstance_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsCurrentInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RedirectActivationTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstanceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstanceStatics {
    type Vtable = IAppInstanceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstanceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d11e77f_9ea6_47af_a6ec_46784c5ba254);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RecommendedInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub GetActivatedEventArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    GetActivatedEventArgs: usize,
    pub FindOrRegisterInstanceForKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInstances: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraApplicationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICameraApplicationManagerStatics {
    type Vtable = ICameraApplicationManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICameraApplicationManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9599ddce_9bd3_435c_8054_c1add50028fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraApplicationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ShowInstalledApplicationsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignModeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDesignModeStatics {
    type Vtable = IDesignModeStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDesignModeStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c3893cc_f81a_4e7a_b857_76a80887e185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignModeStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DesignModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignModeStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDesignModeStatics2 {
    type Vtable = IDesignModeStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDesignModeStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80cf8137_b064_4858_bec8_3eba22357535);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignModeStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DesignMode2Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct IEnteredBackgroundEventArgs(::windows::core::IUnknown);
impl IEnteredBackgroundEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IEnteredBackgroundEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Vtable for IEnteredBackgroundEventArgs {
    type Vtable = IEnteredBackgroundEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnteredBackgroundEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf722dcc2_9827_403d_aaed_ecca9ac17398);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnteredBackgroundEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFindRelatedPackagesOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFindRelatedPackagesOptions {
    type Vtable = IFindRelatedPackagesOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IFindRelatedPackagesOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41dd7eea_b335_521f_b96c_5ea07f5b7329);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindRelatedPackagesOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Relationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageRelationship) -> ::windows::core::HRESULT,
    pub SetRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PackageRelationship) -> ::windows::core::HRESULT,
    pub IncludeFrameworks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeFrameworks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncludeHostRuntimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeHostRuntimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncludeOptionals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeOptionals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncludeResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFindRelatedPackagesOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFindRelatedPackagesOptionsFactory {
    type Vtable = IFindRelatedPackagesOptionsFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IFindRelatedPackagesOptionsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d17254_a4fd_55c4_98cf_f2710b7d8be2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindRelatedPackagesOptionsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationship: PackageRelationship, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullTrustProcessLaunchResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFullTrustProcessLaunchResult {
    type Vtable = IFullTrustProcessLaunchResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFullTrustProcessLaunchResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8917d888_edfb_515f_8e22_5ebceb69dfd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullTrustProcessLaunchResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LaunchResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FullTrustLaunchResult) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFullTrustProcessLauncherStatics {
    type Vtable = IFullTrustProcessLauncherStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IFullTrustProcessLauncherStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd784837f_1100_3c6b_a455_f6262cc331b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullTrustProcessLauncherStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForCurrentAppAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForCurrentAppWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parametergroupid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForCurrentAppWithParametersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForAppAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForAppWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: *mut ::core::ffi::c_void, parametergroupid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForAppWithParametersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFullTrustProcessLauncherStatics2 {
    type Vtable = IFullTrustProcessLauncherStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFullTrustProcessLauncherStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b8ed72f_b65c_56cf_a1a7_2bf77cbc6ea8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullTrustProcessLauncherStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForCurrentAppWithArgumentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandline: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForCurrentAppWithArgumentsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForAppWithArgumentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: *mut ::core::ffi::c_void, commandline: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForAppWithArgumentsAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct ILeavingBackgroundEventArgs(::windows::core::IUnknown);
impl ILeavingBackgroundEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ILeavingBackgroundEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Vtable for ILeavingBackgroundEventArgs {
    type Vtable = ILeavingBackgroundEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ILeavingBackgroundEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39c6ec9a_ae6e_46f9_a07a_cfc23f88733e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILeavingBackgroundEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimitedAccessFeatureRequestResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILimitedAccessFeatureRequestResult {
    type Vtable = ILimitedAccessFeatureRequestResult_Vtbl;
}
unsafe impl ::windows::core::Interface for ILimitedAccessFeatureRequestResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45156a6_1e24_5ddd_abb4_6188aba4d5bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimitedAccessFeatureRequestResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FeatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LimitedAccessFeatureStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EstimatedRemovalDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EstimatedRemovalDate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimitedAccessFeaturesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILimitedAccessFeaturesStatics {
    type Vtable = ILimitedAccessFeaturesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ILimitedAccessFeaturesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be612d4_302b_5fbf_a632_1a99e43e8925);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimitedAccessFeaturesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryUnlockFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, featureid: *mut ::core::ffi::c_void, token: *mut ::core::ffi::c_void, attestation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackage {
    type Vtable = IPackage_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x163c792f_bd75_413c_bf23_b1fe7b95d825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub InstalledLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    InstalledLocation: usize,
    pub IsFramework: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Dependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Dependencies: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackage2 {
    type Vtable = IPackage2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6612fb6_7688_4ace_95fb_359538e7aa01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PublisherDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Logo: usize,
    pub IsResourcePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBundle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDevelopmentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackage3 {
    type Vtable = IPackage3_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackage3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f738b61_f86a_4917_93d1_f1ee9d3b35d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstalledDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstalledDate: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntriesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntriesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackage4 {
    type Vtable = IPackage4_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackage4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65aed1ae_b95b_450c_882b_6255187f397e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SignatureKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageSignatureKind) -> ::windows::core::HRESULT,
    pub IsOptional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VerifyContentIntegrityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VerifyContentIntegrityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackage5 {
    type Vtable = IPackage5_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackage5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e842dd4_d9ac_45ed_9a1e_74ce056b2635);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetContentGroupsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetContentGroupsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContentGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContentGroupAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsWithPriorityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: *mut ::core::ffi::c_void, movetoheadofqueue: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsWithPriorityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetInUseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inuse: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInUseAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage6(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackage6 {
    type Vtable = IPackage6_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackage6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b1ad942_12d7_4754_ae4e_638cbc0e3a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage6_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetAppInstallerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CheckUpdateAvailabilityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckUpdateAvailabilityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage7(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackage7 {
    type Vtable = IPackage7_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackage7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86ff8d31_a2e4_45e0_9732_283a6d88fde1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage7_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub MutableLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MutableLocation: usize,
    #[cfg(feature = "Storage")]
    pub EffectiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveLocation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage8(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackage8 {
    type Vtable = IPackage8_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackage8 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c584f7b_ce2a_4be6_a093_77cfbb2a7ea1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage8_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub EffectiveExternalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub MachineExternalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MachineExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub UserExternalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    UserExternalLocation: usize,
    pub InstalledPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MutablePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EffectivePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EffectiveExternalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MachineExternalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserExternalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetLogoAsRandomAccessStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetLogoAsRandomAccessStreamReference: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntries: usize,
    pub IsStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage9(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackage9 {
    type Vtable = IPackage9_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackage9 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5ab224f_d7e1_49ec_90ce_720cdbd02e9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage9_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindRelatedPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindRelatedPackages: usize,
    pub SourceUriSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageCatalog {
    type Vtable = IPackageCatalog_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x230a3751_9de3_4445_be74_91fb325abefe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PackageStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageStaging: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageStaging: usize,
    #[cfg(feature = "Foundation")]
    pub PackageInstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageInstalling: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageInstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageInstalling: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub PackageStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IPackageCatalog2 {
    type Vtable = IPackageCatalog2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalog2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96a60c36_8ff7_4344_b6bf_ee64c2207ed2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PackageContentGroupStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageContentGroupStaging: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageContentGroupStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageContentGroupStaging: usize,
    #[cfg(feature = "Foundation")]
    pub AddOptionalPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackagefamilyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddOptionalPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageCatalog3 {
    type Vtable = IPackageCatalog3_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalog3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96dd5c88_8837_43f9_9015_033434ba14f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveOptionalPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveOptionalPackagesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageCatalog4 {
    type Vtable = IPackageCatalog4_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalog4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc37c399b_44cc_4b7b_8baf_796c04ead3b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AddResourcePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcepackagefamilyname: *mut ::core::ffi::c_void, resourceid: *mut ::core::ffi::c_void, options: AddResourcePackageOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddResourcePackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveResourcePackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcepackages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveResourcePackagesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogAddOptionalPackageResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageCatalogAddOptionalPackageResult {
    type Vtable = IPackageCatalogAddOptionalPackageResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalogAddOptionalPackageResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bf10cd4_b4df_47b3_a963_e2fa832f7dd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogAddOptionalPackageResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogAddResourcePackageResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageCatalogAddResourcePackageResult {
    type Vtable = IPackageCatalogAddResourcePackageResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalogAddResourcePackageResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9636ce0d_3e17_493f_aa08_ccec6fdef699);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogAddResourcePackageResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogRemoveOptionalPackagesResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageCatalogRemoveOptionalPackagesResult {
    type Vtable = IPackageCatalogRemoveOptionalPackagesResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalogRemoveOptionalPackagesResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29d2f97b_d974_4e64_9359_22cadfd79828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogRemoveOptionalPackagesResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogRemoveResourcePackagesResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageCatalogRemoveResourcePackagesResult {
    type Vtable = IPackageCatalogRemoveResourcePackagesResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalogRemoveResourcePackagesResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae719709_1a52_4321_87b3_e5a1a17981a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogRemoveResourcePackagesResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageCatalogStatics {
    type Vtable = IPackageCatalogStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalogStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa18c9696_e65b_4634_ba21_5e63eb7244a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OpenForCurrentPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenForCurrentUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct IPackageCatalogStatics2(::windows::core::IUnknown);
impl IPackageCatalogStatics2 {
    pub fn OpenForPackage(&self, package: &Package) -> ::windows::core::Result<PackageCatalog> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenForPackage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(package), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IPackageCatalogStatics2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPackageCatalogStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPackageCatalogStatics2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPackageCatalogStatics2 {}
impl ::core::fmt::Debug for IPackageCatalogStatics2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPackageCatalogStatics2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPackageCatalogStatics2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4c11c159-9a28-598c-b185-55e1899b2be4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPackageCatalogStatics2 {
    type Vtable = IPackageCatalogStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageCatalogStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c11c159_9a28_598c_b185_55e1899b2be4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OpenForPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageContentGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageContentGroup {
    type Vtable = IPackageContentGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageContentGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f62695d_120a_4798_b5e1_5800dda8f2e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageContentGroup_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageContentGroupState) -> ::windows::core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageContentGroupStagingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageContentGroupStagingEventArgs {
    type Vtable = IPackageContentGroupStagingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageContentGroupStagingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d7bc27e_6f27_446c_986e_d4733d4d9113);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageContentGroupStagingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ContentGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsContentGroupRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageContentGroupStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageContentGroupStatics {
    type Vtable = IPackageContentGroupStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageContentGroupStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70ee7619_5f12_4b92_b9ea_6ccada13bc75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageContentGroupStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequiredGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageId(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageId {
    type Vtable = IPackageId_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1adb665e_37c7_4790_9980_dd7ae74e8bb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageVersion) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub Architecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::System::ProcessorArchitecture) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Architecture: usize,
    pub ResourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PublisherId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageIdWithMetadata(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageIdWithMetadata {
    type Vtable = IPackageIdWithMetadata_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageIdWithMetadata {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40577a7c_0c9e_443d_9074_855f5ce0a08d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageIdWithMetadata_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageInstallingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageInstallingEventArgs {
    type Vtable = IPackageInstallingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageInstallingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97741eb7_ab7a_401a_8b61_eb0e7faff237);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStagingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageStagingEventArgs {
    type Vtable = IPackageStagingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageStagingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1041682d_54e2_4f51_b828_9ef7046c210f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStagingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageStatics {
    type Vtable = IPackageStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e534bdf_2960_4878_97a4_9624deb72f2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageStatus {
    type Vtable = IPackageStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fe74f71_a365_4c09_a02d_046d525ea1da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatus_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
unsafe impl ::windows::core::Vtable for IPackageStatus2 {
    type Vtable = IPackageStatus2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageStatus2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf428fa93_7c56_4862_acfa_abaedcc0694d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatus2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPartiallyStaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageStatusChangedEventArgs {
    type Vtable = IPackageStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageStatusChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x437d714d_bd80_4a70_bc50_f6e796509575);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatusChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUninstallingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageUninstallingEventArgs {
    type Vtable = IPackageUninstallingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageUninstallingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4443aa52_ab22_44cd_82bb_4ec9b827367a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUninstallingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUpdateAvailabilityResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageUpdateAvailabilityResult {
    type Vtable = IPackageUpdateAvailabilityResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageUpdateAvailabilityResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x114e5009_199a_48a1_a079_313c45634a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUpdateAvailabilityResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Availability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageUpdateAvailability) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUpdatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageUpdatingEventArgs {
    type Vtable = IPackageUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageUpdatingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd7b4228_fd74_443e_b114_23e677b0e86f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUpdatingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SourcePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TargetPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageWithMetadata(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageWithMetadata {
    type Vtable = IPackageWithMetadata_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageWithMetadata {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95949780_1de9_40f2_b452_0de9f1910012);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageWithMetadata_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InstallDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstallDate: usize,
    pub GetThumbnailToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Launch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Launch: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartupTask(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStartupTask {
    type Vtable = IStartupTask_Vtbl;
}
unsafe impl ::windows::core::Interface for IStartupTask {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf75c23c8_b5f2_4f6c_88dd_36cb1d599d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTask_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestEnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestEnableAsync: usize,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StartupTaskState) -> ::windows::core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartupTaskStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStartupTaskStatics {
    type Vtable = IStartupTaskStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IStartupTaskStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5b60bd_a148_41a7_b26e_e8b88a1e62f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetForCurrentPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetForCurrentPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct ISuspendingDeferral(::windows::core::IUnknown);
impl ISuspendingDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(ISuspendingDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Vtable for ISuspendingDeferral {
    type Vtable = ISuspendingDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for ISuspendingDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59140509_8bc9_4eb4_b636_dabdc4f46f66);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuspendingDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct ISuspendingEventArgs(::windows::core::IUnknown);
impl ISuspendingEventArgs {
    pub fn SuspendingOperation(&self) -> ::windows::core::Result<SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuspendingOperation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ISuspendingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Vtable for ISuspendingEventArgs {
    type Vtable = ISuspendingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ISuspendingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96061c05_2dba_4d08_b0bd_2b30a131c6aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuspendingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SuspendingOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct ISuspendingOperation(::windows::core::IUnknown);
impl ISuspendingOperation {
    pub fn GetDeferral(&self) -> ::windows::core::Result<SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Deadline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ISuspendingOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Vtable for ISuspendingOperation {
    type Vtable = ISuspendingOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for ISuspendingOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9da4ca41_20e1_4e9b_9f65_a9f435340c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuspendingOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppDisplayInfo(::windows::core::IUnknown);
impl AppDisplayInfo {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetLogo(&self, size: super::Foundation::Size) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetLogo)(::windows::core::Vtable::as_raw(this), size, result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for AppDisplayInfo {
    type Vtable = IAppDisplayInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for AppDisplayInfo {
    const IID: ::windows::core::GUID = <IAppDisplayInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppDisplayInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppDisplayInfo";
}
::windows::core::interface_hierarchy!(AppDisplayInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppDisplayInfo {}
unsafe impl ::core::marker::Sync for AppDisplayInfo {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppInfo(::windows::core::IUnknown);
impl AppInfo {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppUserModelId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayInfo(&self) -> ::windows::core::Result<AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = &::windows::core::Interface::cast::<IAppInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExecutionContext(&self) -> ::windows::core::Result<AppExecutionContext> {
        let this = &::windows::core::Interface::cast::<IAppInfo3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExecutionContext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SupportedFileExtensions(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IAppInfo4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedFileExtensions)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<::windows::core::HSTRING>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn Current() -> ::windows::core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetFromAppUserModelId(appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFromAppUserModelId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appusermodelid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetFromAppUserModelIdForUser(user: &super::System::User, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFromAppUserModelIdForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(appusermodelid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppInfoStatics<R, F: FnOnce(&IAppInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppInfo, IAppInfoStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for AppInfo {
    type Vtable = IAppInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for AppInfo {
    const IID: ::windows::core::GUID = <IAppInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppInfo";
}
::windows::core::interface_hierarchy!(AppInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppInfo {}
unsafe impl ::core::marker::Sync for AppInfo {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppInstallerInfo(::windows::core::IUnknown);
impl AppInstallerInfo {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OnLaunch(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OnLaunch)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HoursBetweenUpdateChecks(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HoursBetweenUpdateChecks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ShowPrompt(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowPrompt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UpdateBlocksActivation(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateBlocksActivation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AutomaticBackgroundTask(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutomaticBackgroundTask)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForceUpdateFromAnyVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsAutoRepairEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAutoRepairEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Version(&self) -> ::windows::core::Result<PackageVersion> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Version)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastChecked(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastChecked)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PausedUntil(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PausedUntil)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateUris)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RepairUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RepairUris)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DependencyPackageUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DependencyPackageUris)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OptionalPackageUris)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PolicySource(&self) -> ::windows::core::Result<AppInstallerPolicySource> {
        let this = &::windows::core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PolicySource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for AppInstallerInfo {
    type Vtable = IAppInstallerInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for AppInstallerInfo {
    const IID: ::windows::core::GUID = <IAppInstallerInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppInstallerInfo";
}
::windows::core::interface_hierarchy!(AppInstallerInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallerInfo {}
unsafe impl ::core::marker::Sync for AppInstallerInfo {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppInstance(::windows::core::IUnknown);
impl AppInstance {
    pub fn Key(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Key)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCurrentInstance(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCurrentInstance)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RedirectActivationTo(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RedirectActivationTo)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn RecommendedInstance() -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RecommendedInstance)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn GetActivatedEventArgs() -> ::windows::core::Result<Activation::IActivatedEventArgs> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetActivatedEventArgs)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FindOrRegisterInstanceForKey(key: &::windows::core::HSTRING) -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindOrRegisterInstanceForKey)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Unregister() -> ::windows::core::Result<()> {
        Self::IAppInstanceStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).Unregister)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInstances() -> ::windows::core::Result<super::Foundation::Collections::IVector<AppInstance>> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInstances)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppInstanceStatics<R, F: FnOnce(&IAppInstanceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppInstance, IAppInstanceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for AppInstance {
    type Vtable = IAppInstance_Vtbl;
}
unsafe impl ::windows::core::Interface for AppInstance {
    const IID: ::windows::core::GUID = <IAppInstance as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstance {
    const NAME: &'static str = "Windows.ApplicationModel.AppInstance";
}
::windows::core::interface_hierarchy!(AppInstance, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppInstance {}
unsafe impl ::core::marker::Sync for AppInstance {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct CameraApplicationManager;
impl CameraApplicationManager {
    pub fn ShowInstalledApplicationsUI() -> ::windows::core::Result<()> {
        Self::ICameraApplicationManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowInstalledApplicationsUI)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ICameraApplicationManagerStatics<R, F: FnOnce(&ICameraApplicationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CameraApplicationManager, ICameraApplicationManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CameraApplicationManager {
    const NAME: &'static str = "Windows.ApplicationModel.CameraApplicationManager";
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct DesignMode;
impl DesignMode {
    pub fn DesignModeEnabled() -> ::windows::core::Result<bool> {
        Self::IDesignModeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesignModeEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn DesignMode2Enabled() -> ::windows::core::Result<bool> {
        Self::IDesignModeStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesignMode2Enabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDesignModeStatics<R, F: FnOnce(&IDesignModeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DesignMode, IDesignModeStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDesignModeStatics2<R, F: FnOnce(&IDesignModeStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DesignMode, IDesignModeStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for DesignMode {
    const NAME: &'static str = "Windows.ApplicationModel.DesignMode";
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct EnteredBackgroundEventArgs(::windows::core::IUnknown);
impl EnteredBackgroundEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for EnteredBackgroundEventArgs {
    type Vtable = IEnteredBackgroundEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EnteredBackgroundEventArgs {
    const IID: ::windows::core::GUID = <IEnteredBackgroundEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.EnteredBackgroundEventArgs";
}
::windows::core::interface_hierarchy!(EnteredBackgroundEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&EnteredBackgroundEventArgs> for ::windows::core::InParam<IEnteredBackgroundEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &EnteredBackgroundEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for EnteredBackgroundEventArgs {}
unsafe impl ::core::marker::Sync for EnteredBackgroundEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct FindRelatedPackagesOptions(::windows::core::IUnknown);
impl FindRelatedPackagesOptions {
    pub fn Relationship(&self) -> ::windows::core::Result<PackageRelationship> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Relationship)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRelationship(&self, value: PackageRelationship) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRelationship)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IncludeFrameworks(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncludeFrameworks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIncludeFrameworks(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIncludeFrameworks)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IncludeHostRuntimes(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncludeHostRuntimes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIncludeHostRuntimes(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIncludeHostRuntimes)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IncludeOptionals(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncludeOptionals)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIncludeOptionals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIncludeOptionals)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IncludeResources(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncludeResources)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIncludeResources(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIncludeResources)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CreateInstance(relationship: PackageRelationship) -> ::windows::core::Result<FindRelatedPackagesOptions> {
        Self::IFindRelatedPackagesOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), relationship, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFindRelatedPackagesOptionsFactory<R, F: FnOnce(&IFindRelatedPackagesOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FindRelatedPackagesOptions, IFindRelatedPackagesOptionsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for FindRelatedPackagesOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FindRelatedPackagesOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindRelatedPackagesOptions {}
impl ::core::fmt::Debug for FindRelatedPackagesOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindRelatedPackagesOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FindRelatedPackagesOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.FindRelatedPackagesOptions;{41dd7eea-b335-521f-b96c-5ea07f5b7329})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FindRelatedPackagesOptions {
    type Vtable = IFindRelatedPackagesOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for FindRelatedPackagesOptions {
    const IID: ::windows::core::GUID = <IFindRelatedPackagesOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FindRelatedPackagesOptions {
    const NAME: &'static str = "Windows.ApplicationModel.FindRelatedPackagesOptions";
}
::windows::core::interface_hierarchy!(FindRelatedPackagesOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for FindRelatedPackagesOptions {}
unsafe impl ::core::marker::Sync for FindRelatedPackagesOptions {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct FullTrustProcessLaunchResult(::windows::core::IUnknown);
impl FullTrustProcessLaunchResult {
    pub fn LaunchResult(&self) -> ::windows::core::Result<FullTrustLaunchResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchResult)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for FullTrustProcessLaunchResult {
    type Vtable = IFullTrustProcessLaunchResult_Vtbl;
}
unsafe impl ::windows::core::Interface for FullTrustProcessLaunchResult {
    const IID: ::windows::core::GUID = <IFullTrustProcessLaunchResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FullTrustProcessLaunchResult {
    const NAME: &'static str = "Windows.ApplicationModel.FullTrustProcessLaunchResult";
}
::windows::core::interface_hierarchy!(FullTrustProcessLaunchResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for FullTrustProcessLaunchResult {}
unsafe impl ::core::marker::Sync for FullTrustProcessLaunchResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct FullTrustProcessLauncher;
impl FullTrustProcessLauncher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForCurrentAppAsync() -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFullTrustProcessForCurrentAppAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForCurrentAppWithParametersAsync(parametergroupid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFullTrustProcessForCurrentAppWithParametersAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(parametergroupid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForAppAsync(fulltrustpackagerelativeappid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFullTrustProcessForAppAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(fulltrustpackagerelativeappid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForAppWithParametersAsync(fulltrustpackagerelativeappid: &::windows::core::HSTRING, parametergroupid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFullTrustProcessForAppWithParametersAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(fulltrustpackagerelativeappid), ::core::mem::transmute_copy(parametergroupid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForCurrentAppWithArgumentsAsync(commandline: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>> {
        Self::IFullTrustProcessLauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFullTrustProcessForCurrentAppWithArgumentsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(commandline), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullTrustProcessForAppWithArgumentsAsync(fulltrustpackagerelativeappid: &::windows::core::HSTRING, commandline: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>> {
        Self::IFullTrustProcessLauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFullTrustProcessForAppWithArgumentsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(fulltrustpackagerelativeappid), ::core::mem::transmute_copy(commandline), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFullTrustProcessLauncherStatics<R, F: FnOnce(&IFullTrustProcessLauncherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FullTrustProcessLauncher, IFullTrustProcessLauncherStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFullTrustProcessLauncherStatics2<R, F: FnOnce(&IFullTrustProcessLauncherStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FullTrustProcessLauncher, IFullTrustProcessLauncherStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for FullTrustProcessLauncher {
    const NAME: &'static str = "Windows.ApplicationModel.FullTrustProcessLauncher";
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct LeavingBackgroundEventArgs(::windows::core::IUnknown);
impl LeavingBackgroundEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for LeavingBackgroundEventArgs {
    type Vtable = ILeavingBackgroundEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for LeavingBackgroundEventArgs {
    const IID: ::windows::core::GUID = <ILeavingBackgroundEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.LeavingBackgroundEventArgs";
}
::windows::core::interface_hierarchy!(LeavingBackgroundEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&LeavingBackgroundEventArgs> for ::windows::core::InParam<ILeavingBackgroundEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LeavingBackgroundEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for LeavingBackgroundEventArgs {}
unsafe impl ::core::marker::Sync for LeavingBackgroundEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct LimitedAccessFeatureRequestResult(::windows::core::IUnknown);
impl LimitedAccessFeatureRequestResult {
    pub fn FeatureId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FeatureId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<LimitedAccessFeatureStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EstimatedRemovalDate(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EstimatedRemovalDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for LimitedAccessFeatureRequestResult {
    type Vtable = ILimitedAccessFeatureRequestResult_Vtbl;
}
unsafe impl ::windows::core::Interface for LimitedAccessFeatureRequestResult {
    const IID: ::windows::core::GUID = <ILimitedAccessFeatureRequestResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LimitedAccessFeatureRequestResult {
    const NAME: &'static str = "Windows.ApplicationModel.LimitedAccessFeatureRequestResult";
}
::windows::core::interface_hierarchy!(LimitedAccessFeatureRequestResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LimitedAccessFeatureRequestResult {}
unsafe impl ::core::marker::Sync for LimitedAccessFeatureRequestResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct LimitedAccessFeatures;
impl LimitedAccessFeatures {
    pub fn TryUnlockFeature(featureid: &::windows::core::HSTRING, token: &::windows::core::HSTRING, attestation: &::windows::core::HSTRING) -> ::windows::core::Result<LimitedAccessFeatureRequestResult> {
        Self::ILimitedAccessFeaturesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUnlockFeature)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(featureid), ::core::mem::transmute_copy(token), ::core::mem::transmute_copy(attestation), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILimitedAccessFeaturesStatics<R, F: FnOnce(&ILimitedAccessFeaturesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LimitedAccessFeatures, ILimitedAccessFeaturesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for LimitedAccessFeatures {
    const NAME: &'static str = "Windows.ApplicationModel.LimitedAccessFeatures";
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct Package(::windows::core::IUnknown);
impl Package {
    pub fn Id(&self) -> ::windows::core::Result<PackageId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn InstalledLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstalledLocation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsFramework(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsFramework)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Dependencies(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dependencies)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PublisherDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublisherDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Logo(&self) -> ::windows::core::Result<super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsResourcePackage(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsResourcePackage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBundle(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBundle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDevelopmentMode(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDevelopmentMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<PackageStatus> {
        let this = &::windows::core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstalledDate(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstalledDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub fn GetAppListEntriesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<Core::AppListEntry>>> {
        let this = &::windows::core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAppListEntriesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SignatureKind(&self) -> ::windows::core::Result<PackageSignatureKind> {
        let this = &::windows::core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignatureKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsOptional(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOptional)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VerifyContentIntegrityAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VerifyContentIntegrityAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetContentGroupsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>> {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContentGroupsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContentGroupAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageContentGroup>> {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContentGroupAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StageContentGroupsAsync<P0, E0>(&self, names: P0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StageContentGroupsAsync)(::windows::core::Vtable::as_raw(this), names.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StageContentGroupsWithPriorityAsync<P0, E0>(&self, names: P0, movetoheadofqueue: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StageContentGroupsWithPriorityAsync)(::windows::core::Vtable::as_raw(this), names.try_into().map_err(|e| e.into())?.abi(), movetoheadofqueue, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInUseAsync(&self, inuse: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetInUseAsync)(::windows::core::Vtable::as_raw(this), inuse, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetAppInstallerInfo(&self) -> ::windows::core::Result<AppInstallerInfo> {
        let this = &::windows::core::Interface::cast::<IPackage6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAppInstallerInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckUpdateAvailabilityAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageUpdateAvailabilityResult>> {
        let this = &::windows::core::Interface::cast::<IPackage6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckUpdateAvailabilityAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn MutableLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MutableLocation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn EffectiveLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EffectiveLocation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn EffectiveExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EffectiveExternalLocation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn MachineExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MachineExternalLocation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn UserExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserExternalLocation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InstalledPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstalledPath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MutablePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MutablePath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EffectivePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EffectivePath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EffectiveExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EffectiveExternalPath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MachineExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MachineExternalPath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UserExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserExternalPath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetLogoAsRandomAccessStreamReference(&self, size: super::Foundation::Size) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetLogoAsRandomAccessStreamReference)(::windows::core::Vtable::as_raw(this), size, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub fn GetAppListEntries(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Core::AppListEntry>> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAppListEntries)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStub(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStub)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindRelatedPackages(&self, options: &FindRelatedPackagesOptions) -> ::windows::core::Result<super::Foundation::Collections::IVector<Package>> {
        let this = &::windows::core::Interface::cast::<IPackage9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindRelatedPackages)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SourceUriSchemeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackage9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceUriSchemeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<Package> {
        Self::IPackageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstallDate(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstallDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetThumbnailToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetThumbnailToken)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Launch(&self, parameters: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Launch)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(parameters)).ok() }
    }
    #[doc(hidden)]
    pub fn IPackageStatics<R, F: FnOnce(&IPackageStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Package, IPackageStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for Package {
    type Vtable = IPackage_Vtbl;
}
unsafe impl ::windows::core::Interface for Package {
    const IID: ::windows::core::GUID = <IPackage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Package {
    const NAME: &'static str = "Windows.ApplicationModel.Package";
}
::windows::core::interface_hierarchy!(Package, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Package {}
unsafe impl ::core::marker::Sync for Package {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalog(::windows::core::IUnknown);
impl PackageCatalog {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageStaging(&self, handler: &super::Foundation::TypedEventHandler<PackageCatalog, PackageStagingEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageStaging)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageStaging(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageStaging)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageInstalling(&self, handler: &super::Foundation::TypedEventHandler<PackageCatalog, PackageInstallingEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageInstalling)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageInstalling(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageInstalling)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUpdating(&self, handler: &super::Foundation::TypedEventHandler<PackageCatalog, PackageUpdatingEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageUpdating)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUpdating(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageUpdating)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageUninstalling(&self, handler: &super::Foundation::TypedEventHandler<PackageCatalog, PackageUninstallingEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageUninstalling)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUninstalling(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageUninstalling)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageStatusChanged(&self, handler: &super::Foundation::TypedEventHandler<PackageCatalog, PackageStatusChangedEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageStatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageStatusChanged(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PackageContentGroupStaging(&self, handler: &super::Foundation::TypedEventHandler<PackageCatalog, PackageContentGroupStagingEventArgs>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageContentGroupStaging)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageContentGroupStaging(&self, token: super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePackageContentGroupStaging)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddOptionalPackageAsync(&self, optionalpackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogAddOptionalPackageResult>> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddOptionalPackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(optionalpackagefamilyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveOptionalPackagesAsync<P0, E0>(&self, optionalpackagefamilynames: P0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveOptionalPackagesResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IPackageCatalog3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveOptionalPackagesAsync)(::windows::core::Vtable::as_raw(this), optionalpackagefamilynames.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddResourcePackageAsync(&self, resourcepackagefamilyname: &::windows::core::HSTRING, resourceid: &::windows::core::HSTRING, options: AddResourcePackageOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperationWithProgress<PackageCatalogAddResourcePackageResult, PackageInstallProgress>> {
        let this = &::windows::core::Interface::cast::<IPackageCatalog4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddResourcePackageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(resourcepackagefamilyname), ::core::mem::transmute_copy(resourceid), options, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveResourcePackagesAsync<P0, E0>(&self, resourcepackages: P0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveResourcePackagesResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Foundation::Collections::IIterable<Package>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IPackageCatalog4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveResourcePackagesAsync)(::windows::core::Vtable::as_raw(this), resourcepackages.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OpenForCurrentPackage() -> ::windows::core::Result<PackageCatalog> {
        Self::IPackageCatalogStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenForCurrentPackage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn OpenForCurrentUser() -> ::windows::core::Result<PackageCatalog> {
        Self::IPackageCatalogStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenForCurrentUser)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn OpenForPackage(package: &Package) -> ::windows::core::Result<PackageCatalog> {
        Self::IPackageCatalogStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenForPackage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(package), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageCatalogStatics<R, F: FnOnce(&IPackageCatalogStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PackageCatalog, IPackageCatalogStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPackageCatalogStatics2<R, F: FnOnce(&IPackageCatalogStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PackageCatalog, IPackageCatalogStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for PackageCatalog {
    type Vtable = IPackageCatalog_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageCatalog {
    const IID: ::windows::core::GUID = <IPackageCatalog as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalog";
}
::windows::core::interface_hierarchy!(PackageCatalog, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalogAddOptionalPackageResult(::windows::core::IUnknown);
impl PackageCatalogAddOptionalPackageResult {
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageCatalogAddOptionalPackageResult {
    type Vtable = IPackageCatalogAddOptionalPackageResult_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageCatalogAddOptionalPackageResult {
    const IID: ::windows::core::GUID = <IPackageCatalogAddOptionalPackageResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalogAddOptionalPackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult";
}
::windows::core::interface_hierarchy!(PackageCatalogAddOptionalPackageResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalogAddResourcePackageResult(::windows::core::IUnknown);
impl PackageCatalogAddResourcePackageResult {
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsComplete)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageCatalogAddResourcePackageResult {
    type Vtable = IPackageCatalogAddResourcePackageResult_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageCatalogAddResourcePackageResult {
    const IID: ::windows::core::GUID = <IPackageCatalogAddResourcePackageResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalogAddResourcePackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogAddResourcePackageResult";
}
::windows::core::interface_hierarchy!(PackageCatalogAddResourcePackageResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageCatalogAddResourcePackageResult {}
unsafe impl ::core::marker::Sync for PackageCatalogAddResourcePackageResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalogRemoveOptionalPackagesResult(::windows::core::IUnknown);
impl PackageCatalogRemoveOptionalPackagesResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PackagesRemoved(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackagesRemoved)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageCatalogRemoveOptionalPackagesResult {
    type Vtable = IPackageCatalogRemoveOptionalPackagesResult_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageCatalogRemoveOptionalPackagesResult {
    const IID: ::windows::core::GUID = <IPackageCatalogRemoveOptionalPackagesResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalogRemoveOptionalPackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult";
}
::windows::core::interface_hierarchy!(PackageCatalogRemoveOptionalPackagesResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageCatalogRemoveResourcePackagesResult(::windows::core::IUnknown);
impl PackageCatalogRemoveResourcePackagesResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PackagesRemoved(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackagesRemoved)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageCatalogRemoveResourcePackagesResult {
    type Vtable = IPackageCatalogRemoveResourcePackagesResult_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageCatalogRemoveResourcePackagesResult {
    const IID: ::windows::core::GUID = <IPackageCatalogRemoveResourcePackagesResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageCatalogRemoveResourcePackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult";
}
::windows::core::interface_hierarchy!(PackageCatalogRemoveResourcePackagesResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageCatalogRemoveResourcePackagesResult {}
unsafe impl ::core::marker::Sync for PackageCatalogRemoveResourcePackagesResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageContentGroup(::windows::core::IUnknown);
impl PackageContentGroup {
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<PackageContentGroupState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRequired)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RequiredGroupName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPackageContentGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequiredGroupName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageContentGroupStatics<R, F: FnOnce(&IPackageContentGroupStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PackageContentGroup, IPackageContentGroupStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for PackageContentGroup {
    type Vtable = IPackageContentGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageContentGroup {
    const IID: ::windows::core::GUID = <IPackageContentGroup as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageContentGroup {
    const NAME: &'static str = "Windows.ApplicationModel.PackageContentGroup";
}
::windows::core::interface_hierarchy!(PackageContentGroup, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageContentGroup {}
unsafe impl ::core::marker::Sync for PackageContentGroup {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageContentGroupStagingEventArgs(::windows::core::IUnknown);
impl PackageContentGroupStagingEventArgs {
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivityId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Progress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsComplete)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContentGroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentGroupName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsContentGroupRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsContentGroupRequired)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageContentGroupStagingEventArgs {
    type Vtable = IPackageContentGroupStagingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageContentGroupStagingEventArgs {
    const IID: ::windows::core::GUID = <IPackageContentGroupStagingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageContentGroupStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageContentGroupStagingEventArgs";
}
::windows::core::interface_hierarchy!(PackageContentGroupStagingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageContentGroupStagingEventArgs {}
unsafe impl ::core::marker::Sync for PackageContentGroupStagingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageId(::windows::core::IUnknown);
impl PackageId {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Version(&self) -> ::windows::core::Result<PackageVersion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Version)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn Architecture(&self) -> ::windows::core::Result<super::System::ProcessorArchitecture> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Architecture)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ResourceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResourceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Publisher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PublisherId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublisherId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FullName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackageIdWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPackageIdWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Author)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageId {
    type Vtable = IPackageId_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageId {
    const IID: ::windows::core::GUID = <IPackageId as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageId {
    const NAME: &'static str = "Windows.ApplicationModel.PackageId";
}
::windows::core::interface_hierarchy!(PackageId, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageId {}
unsafe impl ::core::marker::Sync for PackageId {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageInstallingEventArgs(::windows::core::IUnknown);
impl PackageInstallingEventArgs {
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivityId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Progress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsComplete)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageInstallingEventArgs {
    type Vtable = IPackageInstallingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageInstallingEventArgs {
    const IID: ::windows::core::GUID = <IPackageInstallingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageInstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageInstallingEventArgs";
}
::windows::core::interface_hierarchy!(PackageInstallingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageInstallingEventArgs {}
unsafe impl ::core::marker::Sync for PackageInstallingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageStagingEventArgs(::windows::core::IUnknown);
impl PackageStagingEventArgs {
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivityId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Progress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsComplete)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageStagingEventArgs {
    type Vtable = IPackageStagingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageStagingEventArgs {
    const IID: ::windows::core::GUID = <IPackageStagingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStagingEventArgs";
}
::windows::core::interface_hierarchy!(PackageStagingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageStagingEventArgs {}
unsafe impl ::core::marker::Sync for PackageStagingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageStatus(::windows::core::IUnknown);
impl PackageStatus {
    pub fn VerifyIsOK(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VerifyIsOK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NotAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotAvailable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PackageOffline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageOffline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DataOffline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataOffline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Disabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Disabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NeedsRemediation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NeedsRemediation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LicenseIssue(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseIssue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Modified(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Modified)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Tampered(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tampered)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DependencyIssue(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DependencyIssue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Servicing(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Servicing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeploymentInProgress(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeploymentInProgress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPartiallyStaged(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPackageStatus2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPartiallyStaged)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageStatus {
    type Vtable = IPackageStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageStatus {
    const IID: ::windows::core::GUID = <IPackageStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageStatus {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStatus";
}
::windows::core::interface_hierarchy!(PackageStatus, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageStatus {}
unsafe impl ::core::marker::Sync for PackageStatus {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageStatusChangedEventArgs(::windows::core::IUnknown);
impl PackageStatusChangedEventArgs {
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageStatusChangedEventArgs {
    type Vtable = IPackageStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageStatusChangedEventArgs {
    const IID: ::windows::core::GUID = <IPackageStatusChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStatusChangedEventArgs";
}
::windows::core::interface_hierarchy!(PackageStatusChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for PackageStatusChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageUninstallingEventArgs(::windows::core::IUnknown);
impl PackageUninstallingEventArgs {
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivityId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Package(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Package)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Progress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsComplete)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageUninstallingEventArgs {
    type Vtable = IPackageUninstallingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageUninstallingEventArgs {
    const IID: ::windows::core::GUID = <IPackageUninstallingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUninstallingEventArgs";
}
::windows::core::interface_hierarchy!(PackageUninstallingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageUninstallingEventArgs {}
unsafe impl ::core::marker::Sync for PackageUninstallingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageUpdateAvailabilityResult(::windows::core::IUnknown);
impl PackageUpdateAvailabilityResult {
    pub fn Availability(&self) -> ::windows::core::Result<PackageUpdateAvailability> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Availability)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageUpdateAvailabilityResult {
    type Vtable = IPackageUpdateAvailabilityResult_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageUpdateAvailabilityResult {
    const IID: ::windows::core::GUID = <IPackageUpdateAvailabilityResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageUpdateAvailabilityResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUpdateAvailabilityResult";
}
::windows::core::interface_hierarchy!(PackageUpdateAvailabilityResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageUpdateAvailabilityResult {}
unsafe impl ::core::marker::Sync for PackageUpdateAvailabilityResult {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageUpdatingEventArgs(::windows::core::IUnknown);
impl PackageUpdatingEventArgs {
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivityId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SourcePackage(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourcePackage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TargetPackage(&self) -> ::windows::core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetPackage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Progress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsComplete)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PackageUpdatingEventArgs {
    type Vtable = IPackageUpdatingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageUpdatingEventArgs {
    const IID: ::windows::core::GUID = <IPackageUpdatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUpdatingEventArgs";
}
::windows::core::interface_hierarchy!(PackageUpdatingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PackageUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for PackageUpdatingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct StartupTask(::windows::core::IUnknown);
impl StartupTask {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestEnableAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTaskState>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestEnableAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Disable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Disable)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<StartupTaskState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetForCurrentPackageAsync() -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StartupTask>>> {
        Self::IStartupTaskStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentPackageAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAsync(taskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTask>> {
        Self::IStartupTaskStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(taskid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStartupTaskStatics<R, F: FnOnce(&IStartupTaskStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StartupTask, IStartupTaskStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for StartupTask {
    type Vtable = IStartupTask_Vtbl;
}
unsafe impl ::windows::core::Interface for StartupTask {
    const IID: ::windows::core::GUID = <IStartupTask as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StartupTask {
    const NAME: &'static str = "Windows.ApplicationModel.StartupTask";
}
::windows::core::interface_hierarchy!(StartupTask, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StartupTask {}
unsafe impl ::core::marker::Sync for StartupTask {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct SuspendingDeferral(::windows::core::IUnknown);
impl SuspendingDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
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
unsafe impl ::windows::core::Vtable for SuspendingDeferral {
    type Vtable = ISuspendingDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for SuspendingDeferral {
    const IID: ::windows::core::GUID = <ISuspendingDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SuspendingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingDeferral";
}
::windows::core::interface_hierarchy!(SuspendingDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&SuspendingDeferral> for ::windows::core::InParam<ISuspendingDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingDeferral) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SuspendingDeferral {}
unsafe impl ::core::marker::Sync for SuspendingDeferral {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct SuspendingEventArgs(::windows::core::IUnknown);
impl SuspendingEventArgs {
    pub fn SuspendingOperation(&self) -> ::windows::core::Result<SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuspendingOperation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for SuspendingEventArgs {
    type Vtable = ISuspendingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for SuspendingEventArgs {
    const IID: ::windows::core::GUID = <ISuspendingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SuspendingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingEventArgs";
}
::windows::core::interface_hierarchy!(SuspendingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&SuspendingEventArgs> for ::windows::core::InParam<ISuspendingEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SuspendingEventArgs {}
unsafe impl ::core::marker::Sync for SuspendingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct SuspendingOperation(::windows::core::IUnknown);
impl SuspendingOperation {
    pub fn GetDeferral(&self) -> ::windows::core::Result<SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Deadline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for SuspendingOperation {
    type Vtable = ISuspendingOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for SuspendingOperation {
    const IID: ::windows::core::GUID = <ISuspendingOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SuspendingOperation {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingOperation";
}
::windows::core::interface_hierarchy!(SuspendingOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&SuspendingOperation> for ::windows::core::InParam<ISuspendingOperation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingOperation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SuspendingOperation {}
unsafe impl ::core::marker::Sync for SuspendingOperation {}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageRelationship(pub i32);
impl PackageRelationship {
    pub const Dependencies: Self = Self(0i32);
    pub const Dependents: Self = Self(1i32);
    pub const All: Self = Self(2i32);
}
impl ::core::marker::Copy for PackageRelationship {}
impl ::core::clone::Clone for PackageRelationship {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageRelationship {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PackageRelationship {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageRelationship {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageRelationship").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageRelationship {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageRelationship;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
        self.PercentComplete == other.PercentComplete
    }
}
impl ::core::cmp::Eq for PackageInstallProgress {}
impl ::core::default::Default for PackageInstallProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
        self.Major == other.Major && self.Minor == other.Minor && self.Build == other.Build && self.Revision == other.Revision
    }
}
impl ::core::cmp::Eq for PackageVersion {}
impl ::core::default::Default for PackageVersion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
