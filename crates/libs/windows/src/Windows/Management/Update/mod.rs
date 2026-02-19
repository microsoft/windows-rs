windows_core::imp::define_interface!(IPreviewBuildsManager, IPreviewBuildsManager_Vtbl, 0xfa07dd61_7e4f_59f7_7c9f_def9051c5f62);
impl windows_core::RuntimeType for IPreviewBuildsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ArePreviewBuildsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetArePreviewBuildsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPreviewBuildsManagerStatics, IPreviewBuildsManagerStatics_Vtbl, 0x3e422887_b112_5a70_7da1_97d78d32aa29);
impl windows_core::RuntimeType for IPreviewBuildsManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPreviewBuildsState, IPreviewBuildsState_Vtbl, 0xa2f2903e_b223_5f63_7546_3e8eac070a2e);
impl windows_core::RuntimeType for IPreviewBuildsState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdate, IWindowsSoftwareUpdate_Vtbl, 0xd8f19211_98fe_58dd_af0f_470532aa3341);
impl windows_core::RuntimeType for IWindowsSoftwareUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstallationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsSoftwareUpdateInstallationType) -> windows_core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DownloadSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub InstallSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SourceVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProductCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Approve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApproveCurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionResultInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApprovalInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApprovedActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttentionRequiredInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestartReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppPackageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecutionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptionalInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateActionInfo, IWindowsSoftwareUpdateActionInfo_Vtbl, 0x2f6723b5_f704_5362_b600_d18808f3973e);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateActionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateActionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsSoftwareUpdateActionType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateActionInfoFactory, IWindowsSoftwareUpdateActionInfoFactory_Vtbl, 0x5e83b58e_d982_5d93_a7cb_bf6c9b6ee5a6);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateActionInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateActionInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, WindowsSoftwareUpdateActionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateActionProgress, IWindowsSoftwareUpdateActionProgress_Vtbl, 0x17dc15fd_75f2_522b_b555_359da8de5581);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateActionProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateActionProgress_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub TotalProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateActionResultInfo, IWindowsSoftwareUpdateActionResultInfo_Vtbl, 0xbcf26fba_98c8_51cb_8b7e_1eedc3d82a69);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateActionResultInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateActionResultInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateAppPackageInfo, IWindowsSoftwareUpdateAppPackageInfo_Vtbl, 0xa5bd59f4_e624_5552_82f9_267a4574a203);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateAppPackageInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateAppPackageInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PackageArchitecture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsSoftwareUpdateArchitecture) -> windows_core::HRESULT,
    pub InstallUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateAppPackageInfoFactory, IWindowsSoftwareUpdateAppPackageInfoFactory_Vtbl, 0xa8bda639_a4f6_5a4a_8a54_26c1c508cd0f);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateAppPackageInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateAppPackageInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WindowsSoftwareUpdateArchitecture, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateApprovalInfo, IWindowsSoftwareUpdateApprovalInfo_Vtbl, 0x691e6b9e_80af_5882_9404_25437ecb791b);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateApprovalInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateApprovalInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserInitiated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AppClosure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MeteredNetwork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Seeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateApprovalInfoFactory, IWindowsSoftwareUpdateApprovalInfoFactory_Vtbl, 0xab291c7c_d29f_5ac5_b447_0bfcabdc2cc3);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateApprovalInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateApprovalInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool, bool, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateExecutionInfo, IWindowsSoftwareUpdateExecutionInfo_Vtbl, 0x091aea19_9128_5f24_afc1_a62252df55c0);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateExecutionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateExecutionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DownloadInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeployInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptionalActionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateExecutionInfoFactory, IWindowsSoftwareUpdateExecutionInfoFactory_Vtbl, 0x88596f7e_b9ef_5583_8135_94d62ed66ed4);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateExecutionInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateExecutionInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateFactory, IWindowsSoftwareUpdateFactory_Vtbl, 0x28e7e01b_4225_52c8_bb51_c68f0b071be5);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WindowsSoftwareUpdateInstallationType, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u64, u64, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WindowsSoftwareUpdateInstallationType, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u64, u64, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateLocalizationInfo, IWindowsSoftwareUpdateLocalizationInfo_Vtbl, 0xadc2de4b_5966_5f9f_ae07_00d4a285d933);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateLocalizationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateLocalizationInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LanguageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateLocalizationInfoFactory, IWindowsSoftwareUpdateLocalizationInfoFactory_Vtbl, 0x76979b24_f5bd_5c8c_bdb7_a46187374aff);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateLocalizationInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateLocalizationInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateOptionalActionInfo, IWindowsSoftwareUpdateOptionalActionInfo_Vtbl, 0x4ac035d0_e50e_5ccb_bfd8_a303562891d2);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateOptionalActionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateOptionalActionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CloseAndDeployInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseAndInstallInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseAndRestartInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateOptionalActionInfoFactory, IWindowsSoftwareUpdateOptionalActionInfoFactory_Vtbl, 0x88d2fcc1_4791_51b6_b988_966ef93a180b);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateOptionalActionInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateOptionalActionInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateOptionalInfo, IWindowsSoftwareUpdateOptionalInfo_Vtbl, 0x78084a73_50c4_5c33_a751_7a121f5aae70);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateOptionalInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateOptionalInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalizationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateOptionalInfoFactory, IWindowsSoftwareUpdateOptionalInfoFactory_Vtbl, 0xd837deed_a5f2_5c89_8beb_852d2897b2ef);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateOptionalInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateOptionalInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateProvider, IWindowsSoftwareUpdateProvider_Vtbl, 0x20b67f4a_e28e_5d20_9c00_bf249922efbe);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FolderPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CatalogFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScanFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScanFileArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsSoftwareUpdateProviderType) -> windows_core::HRESULT,
    pub PayloadFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrustState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsSoftwareUpdateProviderTrustState) -> windows_core::HRESULT,
    pub RegistrationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsSoftwareUpdateProviderRegistrationType) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub GetPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateProviderActionResult, IWindowsSoftwareUpdateProviderActionResult_Vtbl, 0xafd92b50_6bb9_54de_bdda_9dfb6cc17c16);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateProviderActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateProviderActionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsSoftwareUpdateActionResult) -> windows_core::HRESULT,
    pub RestartReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsSoftwareUpdateRestartReason) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateProviderActionResultFactory, IWindowsSoftwareUpdateProviderActionResultFactory_Vtbl, 0x0c002684_30c9_59e9_b53f_8846a85d2dc6);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateProviderActionResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateProviderActionResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, WindowsSoftwareUpdateActionResult, WindowsSoftwareUpdateRestartReason, u32, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateProviderFactory, IWindowsSoftwareUpdateProviderFactory_Vtbl, 0xfc0d5fc4_e15e_5116_b2ed_db0a64997ffa);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateProviderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateProviderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateProviderPayloadFileInfo, IWindowsSoftwareUpdateProviderPayloadFileInfo_Vtbl, 0xf1da16da_1b01_5367_b4ae_20db8cae1e9b);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateProviderPayloadFileInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateProviderPayloadFileInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Filename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CatalogFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrustState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsSoftwareUpdateProviderTrustState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateProviderStatus, IWindowsSoftwareUpdateProviderStatus_Vtbl, 0x076741b8_7a8e_53b6_9fb7_e290b13c52e9);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateProviderStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateProviderStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CancelRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCancelRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetScanResult: unsafe extern "system" fn(*mut core::ffi::c_void, bool, u32, u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActionProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActionResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateProviderStatusFactory, IWindowsSoftwareUpdateProviderStatusFactory_Vtbl, 0xd1e1b416_7dfd_55ef_9e3c_18d1459e3123);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateProviderStatusFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateProviderStatusFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateResult, IWindowsSoftwareUpdateResult_Vtbl, 0xa6d7ed98_6212_5ad3_aa9d_15e83bb85ee4);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CancelRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateResultFactory, IWindowsSoftwareUpdateResultFactory_Vtbl, 0x512ce0bf_9977_5301_9b29_9e5042c8cf7d);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, bool, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(*mut core::ffi::c_void, bool, u32, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance3: unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool, u32, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateScanResult, IWindowsSoftwareUpdateScanResult_Vtbl, 0x43ca6d96_3e6d_56da_a903_65d4ab729299);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateScanResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateScanResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateScanResultFactory, IWindowsSoftwareUpdateScanResultFactory_Vtbl, 0x21148e4c_e7ce_574e_bfa7_69dc77457d21);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateScanResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateScanResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, bool, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(*mut core::ffi::c_void, bool, u32, u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateVersion, IWindowsSoftwareUpdateVersion_Vtbl, 0x215e22e7_6d57_5305_9c79_4ecd4a4d7871);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateVersion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateVersion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Major: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Minor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RevisionMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RevisionMinor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsSoftwareUpdateVersionFactory, IWindowsSoftwareUpdateVersionFactory_Vtbl, 0x650ed994_0858_5528_a1f2_f73ca64dabc9);
impl windows_core::RuntimeType for IWindowsSoftwareUpdateVersionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsSoftwareUpdateVersionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdate, IWindowsUpdate_Vtbl, 0xc3c88dd7_0ef3_52b2_a9ad_66bfc6bd9582);
impl windows_core::RuntimeType for IWindowsUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFeatureUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMinorImpact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCritical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsForOS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDriver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMandatory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsUrgent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSeeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEulaAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EulaText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttentionRequiredInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptEula: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateActionCompletedEventArgs, IWindowsUpdateActionCompletedEventArgs_Vtbl, 0x2c44b950_a655_5321_aec1_aee762922131);
impl windows_core::RuntimeType for IWindowsUpdateActionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateActionProgress, IWindowsUpdateActionProgress_Vtbl, 0x83b22d8a_4bb0_549f_ba39_59724882d137);
impl windows_core::RuntimeType for IWindowsUpdateActionProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionProgress_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateActionResult, IWindowsUpdateActionResult_Vtbl, 0xe6692c62_f697_51b7_ab7f_e73e5e688f12);
impl windows_core::RuntimeType for IWindowsUpdateActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateAdministrator, IWindowsUpdateAdministrator_Vtbl, 0x7a60181c_ba1e_5cf9_aa65_304120b73d72);
impl windows_core::RuntimeType for IWindowsUpdateAdministrator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAdministrator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartAdministratorScan: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApproveWindowsUpdateAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeWindowsUpdateActionApproval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApproveWindowsUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeWindowsUpdateApproval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateAdministratorStatics, IWindowsUpdateAdministratorStatics_Vtbl, 0x013e6d36_ef69_53bc_8db8_c403bca550ed);
impl windows_core::RuntimeType for IWindowsUpdateAdministratorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAdministratorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetRegisteredAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterForAdministration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WindowsUpdateAdministratorOptions, *mut WindowsUpdateAdministratorStatus) -> windows_core::HRESULT,
    pub UnregisterForAdministration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut WindowsUpdateAdministratorStatus) -> windows_core::HRESULT,
    pub GetRegisteredAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestRestart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelRestartRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateApprovalData, IWindowsUpdateApprovalData_Vtbl, 0xaadf5bfd_84db_59bc_85e2_ad4fc1f62f7c);
impl windows_core::RuntimeType for IWindowsUpdateApprovalData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateApprovalData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Seeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSeeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowDownloadOnMetered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAllowDownloadOnMetered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateAttentionRequiredInfo, IWindowsUpdateAttentionRequiredInfo_Vtbl, 0x44df2579_74d3_5ffa_b6ce_09e187e1e0ed);
impl windows_core::RuntimeType for IWindowsUpdateAttentionRequiredInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAttentionRequiredInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsUpdateAttentionRequiredReason) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateAttentionRequiredReasonChangedEventArgs, IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl, 0x0627abca_dbb8_524a_b1d2_d9df004eeb31);
impl windows_core::RuntimeType for IWindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsUpdateAttentionRequiredReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateGetAdministratorResult, IWindowsUpdateGetAdministratorResult_Vtbl, 0xbb39ffc4_2c42_5b1c_8995_343341c92c50);
impl windows_core::RuntimeType for IWindowsUpdateGetAdministratorResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateGetAdministratorResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Administrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsUpdateAdministratorStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateItem, IWindowsUpdateItem_Vtbl, 0xb222e44a_49b6_59bf_a033_ef617cd73a98);
impl windows_core::RuntimeType for IWindowsUpdateItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Category: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateManager, IWindowsUpdateManager_Vtbl, 0x5dd966c0_a71a_5602_bbd0_09a70e4573fa);
impl windows_core::RuntimeType for IWindowsUpdateManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScanningStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveScanningStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub WorkingStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveWorkingStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub AttentionRequiredReasonChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveAttentionRequiredReasonChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ActionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveActionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ScanCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveScanCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub IsScanning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsWorking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub LastSuccessfulScanTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetApplicableUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMostRecentCompletedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMostRecentCompletedUpdatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartScan: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateManager2, IWindowsUpdateManager2_Vtbl, 0x564e7683_bd21_57a4_b17f_7bf6350f4c75);
impl windows_core::RuntimeType for IWindowsUpdateManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetApplicableSoftwareUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PerformScan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateManagerFactory, IWindowsUpdateManagerFactory_Vtbl, 0x1b394df8_decb_5f44_b47c_6ccf3bcfdb37);
impl windows_core::RuntimeType for IWindowsUpdateManagerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManagerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateManagerFactory2, IWindowsUpdateManagerFactory2_Vtbl, 0xba08d663_d160_59b9_9898_97a186ad52ea);
impl windows_core::RuntimeType for IWindowsUpdateManagerFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManagerFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::HSTRING, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateManagerScanOptions, IWindowsUpdateManagerScanOptions_Vtbl, 0xb7c30113_5e4b_59d8_99ad_f58d67b2aefc);
impl windows_core::RuntimeType for IWindowsUpdateManagerScanOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManagerScanOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsUserInitiated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsUserInitiated: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowBypassThrottling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowBypassThrottling: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PerformUpdateActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPerformUpdateActions: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateManagerScanOptionsFactory, IWindowsUpdateManagerScanOptionsFactory_Vtbl, 0x1a0f9198_f18d_5cfd_8cb9_08f3fb74da70);
impl windows_core::RuntimeType for IWindowsUpdateManagerScanOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManagerScanOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateProgressChangedEventArgs, IWindowsUpdateProgressChangedEventArgs_Vtbl, 0xbbfbdeeb_94c8_5aa7_b0fb_66c67c233b0a);
impl windows_core::RuntimeType for IWindowsUpdateProgressChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateProgressChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateRestartRequestOptions, IWindowsUpdateRestartRequestOptions_Vtbl, 0x38cfb7d3_4188_5222_905c_6c4443c951ee);
impl windows_core::RuntimeType for IWindowsUpdateRestartRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateRestartRequestOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub OrganizationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOrganizationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateRestartRequestOptionsFactory, IWindowsUpdateRestartRequestOptionsFactory_Vtbl, 0x75f41d04_0e17_50d0_8c15_6b9d0539b3a9);
impl windows_core::RuntimeType for IWindowsUpdateRestartRequestOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateRestartRequestOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateScanCompletedEventArgs, IWindowsUpdateScanCompletedEventArgs_Vtbl, 0x95b6953e_ba5c_5fe8_b115_12de184a6bb0);
impl windows_core::RuntimeType for IWindowsUpdateScanCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateScanCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewBuildsManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PreviewBuildsManager, windows_core::IUnknown, windows_core::IInspectable);
impl PreviewBuildsManager {
    pub fn ArePreviewBuildsAllowed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArePreviewBuildsAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetArePreviewBuildsAllowed(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetArePreviewBuildsAllowed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrentState(&self) -> windows_core::Result<PreviewBuildsState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentState)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SyncAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<PreviewBuildsManager> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IPreviewBuildsManagerStatics<R, F: FnOnce(&IPreviewBuildsManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PreviewBuildsManager, IPreviewBuildsManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PreviewBuildsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPreviewBuildsManager>();
}
unsafe impl windows_core::Interface for PreviewBuildsManager {
    type Vtable = <IPreviewBuildsManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPreviewBuildsManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsManager";
}
unsafe impl Send for PreviewBuildsManager {}
unsafe impl Sync for PreviewBuildsManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewBuildsState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PreviewBuildsState, windows_core::IUnknown, windows_core::IInspectable);
impl PreviewBuildsState {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PreviewBuildsState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPreviewBuildsState>();
}
unsafe impl windows_core::Interface for PreviewBuildsState {
    type Vtable = <IPreviewBuildsState as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPreviewBuildsState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsState";
}
unsafe impl Send for PreviewBuildsState {}
unsafe impl Sync for PreviewBuildsState {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdate, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdate {
    pub fn InstallationType(&self) -> windows_core::Result<WindowsSoftwareUpdateInstallationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DownloadSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InstallSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SourceVersion(&self) -> windows_core::Result<WindowsSoftwareUpdateVersion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceVersion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TargetVersion(&self) -> windows_core::Result<WindowsSoftwareUpdateVersion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetVersion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProductCode(&self) -> windows_core::Result<super::super::Foundation::IReference<windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Approve<P0>(&self, approvalinfo: P0) -> windows_core::Result<WindowsSoftwareUpdateResult>
    where
        P0: windows_core::Param<WindowsSoftwareUpdateApprovalInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Approve)(windows_core::Interface::as_raw(this), approvalinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApproveCurrentAction(&self, approve: bool) -> windows_core::Result<WindowsSoftwareUpdateResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApproveCurrentAction)(windows_core::Interface::as_raw(this), approve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentAction(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentAction)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ActionResultInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateActionResultInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionResultInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApprovalInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateApprovalInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApprovalInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApprovedActions(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsSoftwareUpdateActionType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApprovedActions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AttentionRequiredInfo(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttentionRequiredInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActionProgress(&self) -> windows_core::Result<WindowsSoftwareUpdateActionProgress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionProgress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RestartReason(&self) -> windows_core::Result<super::super::Foundation::IReference<WindowsSoftwareUpdateRestartReason>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RestartReason)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppPackageInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateAppPackageInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppPackageInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExecutionInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateExecutionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecutionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OptionalInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateOptionalInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P5, P8, P9, P10, P11, P12>(providerid: &windows_core::HSTRING, installationtype: WindowsSoftwareUpdateInstallationType, updateid: &windows_core::HSTRING, title: &windows_core::HSTRING, description: &windows_core::HSTRING, moreinfourl: P5, downloadsizeinbytes: u64, installsizeinbytes: u64, sourceversion: P8, targetversion: P9, apppackageinfo: P10, executioninfo: P11, optionalinfo: P12) -> windows_core::Result<WindowsSoftwareUpdate>
    where
        P5: windows_core::Param<super::super::Foundation::Uri>,
        P8: windows_core::Param<WindowsSoftwareUpdateVersion>,
        P9: windows_core::Param<WindowsSoftwareUpdateVersion>,
        P10: windows_core::Param<WindowsSoftwareUpdateAppPackageInfo>,
        P11: windows_core::Param<WindowsSoftwareUpdateExecutionInfo>,
        P12: windows_core::Param<WindowsSoftwareUpdateOptionalInfo>,
    {
        Self::IWindowsSoftwareUpdateFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(providerid), installationtype, core::mem::transmute_copy(updateid), core::mem::transmute_copy(title), core::mem::transmute_copy(description), moreinfourl.param().abi(), downloadsizeinbytes, installsizeinbytes, sourceversion.param().abi(), targetversion.param().abi(), apppackageinfo.param().abi(), executioninfo.param().abi(), optionalinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2<P5, P8, P10, P11, P12, P13, P14>(providerid: &windows_core::HSTRING, installationtype: WindowsSoftwareUpdateInstallationType, updateid: &windows_core::HSTRING, title: &windows_core::HSTRING, description: &windows_core::HSTRING, moreinfourl: P5, downloadsizeinbytes: u64, installsizeinbytes: u64, productcode: P8, packagefamilyname: &windows_core::HSTRING, sourceversion: P10, targetversion: P11, apppackageinfo: P12, executioninfo: P13, optionalinfo: P14) -> windows_core::Result<WindowsSoftwareUpdate>
    where
        P5: windows_core::Param<super::super::Foundation::Uri>,
        P8: windows_core::Param<super::super::Foundation::IReference<windows_core::GUID>>,
        P10: windows_core::Param<WindowsSoftwareUpdateVersion>,
        P11: windows_core::Param<WindowsSoftwareUpdateVersion>,
        P12: windows_core::Param<WindowsSoftwareUpdateAppPackageInfo>,
        P13: windows_core::Param<WindowsSoftwareUpdateExecutionInfo>,
        P14: windows_core::Param<WindowsSoftwareUpdateOptionalInfo>,
    {
        Self::IWindowsSoftwareUpdateFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance2)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(providerid),
                installationtype,
                core::mem::transmute_copy(updateid),
                core::mem::transmute_copy(title),
                core::mem::transmute_copy(description),
                moreinfourl.param().abi(),
                downloadsizeinbytes,
                installsizeinbytes,
                productcode.param().abi(),
                core::mem::transmute_copy(packagefamilyname),
                sourceversion.param().abi(),
                targetversion.param().abi(),
                apppackageinfo.param().abi(),
                executioninfo.param().abi(),
                optionalinfo.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateFactory<R, F: FnOnce(&IWindowsSoftwareUpdateFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdate, IWindowsSoftwareUpdateFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdate>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdate {
    type Vtable = <IWindowsSoftwareUpdate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdate {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdate";
}
unsafe impl Send for WindowsSoftwareUpdate {}
unsafe impl Sync for WindowsSoftwareUpdate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateActionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateActionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateActionInfo {
    pub fn FileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FileArguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileArguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ActionType(&self) -> windows_core::Result<WindowsSoftwareUpdateActionType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(filename: &windows_core::HSTRING, filearguments: &windows_core::HSTRING, actiontype: WindowsSoftwareUpdateActionType) -> windows_core::Result<WindowsSoftwareUpdateActionInfo> {
        Self::IWindowsSoftwareUpdateActionInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filename), core::mem::transmute_copy(filearguments), actiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateActionInfoFactory<R, F: FnOnce(&IWindowsSoftwareUpdateActionInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateActionInfo, IWindowsSoftwareUpdateActionInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateActionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateActionInfo>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateActionInfo {
    type Vtable = <IWindowsSoftwareUpdateActionInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateActionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateActionInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateActionInfo";
}
unsafe impl Send for WindowsSoftwareUpdateActionInfo {}
unsafe impl Sync for WindowsSoftwareUpdateActionInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateActionProgress(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateActionProgress, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateActionProgress {
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CurrentProgress(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TotalProgress(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TotalProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateActionProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateActionProgress>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateActionProgress {
    type Vtable = <IWindowsSoftwareUpdateActionProgress as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateActionProgress as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateActionProgress {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateActionProgress";
}
unsafe impl Send for WindowsSoftwareUpdateActionProgress {}
unsafe impl Sync for WindowsSoftwareUpdateActionProgress {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateActionResult(pub i32);
impl WindowsSoftwareUpdateActionResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Continue: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Canceled: Self = Self(3i32);
    pub const Removed: Self = Self(4i32);
}
impl windows_core::TypeKind for WindowsSoftwareUpdateActionResult {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsSoftwareUpdateActionResult;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateActionResultInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateActionResultInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateActionResultInfo {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResultCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateActionResultInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateActionResultInfo>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateActionResultInfo {
    type Vtable = <IWindowsSoftwareUpdateActionResultInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateActionResultInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateActionResultInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateActionResultInfo";
}
unsafe impl Send for WindowsSoftwareUpdateActionResultInfo {}
unsafe impl Sync for WindowsSoftwareUpdateActionResultInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateActionType(pub i32);
impl WindowsSoftwareUpdateActionType {
    pub const Download: Self = Self(0i32);
    pub const Install: Self = Self(1i32);
    pub const Deploy: Self = Self(2i32);
    pub const Reboot: Self = Self(3i32);
    pub const AppRestart: Self = Self(4i32);
}
impl windows_core::TypeKind for WindowsSoftwareUpdateActionType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateActionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsSoftwareUpdateActionType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateAppPackageInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateAppPackageInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateAppPackageInfo {
    pub fn PackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn PackageArchitecture(&self) -> windows_core::Result<WindowsSoftwareUpdateArchitecture> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageArchitecture)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InstallUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P2>(packagefamilyname: &windows_core::HSTRING, packagearchitecture: WindowsSoftwareUpdateArchitecture, installuri: P2) -> windows_core::Result<WindowsSoftwareUpdateAppPackageInfo>
    where
        P2: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IWindowsSoftwareUpdateAppPackageInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), packagearchitecture, installuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateAppPackageInfoFactory<R, F: FnOnce(&IWindowsSoftwareUpdateAppPackageInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateAppPackageInfo, IWindowsSoftwareUpdateAppPackageInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateAppPackageInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateAppPackageInfo>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateAppPackageInfo {
    type Vtable = <IWindowsSoftwareUpdateAppPackageInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateAppPackageInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateAppPackageInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateAppPackageInfo";
}
unsafe impl Send for WindowsSoftwareUpdateAppPackageInfo {}
unsafe impl Sync for WindowsSoftwareUpdateAppPackageInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateApprovalInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateApprovalInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateApprovalInfo {
    pub fn UserInitiated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserInitiated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppClosure(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppClosure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MeteredNetwork(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeteredNetwork)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seeker(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Seeker)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(userinitiated: bool, appclosure: bool, meterednetwork: bool, seeker: bool) -> windows_core::Result<WindowsSoftwareUpdateApprovalInfo> {
        Self::IWindowsSoftwareUpdateApprovalInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), userinitiated, appclosure, meterednetwork, seeker, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateApprovalInfoFactory<R, F: FnOnce(&IWindowsSoftwareUpdateApprovalInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateApprovalInfo, IWindowsSoftwareUpdateApprovalInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateApprovalInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateApprovalInfo>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateApprovalInfo {
    type Vtable = <IWindowsSoftwareUpdateApprovalInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateApprovalInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateApprovalInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateApprovalInfo";
}
unsafe impl Send for WindowsSoftwareUpdateApprovalInfo {}
unsafe impl Sync for WindowsSoftwareUpdateApprovalInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateArchitecture(pub i32);
impl WindowsSoftwareUpdateArchitecture {
    pub const Neutral: Self = Self(0i32);
    pub const X86: Self = Self(1i32);
    pub const X64: Self = Self(2i32);
    pub const Arm: Self = Self(3i32);
    pub const Arm64: Self = Self(4i32);
}
impl windows_core::TypeKind for WindowsSoftwareUpdateArchitecture {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateArchitecture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsSoftwareUpdateArchitecture;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateExecutionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateExecutionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateExecutionInfo {
    pub fn DownloadInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateActionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InstallInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateActionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeployInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateActionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeployInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OptionalActionInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateOptionalActionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalActionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0, P1, P2>(downloadinfo: P0, installinfo: P1, actions: P2) -> windows_core::Result<WindowsSoftwareUpdateExecutionInfo>
    where
        P0: windows_core::Param<WindowsSoftwareUpdateActionInfo>,
        P1: windows_core::Param<WindowsSoftwareUpdateActionInfo>,
        P2: windows_core::Param<WindowsSoftwareUpdateOptionalActionInfo>,
    {
        Self::IWindowsSoftwareUpdateExecutionInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), downloadinfo.param().abi(), installinfo.param().abi(), actions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2<P0, P1>(deployinfo: P0, actions: P1) -> windows_core::Result<WindowsSoftwareUpdateExecutionInfo>
    where
        P0: windows_core::Param<WindowsSoftwareUpdateActionInfo>,
        P1: windows_core::Param<WindowsSoftwareUpdateOptionalActionInfo>,
    {
        Self::IWindowsSoftwareUpdateExecutionInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance2)(windows_core::Interface::as_raw(this), deployinfo.param().abi(), actions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateExecutionInfoFactory<R, F: FnOnce(&IWindowsSoftwareUpdateExecutionInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateExecutionInfo, IWindowsSoftwareUpdateExecutionInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateExecutionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateExecutionInfo>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateExecutionInfo {
    type Vtable = <IWindowsSoftwareUpdateExecutionInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateExecutionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateExecutionInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateExecutionInfo";
}
unsafe impl Send for WindowsSoftwareUpdateExecutionInfo {}
unsafe impl Sync for WindowsSoftwareUpdateExecutionInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateInstallationType(pub i32);
impl WindowsSoftwareUpdateInstallationType {
    pub const WindowsUpdate: Self = Self(0i32);
    pub const AppPackage: Self = Self(1i32);
    pub const Executable: Self = Self(2i32);
    pub const Powershell: Self = Self(3i32);
}
impl windows_core::TypeKind for WindowsSoftwareUpdateInstallationType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateInstallationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsSoftwareUpdateInstallationType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateLocalizationInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateLocalizationInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateLocalizationInfo {
    pub fn LanguageId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LanguageId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P3>(languageid: u32, title: &windows_core::HSTRING, description: &windows_core::HSTRING, moreinfourl: P3) -> windows_core::Result<WindowsSoftwareUpdateLocalizationInfo>
    where
        P3: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IWindowsSoftwareUpdateLocalizationInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), languageid, core::mem::transmute_copy(title), core::mem::transmute_copy(description), moreinfourl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateLocalizationInfoFactory<R, F: FnOnce(&IWindowsSoftwareUpdateLocalizationInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateLocalizationInfo, IWindowsSoftwareUpdateLocalizationInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateLocalizationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateLocalizationInfo>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateLocalizationInfo {
    type Vtable = <IWindowsSoftwareUpdateLocalizationInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateLocalizationInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateLocalizationInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo";
}
unsafe impl Send for WindowsSoftwareUpdateLocalizationInfo {}
unsafe impl Sync for WindowsSoftwareUpdateLocalizationInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateOptionalActionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateOptionalActionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateOptionalActionInfo {
    pub fn CloseAndDeployInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateActionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseAndDeployInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloseAndInstallInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateActionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseAndInstallInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloseAndRestartInfo(&self) -> windows_core::Result<WindowsSoftwareUpdateActionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseAndRestartInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0, P1, P2>(closeanddeployinfo: P0, closeandinstallinfo: P1, closeandrestartinfo: P2) -> windows_core::Result<WindowsSoftwareUpdateOptionalActionInfo>
    where
        P0: windows_core::Param<WindowsSoftwareUpdateActionInfo>,
        P1: windows_core::Param<WindowsSoftwareUpdateActionInfo>,
        P2: windows_core::Param<WindowsSoftwareUpdateActionInfo>,
    {
        Self::IWindowsSoftwareUpdateOptionalActionInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), closeanddeployinfo.param().abi(), closeandinstallinfo.param().abi(), closeandrestartinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateOptionalActionInfoFactory<R, F: FnOnce(&IWindowsSoftwareUpdateOptionalActionInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateOptionalActionInfo, IWindowsSoftwareUpdateOptionalActionInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateOptionalActionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateOptionalActionInfo>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateOptionalActionInfo {
    type Vtable = <IWindowsSoftwareUpdateOptionalActionInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateOptionalActionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateOptionalActionInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateOptionalActionInfo";
}
unsafe impl Send for WindowsSoftwareUpdateOptionalActionInfo {}
unsafe impl Sync for WindowsSoftwareUpdateOptionalActionInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateOptionalInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateOptionalInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateOptionalInfo {
    pub fn LocalizationInfo(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsSoftwareUpdateLocalizationInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalizationInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ComplianceDeadlineInDays(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceDeadlineInDays)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ComplianceGracePeriodInDays(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceGracePeriodInDays)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0, P1>(compliancedeadlineindays: P0, compliancegraceperiodindays: P1) -> windows_core::Result<WindowsSoftwareUpdateOptionalInfo>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
        P1: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        Self::IWindowsSoftwareUpdateOptionalInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), compliancedeadlineindays.param().abi(), compliancegraceperiodindays.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2<P0, P1, P2>(localizationinfo: P0, compliancedeadlineindays: P1, compliancegraceperiodindays: P2) -> windows_core::Result<WindowsSoftwareUpdateOptionalInfo>
    where
        P0: windows_core::Param<windows_collections::IIterable<WindowsSoftwareUpdateLocalizationInfo>>,
        P1: windows_core::Param<super::super::Foundation::IReference<i32>>,
        P2: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        Self::IWindowsSoftwareUpdateOptionalInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance2)(windows_core::Interface::as_raw(this), localizationinfo.param().abi(), compliancedeadlineindays.param().abi(), compliancegraceperiodindays.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateOptionalInfoFactory<R, F: FnOnce(&IWindowsSoftwareUpdateOptionalInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateOptionalInfo, IWindowsSoftwareUpdateOptionalInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateOptionalInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateOptionalInfo>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateOptionalInfo {
    type Vtable = <IWindowsSoftwareUpdateOptionalInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateOptionalInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateOptionalInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateOptionalInfo";
}
unsafe impl Send for WindowsSoftwareUpdateOptionalInfo {}
unsafe impl Sync for WindowsSoftwareUpdateOptionalInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateProvider, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateProvider {
    pub fn Register(&self) -> windows_core::Result<WindowsSoftwareUpdateResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Register)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Unregister(&self) -> windows_core::Result<WindowsSoftwareUpdateResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Validate(&self) -> windows_core::Result<WindowsSoftwareUpdateResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Validate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Version(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FolderPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FolderPath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CatalogFile(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CatalogFile)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ScanFileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanFileName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ScanFileArguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanFileArguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<WindowsSoftwareUpdateProviderType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PayloadFiles(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsSoftwareUpdateProviderPayloadFileInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PayloadFiles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrustState(&self) -> windows_core::Result<WindowsSoftwareUpdateProviderTrustState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrustState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RegistrationType(&self) -> windows_core::Result<WindowsSoftwareUpdateProviderRegistrationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegistrationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::PropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPropertyValue(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertyValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(folderpath: &windows_core::HSTRING) -> windows_core::Result<WindowsSoftwareUpdateProvider> {
        Self::IWindowsSoftwareUpdateProviderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(folderpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateProviderFactory<R, F: FnOnce(&IWindowsSoftwareUpdateProviderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateProvider, IWindowsSoftwareUpdateProviderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateProvider>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateProvider {
    type Vtable = <IWindowsSoftwareUpdateProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateProvider {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateProvider";
}
unsafe impl Send for WindowsSoftwareUpdateProvider {}
unsafe impl Sync for WindowsSoftwareUpdateProvider {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateProviderActionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateProviderActionResult, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateProviderActionResult {
    pub fn Result(&self) -> windows_core::Result<WindowsSoftwareUpdateActionResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RestartReason(&self) -> windows_core::Result<WindowsSoftwareUpdateRestartReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RestartReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResultCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(actionresult: WindowsSoftwareUpdateActionResult, restartreason: WindowsSoftwareUpdateRestartReason, resultcode: u32, extendederror: u64) -> windows_core::Result<WindowsSoftwareUpdateProviderActionResult> {
        Self::IWindowsSoftwareUpdateProviderActionResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), actionresult, restartreason, resultcode, extendederror, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateProviderActionResultFactory<R, F: FnOnce(&IWindowsSoftwareUpdateProviderActionResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateProviderActionResult, IWindowsSoftwareUpdateProviderActionResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateProviderActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateProviderActionResult>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateProviderActionResult {
    type Vtable = <IWindowsSoftwareUpdateProviderActionResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateProviderActionResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateProviderActionResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateProviderActionResult";
}
unsafe impl Send for WindowsSoftwareUpdateProviderActionResult {}
unsafe impl Sync for WindowsSoftwareUpdateProviderActionResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateProviderPayloadFileInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateProviderPayloadFileInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateProviderPayloadFileInfo {
    pub fn Filename(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Filename)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FileHash(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileHash)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CatalogFile(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CatalogFile)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TrustState(&self) -> windows_core::Result<WindowsSoftwareUpdateProviderTrustState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrustState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateProviderPayloadFileInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateProviderPayloadFileInfo>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateProviderPayloadFileInfo {
    type Vtable = <IWindowsSoftwareUpdateProviderPayloadFileInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateProviderPayloadFileInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateProviderPayloadFileInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo";
}
unsafe impl Send for WindowsSoftwareUpdateProviderPayloadFileInfo {}
unsafe impl Sync for WindowsSoftwareUpdateProviderPayloadFileInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateProviderRegistrationType(pub i32);
impl WindowsSoftwareUpdateProviderRegistrationType {
    pub const None: Self = Self(0i32);
    pub const System: Self = Self(1i32);
    pub const Windows: Self = Self(2i32);
    pub const Pending: Self = Self(3i32);
    pub const Registered: Self = Self(4i32);
    pub const Unregistered: Self = Self(5i32);
}
impl windows_core::TypeKind for WindowsSoftwareUpdateProviderRegistrationType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateProviderRegistrationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsSoftwareUpdateProviderRegistrationType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateProviderStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateProviderStatus, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateProviderStatus {
    pub fn CancelRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsSoftwareUpdateProviderStatus, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCancelRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCancelRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetScanResult<P3>(&self, succeeded: bool, resultcode: u32, extendederror: u64, updates: P3) -> windows_core::Result<WindowsSoftwareUpdateResult>
    where
        P3: windows_core::Param<windows_collections::IIterable<WindowsSoftwareUpdate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetScanResult)(windows_core::Interface::as_raw(this), succeeded, resultcode, extendederror, updates.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetActionProgress(&self, current: u64, total: u64) -> windows_core::Result<WindowsSoftwareUpdateResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetActionProgress)(windows_core::Interface::as_raw(this), current, total, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetActionResult<P0>(&self, actionresult: P0) -> windows_core::Result<WindowsSoftwareUpdateResult>
    where
        P0: windows_core::Param<WindowsSoftwareUpdateProviderActionResult>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetActionResult)(windows_core::Interface::as_raw(this), actionresult.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(providerid: &windows_core::HSTRING) -> windows_core::Result<WindowsSoftwareUpdateProviderStatus> {
        Self::IWindowsSoftwareUpdateProviderStatusFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(providerid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateProviderStatusFactory<R, F: FnOnce(&IWindowsSoftwareUpdateProviderStatusFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateProviderStatus, IWindowsSoftwareUpdateProviderStatusFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateProviderStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateProviderStatus>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateProviderStatus {
    type Vtable = <IWindowsSoftwareUpdateProviderStatus as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateProviderStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateProviderStatus {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateProviderStatus";
}
unsafe impl Send for WindowsSoftwareUpdateProviderStatus {}
unsafe impl Sync for WindowsSoftwareUpdateProviderStatus {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateProviderTrustState(pub i32);
impl WindowsSoftwareUpdateProviderTrustState {
    pub const SignedTrusted: Self = Self(0i32);
    pub const SignedUntrusted: Self = Self(1i32);
    pub const Unsigned: Self = Self(2i32);
}
impl windows_core::TypeKind for WindowsSoftwareUpdateProviderTrustState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateProviderTrustState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsSoftwareUpdateProviderTrustState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateProviderType(pub i32);
impl WindowsSoftwareUpdateProviderType {
    pub const WindowsUpdate: Self = Self(0i32);
    pub const Executable: Self = Self(1i32);
    pub const Powershell: Self = Self(2i32);
}
impl windows_core::TypeKind for WindowsSoftwareUpdateProviderType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateProviderType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsSoftwareUpdateProviderType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateRestartReason(pub i32);
impl WindowsSoftwareUpdateRestartReason {
    pub const None: Self = Self(0i32);
    pub const System: Self = Self(1i32);
    pub const AppClose: Self = Self(2i32);
    pub const AppRestart: Self = Self(3i32);
}
impl windows_core::TypeKind for WindowsSoftwareUpdateRestartReason {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateRestartReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsSoftwareUpdateRestartReason;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateResult, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateResult {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CancelRequested(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelRequested)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResultCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(succeeded: bool, resultcode: u32) -> windows_core::Result<WindowsSoftwareUpdateResult> {
        Self::IWindowsSoftwareUpdateResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), succeeded, resultcode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2(succeeded: bool, resultcode: u32, extendederror: u64) -> windows_core::Result<WindowsSoftwareUpdateResult> {
        Self::IWindowsSoftwareUpdateResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance2)(windows_core::Interface::as_raw(this), succeeded, resultcode, extendederror, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance3(succeeded: bool, cancelrequested: bool, resultcode: u32, extendederror: u64) -> windows_core::Result<WindowsSoftwareUpdateResult> {
        Self::IWindowsSoftwareUpdateResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance3)(windows_core::Interface::as_raw(this), succeeded, cancelrequested, resultcode, extendederror, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateResultFactory<R, F: FnOnce(&IWindowsSoftwareUpdateResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateResult, IWindowsSoftwareUpdateResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateResult>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateResult {
    type Vtable = <IWindowsSoftwareUpdateResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateResult";
}
unsafe impl Send for WindowsSoftwareUpdateResult {}
unsafe impl Sync for WindowsSoftwareUpdateResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateScanResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateScanResult, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateScanResult {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResultCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Updates(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsSoftwareUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Updates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P2>(succeeded: bool, resultcode: u32, updates: P2) -> windows_core::Result<WindowsSoftwareUpdateScanResult>
    where
        P2: windows_core::Param<windows_collections::IIterable<WindowsSoftwareUpdate>>,
    {
        Self::IWindowsSoftwareUpdateScanResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), succeeded, resultcode, updates.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2<P3>(succeeded: bool, resultcode: u32, extendederror: u64, updates: P3) -> windows_core::Result<WindowsSoftwareUpdateScanResult>
    where
        P3: windows_core::Param<windows_collections::IIterable<WindowsSoftwareUpdate>>,
    {
        Self::IWindowsSoftwareUpdateScanResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance2)(windows_core::Interface::as_raw(this), succeeded, resultcode, extendederror, updates.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateScanResultFactory<R, F: FnOnce(&IWindowsSoftwareUpdateScanResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateScanResult, IWindowsSoftwareUpdateScanResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateScanResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateScanResult>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateScanResult {
    type Vtable = <IWindowsSoftwareUpdateScanResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateScanResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateScanResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateScanResult";
}
unsafe impl Send for WindowsSoftwareUpdateScanResult {}
unsafe impl Sync for WindowsSoftwareUpdateScanResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsSoftwareUpdateVersion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsSoftwareUpdateVersion, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsSoftwareUpdateVersion {
    pub fn Major(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Major)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Minor(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Minor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RevisionMajor(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RevisionMajor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RevisionMinor(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RevisionMinor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(major: u32, minor: u32, revisionmajor: u32, revisionminor: u32) -> windows_core::Result<WindowsSoftwareUpdateVersion> {
        Self::IWindowsSoftwareUpdateVersionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), major, minor, revisionmajor, revisionminor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsSoftwareUpdateVersionFactory<R, F: FnOnce(&IWindowsSoftwareUpdateVersionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsSoftwareUpdateVersion, IWindowsSoftwareUpdateVersionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsSoftwareUpdateVersion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsSoftwareUpdateVersion>();
}
unsafe impl windows_core::Interface for WindowsSoftwareUpdateVersion {
    type Vtable = <IWindowsSoftwareUpdateVersion as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsSoftwareUpdateVersion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsSoftwareUpdateVersion {
    const NAME: &'static str = "Windows.Management.Update.WindowsSoftwareUpdateVersion";
}
unsafe impl Send for WindowsSoftwareUpdateVersion {}
unsafe impl Sync for WindowsSoftwareUpdateVersion {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdate, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdate {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsFeatureUpdate(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFeatureUpdate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMinorImpact(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMinorImpact)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSecurity(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSecurity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCritical(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCritical)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsForOS(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsForOS)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDriver(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDriver)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMandatory(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMandatory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUrgent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUrgent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSeeker(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSeeker)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SupportUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEulaAccepted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEulaAccepted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EulaText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EulaText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AttentionRequiredInfo(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttentionRequiredInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActionResult(&self) -> windows_core::Result<WindowsUpdateActionResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionResult)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentAction(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentAction)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ActionProgress(&self) -> windows_core::Result<WindowsUpdateActionProgress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionProgress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPropertyValue(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertyValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AcceptEula(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AcceptEula)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for WindowsUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdate>();
}
unsafe impl windows_core::Interface for WindowsUpdate {
    type Vtable = <IWindowsUpdate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdate {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdate";
}
unsafe impl Send for WindowsUpdate {}
unsafe impl Sync for WindowsUpdate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateActionCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateActionCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateActionCompletedEventArgs {
    pub fn Update(&self) -> windows_core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateActionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateActionCompletedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateActionCompletedEventArgs {
    type Vtable = <IWindowsUpdateActionCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateActionCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateActionCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionCompletedEventArgs";
}
unsafe impl Send for WindowsUpdateActionCompletedEventArgs {}
unsafe impl Sync for WindowsUpdateActionCompletedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateActionProgress(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateActionProgress, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateActionProgress {
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateActionProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateActionProgress>();
}
unsafe impl windows_core::Interface for WindowsUpdateActionProgress {
    type Vtable = <IWindowsUpdateActionProgress as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateActionProgress as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateActionProgress {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionProgress";
}
unsafe impl Send for WindowsUpdateActionProgress {}
unsafe impl Sync for WindowsUpdateActionProgress {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateActionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateActionResult, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateActionResult {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateActionResult>();
}
unsafe impl windows_core::Interface for WindowsUpdateActionResult {
    type Vtable = <IWindowsUpdateActionResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateActionResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateActionResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionResult";
}
unsafe impl Send for WindowsUpdateActionResult {}
unsafe impl Sync for WindowsUpdateActionResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateAdministrator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateAdministrator, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateAdministrator {
    pub fn StartAdministratorScan(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartAdministratorScan)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApproveWindowsUpdateAction(&self, updateid: &windows_core::HSTRING, action: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ApproveWindowsUpdateAction)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), core::mem::transmute_copy(action)).ok() }
    }
    pub fn RevokeWindowsUpdateActionApproval(&self, updateid: &windows_core::HSTRING, action: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RevokeWindowsUpdateActionApproval)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), core::mem::transmute_copy(action)).ok() }
    }
    pub fn ApproveWindowsUpdate<P1>(&self, updateid: &windows_core::HSTRING, approvaldata: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<WindowsUpdateApprovalData>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ApproveWindowsUpdate)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), approvaldata.param().abi()).ok() }
    }
    pub fn RevokeWindowsUpdateApproval(&self, updateid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RevokeWindowsUpdateApproval)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid)).ok() }
    }
    pub fn GetUpdates(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUpdates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRegisteredAdministrator(organizationname: &windows_core::HSTRING) -> windows_core::Result<WindowsUpdateGetAdministratorResult> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRegisteredAdministrator)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(organizationname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RegisterForAdministration(organizationname: &windows_core::HSTRING, options: WindowsUpdateAdministratorOptions) -> windows_core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterForAdministration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(organizationname), options, &mut result__).map(|| result__)
        })
    }
    pub fn UnregisterForAdministration(organizationname: &windows_core::HSTRING) -> windows_core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnregisterForAdministration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(organizationname), &mut result__).map(|| result__)
        })
    }
    pub fn GetRegisteredAdministratorName() -> windows_core::Result<windows_core::HSTRING> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRegisteredAdministratorName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn RequestRestart<P0>(restartoptions: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<WindowsUpdateRestartRequestOptions>,
    {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestRestart)(windows_core::Interface::as_raw(this), restartoptions.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn CancelRestartRequest(requestrestarttoken: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe { (windows_core::Interface::vtable(this).CancelRestartRequest)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(requestrestarttoken)).ok() })
    }
    fn IWindowsUpdateAdministratorStatics<R, F: FnOnce(&IWindowsUpdateAdministratorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateAdministrator, IWindowsUpdateAdministratorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateAdministrator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateAdministrator>();
}
unsafe impl windows_core::Interface for WindowsUpdateAdministrator {
    type Vtable = <IWindowsUpdateAdministrator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateAdministrator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateAdministrator {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAdministrator";
}
unsafe impl Send for WindowsUpdateAdministrator {}
unsafe impl Sync for WindowsUpdateAdministrator {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsUpdateAdministratorOptions(pub u32);
impl WindowsUpdateAdministratorOptions {
    pub const None: Self = Self(0u32);
    pub const RequireAdministratorApprovalForScans: Self = Self(1u32);
    pub const RequireAdministratorApprovalForUpdates: Self = Self(2u32);
    pub const RequireAdministratorApprovalForActions: Self = Self(4u32);
}
impl windows_core::TypeKind for WindowsUpdateAdministratorOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsUpdateAdministratorOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorOptions;u4)");
}
impl WindowsUpdateAdministratorOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WindowsUpdateAdministratorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WindowsUpdateAdministratorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsUpdateAdministratorStatus(pub i32);
impl WindowsUpdateAdministratorStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoAdministratorRegistered: Self = Self(1i32);
    pub const OtherAdministratorIsRegistered: Self = Self(2i32);
}
impl windows_core::TypeKind for WindowsUpdateAdministratorStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsUpdateAdministratorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateApprovalData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateApprovalData, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateApprovalData {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateApprovalData, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Seeker(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Seeker)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSeeker<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSeeker)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowDownloadOnMetered(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDownloadOnMetered)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAllowDownloadOnMetered<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDownloadOnMetered)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ComplianceDeadlineInDays(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceDeadlineInDays)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComplianceDeadlineInDays<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetComplianceDeadlineInDays)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ComplianceGracePeriodInDays(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceGracePeriodInDays)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComplianceGracePeriodInDays<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetComplianceGracePeriodInDays)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OptOutOfAutoReboot(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptOutOfAutoReboot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOptOutOfAutoReboot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOptOutOfAutoReboot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for WindowsUpdateApprovalData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateApprovalData>();
}
unsafe impl windows_core::Interface for WindowsUpdateApprovalData {
    type Vtable = <IWindowsUpdateApprovalData as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateApprovalData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateApprovalData {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateApprovalData";
}
unsafe impl Send for WindowsUpdateApprovalData {}
unsafe impl Sync for WindowsUpdateApprovalData {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateAttentionRequiredInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateAttentionRequiredInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateAttentionRequiredInfo {
    pub fn Reason(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateAttentionRequiredInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateAttentionRequiredInfo>();
}
unsafe impl windows_core::Interface for WindowsUpdateAttentionRequiredInfo {
    type Vtable = <IWindowsUpdateAttentionRequiredInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateAttentionRequiredInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateAttentionRequiredInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredInfo";
}
unsafe impl Send for WindowsUpdateAttentionRequiredInfo {}
unsafe impl Sync for WindowsUpdateAttentionRequiredInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsUpdateAttentionRequiredReason(pub i32);
impl WindowsUpdateAttentionRequiredReason {
    pub const None: Self = Self(0i32);
    pub const SeekerUpdate: Self = Self(1i32);
    pub const ReadyToReboot: Self = Self(2i32);
    pub const NeedNonMeteredNetwork: Self = Self(3i32);
    pub const NeedUserAgreementForMeteredNetwork: Self = Self(4i32);
    pub const NeedNetwork: Self = Self(5i32);
    pub const NeedMoreSpace: Self = Self(6i32);
    pub const BatterySaverEnabled: Self = Self(7i32);
    pub const NeedUserInteraction: Self = Self(8i32);
    pub const NeedUserAgreementForPolicy: Self = Self(9i32);
    pub const CompatibilityError: Self = Self(10i32);
    pub const NeedUserInteractionForEula: Self = Self(11i32);
    pub const NeedUserInteractionForCta: Self = Self(12i32);
    pub const Regulated: Self = Self(13i32);
    pub const ExternalReboot: Self = Self(14i32);
    pub const OtherUpdate: Self = Self(15i32);
    pub const BlockedByProvider: Self = Self(16i32);
    pub const BlockedByPostRebootFailure: Self = Self(17i32);
    pub const UserEngaged: Self = Self(18i32);
    pub const BlockedByBattery: Self = Self(19i32);
    pub const Exclusivity: Self = Self(20i32);
    pub const BlockedBySerialization: Self = Self(21i32);
    pub const ConflictClass: Self = Self(22i32);
    pub const BlockedByAdminApproval: Self = Self(23i32);
    pub const BlockedByTooManyAttempts: Self = Self(24i32);
    pub const BlockedByFailure: Self = Self(25i32);
    pub const Demotion: Self = Self(26i32);
    pub const BlockedByActiveHours: Self = Self(27i32);
    pub const ScheduledForMaintenance: Self = Self(28i32);
    pub const PolicyScheduledInstallTime: Self = Self(29i32);
    pub const BlockedByOobe: Self = Self(30i32);
    pub const DeferredDuringOobe: Self = Self(31i32);
    pub const DeferredForSustainableTime: Self = Self(32i32);
    pub const BlockedByAppClose: Self = Self(33i32);
    pub const BlockedByAppRestart: Self = Self(34i32);
    pub const OtherUpdateReverting: Self = Self(35i32);
}
impl windows_core::TypeKind for WindowsUpdateAttentionRequiredReason {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowsUpdateAttentionRequiredReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAttentionRequiredReason;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateAttentionRequiredReasonChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateAttentionRequiredReasonChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    pub fn Update(&self) -> windows_core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reason(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateAttentionRequiredReasonChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    type Vtable = <IWindowsUpdateAttentionRequiredReasonChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateAttentionRequiredReasonChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs";
}
unsafe impl Send for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
unsafe impl Sync for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateGetAdministratorResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateGetAdministratorResult, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateGetAdministratorResult {
    pub fn Administrator(&self) -> windows_core::Result<WindowsUpdateAdministrator> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Administrator)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<WindowsUpdateAdministratorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateGetAdministratorResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateGetAdministratorResult>();
}
unsafe impl windows_core::Interface for WindowsUpdateGetAdministratorResult {
    type Vtable = <IWindowsUpdateGetAdministratorResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateGetAdministratorResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateGetAdministratorResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateGetAdministratorResult";
}
unsafe impl Send for WindowsUpdateGetAdministratorResult {}
unsafe impl Sync for WindowsUpdateGetAdministratorResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateItem, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateItem {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Category(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Category)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Operation(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateItem>();
}
unsafe impl windows_core::Interface for WindowsUpdateItem {
    type Vtable = <IWindowsUpdateItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateItem {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateItem";
}
unsafe impl Send for WindowsUpdateItem {}
unsafe impl Sync for WindowsUpdateItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateManager, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateManager {
    pub fn ScanningStateChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanningStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScanningStateChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScanningStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn WorkingStateChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WorkingStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWorkingStateChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWorkingStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ProgressChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateProgressChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProgressChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProgressChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProgressChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AttentionRequiredReasonChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateAttentionRequiredReasonChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttentionRequiredReasonChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAttentionRequiredReasonChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAttentionRequiredReasonChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ActionCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateActionCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActionCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveActionCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ScanCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateScanCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScanCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScanCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsScanning(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScanning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsWorking(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWorking)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LastSuccessfulScanTimestamp(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastSuccessfulScanTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetApplicableUpdates(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetApplicableUpdates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMostRecentCompletedUpdates(&self, count: i32) -> windows_core::Result<windows_collections::IVectorView<WindowsUpdateItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMostRecentCompletedUpdates)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMostRecentCompletedUpdatesAsync(&self, count: i32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<WindowsUpdateItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMostRecentCompletedUpdatesAsync)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartScan(&self, userinitiated: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartScan)(windows_core::Interface::as_raw(this), userinitiated).ok() }
    }
    pub fn GetProvider(&self, id: &windows_core::HSTRING) -> windows_core::Result<WindowsSoftwareUpdateProvider> {
        let this = &windows_core::Interface::cast::<IWindowsUpdateManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProvider)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderIds(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IWindowsUpdateManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ProviderIds)(windows_core::Interface::as_raw(this), windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetApplicableSoftwareUpdates(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsSoftwareUpdate>> {
        let this = &windows_core::Interface::cast::<IWindowsUpdateManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetApplicableSoftwareUpdates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PerformScan<P0>(&self, options: P0) -> windows_core::Result<WindowsSoftwareUpdateScanResult>
    where
        P0: windows_core::Param<WindowsUpdateManagerScanOptions>,
    {
        let this = &windows_core::Interface::cast::<IWindowsUpdateManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformScan)(windows_core::Interface::as_raw(this), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(clientid: &windows_core::HSTRING) -> windows_core::Result<WindowsUpdateManager> {
        Self::IWindowsUpdateManagerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(clientid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2(clientid: &windows_core::HSTRING, provideridfilter: &[windows_core::HSTRING]) -> windows_core::Result<WindowsUpdateManager> {
        Self::IWindowsUpdateManagerFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(clientid), provideridfilter.len().try_into().unwrap(), core::mem::transmute(provideridfilter.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsUpdateManagerFactory<R, F: FnOnce(&IWindowsUpdateManagerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateManager, IWindowsUpdateManagerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IWindowsUpdateManagerFactory2<R, F: FnOnce(&IWindowsUpdateManagerFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateManager, IWindowsUpdateManagerFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateManager>();
}
unsafe impl windows_core::Interface for WindowsUpdateManager {
    type Vtable = <IWindowsUpdateManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateManager {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateManager";
}
unsafe impl Send for WindowsUpdateManager {}
unsafe impl Sync for WindowsUpdateManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateManagerScanOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateManagerScanOptions, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateManagerScanOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateManagerScanOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsUserInitiated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUserInitiated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsUserInitiated(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsUserInitiated)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowBypassThrottling(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowBypassThrottling)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowBypassThrottling(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowBypassThrottling)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PerformUpdateActions(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformUpdateActions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPerformUpdateActions(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPerformUpdateActions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance(isuserinitiated: bool) -> windows_core::Result<WindowsUpdateManagerScanOptions> {
        Self::IWindowsUpdateManagerScanOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), isuserinitiated, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsUpdateManagerScanOptionsFactory<R, F: FnOnce(&IWindowsUpdateManagerScanOptionsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateManagerScanOptions, IWindowsUpdateManagerScanOptionsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateManagerScanOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateManagerScanOptions>();
}
unsafe impl windows_core::Interface for WindowsUpdateManagerScanOptions {
    type Vtable = <IWindowsUpdateManagerScanOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateManagerScanOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateManagerScanOptions {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateManagerScanOptions";
}
unsafe impl Send for WindowsUpdateManagerScanOptions {}
unsafe impl Sync for WindowsUpdateManagerScanOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateProgressChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateProgressChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateProgressChangedEventArgs {
    pub fn Update(&self) -> windows_core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActionProgress(&self) -> windows_core::Result<WindowsUpdateActionProgress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionProgress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateProgressChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateProgressChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateProgressChangedEventArgs {
    type Vtable = <IWindowsUpdateProgressChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateProgressChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateProgressChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateProgressChangedEventArgs";
}
unsafe impl Send for WindowsUpdateProgressChangedEventArgs {}
unsafe impl Sync for WindowsUpdateProgressChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateRestartRequestOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateRestartRequestOptions, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateRestartRequestOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateRestartRequestOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMoreInfoUrl<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMoreInfoUrl)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ComplianceDeadlineInDays(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceDeadlineInDays)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetComplianceDeadlineInDays(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetComplianceDeadlineInDays)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ComplianceGracePeriodInDays(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceGracePeriodInDays)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetComplianceGracePeriodInDays(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetComplianceGracePeriodInDays)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OrganizationName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrganizationName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetOrganizationName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrganizationName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn OptOutOfAutoReboot(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptOutOfAutoReboot)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOptOutOfAutoReboot(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOptOutOfAutoReboot)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<P2>(title: &windows_core::HSTRING, description: &windows_core::HSTRING, moreinfourl: P2, compliancedeadlineindays: i32, compliancegraceperiodindays: i32) -> windows_core::Result<WindowsUpdateRestartRequestOptions>
    where
        P2: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IWindowsUpdateRestartRequestOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(title), core::mem::transmute_copy(description), moreinfourl.param().abi(), compliancedeadlineindays, compliancegraceperiodindays, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsUpdateRestartRequestOptionsFactory<R, F: FnOnce(&IWindowsUpdateRestartRequestOptionsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateRestartRequestOptions, IWindowsUpdateRestartRequestOptionsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateRestartRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateRestartRequestOptions>();
}
unsafe impl windows_core::Interface for WindowsUpdateRestartRequestOptions {
    type Vtable = <IWindowsUpdateRestartRequestOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateRestartRequestOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateRestartRequestOptions {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateRestartRequestOptions";
}
unsafe impl Send for WindowsUpdateRestartRequestOptions {}
unsafe impl Sync for WindowsUpdateRestartRequestOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsUpdateScanCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateScanCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateScanCompletedEventArgs {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Updates(&self) -> windows_core::Result<windows_collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Updates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateScanCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateScanCompletedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateScanCompletedEventArgs {
    type Vtable = <IWindowsUpdateScanCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsUpdateScanCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateScanCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateScanCompletedEventArgs";
}
unsafe impl Send for WindowsUpdateScanCompletedEventArgs {}
unsafe impl Sync for WindowsUpdateScanCompletedEventArgs {}
