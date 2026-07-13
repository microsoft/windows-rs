pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8c47866b_7583_450d_f0f0_6bada895af4b);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x248e2800_a793_4724_abaa_23a6de1be090);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GENERIC_MEDIA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8eb2c848_82f6_4b49_aa87_aecfcf0174c6);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GENERIC_ML: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb71b0d41_1088_422f_a27c_0250b7d3a988);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0c9ece4d_2f6e_4f01_8c96_e89e331b47b1);
pub const DXCORE_HARDWARE_TYPE_ATTRIBUTE_COMPUTE_ACCELERATOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe0b195da_58ef_4a22_90f1_1f28169cab8d);
pub const DXCORE_HARDWARE_TYPE_ATTRIBUTE_GPU: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb69eb219_3ded_4464_979f_a00bd4687006);
pub const DXCORE_HARDWARE_TYPE_ATTRIBUTE_MEDIA_ACCELERATOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66bdb96a_050b_44c7_a4fd_d144ce0ab443);
pub const DXCORE_HARDWARE_TYPE_ATTRIBUTE_NPU: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd46140c4_add7_451b_9e56_06fe8c3b58ed);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreAdapterEngineIndex {
    pub physicalAdapterIndex: u32,
    pub engineIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreAdapterMemoryBudget {
    pub budget: u64,
    pub currentUsage: u64,
    pub availableForReservation: u64,
    pub currentReservation: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    pub nodeIndex: u32,
    pub segmentGroup: DXCoreSegmentGroup,
}
impl Default for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreAdapterPreference(pub i32);
impl DXCoreAdapterPreference {
    pub const Hardware: Self = Self(0);
    pub const MinimumPower: Self = Self(1);
    pub const HighPerformance: Self = Self(2);
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXCoreAdapterProcessSetQueryInput {
    pub arraySize: u32,
    pub processIds: *mut u32,
}
impl Default for DXCoreAdapterProcessSetQueryInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreAdapterProcessSetQueryOutput {
    pub processesWritten: u32,
    pub processesTotal: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreAdapterProperty(pub i32);
impl DXCoreAdapterProperty {
    pub const InstanceLuid: Self = Self(0);
    pub const DriverVersion: Self = Self(1);
    pub const DriverDescription: Self = Self(2);
    pub const HardwareID: Self = Self(3);
    pub const KmdModelVersion: Self = Self(4);
    pub const ComputePreemptionGranularity: Self = Self(5);
    pub const GraphicsPreemptionGranularity: Self = Self(6);
    pub const DedicatedAdapterMemory: Self = Self(7);
    pub const DedicatedSystemMemory: Self = Self(8);
    pub const SharedSystemMemory: Self = Self(9);
    pub const AcgCompatible: Self = Self(10);
    pub const IsHardware: Self = Self(11);
    pub const IsIntegrated: Self = Self(12);
    pub const IsDetachable: Self = Self(13);
    pub const HardwareIDParts: Self = Self(14);
    pub const PhysicalAdapterCount: Self = Self(15);
    pub const AdapterEngineCount: Self = Self(16);
    pub const AdapterEngineName: Self = Self(17);
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreAdapterState(pub i32);
impl DXCoreAdapterState {
    pub const IsDriverUpdateInProgress: Self = Self(0);
    pub const AdapterMemoryBudget: Self = Self(1);
    pub const AdapterMemoryUsageBytes: Self = Self(2);
    pub const AdapterMemoryUsageByProcessBytes: Self = Self(3);
    pub const AdapterEngineRunningTimeMicroseconds: Self = Self(4);
    pub const AdapterEngineRunningTimeByProcessMicroseconds: Self = Self(5);
    pub const AdapterTemperatureCelsius: Self = Self(6);
    pub const AdapterInUseProcessCount: Self = Self(7);
    pub const AdapterInUseProcessSet: Self = Self(8);
    pub const AdapterEngineFrequencyHertz: Self = Self(9);
    pub const AdapterMemoryFrequencyHertz: Self = Self(10);
    pub const SingleAdapterHybridMode: Self = Self(11);
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXCoreEngineNamePropertyInput {
    pub adapterEngineIndex: DXCoreAdapterEngineIndex,
    pub engineNameLength: u32,
    pub engineName: *mut u16,
}
impl Default for DXCoreEngineNamePropertyInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreEngineNamePropertyOutput {
    pub engineNameLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreEngineQueryInput {
    pub adapterEngineIndex: DXCoreAdapterEngineIndex,
    pub processId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreEngineQueryOutput {
    pub runningTime: u64,
    pub processQuerySucceeded: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreFrequencyQueryOutput {
    pub frequency: u64,
    pub maxFrequency: u64,
    pub maxOverclockedFrequency: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreHardwareID {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSysID: u32,
    pub revision: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreHardwareIDParts {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSystemID: u32,
    pub subVendorID: u32,
    pub revisionID: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreHardwareTypeFilterFlags(pub u32);
impl DXCoreHardwareTypeFilterFlags {
    pub const None: Self = Self(0);
    pub const GPU: Self = Self(1);
    pub const ComputeAccelerator: Self = Self(2);
    pub const NPU: Self = Self(4);
    pub const MediaAccelerator: Self = Self(8);
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXCoreMemoryQueryInput {
    pub physicalAdapterIndex: u32,
    pub memoryType: DXCoreMemoryType,
}
impl Default for DXCoreMemoryQueryInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreMemoryType(pub i32);
impl DXCoreMemoryType {
    pub const Dedicated: Self = Self(0);
    pub const Shared: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreMemoryUsage {
    pub committed: u64,
    pub resident: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreNotificationType(pub i32);
impl DXCoreNotificationType {
    pub const AdapterListStale: Self = Self(0);
    pub const AdapterNoLongerValid: Self = Self(1);
    pub const AdapterBudgetChange: Self = Self(2);
    pub const AdapterHardwareContentProtectionTeardown: Self = Self(3);
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXCoreProcessMemoryQueryInput {
    pub physicalAdapterIndex: u32,
    pub memoryType: DXCoreMemoryType,
    pub processId: u32,
}
impl Default for DXCoreProcessMemoryQueryInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXCoreProcessMemoryQueryOutput {
    pub memoryUsage: DXCoreMemoryUsage,
    pub processQuerySucceeded: bool,
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreRuntimeFilterFlags(pub u32);
impl DXCoreRuntimeFilterFlags {
    pub const None: Self = Self(0);
    pub const D3D11: Self = Self(1);
    pub const D3D12: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreSegmentGroup(pub i32);
impl DXCoreSegmentGroup {
    pub const Local: Self = Self(0);
    pub const NonLocal: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreSingleAdapterHybridMode(pub i32);
impl DXCoreSingleAdapterHybridMode {
    pub const Unspecified: Self = Self(0);
    pub const MinimumPower: Self = Self(1);
    pub const HighPerformance: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DXCoreWorkload(pub i32);
impl DXCoreWorkload {
    pub const Graphics: Self = Self(0);
    pub const Compute: Self = Self(1);
    pub const Media: Self = Self(2);
    pub const MachineLearning: Self = Self(3);
}
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: *mut core::ffi::c_void, context: *const core::ffi::c_void)>;
