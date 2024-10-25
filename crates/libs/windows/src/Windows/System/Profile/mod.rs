#[cfg(feature = "System_Profile_SystemManufacturers")]
pub mod SystemManufacturers;
windows_core::imp::define_interface!(IAnalyticsInfoStatics, IAnalyticsInfoStatics_Vtbl, 0x1d5ee066_188d_5ba9_4387_acaeb0e7e305);
impl windows_core::RuntimeType for IAnalyticsInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAnalyticsInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VersionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAnalyticsInfoStatics2, IAnalyticsInfoStatics2_Vtbl, 0x101704ea_a7f9_46d2_ab94_016865afdb25);
impl windows_core::RuntimeType for IAnalyticsInfoStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAnalyticsInfoStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemPropertiesAsync: usize,
}
windows_core::imp::define_interface!(IAnalyticsVersionInfo, IAnalyticsVersionInfo_Vtbl, 0x926130b8_9955_4c74_bdc1_7cd0decf9b03);
impl windows_core::RuntimeType for IAnalyticsVersionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAnalyticsVersionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceFamily: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceFamilyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAnalyticsVersionInfo2, IAnalyticsVersionInfo2_Vtbl, 0x76e915b1_ff36_407c_9f57_160d3e540747);
impl windows_core::RuntimeType for IAnalyticsVersionInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAnalyticsVersionInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppApplicabilityStatics, IAppApplicabilityStatics_Vtbl, 0x1664a082_0f38_5c99_83e4_48995970861c);
impl windows_core::RuntimeType for IAppApplicabilityStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppApplicabilityStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnsupportedAppRequirements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnsupportedAppRequirements: usize,
}
windows_core::imp::define_interface!(IEducationSettingsStatics, IEducationSettingsStatics_Vtbl, 0xfc53f0ef_4d3e_4e13_9b23_505f4d091e92);
impl windows_core::RuntimeType for IEducationSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEducationSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEducationEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHardwareIdentificationStatics, IHardwareIdentificationStatics_Vtbl, 0x971260e0_f170_4a42_bd55_a900b212dae2);
impl windows_core::RuntimeType for IHardwareIdentificationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHardwareIdentificationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetPackageSpecificToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPackageSpecificToken: usize,
}
windows_core::imp::define_interface!(IHardwareToken, IHardwareToken_Vtbl, 0x28f6d4c0_fb12_40a4_8167_7f4e03d2724c);
impl windows_core::RuntimeType for IHardwareToken {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHardwareToken_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Signature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Signature: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Certificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Certificate: usize,
}
windows_core::imp::define_interface!(IKnownRetailInfoPropertiesStatics, IKnownRetailInfoPropertiesStatics_Vtbl, 0x99571178_500f_487e_8e75_29e551728712);
impl windows_core::RuntimeType for IKnownRetailInfoPropertiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownRetailInfoPropertiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RetailAccessCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayModelName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Price: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsFeatured: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FormFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ScreenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Weight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BatteryLifeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProcessorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Memory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StorageDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GraphicsDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FrontCameraDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RearCameraDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HasNfc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HasSdSlot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HasOpticalDrive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsOfficeInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub WindowsEdition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlatformAutomaticAppSignInManagerStatics, IPlatformAutomaticAppSignInManagerStatics_Vtbl, 0x1ac9afce_8dd5_5c2d_b420_767d1f3b7d03);
impl windows_core::RuntimeType for IPlatformAutomaticAppSignInManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlatformAutomaticAppSignInManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Policy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlatformAutomaticAppSignInPolicy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlatformDiagnosticsAndUsageDataSettingsStatics, IPlatformDiagnosticsAndUsageDataSettingsStatics_Vtbl, 0xb6e24c1b_7b1c_4b32_8c62_a66597ce723a);
impl windows_core::RuntimeType for IPlatformDiagnosticsAndUsageDataSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CollectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlatformDataCollectionLevel) -> windows_core::HRESULT,
    pub CollectionLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCollectionLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CanCollectDiagnostics: unsafe extern "system" fn(*mut core::ffi::c_void, PlatformDataCollectionLevel, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRetailInfoStatics, IRetailInfoStatics_Vtbl, 0x0712c6b8_8b92_4f2a_8499_031f1798d6ef);
impl windows_core::RuntimeType for IRetailInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRetailInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsDemoModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(ISharedModeSettingsStatics, ISharedModeSettingsStatics_Vtbl, 0x893df40e_cad6_4d50_8c49_6fcfc03edb29);
impl windows_core::RuntimeType for ISharedModeSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISharedModeSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISharedModeSettingsStatics2, ISharedModeSettingsStatics2_Vtbl, 0x608988a4_ccf1_4ee8_a5e2_fd6a1d0cfac8);
impl windows_core::RuntimeType for ISharedModeSettingsStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISharedModeSettingsStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShouldAvoidLocalStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartAppControlPolicyStatics, ISmartAppControlPolicyStatics_Vtbl, 0x5ff8c75b_073e_5015_8d98_5ff224180a0b);
impl windows_core::RuntimeType for ISmartAppControlPolicyStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartAppControlPolicyStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemIdentificationInfo, ISystemIdentificationInfo_Vtbl, 0x0c659e7d_c3c2_4d33_a2df_21bc41916eb3);
impl windows_core::RuntimeType for ISystemIdentificationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemIdentificationInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemIdentificationSource) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemIdentificationStatics, ISystemIdentificationStatics_Vtbl, 0x5581f42a_d3df_4d93_a37d_c41a616c6d01);
impl windows_core::RuntimeType for ISystemIdentificationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemIdentificationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetSystemIdForPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemIdForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemSetupInfoStatics, ISystemSetupInfoStatics_Vtbl, 0xb8366a4b_fb6a_4571_be0a_9a0f67954123);
impl windows_core::RuntimeType for ISystemSetupInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemSetupInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OutOfBoxExperienceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemOutOfBoxExperienceState) -> windows_core::HRESULT,
    pub OutOfBoxExperienceStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveOutOfBoxExperienceStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUnsupportedAppRequirement, IUnsupportedAppRequirement_Vtbl, 0x6182445c_894b_5cbc_8976_a98e0a9b998d);
impl windows_core::RuntimeType for IUnsupportedAppRequirement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUnsupportedAppRequirement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Requirement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Reasons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnsupportedAppRequirementReasons) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsIntegrityPolicyStatics, IWindowsIntegrityPolicyStatics_Vtbl, 0x7d1d81db_8d63_4789_9ea5_ddcf65a94f3c);
impl windows_core::RuntimeType for IWindowsIntegrityPolicyStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsIntegrityPolicyStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEnabledForTrial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanDisable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDisableSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PolicyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePolicyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
pub struct AnalyticsInfo;
impl AnalyticsInfo {}
impl windows_core::RuntimeName for AnalyticsInfo {
    const NAME: &'static str = "Windows.System.Profile.AnalyticsInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AnalyticsVersionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AnalyticsVersionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AnalyticsVersionInfo {
    pub fn DeviceFamily(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceFamily)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceFamilyVersion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceFamilyVersion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProductName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAnalyticsVersionInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AnalyticsVersionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAnalyticsVersionInfo>();
}
unsafe impl windows_core::Interface for AnalyticsVersionInfo {
    type Vtable = <IAnalyticsVersionInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAnalyticsVersionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AnalyticsVersionInfo {
    const NAME: &'static str = "Windows.System.Profile.AnalyticsVersionInfo";
}
pub struct AppApplicability;
impl AppApplicability {}
impl windows_core::RuntimeName for AppApplicability {
    const NAME: &'static str = "Windows.System.Profile.AppApplicability";
}
pub struct EducationSettings;
impl EducationSettings {}
impl windows_core::RuntimeName for EducationSettings {
    const NAME: &'static str = "Windows.System.Profile.EducationSettings";
}
pub struct HardwareIdentification;
impl HardwareIdentification {}
impl windows_core::RuntimeName for HardwareIdentification {
    const NAME: &'static str = "Windows.System.Profile.HardwareIdentification";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HardwareToken(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HardwareToken, windows_core::IUnknown, windows_core::IInspectable);
impl HardwareToken {
    #[cfg(feature = "Storage_Streams")]
    pub fn Id(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Signature(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Signature)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Certificate(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Certificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HardwareToken {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHardwareToken>();
}
unsafe impl windows_core::Interface for HardwareToken {
    type Vtable = <IHardwareToken as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHardwareToken as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HardwareToken {
    const NAME: &'static str = "Windows.System.Profile.HardwareToken";
}
pub struct KnownRetailInfoProperties;
impl KnownRetailInfoProperties {}
impl windows_core::RuntimeName for KnownRetailInfoProperties {
    const NAME: &'static str = "Windows.System.Profile.KnownRetailInfoProperties";
}
pub struct PlatformAutomaticAppSignInManager;
impl PlatformAutomaticAppSignInManager {}
impl windows_core::RuntimeName for PlatformAutomaticAppSignInManager {
    const NAME: &'static str = "Windows.System.Profile.PlatformAutomaticAppSignInManager";
}
pub struct PlatformDiagnosticsAndUsageDataSettings;
impl PlatformDiagnosticsAndUsageDataSettings {}
impl windows_core::RuntimeName for PlatformDiagnosticsAndUsageDataSettings {
    const NAME: &'static str = "Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings";
}
pub struct RetailInfo;
impl RetailInfo {}
impl windows_core::RuntimeName for RetailInfo {
    const NAME: &'static str = "Windows.System.Profile.RetailInfo";
}
pub struct SharedModeSettings;
impl SharedModeSettings {}
impl windows_core::RuntimeName for SharedModeSettings {
    const NAME: &'static str = "Windows.System.Profile.SharedModeSettings";
}
pub struct SmartAppControlPolicy;
impl SmartAppControlPolicy {}
impl windows_core::RuntimeName for SmartAppControlPolicy {
    const NAME: &'static str = "Windows.System.Profile.SmartAppControlPolicy";
}
pub struct SystemIdentification;
impl SystemIdentification {}
impl windows_core::RuntimeName for SystemIdentification {
    const NAME: &'static str = "Windows.System.Profile.SystemIdentification";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemIdentificationInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemIdentificationInfo, windows_core::IUnknown, windows_core::IInspectable);
impl SystemIdentificationInfo {
    #[cfg(feature = "Storage_Streams")]
    pub fn Id(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Source(&self) -> windows_core::Result<SystemIdentificationSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SystemIdentificationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemIdentificationInfo>();
}
unsafe impl windows_core::Interface for SystemIdentificationInfo {
    type Vtable = <ISystemIdentificationInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISystemIdentificationInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemIdentificationInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemIdentificationInfo";
}
pub struct SystemSetupInfo;
impl SystemSetupInfo {}
impl windows_core::RuntimeName for SystemSetupInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemSetupInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UnsupportedAppRequirement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UnsupportedAppRequirement, windows_core::IUnknown, windows_core::IInspectable);
impl UnsupportedAppRequirement {
    pub fn Requirement(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Requirement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reasons(&self) -> windows_core::Result<UnsupportedAppRequirementReasons> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reasons)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for UnsupportedAppRequirement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUnsupportedAppRequirement>();
}
unsafe impl windows_core::Interface for UnsupportedAppRequirement {
    type Vtable = <IUnsupportedAppRequirement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUnsupportedAppRequirement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UnsupportedAppRequirement {
    const NAME: &'static str = "Windows.System.Profile.UnsupportedAppRequirement";
}
pub struct WindowsIntegrityPolicy;
impl WindowsIntegrityPolicy {}
impl windows_core::RuntimeName for WindowsIntegrityPolicy {
    const NAME: &'static str = "Windows.System.Profile.WindowsIntegrityPolicy";
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformAutomaticAppSignInPolicy(pub i32);
impl PlatformAutomaticAppSignInPolicy {
    pub const Unknown: Self = Self(0i32);
    pub const PermissionRequired: Self = Self(1i32);
    pub const AlwaysAllowed: Self = Self(2i32);
}
impl windows_core::TypeKind for PlatformAutomaticAppSignInPolicy {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlatformAutomaticAppSignInPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Profile.PlatformAutomaticAppSignInPolicy;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformDataCollectionLevel(pub i32);
impl PlatformDataCollectionLevel {
    pub const Security: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enhanced: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl windows_core::TypeKind for PlatformDataCollectionLevel {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlatformDataCollectionLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Profile.PlatformDataCollectionLevel;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SystemIdentificationSource(pub i32);
impl SystemIdentificationSource {
    pub const None: Self = Self(0i32);
    pub const Tpm: Self = Self(1i32);
    pub const Uefi: Self = Self(2i32);
    pub const Registry: Self = Self(3i32);
}
impl windows_core::TypeKind for SystemIdentificationSource {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SystemIdentificationSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Profile.SystemIdentificationSource;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SystemOutOfBoxExperienceState(pub i32);
impl SystemOutOfBoxExperienceState {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl windows_core::TypeKind for SystemOutOfBoxExperienceState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SystemOutOfBoxExperienceState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Profile.SystemOutOfBoxExperienceState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnsupportedAppRequirementReasons(pub u32);
impl UnsupportedAppRequirementReasons {
    pub const Unknown: Self = Self(0u32);
    pub const DeniedBySystem: Self = Self(1u32);
}
impl windows_core::TypeKind for UnsupportedAppRequirementReasons {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for UnsupportedAppRequirementReasons {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Profile.UnsupportedAppRequirementReasons;u4)");
}
