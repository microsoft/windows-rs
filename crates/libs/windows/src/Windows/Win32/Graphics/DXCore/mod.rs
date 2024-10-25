pub const AcgCompatible: DXCoreAdapterProperty = 10u32;
pub const AdapterBudgetChange: DXCoreNotificationType = 2u32;
pub const AdapterHardwareContentProtectionTeardown: DXCoreNotificationType = 3u32;
pub const AdapterListStale: DXCoreNotificationType = 0u32;
pub const AdapterMemoryBudget: DXCoreAdapterState = 1u32;
pub const AdapterNoLongerValid: DXCoreNotificationType = 1u32;
pub const ComputePreemptionGranularity: DXCoreAdapterProperty = 5u32;
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: windows_core::GUID = windows_core::GUID::from_u128(0x8c47866b_7583_450d_f0f0_6bada895af4b);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: windows_core::GUID = windows_core::GUID::from_u128(0x248e2800_a793_4724_abaa_23a6de1be090);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: windows_core::GUID = windows_core::GUID::from_u128(0x0c9ece4d_2f6e_4f01_8c96_e89e331b47b1);
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = 7u32;
pub const DedicatedSystemMemory: DXCoreAdapterProperty = 8u32;
pub const DriverDescription: DXCoreAdapterProperty = 2u32;
pub const DriverVersion: DXCoreAdapterProperty = 1u32;
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = 6u32;
pub const Hardware: DXCoreAdapterPreference = 0u32;
pub const HardwareID: DXCoreAdapterProperty = 3u32;
pub const HardwareIDParts: DXCoreAdapterProperty = 14u32;
pub const HighPerformance: DXCoreAdapterPreference = 2u32;
pub const InstanceLuid: DXCoreAdapterProperty = 0u32;
pub const IsDetachable: DXCoreAdapterProperty = 13u32;
pub const IsDriverUpdateInProgress: DXCoreAdapterState = 0u32;
pub const IsHardware: DXCoreAdapterProperty = 11u32;
pub const IsIntegrated: DXCoreAdapterProperty = 12u32;
pub const KmdModelVersion: DXCoreAdapterProperty = 4u32;
pub const Local: DXCoreSegmentGroup = 0u32;
pub const MinimumPower: DXCoreAdapterPreference = 1u32;
pub const NonLocal: DXCoreSegmentGroup = 1u32;
pub const SharedSystemMemory: DXCoreAdapterProperty = 9u32;
pub const _FACDXCORE: u32 = 2176u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DXCoreAdapterPreference(pub u32);
impl windows_core::TypeKind for DXCoreAdapterPreference {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DXCoreAdapterProperty(pub u32);
impl windows_core::TypeKind for DXCoreAdapterProperty {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DXCoreAdapterState(pub u32);
impl windows_core::TypeKind for DXCoreAdapterState {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DXCoreNotificationType(pub u32);
impl windows_core::TypeKind for DXCoreNotificationType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DXCoreSegmentGroup(pub u32);
impl windows_core::TypeKind for DXCoreSegmentGroup {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXCoreAdapterMemoryBudget {
    pub budget: u64,
    pub currentUsage: u64,
    pub availableForReservation: u64,
    pub currentReservation: u64,
}
impl Default for DXCoreAdapterMemoryBudget {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DXCoreAdapterMemoryBudget {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    pub nodeIndex: u32,
    pub segmentGroup: DXCoreSegmentGroup,
}
impl Default for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXCoreHardwareID {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSysID: u32,
    pub revision: u32,
}
impl Default for DXCoreHardwareID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DXCoreHardwareID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXCoreHardwareIDParts {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSystemID: u32,
    pub subVendorID: u32,
    pub revisionID: u32,
}
impl Default for DXCoreHardwareIDParts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DXCoreHardwareIDParts {
    type TypeKind = windows_core::CopyType;
}
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: Option<windows_core::IUnknown>, context: *const core::ffi::c_void)>;
