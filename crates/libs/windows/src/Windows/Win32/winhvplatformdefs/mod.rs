#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PWHV_HYPERCALL_CONTEXT = *mut WHV_HYPERCALL_CONTEXT;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PWHV_PROCESSOR_PERFMON_FEATURES = *mut WHV_PROCESSOR_PERFMON_FEATURES;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PWHV_PROCESSOR_XSAVE_FEATURES = *mut WHV_PROCESSOR_XSAVE_FEATURES;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PWHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT = *mut WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PWHV_X64_PENDING_INTERRUPTION_TYPE = *mut WHV_X64_PENDING_INTERRUPTION_TYPE;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_ACCESS_GPA_CONTROLS {
    pub AsUINT64: u64,
    pub Anonymous: WHV_ACCESS_GPA_CONTROLS_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_ACCESS_GPA_CONTROLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_ACCESS_GPA_CONTROLS_0 {
    pub CacheType: WHV_CACHE_TYPE,
    pub InputVtl: WHV_INPUT_VTL,
    pub Reserved: u8,
    pub Reserved1: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_ACCESS_GPA_CONTROLS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_ADVISE_GPA_RANGE {
    pub Populate: WHV_ADVISE_GPA_RANGE_POPULATE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_ADVISE_GPA_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_ADVISE_GPA_RANGE_CODE = i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_ADVISE_GPA_RANGE_POPULATE {
    pub Flags: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS,
    pub AccessType: WHV_MEMORY_ACCESS_TYPE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_ADVISE_GPA_RANGE_POPULATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {
    pub AsUINT32: u32,
    pub Anonymous: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {
    pub fn Prefetch(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Prefetch(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn AvoidHardFaults(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_AvoidHardFaults(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_ALLOCATE_VPCI_RESOURCE_FLAGS = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_ANY_VP: u32 = 4294967295;
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_ARM64_GLOBAL_INTERRUPT_CONTROLLER_STATE {
    pub Version: u8,
    pub GicVersion: u8,
    pub Reserved0: [u8; 2],
    pub NumInterrupts: u32,
    pub GicdCtlrEnableGrp1A: u64,
    pub Interrupts: [WHV_ARM64_GLOBAL_INTERRUPT_STATE; 1],
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_GLOBAL_INTERRUPT_CONTROLLER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub const WHV_ARM64_GLOBAL_INTERRUPT_CONTROLLER_STATE_VERSION_CURRENT: u32 = 1;
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_ARM64_GLOBAL_INTERRUPT_STATE {
    pub InterruptId: u32,
    pub ActiveVpIndex: u32,
    pub Anonymous: WHV_ARM64_GLOBAL_INTERRUPT_STATE_0,
    pub InterruptState: WHV_ARM64_INTERRUPT_STATE,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_GLOBAL_INTERRUPT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_ARM64_GLOBAL_INTERRUPT_STATE_0 {
    pub TargetMpidr: u32,
    pub TargetVpIndex: u32,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_GLOBAL_INTERRUPT_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub type WHV_ARM64_HYPERCALL_CONTEXT = WHV_HYPERCALL_CONTEXT;
#[cfg(target_arch = "aarch64")]
pub type WHV_ARM64_IC_EMULATION_MODE = i32;
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_ARM64_IC_GIC_V3_PARAMETERS {
    pub GicdBaseAddress: WHV_GUEST_PHYSICAL_ADDRESS,
    pub GitsTranslaterBaseAddress: WHV_GUEST_PHYSICAL_ADDRESS,
    pub Reserved: u32,
    pub GicLpiIntIdBits: u32,
    pub GicPpiOverflowInterruptFromCntv: WHV_ARM64_INTERRUPT_VECTOR,
    pub GicPpiPerformanceMonitorsInterrupt: WHV_ARM64_INTERRUPT_VECTOR,
    pub Reserved1: [u32; 6],
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_IC_GIC_V3_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_ARM64_IC_PARAMETERS {
    pub EmulationMode: WHV_ARM64_IC_EMULATION_MODE,
    pub Reserved: u32,
    pub Anonymous: WHV_ARM64_IC_PARAMETERS_0,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_IC_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_ARM64_IC_PARAMETERS_0 {
    pub GicV3Parameters: WHV_ARM64_IC_GIC_V3_PARAMETERS,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_IC_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub const WHV_ARM64_INTERRUPT_CONTROLLER_STATE_VERSION_CURRENT: u32 = 1;
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_ARM64_INTERRUPT_STATE {
    pub Anonymous: WHV_ARM64_INTERRUPT_STATE_0,
    pub GicrIpriorityrConfigured: u8,
    pub GicrIpriorityrActive: u8,
    pub Reserved1: u8,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_ARM64_INTERRUPT_STATE_0 {
    pub _bitfield: u8,
}
#[cfg(target_arch = "aarch64")]
impl WHV_ARM64_INTERRUPT_STATE_0 {
    pub fn Enabled(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Enabled(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn EdgeTriggered(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_EdgeTriggered(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn Asserted(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Asserted(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn SetPending(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_SetPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn Active(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_Active(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn Direct(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_Direct(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u8) << 5);
    }
    pub fn Reserved0(&self) -> u8 {
        self._bitfield >> 6
    }
    pub fn set_Reserved0(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
}
#[cfg(target_arch = "aarch64")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WHV_ARM64_INTERRUPT_VECTOR(pub u32);
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_ARM64_LOCAL_INTERRUPT_CONTROLLER_STATE {
    pub Version: u8,
    pub GicVersion: u8,
    pub Reserved0: [u8; 6],
    pub IccIgrpen1El1: u64,
    pub GicrCtlrEnableLpis: u64,
    pub IccBpr1El1: u64,
    pub IccPmrEl1: u64,
    pub GicrPropbaser: u64,
    pub GicrPendbaser: u64,
    pub IchAp1REl2: [u32; 4],
    pub BankedInterruptState: [WHV_ARM64_INTERRUPT_STATE; 32],
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_LOCAL_INTERRUPT_CONTROLLER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_ARM64_PENDING_EVENT {
    pub Anonymous: WHV_ARM64_PENDING_EVENT_0,
    pub Anonymous2: WHV_ARM64_PENDING_EVENT_1,
    pub Exception: WHV_ARM64_PENDING_EXCEPTION_EVENT,
    pub SyntheticException: WHV_ARM64_PENDING_SYNTHETIC_EXCEPTION_EVENT,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_PENDING_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_ARM64_PENDING_EVENT_0 {
    pub Reg0: WHV_UINT128,
    pub Reg1: WHV_UINT128,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_PENDING_EVENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_ARM64_PENDING_EVENT_1 {
    pub _bitfield: u8,
    pub EventData: [u8; 15],
}
#[cfg(target_arch = "aarch64")]
impl WHV_ARM64_PENDING_EVENT_1 {
    pub fn EventPending(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EventPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn EventType(&self) -> u8 {
        (self._bitfield << 4) >> 5
    }
    pub fn set_EventType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_PENDING_EVENT_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub type WHV_ARM64_PENDING_EVENT_TYPE = i32;
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_ARM64_PENDING_EXCEPTION_EVENT {
    pub AsUINT64: [u64; 3],
    pub Anonymous: WHV_ARM64_PENDING_EXCEPTION_EVENT_0,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_PENDING_EXCEPTION_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_ARM64_PENDING_EXCEPTION_EVENT_0 {
    pub _bitfield: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
    pub EsrElx: u64,
    pub FarElx: u64,
}
#[cfg(target_arch = "aarch64")]
impl WHV_ARM64_PENDING_EXCEPTION_EVENT_0 {
    pub fn EventPending(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EventPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn EventType(&self) -> u8 {
        (self._bitfield << 4) >> 5
    }
    pub fn set_EventType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_ARM64_PENDING_SYNTHETIC_EXCEPTION_EVENT {
    pub AsUINT64: [u64; 2],
    pub Anonymous: WHV_ARM64_PENDING_SYNTHETIC_EXCEPTION_EVENT_0,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_PENDING_SYNTHETIC_EXCEPTION_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_ARM64_PENDING_SYNTHETIC_EXCEPTION_EVENT_0 {
    pub _bitfield: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub ExceptionType: u32,
    pub Context: u64,
}
#[cfg(target_arch = "aarch64")]
impl WHV_ARM64_PENDING_SYNTHETIC_EXCEPTION_EVENT_0 {
    pub fn EventPending(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EventPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn EventType(&self) -> u8 {
        (self._bitfield << 4) >> 5
    }
    pub fn set_EventType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[cfg(target_arch = "aarch64")]
pub type WHV_ARM64_PROCESSOR_FEATURES = WHV_PROCESSOR_FEATURES;
#[cfg(target_arch = "aarch64")]
pub type WHV_ARM64_PROCESSOR_FEATURES1 = WHV_PROCESSOR_FEATURES1;
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_ARM64_RESET_CONTEXT {
    pub Header: WHV_INTERCEPT_MESSAGE_HEADER,
    pub ResetType: WHV_ARM64_RESET_TYPE,
    pub Reserved: u32,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_ARM64_RESET_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub type WHV_ARM64_RESET_TYPE = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_ARM64_SYNTHETIC_EXCEPTION_TYPE = i32;
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_ARM64_VP_STATE_SME {
    pub Version: u16,
    pub RegisterDataOffset: u16,
    pub StreamingVectorLength: u32,
    pub Reserved0: u64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_ARM64_VP_STATE_SVE {
    pub Version: u16,
    pub RegisterDataOffset: u16,
    pub VectorLength: u32,
    pub Reserved0: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_CACHE_TYPE = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_CACHE_TYPE = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_CAPABILITY {
    pub HypervisorPresent: windows_core::BOOL,
    pub Features: WHV_CAPABILITY_FEATURES,
    pub ExtendedVmExits: WHV_EXTENDED_VM_EXITS,
    pub ProcessorVendor: WHV_PROCESSOR_VENDOR,
    pub ProcessorFeatures: WHV_PROCESSOR_FEATURES,
    pub SyntheticProcessorFeaturesBanks: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS,
    pub ProcessorClFlushSize: u8,
    pub ProcessorClockFrequency: u64,
    pub ProcessorFeaturesBanks: WHV_PROCESSOR_FEATURES_BANKS,
    pub GpaRangePopulateFlags: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS,
    pub ProcessorFrequencyCap: WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP,
    pub SchedulerFeatures: WHV_SCHEDULER_FEATURES,
    pub PhysicalAddressWidth: u32,
    pub NestedFeatureRegister: u64,
    pub ProcessorXsaveFeatures: WHV_PROCESSOR_XSAVE_FEATURES,
    pub InterruptClockFrequency: u64,
    pub ProcessorPerfmonFeatures: WHV_PROCESSOR_PERFMON_FEATURES,
    pub X64MsrExitBitmap: WHV_X64_MSR_EXIT_BITMAP,
    pub ExceptionExitBitmap: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_CAPABILITY {
    pub HypervisorPresent: windows_core::BOOL,
    pub Features: WHV_CAPABILITY_FEATURES,
    pub ExtendedVmExits: WHV_EXTENDED_VM_EXITS,
    pub ProcessorVendor: WHV_PROCESSOR_VENDOR,
    pub ProcessorFeatures: WHV_PROCESSOR_FEATURES,
    pub SyntheticProcessorFeaturesBanks: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS,
    pub ProcessorClFlushSize: u8,
    pub ProcessorClockFrequency: u64,
    pub ProcessorFeaturesBanks: WHV_PROCESSOR_FEATURES_BANKS,
    pub GpaRangePopulateFlags: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS,
    pub ProcessorFrequencyCap: WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP,
    pub SchedulerFeatures: WHV_SCHEDULER_FEATURES,
    pub PhysicalAddressWidth: u32,
    pub NestedFeatureRegister: u64,
    pub GicLpiIntIdBits: u32,
    pub MaxSveVectorLength: u32,
    pub MaxSmeStreamingVectorLength: u32,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_CAPABILITY_CODE = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_CAPABILITY_CODE = i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_CAPABILITY_FEATURES {
    pub Anonymous: WHV_CAPABILITY_FEATURES_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_CAPABILITY_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_CAPABILITY_FEATURES_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_CAPABILITY_FEATURES_0 {
    pub fn PartialUnmap(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_PartialUnmap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn LocalApicEmulation(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_LocalApicEmulation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn Xsave(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Xsave(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn DirtyPageTracking(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_DirtyPageTracking(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn SpeculationControl(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_SpeculationControl(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn ApicRemoteRead(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_ApicRemoteRead(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u64) << 5);
    }
    pub fn IdleSuspend(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_IdleSuspend(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u64) << 6);
    }
    pub fn VirtualPciDeviceSupport(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_VirtualPciDeviceSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u64) << 7);
    }
    pub fn IommuSupport(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_IommuSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u64) << 8);
    }
    pub fn VpHotAddRemove(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_VpHotAddRemove(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u64) << 9);
    }
    pub fn DeviceAccessTracking(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_DeviceAccessTracking(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u64) << 10);
    }
    pub fn ReservedX640(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_ReservedX640(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u64) << 11);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 12
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(4503599627370495 << 12)) | ((value & 4503599627370495) << 12);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    pub _bitfield: u32,
    pub HighestFrequencyMhz: u32,
    pub NominalFrequencyMhz: u32,
    pub LowestFrequencyMhz: u32,
    pub FrequencyStepMhz: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    pub fn IsSupported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_IsSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_CPUID_OUTPUT {
    pub Eax: u32,
    pub Ebx: u32,
    pub Ecx: u32,
    pub Edx: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_CREATE_VPCI_DEVICE_FLAGS = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_DELIVERABILITY_NOTIFICATIONS_REGISTER = WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_DISABLE_PARTITION_VTL_FLAGS {
    pub AsUINT8: u8,
    pub Anonymous: WHV_DISABLE_PARTITION_VTL_FLAGS_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_DISABLE_PARTITION_VTL_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_DISABLE_PARTITION_VTL_FLAGS_0 {
    pub _bitfield: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_DISABLE_PARTITION_VTL_FLAGS_0 {
    pub fn ScrubOnly(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ScrubOnly(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_DISABLE_VP_VTL_FLAGS {
    pub AsUINT8: u8,
    pub Anonymous: WHV_DISABLE_VP_VTL_FLAGS_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_DISABLE_VP_VTL_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_DISABLE_VP_VTL_FLAGS_0 {
    pub _bitfield: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_DISABLE_VP_VTL_FLAGS_0 {
    pub fn ScrubOnly(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ScrubOnly(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_DOORBELL_MATCH_DATA {
    pub GuestAddress: WHV_GUEST_PHYSICAL_ADDRESS,
    pub Value: u64,
    pub Length: u32,
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_DOORBELL_MATCH_DATA {
    pub fn MatchOnValue(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_MatchOnValue(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn MatchOnLength(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_MatchOnLength(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_ENABLE_PARTITION_VTL_FLAGS {
    pub AsUINT8: u8,
    pub Anonymous: WHV_ENABLE_PARTITION_VTL_FLAGS_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_ENABLE_PARTITION_VTL_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_ENABLE_PARTITION_VTL_FLAGS_0 {
    pub _bitfield: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_ENABLE_PARTITION_VTL_FLAGS_0 {
    pub fn EnableMbec(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EnableMbec(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn EnableSupervisorShadowStack(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_EnableSupervisorShadowStack(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn EnableHardwareHvpt(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_EnableHardwareHvpt(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_EXCEPTION_TYPE = i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_EXTENDED_VM_EXITS {
    pub Anonymous: WHV_EXTENDED_VM_EXITS_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_EXTENDED_VM_EXITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_EXTENDED_VM_EXITS_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_EXTENDED_VM_EXITS_0 {
    pub fn X64CpuidExit(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_X64CpuidExit(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn X64MsrExit(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_X64MsrExit(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn ExceptionExit(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ExceptionExit(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn X64RdtscExit(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_X64RdtscExit(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn X64ApicSmiExitTrap(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_X64ApicSmiExitTrap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn HypercallExit(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_HypercallExit(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u64) << 5);
    }
    pub fn X64ApicInitSipiExitTrap(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_X64ApicInitSipiExitTrap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u64) << 6);
    }
    pub fn X64ApicWriteLint0ExitTrap(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_X64ApicWriteLint0ExitTrap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u64) << 7);
    }
    pub fn X64ApicWriteLint1ExitTrap(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_X64ApicWriteLint1ExitTrap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u64) << 8);
    }
    pub fn X64ApicWriteSvrExitTrap(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_X64ApicWriteSvrExitTrap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u64) << 9);
    }
    pub fn UnknownSynicConnection(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_UnknownSynicConnection(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u64) << 10);
    }
    pub fn RetargetUnknownVpciDevice(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_RetargetUnknownVpciDevice(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u64) << 11);
    }
    pub fn X64ApicWriteLdrExitTrap(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_X64ApicWriteLdrExitTrap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u64) << 12);
    }
    pub fn X64ApicWriteDfrExitTrap(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_X64ApicWriteDfrExitTrap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u64) << 13);
    }
    pub fn GpaAccessFaultExit(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_GpaAccessFaultExit(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u64) << 14);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 15
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(562949953421311 << 15)) | ((value & 562949953421311) << 15);
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_EXTENDED_VM_EXITS_RESERVED_BITFIELD_COUNT: u32 = 49;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WHV_GUEST_PHYSICAL_ADDRESS(pub u64);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WHV_GUEST_VIRTUAL_ADDRESS(pub u64);
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_HYPERCALL_CONTEXT {
    pub Rax: u64,
    pub Rbx: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub R8: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub Reserved0: u64,
    pub XmmRegisters: [WHV_UINT128; 6],
    pub Reserved1: [u64; 2],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_HYPERCALL_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_HYPERCALL_CONTEXT {
    pub Header: WHV_INTERCEPT_MESSAGE_HEADER,
    pub Immediate: u16,
    pub Reserved1: u16,
    pub Reserved2: u32,
    pub X: [u64; 18],
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_HYPERCALL_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_HYPERCALL_CONTEXT_MAX_XMM_REGISTERS: u32 = 6;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_INITIAL_VP_CONTEXT {
    pub Rip: u64,
    pub Rsp: u64,
    pub Rflags: u64,
    pub Cs: WHV_X64_SEGMENT_REGISTER,
    pub Ds: WHV_X64_SEGMENT_REGISTER,
    pub Es: WHV_X64_SEGMENT_REGISTER,
    pub Fs: WHV_X64_SEGMENT_REGISTER,
    pub Gs: WHV_X64_SEGMENT_REGISTER,
    pub Ss: WHV_X64_SEGMENT_REGISTER,
    pub Tr: WHV_X64_SEGMENT_REGISTER,
    pub Ldtr: WHV_X64_SEGMENT_REGISTER,
    pub Idtr: WHV_X64_TABLE_REGISTER,
    pub Gdtr: WHV_X64_TABLE_REGISTER,
    pub Efer: u64,
    pub Cr0: u64,
    pub Cr3: u64,
    pub Cr4: u64,
    pub MsrCrPat: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_INITIAL_VP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_INITIAL_VP_CONTEXT {
    pub Pc: u64,
    pub Sp_ELh: u64,
    pub SCTLR_EL1: u64,
    pub MAIR_EL1: u64,
    pub TCR_EL1: u64,
    pub VBAR_EL1: u64,
    pub TTBR0_EL1: u64,
    pub TTBR1_EL1: u64,
    pub X18: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_INPUT_VTL {
    pub AsUINT8: u8,
    pub Anonymous: WHV_INPUT_VTL_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_INPUT_VTL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_INPUT_VTL_0 {
    pub _bitfield: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_INPUT_VTL_0 {
    pub fn TargetVtl(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_TargetVtl(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn UseTargetVtl(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_UseTargetVtl(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_INTERCEPT_MESSAGE_HEADER {
    pub VpIndex: u32,
    pub InstructionLength: u8,
    pub InterceptAccessType: u8,
    pub ExecutionState: WHV_VP_EXECUTION_STATE,
    pub Pc: u64,
    pub Cpsr: u64,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_INTERCEPT_MESSAGE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_INTERNAL_ACTIVITY_REGISTER {
    pub Anonymous: WHV_INTERNAL_ACTIVITY_REGISTER_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_INTERNAL_ACTIVITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_INTERNAL_ACTIVITY_REGISTER_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_INTERNAL_ACTIVITY_REGISTER_0 {
    pub fn StartupSuspend(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_StartupSuspend(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn HaltSuspend(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_HaltSuspend(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn IdleSuspend(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_IdleSuspend(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(2305843009213693951 << 3)) | ((value & 2305843009213693951) << 3);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_INTERRUPT_CONTROL {
    pub _bitfield: u64,
    pub Destination: u32,
    pub Vector: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_INTERRUPT_CONTROL {
    pub fn Type(&self) -> u64 {
        (self._bitfield << 56) >> 56
    }
    pub fn set_Type(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn DestinationMode(&self) -> u64 {
        (self._bitfield << 52) >> 60
    }
    pub fn set_DestinationMode(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn TriggerMode(&self) -> u64 {
        (self._bitfield << 48) >> 60
    }
    pub fn set_TriggerMode(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
    pub fn TargetVtl(&self) -> u64 {
        (self._bitfield << 40) >> 56
    }
    pub fn set_TargetVtl(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(255 << 16)) | ((value & 255) << 16);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 24
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(1099511627775 << 24)) | ((value & 1099511627775) << 24);
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_INTERRUPT_CONTROL {
    pub TargetPartition: u64,
    pub InterruptControl: WHV_INTERRUPT_CONTROL2,
    pub DestinationAddress: u64,
    pub RequestedVector: u32,
    pub TargetVtl: u8,
    pub ReservedZ0: u8,
    pub ReservedZ1: u16,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_INTERRUPT_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_INTERRUPT_CONTROL2 {
    pub AsUINT64: u64,
    pub Anonymous: WHV_INTERRUPT_CONTROL2_0,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_INTERRUPT_CONTROL2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_INTERRUPT_CONTROL2_0 {
    pub InterruptType: WHV_INTERRUPT_TYPE,
    pub _bitfield: u32,
}
#[cfg(target_arch = "aarch64")]
impl WHV_INTERRUPT_CONTROL2_0 {
    pub fn Reserved1(&self) -> u32 {
        (self._bitfield << 30) >> 30
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn Asserted(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Asserted(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn Retarget(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Retarget(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn Reserved2(&self) -> u32 {
        self._bitfield >> 4
    }
    pub fn set_Reserved2(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_INTERRUPT_DESTINATION_MODE = i32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_INTERRUPT_TRIGGER_MODE = i32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_INTERRUPT_TYPE = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_INTERRUPT_TYPE = i32;
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_INVALID_VP_REGISTER_CONTEXT {
    pub VpIndex: u32,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_MAP_GPA_RANGE_FLAGS = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_MAX_DEVICE_ID_SIZE_IN_CHARS: u32 = 200;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_MEMORY_ACCESS_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub AccessInfo: WHV_MEMORY_ACCESS_INFO,
    pub Gpa: WHV_GUEST_PHYSICAL_ADDRESS,
    pub Gva: WHV_GUEST_VIRTUAL_ADDRESS,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_MEMORY_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_MEMORY_ACCESS_CONTEXT {
    pub Header: WHV_INTERCEPT_MESSAGE_HEADER,
    pub Reserved0: u32,
    pub InstructionByteCount: u8,
    pub AccessInfo: WHV_MEMORY_ACCESS_INFO,
    pub Reserved1: u16,
    pub InstructionBytes: [u8; 4],
    pub Reserved2: u32,
    pub Gva: u64,
    pub Gpa: u64,
    pub Syndrome: u64,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_MEMORY_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_MEMORY_ACCESS_INFO {
    pub Anonymous: WHV_MEMORY_ACCESS_INFO_0,
    pub AsUINT32: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_MEMORY_ACCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_MEMORY_ACCESS_INFO_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_MEMORY_ACCESS_INFO_0 {
    pub fn AccessType(&self) -> u32 {
        (self._bitfield << 30) >> 30
    }
    pub fn set_AccessType(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn GpaUnmapped(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_GpaUnmapped(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn GvaValid(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_GvaValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_MEMORY_ACCESS_INFO {
    pub AsUINT8: u8,
    pub Anonymous: WHV_MEMORY_ACCESS_INFO_0,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_MEMORY_ACCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_MEMORY_ACCESS_INFO_0 {
    pub _bitfield: u8,
}
#[cfg(target_arch = "aarch64")]
impl WHV_MEMORY_ACCESS_INFO_0 {
    pub fn GvaValid(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_GvaValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn GvaGpaValid(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_GvaGpaValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn HypercallOutputPending(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_HypercallOutputPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_MEMORY_ACCESS_TYPE = i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_MEMORY_RANGE_ENTRY {
    pub GuestAddress: u64,
    pub SizeInBytes: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_MSR_ACTION = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_MSR_ACTION_ENTRY {
    pub Index: u32,
    pub ReadAction: u8,
    pub WriteAction: u8,
    pub Reserved: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_NESTED_ENLIGHTENMENTS_CONTROL {
    pub Features: WHV_NESTED_ENLIGHTENMENTS_CONTROL_0,
    pub HypercallControls: WHV_NESTED_ENLIGHTENMENTS_CONTROL_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_NESTED_ENLIGHTENMENTS_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_NESTED_ENLIGHTENMENTS_CONTROL_0 {
    pub AsUINT32: u32,
    pub Anonymous: WHV_NESTED_ENLIGHTENMENTS_CONTROL_0_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_NESTED_ENLIGHTENMENTS_CONTROL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_NESTED_ENLIGHTENMENTS_CONTROL_0_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_NESTED_ENLIGHTENMENTS_CONTROL_0_0 {
    pub fn DirectHypercall(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_DirectHypercall(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn VirtualizationException(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_VirtualizationException(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_NESTED_ENLIGHTENMENTS_CONTROL_1 {
    pub AsUINT32: u32,
    pub Anonymous: WHV_NESTED_ENLIGHTENMENTS_CONTROL_1_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_NESTED_ENLIGHTENMENTS_CONTROL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_NESTED_ENLIGHTENMENTS_CONTROL_1_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_NESTED_ENLIGHTENMENTS_CONTROL_1_0 {
    pub fn InterPartitionCommunication(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_InterPartitionCommunication(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_NESTED_STATE_TYPE = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WHV_NOTIFICATION_PORT_HANDLE(pub *mut core::ffi::c_void);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_NOTIFICATION_PORT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_NOTIFICATION_PORT_PARAMETERS {
    pub NotificationPortType: WHV_NOTIFICATION_PORT_TYPE,
    pub Reserved: u16,
    pub Reserved1: u8,
    pub ConnectionVtl: u8,
    pub Anonymous: WHV_NOTIFICATION_PORT_PARAMETERS_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_NOTIFICATION_PORT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_NOTIFICATION_PORT_PARAMETERS_0 {
    pub Doorbell: WHV_DOORBELL_MATCH_DATA,
    pub Event: WHV_NOTIFICATION_PORT_PARAMETERS_0_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_NOTIFICATION_PORT_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {
    pub ConnectionId: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_NOTIFICATION_PORT_PREFERRED_DURATION_MAX: i32 = -1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WHV_NOTIFICATION_PORT_PROPERTY(pub u64);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_NOTIFICATION_PORT_PROPERTY_CODE = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_NOTIFICATION_PORT_TYPE = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_PARTITION_COUNTER_SET = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WHV_PARTITION_HANDLE(pub *mut core::ffi::c_void);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_PARTITION_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PARTITION_MEMORY_COUNTERS {
    pub Mapped4KPageCount: u64,
    pub Mapped2MPageCount: u64,
    pub Mapped1GPageCount: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_PARTITION_PROPERTY {
    pub ExtendedVmExits: WHV_EXTENDED_VM_EXITS,
    pub ProcessorFeatures: WHV_PROCESSOR_FEATURES,
    pub SyntheticProcessorFeaturesBanks: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS,
    pub ProcessorClFlushSize: u8,
    pub ProcessorCount: u32,
    pub SeparateSecurityDomain: windows_core::BOOL,
    pub NestedVirtualization: windows_core::BOOL,
    pub ProcessorClockFrequency: u64,
    pub ProcessorFeaturesBanks: WHV_PROCESSOR_FEATURES_BANKS,
    pub ReferenceTime: u64,
    pub PrimaryNumaNode: u16,
    pub CpuReserve: u32,
    pub CpuCap: u32,
    pub CpuWeight: u32,
    pub CpuGroupId: u64,
    pub ProcessorFrequencyCap: u32,
    pub AllowDeviceAssignment: windows_core::BOOL,
    pub DisableSmt: windows_core::BOOL,
    pub PhysicalAddressWidth: u32,
    pub ProcessorXsaveFeatures: WHV_PROCESSOR_XSAVE_FEATURES,
    pub CpuidExitList: [u32; 1],
    pub ExceptionExitBitmap: u64,
    pub ApicRemoteRead: windows_core::BOOL,
    pub X64MsrExitBitmap: WHV_X64_MSR_EXIT_BITMAP,
    pub ProcessorPerfmonFeatures: WHV_PROCESSOR_PERFMON_FEATURES,
    pub InterruptClockFrequency: u64,
    pub CpuidResultList: [WHV_X64_CPUID_RESULT; 1],
    pub CpuidResultList2: [WHV_X64_CPUID_RESULT2; 1],
    pub MsrActionList: [WHV_MSR_ACTION_ENTRY; 1],
    pub UnimplementedMsrAction: WHV_MSR_ACTION,
    pub LocalApicEmulationMode: WHV_X64_LOCAL_APIC_EMULATION_MODE,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_PARTITION_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_PARTITION_PROPERTY {
    pub ExtendedVmExits: WHV_EXTENDED_VM_EXITS,
    pub ProcessorFeatures: WHV_PROCESSOR_FEATURES,
    pub SyntheticProcessorFeaturesBanks: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS,
    pub ProcessorClFlushSize: u8,
    pub ProcessorCount: u32,
    pub SeparateSecurityDomain: windows_core::BOOL,
    pub NestedVirtualization: windows_core::BOOL,
    pub ProcessorClockFrequency: u64,
    pub ProcessorFeaturesBanks: WHV_PROCESSOR_FEATURES_BANKS,
    pub ReferenceTime: u64,
    pub PrimaryNumaNode: u16,
    pub CpuReserve: u32,
    pub CpuCap: u32,
    pub CpuWeight: u32,
    pub CpuGroupId: u64,
    pub ProcessorFrequencyCap: u32,
    pub AllowDeviceAssignment: windows_core::BOOL,
    pub DisableSmt: windows_core::BOOL,
    pub PhysicalAddressWidth: u32,
    pub Arm64IcParameters: WHV_ARM64_IC_PARAMETERS,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_PARTITION_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_PARTITION_PROPERTY_CODE = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_PARTITION_PROPERTY_CODE = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_ACTIVITY_COUNTERS {
    pub PageInvalidations: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub ControlRegisterAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub IoInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub HaltInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub CpuidInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub MsrAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub OtherIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub PendingInterrupts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub EmulatedInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub DebugRegisterAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub PageFaultIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub NestedPageFaultIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub Hypercalls: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub RdpmcInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_PROCESSOR_ACTIVITY_COUNTERS {
    pub OtherIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub PendingInterrupts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub NestedPageFaultIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub Hypercalls: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub Reserved: [WHV_PROCESSOR_INTERCEPT_COUNTER; 10],
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_PROCESSOR_ACTIVITY_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_APIC_COUNTERS {
    pub MmioAccessCount: u64,
    pub EoiAccessCount: u64,
    pub TprAccessCount: u64,
    pub SentIpiCount: u64,
    pub SelfIpiCount: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_PROCESSOR_COUNTER_SET = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_PROCESSOR_COUNTER_SET = i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_PROCESSOR_FEATURES {
    pub Anonymous: WHV_PROCESSOR_FEATURES_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_PROCESSOR_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_FEATURES_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_PROCESSOR_FEATURES_0 {
    pub fn Sse3Support(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Sse3Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn LahfSahfSupport(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_LahfSahfSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn Ssse3Support(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Ssse3Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn Sse4_1Support(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Sse4_1Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn Sse4_2Support(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_Sse4_2Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn Sse4aSupport(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_Sse4aSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u64) << 5);
    }
    pub fn XopSupport(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_XopSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u64) << 6);
    }
    pub fn PopCntSupport(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_PopCntSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u64) << 7);
    }
    pub fn Cmpxchg16bSupport(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_Cmpxchg16bSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u64) << 8);
    }
    pub fn Altmovcr8Support(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_Altmovcr8Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u64) << 9);
    }
    pub fn LzcntSupport(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_LzcntSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u64) << 10);
    }
    pub fn MisAlignSseSupport(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_MisAlignSseSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u64) << 11);
    }
    pub fn MmxExtSupport(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_MmxExtSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u64) << 12);
    }
    pub fn Amd3DNowSupport(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_Amd3DNowSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u64) << 13);
    }
    pub fn ExtendedAmd3DNowSupport(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_ExtendedAmd3DNowSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u64) << 14);
    }
    pub fn Page1GbSupport(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_Page1GbSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u64) << 15);
    }
    pub fn AesSupport(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_AesSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u64) << 16);
    }
    pub fn PclmulqdqSupport(&self) -> bool {
        (self._bitfield >> 17) & 1 != 0
    }
    pub fn set_PclmulqdqSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 17)) | ((value as u64) << 17);
    }
    pub fn PcidSupport(&self) -> bool {
        (self._bitfield >> 18) & 1 != 0
    }
    pub fn set_PcidSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 18)) | ((value as u64) << 18);
    }
    pub fn Fma4Support(&self) -> bool {
        (self._bitfield >> 19) & 1 != 0
    }
    pub fn set_Fma4Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value as u64) << 19);
    }
    pub fn F16CSupport(&self) -> bool {
        (self._bitfield >> 20) & 1 != 0
    }
    pub fn set_F16CSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value as u64) << 20);
    }
    pub fn RdRandSupport(&self) -> bool {
        (self._bitfield >> 21) & 1 != 0
    }
    pub fn set_RdRandSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value as u64) << 21);
    }
    pub fn RdWrFsGsSupport(&self) -> bool {
        (self._bitfield >> 22) & 1 != 0
    }
    pub fn set_RdWrFsGsSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value as u64) << 22);
    }
    pub fn SmepSupport(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_SmepSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u64) << 23);
    }
    pub fn EnhancedFastStringSupport(&self) -> bool {
        (self._bitfield >> 24) & 1 != 0
    }
    pub fn set_EnhancedFastStringSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value as u64) << 24);
    }
    pub fn Bmi1Support(&self) -> bool {
        (self._bitfield >> 25) & 1 != 0
    }
    pub fn set_Bmi1Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value as u64) << 25);
    }
    pub fn Bmi2Support(&self) -> bool {
        (self._bitfield >> 26) & 1 != 0
    }
    pub fn set_Bmi2Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 26)) | ((value as u64) << 26);
    }
    pub fn Reserved1(&self) -> u64 {
        (self._bitfield << 35) >> 62
    }
    pub fn set_Reserved1(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(3 << 27)) | ((value & 3) << 27);
    }
    pub fn MovbeSupport(&self) -> bool {
        (self._bitfield >> 29) & 1 != 0
    }
    pub fn set_MovbeSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 29)) | ((value as u64) << 29);
    }
    pub fn Npiep1Support(&self) -> bool {
        (self._bitfield >> 30) & 1 != 0
    }
    pub fn set_Npiep1Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value as u64) << 30);
    }
    pub fn DepX87FPUSaveSupport(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_DepX87FPUSaveSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u64) << 31);
    }
    pub fn RdSeedSupport(&self) -> bool {
        (self._bitfield >> 32) & 1 != 0
    }
    pub fn set_RdSeedSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 32)) | ((value as u64) << 32);
    }
    pub fn AdxSupport(&self) -> bool {
        (self._bitfield >> 33) & 1 != 0
    }
    pub fn set_AdxSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 33)) | ((value as u64) << 33);
    }
    pub fn IntelPrefetchSupport(&self) -> bool {
        (self._bitfield >> 34) & 1 != 0
    }
    pub fn set_IntelPrefetchSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 34)) | ((value as u64) << 34);
    }
    pub fn SmapSupport(&self) -> bool {
        (self._bitfield >> 35) & 1 != 0
    }
    pub fn set_SmapSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 35)) | ((value as u64) << 35);
    }
    pub fn HleSupport(&self) -> bool {
        (self._bitfield >> 36) & 1 != 0
    }
    pub fn set_HleSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 36)) | ((value as u64) << 36);
    }
    pub fn RtmSupport(&self) -> bool {
        (self._bitfield >> 37) & 1 != 0
    }
    pub fn set_RtmSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 37)) | ((value as u64) << 37);
    }
    pub fn RdtscpSupport(&self) -> bool {
        (self._bitfield >> 38) & 1 != 0
    }
    pub fn set_RdtscpSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 38)) | ((value as u64) << 38);
    }
    pub fn ClflushoptSupport(&self) -> bool {
        (self._bitfield >> 39) & 1 != 0
    }
    pub fn set_ClflushoptSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 39)) | ((value as u64) << 39);
    }
    pub fn ClwbSupport(&self) -> bool {
        (self._bitfield >> 40) & 1 != 0
    }
    pub fn set_ClwbSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 40)) | ((value as u64) << 40);
    }
    pub fn ShaSupport(&self) -> bool {
        (self._bitfield >> 41) & 1 != 0
    }
    pub fn set_ShaSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 41)) | ((value as u64) << 41);
    }
    pub fn X87PointersSavedSupport(&self) -> bool {
        (self._bitfield >> 42) & 1 != 0
    }
    pub fn set_X87PointersSavedSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 42)) | ((value as u64) << 42);
    }
    pub fn InvpcidSupport(&self) -> bool {
        (self._bitfield >> 43) & 1 != 0
    }
    pub fn set_InvpcidSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 43)) | ((value as u64) << 43);
    }
    pub fn IbrsSupport(&self) -> bool {
        (self._bitfield >> 44) & 1 != 0
    }
    pub fn set_IbrsSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 44)) | ((value as u64) << 44);
    }
    pub fn StibpSupport(&self) -> bool {
        (self._bitfield >> 45) & 1 != 0
    }
    pub fn set_StibpSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 45)) | ((value as u64) << 45);
    }
    pub fn IbpbSupport(&self) -> bool {
        (self._bitfield >> 46) & 1 != 0
    }
    pub fn set_IbpbSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 46)) | ((value as u64) << 46);
    }
    pub fn UnrestrictedGuestSupport(&self) -> bool {
        (self._bitfield >> 47) & 1 != 0
    }
    pub fn set_UnrestrictedGuestSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 47)) | ((value as u64) << 47);
    }
    pub fn SsbdSupport(&self) -> bool {
        (self._bitfield >> 48) & 1 != 0
    }
    pub fn set_SsbdSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 48)) | ((value as u64) << 48);
    }
    pub fn FastShortRepMovSupport(&self) -> bool {
        (self._bitfield >> 49) & 1 != 0
    }
    pub fn set_FastShortRepMovSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 49)) | ((value as u64) << 49);
    }
    pub fn Reserved3(&self) -> bool {
        (self._bitfield >> 50) & 1 != 0
    }
    pub fn set_Reserved3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 50)) | ((value as u64) << 50);
    }
    pub fn RdclNo(&self) -> bool {
        (self._bitfield >> 51) & 1 != 0
    }
    pub fn set_RdclNo(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 51)) | ((value as u64) << 51);
    }
    pub fn IbrsAllSupport(&self) -> bool {
        (self._bitfield >> 52) & 1 != 0
    }
    pub fn set_IbrsAllSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 52)) | ((value as u64) << 52);
    }
    pub fn Reserved4(&self) -> bool {
        (self._bitfield >> 53) & 1 != 0
    }
    pub fn set_Reserved4(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 53)) | ((value as u64) << 53);
    }
    pub fn SsbNo(&self) -> bool {
        (self._bitfield >> 54) & 1 != 0
    }
    pub fn set_SsbNo(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 54)) | ((value as u64) << 54);
    }
    pub fn RsbANo(&self) -> bool {
        (self._bitfield >> 55) & 1 != 0
    }
    pub fn set_RsbANo(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 55)) | ((value as u64) << 55);
    }
    pub fn Reserved5(&self) -> bool {
        (self._bitfield >> 56) & 1 != 0
    }
    pub fn set_Reserved5(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 56)) | ((value as u64) << 56);
    }
    pub fn RdPidSupport(&self) -> bool {
        (self._bitfield >> 57) & 1 != 0
    }
    pub fn set_RdPidSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 57)) | ((value as u64) << 57);
    }
    pub fn UmipSupport(&self) -> bool {
        (self._bitfield >> 58) & 1 != 0
    }
    pub fn set_UmipSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 58)) | ((value as u64) << 58);
    }
    pub fn MdsNoSupport(&self) -> bool {
        (self._bitfield >> 59) & 1 != 0
    }
    pub fn set_MdsNoSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 59)) | ((value as u64) << 59);
    }
    pub fn MdClearSupport(&self) -> bool {
        (self._bitfield >> 60) & 1 != 0
    }
    pub fn set_MdClearSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 60)) | ((value as u64) << 60);
    }
    pub fn TaaNoSupport(&self) -> bool {
        (self._bitfield >> 61) & 1 != 0
    }
    pub fn set_TaaNoSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 61)) | ((value as u64) << 61);
    }
    pub fn TsxCtrlSupport(&self) -> bool {
        (self._bitfield >> 62) & 1 != 0
    }
    pub fn set_TsxCtrlSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 62)) | ((value as u64) << 62);
    }
    pub fn Reserved6(&self) -> bool {
        (self._bitfield >> 63) & 1 != 0
    }
    pub fn set_Reserved6(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 63)) | ((value as u64) << 63);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_PROCESSOR_FEATURES1 {
    pub Anonymous: WHV_PROCESSOR_FEATURES1_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_PROCESSOR_FEATURES1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_FEATURES1_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_PROCESSOR_FEATURES1_0 {
    pub fn ACountMCountSupport(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ACountMCountSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn TscInvariantSupport(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_TscInvariantSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn ClZeroSupport(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ClZeroSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn RdpruSupport(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_RdpruSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn La57Support(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_La57Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn MbecSupport(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_MbecSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u64) << 5);
    }
    pub fn NestedVirtSupport(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_NestedVirtSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u64) << 6);
    }
    pub fn PsfdSupport(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_PsfdSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u64) << 7);
    }
    pub fn CetSsSupport(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_CetSsSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u64) << 8);
    }
    pub fn CetIbtSupport(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_CetIbtSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u64) << 9);
    }
    pub fn VmxExceptionInjectSupport(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_VmxExceptionInjectSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u64) << 10);
    }
    pub fn Reserved2(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_Reserved2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u64) << 11);
    }
    pub fn UmwaitTpauseSupport(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_UmwaitTpauseSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u64) << 12);
    }
    pub fn MovdiriSupport(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_MovdiriSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u64) << 13);
    }
    pub fn Movdir64bSupport(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_Movdir64bSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u64) << 14);
    }
    pub fn CldemoteSupport(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_CldemoteSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u64) << 15);
    }
    pub fn SerializeSupport(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_SerializeSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u64) << 16);
    }
    pub fn TscDeadlineTmrSupport(&self) -> bool {
        (self._bitfield >> 17) & 1 != 0
    }
    pub fn set_TscDeadlineTmrSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 17)) | ((value as u64) << 17);
    }
    pub fn TscAdjustSupport(&self) -> bool {
        (self._bitfield >> 18) & 1 != 0
    }
    pub fn set_TscAdjustSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 18)) | ((value as u64) << 18);
    }
    pub fn FZLRepMovsb(&self) -> bool {
        (self._bitfield >> 19) & 1 != 0
    }
    pub fn set_FZLRepMovsb(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value as u64) << 19);
    }
    pub fn FSRepStosb(&self) -> bool {
        (self._bitfield >> 20) & 1 != 0
    }
    pub fn set_FSRepStosb(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value as u64) << 20);
    }
    pub fn FSRepCmpsb(&self) -> bool {
        (self._bitfield >> 21) & 1 != 0
    }
    pub fn set_FSRepCmpsb(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value as u64) << 21);
    }
    pub fn TsxLdTrkSupport(&self) -> bool {
        (self._bitfield >> 22) & 1 != 0
    }
    pub fn set_TsxLdTrkSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value as u64) << 22);
    }
    pub fn VmxInsOutsExitInfoSupport(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_VmxInsOutsExitInfoSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u64) << 23);
    }
    pub fn Reserved3(&self) -> bool {
        (self._bitfield >> 24) & 1 != 0
    }
    pub fn set_Reserved3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value as u64) << 24);
    }
    pub fn SbdrSsdpNoSupport(&self) -> bool {
        (self._bitfield >> 25) & 1 != 0
    }
    pub fn set_SbdrSsdpNoSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value as u64) << 25);
    }
    pub fn FbsdpNoSupport(&self) -> bool {
        (self._bitfield >> 26) & 1 != 0
    }
    pub fn set_FbsdpNoSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 26)) | ((value as u64) << 26);
    }
    pub fn PsdpNoSupport(&self) -> bool {
        (self._bitfield >> 27) & 1 != 0
    }
    pub fn set_PsdpNoSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 27)) | ((value as u64) << 27);
    }
    pub fn FbClearSupport(&self) -> bool {
        (self._bitfield >> 28) & 1 != 0
    }
    pub fn set_FbClearSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 28)) | ((value as u64) << 28);
    }
    pub fn BtcNoSupport(&self) -> bool {
        (self._bitfield >> 29) & 1 != 0
    }
    pub fn set_BtcNoSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 29)) | ((value as u64) << 29);
    }
    pub fn IbpbRsbFlushSupport(&self) -> bool {
        (self._bitfield >> 30) & 1 != 0
    }
    pub fn set_IbpbRsbFlushSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value as u64) << 30);
    }
    pub fn StibpAlwaysOnSupport(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_StibpAlwaysOnSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u64) << 31);
    }
    pub fn PerfGlobalCtrlSupport(&self) -> bool {
        (self._bitfield >> 32) & 1 != 0
    }
    pub fn set_PerfGlobalCtrlSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 32)) | ((value as u64) << 32);
    }
    pub fn NptExecuteOnlySupport(&self) -> bool {
        (self._bitfield >> 33) & 1 != 0
    }
    pub fn set_NptExecuteOnlySupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 33)) | ((value as u64) << 33);
    }
    pub fn NptADFlagsSupport(&self) -> bool {
        (self._bitfield >> 34) & 1 != 0
    }
    pub fn set_NptADFlagsSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 34)) | ((value as u64) << 34);
    }
    pub fn Npt1GbPageSupport(&self) -> bool {
        (self._bitfield >> 35) & 1 != 0
    }
    pub fn set_Npt1GbPageSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 35)) | ((value as u64) << 35);
    }
    pub fn Reserved4(&self) -> bool {
        (self._bitfield >> 36) & 1 != 0
    }
    pub fn set_Reserved4(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 36)) | ((value as u64) << 36);
    }
    pub fn Reserved5(&self) -> bool {
        (self._bitfield >> 37) & 1 != 0
    }
    pub fn set_Reserved5(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 37)) | ((value as u64) << 37);
    }
    pub fn Reserved6(&self) -> bool {
        (self._bitfield >> 38) & 1 != 0
    }
    pub fn set_Reserved6(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 38)) | ((value as u64) << 38);
    }
    pub fn Reserved7(&self) -> bool {
        (self._bitfield >> 39) & 1 != 0
    }
    pub fn set_Reserved7(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 39)) | ((value as u64) << 39);
    }
    pub fn CmpccxaddSupport(&self) -> bool {
        (self._bitfield >> 40) & 1 != 0
    }
    pub fn set_CmpccxaddSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 40)) | ((value as u64) << 40);
    }
    pub fn Reserved8(&self) -> bool {
        (self._bitfield >> 41) & 1 != 0
    }
    pub fn set_Reserved8(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 41)) | ((value as u64) << 41);
    }
    pub fn Reserved9(&self) -> bool {
        (self._bitfield >> 42) & 1 != 0
    }
    pub fn set_Reserved9(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 42)) | ((value as u64) << 42);
    }
    pub fn Reserved10(&self) -> bool {
        (self._bitfield >> 43) & 1 != 0
    }
    pub fn set_Reserved10(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 43)) | ((value as u64) << 43);
    }
    pub fn Reserved11(&self) -> bool {
        (self._bitfield >> 44) & 1 != 0
    }
    pub fn set_Reserved11(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 44)) | ((value as u64) << 44);
    }
    pub fn PrefetchISupport(&self) -> bool {
        (self._bitfield >> 45) & 1 != 0
    }
    pub fn set_PrefetchISupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 45)) | ((value as u64) << 45);
    }
    pub fn Sha512Support(&self) -> bool {
        (self._bitfield >> 46) & 1 != 0
    }
    pub fn set_Sha512Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 46)) | ((value as u64) << 46);
    }
    pub fn Reserved12(&self) -> bool {
        (self._bitfield >> 47) & 1 != 0
    }
    pub fn set_Reserved12(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 47)) | ((value as u64) << 47);
    }
    pub fn Reserved13(&self) -> bool {
        (self._bitfield >> 48) & 1 != 0
    }
    pub fn set_Reserved13(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 48)) | ((value as u64) << 48);
    }
    pub fn Reserved14(&self) -> bool {
        (self._bitfield >> 49) & 1 != 0
    }
    pub fn set_Reserved14(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 49)) | ((value as u64) << 49);
    }
    pub fn SM3Support(&self) -> bool {
        (self._bitfield >> 50) & 1 != 0
    }
    pub fn set_SM3Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 50)) | ((value as u64) << 50);
    }
    pub fn SM4Support(&self) -> bool {
        (self._bitfield >> 51) & 1 != 0
    }
    pub fn set_SM4Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 51)) | ((value as u64) << 51);
    }
    pub fn Reserved15(&self) -> bool {
        (self._bitfield >> 52) & 1 != 0
    }
    pub fn set_Reserved15(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 52)) | ((value as u64) << 52);
    }
    pub fn Reserved16(&self) -> bool {
        (self._bitfield >> 53) & 1 != 0
    }
    pub fn set_Reserved16(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 53)) | ((value as u64) << 53);
    }
    pub fn SbpbSupported(&self) -> bool {
        (self._bitfield >> 54) & 1 != 0
    }
    pub fn set_SbpbSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 54)) | ((value as u64) << 54);
    }
    pub fn IbpbBrTypeSupported(&self) -> bool {
        (self._bitfield >> 55) & 1 != 0
    }
    pub fn set_IbpbBrTypeSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 55)) | ((value as u64) << 55);
    }
    pub fn SrsoNoSupported(&self) -> bool {
        (self._bitfield >> 56) & 1 != 0
    }
    pub fn set_SrsoNoSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 56)) | ((value as u64) << 56);
    }
    pub fn SrsoUserKernelNoSupported(&self) -> bool {
        (self._bitfield >> 57) & 1 != 0
    }
    pub fn set_SrsoUserKernelNoSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 57)) | ((value as u64) << 57);
    }
    pub fn Reserved17(&self) -> bool {
        (self._bitfield >> 58) & 1 != 0
    }
    pub fn set_Reserved17(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 58)) | ((value as u64) << 58);
    }
    pub fn Reserved18(&self) -> bool {
        (self._bitfield >> 59) & 1 != 0
    }
    pub fn set_Reserved18(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 59)) | ((value as u64) << 59);
    }
    pub fn Reserved19(&self) -> bool {
        (self._bitfield >> 60) & 1 != 0
    }
    pub fn set_Reserved19(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 60)) | ((value as u64) << 60);
    }
    pub fn LassSupport(&self) -> bool {
        (self._bitfield >> 61) & 1 != 0
    }
    pub fn set_LassSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 61)) | ((value as u64) << 61);
    }
    pub fn IdleHltInterceptSupport(&self) -> bool {
        (self._bitfield >> 62) & 1 != 0
    }
    pub fn set_IdleHltInterceptSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 62)) | ((value as u64) << 62);
    }
    pub fn MsrListSupport(&self) -> bool {
        (self._bitfield >> 63) & 1 != 0
    }
    pub fn set_MsrListSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 63)) | ((value as u64) << 63);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_PROCESSOR_FEATURES_BANKS {
    pub BanksCount: u32,
    pub Reserved0: u32,
    pub Anonymous: WHV_PROCESSOR_FEATURES_BANKS_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_PROCESSOR_FEATURES_BANKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_PROCESSOR_FEATURES_BANKS_0 {
    pub Anonymous: WHV_PROCESSOR_FEATURES_BANKS_0_0,
    pub AsUINT64: [u64; 2],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_PROCESSOR_FEATURES_BANKS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_PROCESSOR_FEATURES_BANKS_0_0 {
    pub Bank0: WHV_PROCESSOR_FEATURES,
    pub Bank1: WHV_PROCESSOR_FEATURES1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_PROCESSOR_FEATURES_BANKS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 2;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_GUEST_EVENT_COUNTERS {
    pub PageFaultCount: u64,
    pub ExceptionCount: u64,
    pub InterruptCount: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_INTERCEPT_COUNTER {
    pub Count: u64,
    pub Time100ns: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_PROCESSOR_PERFMON_FEATURES {
    pub Anonymous: WHV_PROCESSOR_PERFMON_FEATURES_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_PROCESSOR_PERFMON_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_PERFMON_FEATURES_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_PROCESSOR_PERFMON_FEATURES_0 {
    pub fn PmuSupport(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_PmuSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn LbrSupport(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_LbrSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(4611686018427387903 << 2)) | ((value & 4611686018427387903) << 2);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_RUNTIME_COUNTERS {
    pub TotalRuntime100ns: u64,
    pub HypervisorRuntime100ns: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    pub SyntheticInterruptsCount: u64,
    pub LongSpinWaitHypercallsCount: u64,
    pub OtherHypercallsCount: u64,
    pub SyntheticInterruptHypercallsCount: u64,
    pub VirtualInterruptHypercallsCount: u64,
    pub VirtualMmuHypercallsCount: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_PROCESSOR_VENDOR = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_PROCESSOR_XSAVE_FEATURES {
    pub Anonymous: WHV_PROCESSOR_XSAVE_FEATURES_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_PROCESSOR_XSAVE_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_PROCESSOR_XSAVE_FEATURES_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_PROCESSOR_XSAVE_FEATURES_0 {
    pub fn XsaveSupport(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_XsaveSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn XsaveoptSupport(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_XsaveoptSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn AvxSupport(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_AvxSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn Avx2Support(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Avx2Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn FmaSupport(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_FmaSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn MpxSupport(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_MpxSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u64) << 5);
    }
    pub fn Avx512Support(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_Avx512Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u64) << 6);
    }
    pub fn Avx512DQSupport(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Avx512DQSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u64) << 7);
    }
    pub fn Avx512CDSupport(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_Avx512CDSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u64) << 8);
    }
    pub fn Avx512BWSupport(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_Avx512BWSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u64) << 9);
    }
    pub fn Avx512VLSupport(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_Avx512VLSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u64) << 10);
    }
    pub fn XsaveCompSupport(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_XsaveCompSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u64) << 11);
    }
    pub fn XsaveSupervisorSupport(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_XsaveSupervisorSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u64) << 12);
    }
    pub fn Xcr1Support(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_Xcr1Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u64) << 13);
    }
    pub fn Avx512BitalgSupport(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_Avx512BitalgSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u64) << 14);
    }
    pub fn Avx512IfmaSupport(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_Avx512IfmaSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u64) << 15);
    }
    pub fn Avx512VBmiSupport(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_Avx512VBmiSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u64) << 16);
    }
    pub fn Avx512VBmi2Support(&self) -> bool {
        (self._bitfield >> 17) & 1 != 0
    }
    pub fn set_Avx512VBmi2Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 17)) | ((value as u64) << 17);
    }
    pub fn Avx512VnniSupport(&self) -> bool {
        (self._bitfield >> 18) & 1 != 0
    }
    pub fn set_Avx512VnniSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 18)) | ((value as u64) << 18);
    }
    pub fn GfniSupport(&self) -> bool {
        (self._bitfield >> 19) & 1 != 0
    }
    pub fn set_GfniSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value as u64) << 19);
    }
    pub fn VaesSupport(&self) -> bool {
        (self._bitfield >> 20) & 1 != 0
    }
    pub fn set_VaesSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value as u64) << 20);
    }
    pub fn Avx512VPopcntdqSupport(&self) -> bool {
        (self._bitfield >> 21) & 1 != 0
    }
    pub fn set_Avx512VPopcntdqSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value as u64) << 21);
    }
    pub fn VpclmulqdqSupport(&self) -> bool {
        (self._bitfield >> 22) & 1 != 0
    }
    pub fn set_VpclmulqdqSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value as u64) << 22);
    }
    pub fn Avx512Bf16Support(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_Avx512Bf16Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u64) << 23);
    }
    pub fn Avx512Vp2IntersectSupport(&self) -> bool {
        (self._bitfield >> 24) & 1 != 0
    }
    pub fn set_Avx512Vp2IntersectSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value as u64) << 24);
    }
    pub fn Avx512Fp16Support(&self) -> bool {
        (self._bitfield >> 25) & 1 != 0
    }
    pub fn set_Avx512Fp16Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value as u64) << 25);
    }
    pub fn XfdSupport(&self) -> bool {
        (self._bitfield >> 26) & 1 != 0
    }
    pub fn set_XfdSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 26)) | ((value as u64) << 26);
    }
    pub fn AmxTileSupport(&self) -> bool {
        (self._bitfield >> 27) & 1 != 0
    }
    pub fn set_AmxTileSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 27)) | ((value as u64) << 27);
    }
    pub fn AmxBf16Support(&self) -> bool {
        (self._bitfield >> 28) & 1 != 0
    }
    pub fn set_AmxBf16Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 28)) | ((value as u64) << 28);
    }
    pub fn AmxInt8Support(&self) -> bool {
        (self._bitfield >> 29) & 1 != 0
    }
    pub fn set_AmxInt8Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 29)) | ((value as u64) << 29);
    }
    pub fn AvxVnniSupport(&self) -> bool {
        (self._bitfield >> 30) & 1 != 0
    }
    pub fn set_AvxVnniSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value as u64) << 30);
    }
    pub fn AvxIfmaSupport(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_AvxIfmaSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u64) << 31);
    }
    pub fn AvxNeConvertSupport(&self) -> bool {
        (self._bitfield >> 32) & 1 != 0
    }
    pub fn set_AvxNeConvertSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 32)) | ((value as u64) << 32);
    }
    pub fn AvxVnniInt8Support(&self) -> bool {
        (self._bitfield >> 33) & 1 != 0
    }
    pub fn set_AvxVnniInt8Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 33)) | ((value as u64) << 33);
    }
    pub fn AvxVnniInt16Support(&self) -> bool {
        (self._bitfield >> 34) & 1 != 0
    }
    pub fn set_AvxVnniInt16Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 34)) | ((value as u64) << 34);
    }
    pub fn Avx10_1_256Support(&self) -> bool {
        (self._bitfield >> 35) & 1 != 0
    }
    pub fn set_Avx10_1_256Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 35)) | ((value as u64) << 35);
    }
    pub fn Avx10_1_512Support(&self) -> bool {
        (self._bitfield >> 36) & 1 != 0
    }
    pub fn set_Avx10_1_512Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 36)) | ((value as u64) << 36);
    }
    pub fn AmxFp16Support(&self) -> bool {
        (self._bitfield >> 37) & 1 != 0
    }
    pub fn set_AmxFp16Support(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 37)) | ((value as u64) << 37);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 38
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(67108863 << 38)) | ((value & 67108863) << 38);
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_READ_WRITE_GPA_RANGE_MAX_SIZE: u32 = 16;
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_REGISTER_ACCESS_INFO {
    pub SourceValue: WHV_REGISTER_VALUE,
    pub DestinationRegister: WHV_REGISTER_NAME,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_REGISTER_ACCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_REGISTER_CONTEXT {
    pub Header: WHV_INTERCEPT_MESSAGE_HEADER,
    pub Anonymous: WHV_REGISTER_CONTEXT_0,
    pub Reserved8: u8,
    pub Reserved16: u16,
    pub RegisterName: WHV_REGISTER_NAME,
    pub AccessInfo: WHV_REGISTER_ACCESS_INFO,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_REGISTER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_REGISTER_CONTEXT_0 {
    pub _bitfield: u8,
}
#[cfg(target_arch = "aarch64")]
impl WHV_REGISTER_CONTEXT_0 {
    pub fn IsMemoryOp(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_IsMemoryOp(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_REGISTER_NAME = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_REGISTER_NAME = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_REGISTER_VALUE {
    pub Reg128: WHV_UINT128,
    pub Reg64: u64,
    pub Reg32: u32,
    pub Reg16: u16,
    pub Reg8: u8,
    pub InternalActivity: WHV_INTERNAL_ACTIVITY_REGISTER,
    pub DeliverabilityNotifications: WHV_DELIVERABILITY_NOTIFICATIONS_REGISTER,
    pub Fp: WHV_X64_FP_REGISTER,
    pub FpControlStatus: WHV_X64_FP_CONTROL_STATUS_REGISTER,
    pub XmmControlStatus: WHV_X64_XMM_CONTROL_STATUS_REGISTER,
    pub Segment: WHV_X64_SEGMENT_REGISTER,
    pub Table: WHV_X64_TABLE_REGISTER,
    pub InterruptState: WHV_X64_INTERRUPT_STATE_REGISTER,
    pub PendingInterruption: WHV_X64_PENDING_INTERRUPTION_REGISTER,
    pub ExceptionEvent: WHV_X64_PENDING_EXCEPTION_EVENT,
    pub ExtIntEvent: WHV_X64_PENDING_EXT_INT_EVENT,
    pub PendingDebugException: WHV_X64_PENDING_DEBUG_EXCEPTION,
    pub NestedState: WHV_X64_NESTED_GUEST_STATE,
    pub InvEpt: WHV_X64_NESTED_INVEPT_REGISTER,
    pub InvVpid: WHV_X64_NESTED_INVVPID_REGISTER,
    pub SvmNestedExit0: WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT0,
    pub SvmNestedExit1: WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT1,
    pub SvmNestedExit2: WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT2,
    pub SvmNestedExit3: WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT3,
    pub VmxNestedExit0: WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT0,
    pub VmxNestedExit1: WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT1,
    pub VmxNestedExit2: WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT2,
    pub VmxNestedExit3: WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT3,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_REGISTER_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_REGISTER_VALUE {
    pub Reg128: WHV_UINT128,
    pub Reg64: u64,
    pub Reg32: u32,
    pub Reg16: u16,
    pub Reg8: u8,
    pub InternalActivity: WHV_INTERNAL_ACTIVITY_REGISTER,
    pub DeliverabilityNotifications: WHV_DELIVERABILITY_NOTIFICATIONS_REGISTER,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_REGISTER_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_RUN_VP_CANCELED_CONTEXT {
    pub CancelReason: WHV_RUN_VP_CANCEL_REASON,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_RUN_VP_CANCEL_REASON = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_RUN_VP_EXIT_CONTEXT {
    pub ExitReason: WHV_RUN_VP_EXIT_REASON,
    pub Reserved: u32,
    pub VpContext: WHV_VP_EXIT_CONTEXT,
    pub Anonymous: WHV_RUN_VP_EXIT_CONTEXT_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_RUN_VP_EXIT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_RUN_VP_EXIT_CONTEXT_0 {
    pub MemoryAccess: WHV_MEMORY_ACCESS_CONTEXT,
    pub CancelReason: WHV_RUN_VP_CANCELED_CONTEXT,
    pub Hypercall: WHV_HYPERCALL_CONTEXT,
    pub SynicSintDeliverable: WHV_SYNIC_SINT_DELIVERABLE_CONTEXT,
    pub IoPortAccess: WHV_X64_IO_PORT_ACCESS_CONTEXT,
    pub MsrAccess: WHV_X64_MSR_ACCESS_CONTEXT,
    pub CpuidAccess: WHV_X64_CPUID_ACCESS_CONTEXT,
    pub VpException: WHV_VP_EXCEPTION_CONTEXT,
    pub InterruptWindow: WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT,
    pub UnsupportedFeature: WHV_X64_UNSUPPORTED_FEATURE_CONTEXT,
    pub ApicEoi: WHV_X64_APIC_EOI_CONTEXT,
    pub ReadTsc: WHV_X64_RDTSC_CONTEXT,
    pub ApicSmi: WHV_X64_APIC_SMI_CONTEXT,
    pub ApicInitSipi: WHV_X64_APIC_INIT_SIPI_CONTEXT,
    pub ApicWrite: WHV_X64_APIC_WRITE_CONTEXT,
    pub AsUINT64: [u64; 22],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_RUN_VP_EXIT_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_RUN_VP_EXIT_CONTEXT {
    pub ExitReason: WHV_RUN_VP_EXIT_REASON,
    pub Reserved: u32,
    pub Reserved1: u64,
    pub Anonymous: WHV_RUN_VP_EXIT_CONTEXT_0,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_RUN_VP_EXIT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_RUN_VP_EXIT_CONTEXT_0 {
    pub MemoryAccess: WHV_MEMORY_ACCESS_CONTEXT,
    pub CancelReason: WHV_RUN_VP_CANCELED_CONTEXT,
    pub Hypercall: WHV_HYPERCALL_CONTEXT,
    pub SynicSintDeliverable: WHV_SYNIC_SINT_DELIVERABLE_CONTEXT,
    pub UnrecoverableException: WHV_UNRECOVERABLE_EXCEPTION_CONTEXT,
    pub InvalidVpRegister: WHV_INVALID_VP_REGISTER_CONTEXT,
    pub Register: WHV_REGISTER_CONTEXT,
    pub Arm64Reset: WHV_ARM64_RESET_CONTEXT,
    pub AsUINT64: [u64; 32],
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_RUN_VP_EXIT_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_RUN_VP_EXIT_REASON = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_RUN_VP_EXIT_REASON = i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_SCHEDULER_FEATURES {
    pub Anonymous: WHV_SCHEDULER_FEATURES_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_SCHEDULER_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_SCHEDULER_FEATURES_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_SCHEDULER_FEATURES_0 {
    pub fn CpuReserve(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CpuReserve(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn CpuCap(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_CpuCap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn CpuWeight(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_CpuWeight(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn CpuGroupId(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_CpuGroupId(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn DisableSmt(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_DisableSmt(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(576460752303423487 << 5)) | ((value & 576460752303423487) << 5);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_SRIOV_RESOURCE_DESCRIPTOR {
    pub PnpInstanceId: [u16; 200],
    pub VirtualFunctionId: super::LUID,
    pub VirtualFunctionIndex: u16,
    pub Reserved: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for WHV_SRIOV_RESOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_SVM_NESTED_HOST_STATE {
    pub Rip: u64,
    pub Rsp: u64,
    pub Rflags: u64,
    pub Rax: u64,
    pub Es: WHV_SVM_VMCB_SELECTOR,
    pub Cs: WHV_SVM_VMCB_SELECTOR,
    pub Ss: WHV_SVM_VMCB_SELECTOR,
    pub Ds: WHV_SVM_VMCB_SELECTOR,
    pub Gdtr: WHV_SVM_VMCB_SELECTOR,
    pub Idtr: WHV_SVM_VMCB_SELECTOR,
    pub Efer: u64,
    pub Cr0: u64,
    pub Cr3: u64,
    pub Cr4: u64,
    pub VirtualTpr: u64,
    pub Reserved: [u64; 6],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_SVM_NESTED_HOST_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct WHV_SVM_VMCB_SELECTOR {
    pub Selector: u16,
    pub Attrib: u16,
    pub Limit: u32,
    pub Base: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_SYNIC_EVENT_PARAMETERS {
    pub VpIndex: u32,
    pub TargetSint: u8,
    pub TargetVtl: WHV_VTL,
    pub FlagNumber: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_SYNIC_MESSAGE_SIZE: u32 = 256;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    pub DeliverableSints: u16,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    pub Header: WHV_INTERCEPT_MESSAGE_HEADER,
    pub DeliverableSints: u16,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_SYNTHETIC_PROCESSOR_FEATURES {
    pub Anonymous: WHV_SYNTHETIC_PROCESSOR_FEATURES_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_SYNTHETIC_PROCESSOR_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {
    pub fn HypervisorPresent(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_HypervisorPresent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn Hv1(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_Hv1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn AccessVpRunTimeReg(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_AccessVpRunTimeReg(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn AccessPartitionReferenceCounter(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_AccessPartitionReferenceCounter(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn AccessSynicRegs(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_AccessSynicRegs(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn AccessSyntheticTimerRegs(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_AccessSyntheticTimerRegs(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u64) << 5);
    }
    pub fn AccessIntrCtrlRegs(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_AccessIntrCtrlRegs(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u64) << 6);
    }
    pub fn AccessHypercallRegs(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_AccessHypercallRegs(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u64) << 7);
    }
    pub fn AccessVpIndex(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_AccessVpIndex(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u64) << 8);
    }
    pub fn AccessPartitionReferenceTsc(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_AccessPartitionReferenceTsc(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u64) << 9);
    }
    pub fn AccessGuestIdleReg(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_AccessGuestIdleReg(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u64) << 10);
    }
    pub fn AccessFrequencyRegs(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_AccessFrequencyRegs(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u64) << 11);
    }
    pub fn ReservedZ12(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_ReservedZ12(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u64) << 12);
    }
    pub fn ReservedZ13(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_ReservedZ13(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u64) << 13);
    }
    pub fn ReservedZ14(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_ReservedZ14(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u64) << 14);
    }
    pub fn EnableExtendedGvaRangesForFlushVirtualAddressList(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_EnableExtendedGvaRangesForFlushVirtualAddressList(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u64) << 15);
    }
    pub fn ReservedZ16(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_ReservedZ16(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u64) << 16);
    }
    pub fn ReservedZ17(&self) -> bool {
        (self._bitfield >> 17) & 1 != 0
    }
    pub fn set_ReservedZ17(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 17)) | ((value as u64) << 17);
    }
    pub fn FastHypercallOutput(&self) -> bool {
        (self._bitfield >> 18) & 1 != 0
    }
    pub fn set_FastHypercallOutput(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 18)) | ((value as u64) << 18);
    }
    pub fn ReservedZ19(&self) -> bool {
        (self._bitfield >> 19) & 1 != 0
    }
    pub fn set_ReservedZ19(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value as u64) << 19);
    }
    pub fn ReservedZ20(&self) -> bool {
        (self._bitfield >> 20) & 1 != 0
    }
    pub fn set_ReservedZ20(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value as u64) << 20);
    }
    pub fn ReservedZ21(&self) -> bool {
        (self._bitfield >> 21) & 1 != 0
    }
    pub fn set_ReservedZ21(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value as u64) << 21);
    }
    pub fn DirectSyntheticTimers(&self) -> bool {
        (self._bitfield >> 22) & 1 != 0
    }
    pub fn set_DirectSyntheticTimers(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value as u64) << 22);
    }
    pub fn ReservedZ23(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_ReservedZ23(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u64) << 23);
    }
    pub fn ExtendedProcessorMasks(&self) -> bool {
        (self._bitfield >> 24) & 1 != 0
    }
    pub fn set_ExtendedProcessorMasks(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value as u64) << 24);
    }
    pub fn TbFlushHypercalls(&self) -> bool {
        (self._bitfield >> 25) & 1 != 0
    }
    pub fn set_TbFlushHypercalls(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value as u64) << 25);
    }
    pub fn SyntheticClusterIpi(&self) -> bool {
        (self._bitfield >> 26) & 1 != 0
    }
    pub fn set_SyntheticClusterIpi(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 26)) | ((value as u64) << 26);
    }
    pub fn NotifyLongSpinWait(&self) -> bool {
        (self._bitfield >> 27) & 1 != 0
    }
    pub fn set_NotifyLongSpinWait(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 27)) | ((value as u64) << 27);
    }
    pub fn QueryNumaDistance(&self) -> bool {
        (self._bitfield >> 28) & 1 != 0
    }
    pub fn set_QueryNumaDistance(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 28)) | ((value as u64) << 28);
    }
    pub fn SignalEvents(&self) -> bool {
        (self._bitfield >> 29) & 1 != 0
    }
    pub fn set_SignalEvents(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 29)) | ((value as u64) << 29);
    }
    pub fn RetargetDeviceInterrupt(&self) -> bool {
        (self._bitfield >> 30) & 1 != 0
    }
    pub fn set_RetargetDeviceInterrupt(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value as u64) << 30);
    }
    pub fn RestoreTime(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_RestoreTime(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u64) << 31);
    }
    pub fn EnlightenedVmcs(&self) -> bool {
        (self._bitfield >> 32) & 1 != 0
    }
    pub fn set_EnlightenedVmcs(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 32)) | ((value as u64) << 32);
    }
    pub fn NestedDebugCtl(&self) -> bool {
        (self._bitfield >> 33) & 1 != 0
    }
    pub fn set_NestedDebugCtl(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 33)) | ((value as u64) << 33);
    }
    pub fn SyntheticTimeUnhaltedTimer(&self) -> bool {
        (self._bitfield >> 34) & 1 != 0
    }
    pub fn set_SyntheticTimeUnhaltedTimer(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 34)) | ((value as u64) << 34);
    }
    pub fn IdleSpecCtrl(&self) -> bool {
        (self._bitfield >> 35) & 1 != 0
    }
    pub fn set_IdleSpecCtrl(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 35)) | ((value as u64) << 35);
    }
    pub fn ReservedZ36(&self) -> bool {
        (self._bitfield >> 36) & 1 != 0
    }
    pub fn set_ReservedZ36(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 36)) | ((value as u64) << 36);
    }
    pub fn WakeVps(&self) -> bool {
        (self._bitfield >> 37) & 1 != 0
    }
    pub fn set_WakeVps(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 37)) | ((value as u64) << 37);
    }
    pub fn AccessVpRegs(&self) -> bool {
        (self._bitfield >> 38) & 1 != 0
    }
    pub fn set_AccessVpRegs(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 38)) | ((value as u64) << 38);
    }
    pub fn ReservedZ39(&self) -> bool {
        (self._bitfield >> 39) & 1 != 0
    }
    pub fn set_ReservedZ39(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 39)) | ((value as u64) << 39);
    }
    pub fn ReservedZ40(&self) -> bool {
        (self._bitfield >> 40) & 1 != 0
    }
    pub fn set_ReservedZ40(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 40)) | ((value as u64) << 40);
    }
    pub fn ReservedZ41(&self) -> bool {
        (self._bitfield >> 41) & 1 != 0
    }
    pub fn set_ReservedZ41(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 41)) | ((value as u64) << 41);
    }
    pub fn ReservedZ42(&self) -> bool {
        (self._bitfield >> 42) & 1 != 0
    }
    pub fn set_ReservedZ42(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 42)) | ((value as u64) << 42);
    }
    pub fn ReservedZ43(&self) -> bool {
        (self._bitfield >> 43) & 1 != 0
    }
    pub fn set_ReservedZ43(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 43)) | ((value as u64) << 43);
    }
    pub fn ReservedZ44(&self) -> bool {
        (self._bitfield >> 44) & 1 != 0
    }
    pub fn set_ReservedZ44(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 44)) | ((value as u64) << 44);
    }
    pub fn ReservedZ45(&self) -> bool {
        (self._bitfield >> 45) & 1 != 0
    }
    pub fn set_ReservedZ45(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 45)) | ((value as u64) << 45);
    }
    pub fn ReservedZ46(&self) -> bool {
        (self._bitfield >> 46) & 1 != 0
    }
    pub fn set_ReservedZ46(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 46)) | ((value as u64) << 46);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 47
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(131071 << 47)) | ((value & 131071) << 47);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {
    pub BanksCount: u32,
    pub Reserved0: u32,
    pub Anonymous: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {
    pub Anonymous: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0,
    pub AsUINT64: [u64; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {
    pub Bank0: WHV_SYNTHETIC_PROCESSOR_FEATURES,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_SYNTHETIC_PROCESSOR_FEATURES_RESERVED_BITFIELD_COUNT: u32 = 17;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_TRANSLATE_GVA_2_FLAGS {
    pub Anonymous: WHV_TRANSLATE_GVA_2_FLAGS_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_TRANSLATE_GVA_2_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_TRANSLATE_GVA_2_FLAGS_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_TRANSLATE_GVA_2_FLAGS_0 {
    pub fn ValidateRead(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ValidateRead(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn ValidateWrite(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_ValidateWrite(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn ValidateExecute(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ValidateExecute(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn PrivilegeExempt(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_PrivilegeExempt(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn SetPageTableBits(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_SetPageTableBits(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn Reserved1(&self) -> u64 {
        (self._bitfield << 56) >> 61
    }
    pub fn set_Reserved1(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn EnforceSmap(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_EnforceSmap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u64) << 8);
    }
    pub fn OverrideSmap(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_OverrideSmap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u64) << 9);
    }
    pub fn Reserved4(&self) -> u64 {
        (self._bitfield << 8) >> 18
    }
    pub fn set_Reserved4(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(70368744177663 << 10)) | ((value & 70368744177663) << 10);
    }
    pub fn InputVtl(&self) -> u64 {
        self._bitfield >> 56
    }
    pub fn set_InputVtl(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(255 << 56)) | ((value & 255) << 56);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_TRANSLATE_GVA_FLAGS = u32;
#[cfg(target_arch = "aarch64")]
pub type WHV_TRANSLATE_GVA_FLAGS = u32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_TRANSLATE_GVA_RESULT {
    pub ResultCode: WHV_TRANSLATE_GVA_RESULT_CODE,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_TRANSLATE_GVA_RESULT_CODE = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WHV_TRIGGER_HANDLE(pub *mut core::ffi::c_void);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_TRIGGER_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_TRIGGER_PARAMETERS {
    pub TriggerType: WHV_TRIGGER_TYPE,
    pub Reserved: u32,
    pub Anonymous: WHV_TRIGGER_PARAMETERS_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_TRIGGER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_TRIGGER_PARAMETERS_0 {
    pub Interrupt: WHV_INTERRUPT_CONTROL,
    pub SynicEvent: WHV_SYNIC_EVENT_PARAMETERS,
    pub DeviceInterrupt: WHV_TRIGGER_PARAMETERS_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_TRIGGER_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_TRIGGER_PARAMETERS_0_0 {
    pub LogicalDeviceId: u64,
    pub MsiAddress: u64,
    pub MsiData: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_TRIGGER_PARAMETERS {
    pub TriggerType: WHV_TRIGGER_TYPE,
    pub Reserved: u32,
    pub Anonymous: WHV_TRIGGER_PARAMETERS_0,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_TRIGGER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_TRIGGER_PARAMETERS_0 {
    pub SynicEvent: WHV_SYNIC_EVENT_PARAMETERS,
    pub DeviceInterrupt: WHV_TRIGGER_PARAMETERS_0_0,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_TRIGGER_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_TRIGGER_PARAMETERS_0_0 {
    pub LogicalDeviceId: u64,
    pub MsiAddress: u64,
    pub MsiData: u32,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_TRIGGER_TYPE = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_TRIGGER_TYPE = i32;
#[repr(C, align(16))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_UINT128 {
    pub Anonymous: WHV_UINT128_0,
    pub Dword: [u32; 4],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_UINT128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_UINT128_0 {
    pub Low64: u64,
    pub High64: u64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct WHV_UNRECOVERABLE_EXCEPTION_CONTEXT {
    pub Header: WHV_INTERCEPT_MESSAGE_HEADER,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_UNRECOVERABLE_EXCEPTION_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_VIRTUAL_PROCESSOR_PROPERTY {
    pub PropertyCode: WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE,
    pub Reserved: u32,
    pub Anonymous: WHV_VIRTUAL_PROCESSOR_PROPERTY_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VIRTUAL_PROCESSOR_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {
    pub NumaNode: u16,
    pub Padding: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE = i32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_VIRTUAL_PROCESSOR_STATE_TYPE = i32;
#[cfg(target_arch = "aarch64")]
pub type WHV_VIRTUAL_PROCESSOR_STATE_TYPE = i32;
#[cfg(target_arch = "aarch64")]
pub const WHV_VIRTUAL_PROCESSOR_STATE_TYPE_ANY_VP: u32 = 1073741824;
#[cfg(target_arch = "aarch64")]
pub const WHV_VIRTUAL_PROCESSOR_STATE_TYPE_PFN: i32 = -2147483648;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_VPCI_DEVICE_NOTIFICATION {
    pub NotificationType: WHV_VPCI_DEVICE_NOTIFICATION_TYPE,
    pub Reserved1: u32,
    pub Anonymous: WHV_VPCI_DEVICE_NOTIFICATION_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VPCI_DEVICE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_VPCI_DEVICE_NOTIFICATION_0 {
    pub Reserved2: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VPCI_DEVICE_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_VPCI_DEVICE_NOTIFICATION_TYPE = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_VPCI_DEVICE_PROPERTY_CODE = i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_VPCI_DEVICE_REGISTER {
    pub Location: WHV_VPCI_DEVICE_REGISTER_SPACE,
    pub SizeInBytes: u32,
    pub OffsetInBytes: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_VPCI_DEVICE_REGISTER_SPACE = i32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_VPCI_HARDWARE_IDS {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub RevisionID: u8,
    pub ProgIf: u8,
    pub SubClass: u8,
    pub BaseClass: u8,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_VPCI_INTERRUPT_TARGET {
    pub Vector: u32,
    pub Flags: WHV_VPCI_INTERRUPT_TARGET_FLAGS,
    pub ProcessorCount: u32,
    pub Processors: [u32; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VPCI_INTERRUPT_TARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_VPCI_INTERRUPT_TARGET_FLAGS = u32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_VPCI_MMIO_MAPPING {
    pub Location: WHV_VPCI_DEVICE_REGISTER_SPACE,
    pub Flags: WHV_VPCI_MMIO_RANGE_FLAGS,
    pub SizeInBytes: u64,
    pub OffsetInBytes: u64,
    pub VirtualAddress: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VPCI_MMIO_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_VPCI_MMIO_RANGE_FLAGS = u32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_VPCI_PROBED_BARS {
    pub Value: [u32; 6],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VPCI_PROBED_BARS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_VPCI_TYPE0_BAR_COUNT: u32 = 6;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_VP_EXCEPTION_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub ExceptionInfo: WHV_VP_EXCEPTION_INFO,
    pub ExceptionType: u8,
    pub Reserved2: [u8; 3],
    pub ErrorCode: u32,
    pub ExceptionParameter: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VP_EXCEPTION_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_VP_EXCEPTION_INFO {
    pub Anonymous: WHV_VP_EXCEPTION_INFO_0,
    pub AsUINT32: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VP_EXCEPTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_VP_EXCEPTION_INFO_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_VP_EXCEPTION_INFO_0 {
    pub fn ErrorCodeValid(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ErrorCodeValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn SoftwareException(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_SoftwareException(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union WHV_VP_EXECUTION_STATE {
    pub AsUINT16: u16,
    pub Anonymous: WHV_VP_EXECUTION_STATE_0,
}
#[cfg(target_arch = "aarch64")]
impl Default for WHV_VP_EXECUTION_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_VP_EXECUTION_STATE_0 {
    pub _bitfield: u16,
}
#[cfg(target_arch = "aarch64")]
impl WHV_VP_EXECUTION_STATE_0 {
    pub fn Cpl(&self) -> u16 {
        (self._bitfield << 14) >> 14
    }
    pub fn set_Cpl(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn DebugActive(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_DebugActive(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn InterruptionPending(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_InterruptionPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn Vtl(&self) -> u16 {
        (self._bitfield << 8) >> 12
    }
    pub fn set_Vtl(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn VirtualizationFaultActive(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_VirtualizationFaultActive(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u16) << 8);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 9
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(127 << 9)) | ((value & 127) << 9);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_VP_EXIT_CONTEXT {
    pub ExecutionState: WHV_X64_VP_EXECUTION_STATE,
    pub _bitfield: u8,
    pub Reserved: u8,
    pub Reserved2: u32,
    pub Cs: WHV_X64_SEGMENT_REGISTER,
    pub Rip: u64,
    pub Rflags: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_VP_EXIT_CONTEXT {
    pub fn InstructionLength(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_InstructionLength(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Cr8(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Cr8(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VP_EXIT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WHV_VTL(pub u8);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHV_VTL_ALL: u32 = 15;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_VTL_PERMISSION_SET {
    pub AsUINT32: u32,
    pub Anonymous: WHV_VTL_PERMISSION_SET_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VTL_PERMISSION_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_VTL_PERMISSION_SET_0 {
    pub VtlPermissionFrom1: [u16; 2],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_VTL_PERMISSION_SET_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_APIC_EOI_CONTEXT {
    pub InterruptVector: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_APIC_INIT_SIPI_CONTEXT {
    pub ApicIcr: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_APIC_SMI_CONTEXT {
    pub ApicIcr: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_APIC_WRITE_CONTEXT {
    pub Type: WHV_X64_APIC_WRITE_TYPE,
    pub Reserved: u32,
    pub WriteValue: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_APIC_WRITE_TYPE = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_CPUID_ACCESS_CONTEXT {
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub DefaultResultRax: u64,
    pub DefaultResultRcx: u64,
    pub DefaultResultRdx: u64,
    pub DefaultResultRbx: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_X64_CPUID_RESULT {
    pub Function: u32,
    pub Reserved: [u32; 3],
    pub Eax: u32,
    pub Ebx: u32,
    pub Ecx: u32,
    pub Edx: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_CPUID_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_CPUID_RESULT2 {
    pub Function: u32,
    pub Index: u32,
    pub VpIndex: u32,
    pub Flags: WHV_X64_CPUID_RESULT2_FLAGS,
    pub Output: WHV_CPUID_OUTPUT,
    pub Mask: WHV_CPUID_OUTPUT,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_CPUID_RESULT2_FLAGS = u32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {
    pub Anonymous: WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {
    pub fn NmiNotification(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_NmiNotification(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn InterruptNotification(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_InterruptNotification(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn InterruptPriority(&self) -> u64 {
        (self._bitfield << 58) >> 60
    }
    pub fn set_InterruptPriority(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(15 << 2)) | ((value & 15) << 2);
    }
    pub fn Reserved(&self) -> u64 {
        (self._bitfield << 16) >> 22
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(4398046511103 << 6)) | ((value & 4398046511103) << 6);
    }
    pub fn Sint(&self) -> u64 {
        self._bitfield >> 48
    }
    pub fn set_Sint(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(65535 << 48)) | ((value & 65535) << 48);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_FP_CONTROL_STATUS_REGISTER {
    pub Anonymous: WHV_X64_FP_CONTROL_STATUS_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_FP_CONTROL_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {
    pub FpControl: u16,
    pub FpStatus: u16,
    pub FpTag: u8,
    pub Reserved: u8,
    pub LastFpOp: u16,
    pub Anonymous: WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {
    pub LastFpRip: u64,
    pub Anonymous: WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {
    pub LastFpEip: u32,
    pub LastFpCs: u16,
    pub Reserved2: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_FP_REGISTER {
    pub Anonymous: WHV_X64_FP_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_FP_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_FP_REGISTER_0 {
    pub Mantissa: u64,
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_FP_REGISTER_0 {
    pub fn BiasedExponent(&self) -> u64 {
        (self._bitfield << 49) >> 49
    }
    pub fn set_BiasedExponent(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !32767) | (value & 32767);
    }
    pub fn Sign(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_Sign(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u64) << 15);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 16
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(281474976710655 << 16)) | ((value & 281474976710655) << 16);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_HYPERCALL_CONTEXT = WHV_HYPERCALL_CONTEXT;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    pub DeliverableType: WHV_X64_PENDING_INTERRUPTION_TYPE,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_INTERRUPT_STATE_REGISTER {
    pub Anonymous: WHV_X64_INTERRUPT_STATE_REGISTER_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_INTERRUPT_STATE_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_INTERRUPT_STATE_REGISTER_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_INTERRUPT_STATE_REGISTER_0 {
    pub fn InterruptShadow(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_InterruptShadow(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn NmiMasked(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_NmiMasked(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(4611686018427387903 << 2)) | ((value & 4611686018427387903) << 2);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_X64_IO_PORT_ACCESS_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub AccessInfo: WHV_X64_IO_PORT_ACCESS_INFO,
    pub PortNumber: u16,
    pub Reserved2: [u16; 3],
    pub Rax: u64,
    pub Rcx: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub Ds: WHV_X64_SEGMENT_REGISTER,
    pub Es: WHV_X64_SEGMENT_REGISTER,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_IO_PORT_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_IO_PORT_ACCESS_INFO {
    pub Anonymous: WHV_X64_IO_PORT_ACCESS_INFO_0,
    pub AsUINT32: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_IO_PORT_ACCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_IO_PORT_ACCESS_INFO_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_IO_PORT_ACCESS_INFO_0 {
    pub fn IsWrite(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_IsWrite(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn AccessSize(&self) -> u32 {
        (self._bitfield << 28) >> 29
    }
    pub fn set_AccessSize(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn StringOp(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_StringOp(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn RepPrefix(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_RepPrefix(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u32) << 5);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(67108863 << 6)) | ((value & 67108863) << 6);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_LOCAL_APIC_EMULATION_MODE = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_X64_MSR_ACCESS_CONTEXT {
    pub AccessInfo: WHV_X64_MSR_ACCESS_INFO,
    pub MsrNumber: u32,
    pub Rax: u64,
    pub Rdx: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_MSR_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_MSR_ACCESS_INFO {
    pub Anonymous: WHV_X64_MSR_ACCESS_INFO_0,
    pub AsUINT32: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_MSR_ACCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_MSR_ACCESS_INFO_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_MSR_ACCESS_INFO_0 {
    pub fn IsWrite(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_IsWrite(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_MSR_EXIT_BITMAP {
    pub AsUINT64: u64,
    pub Anonymous: WHV_X64_MSR_EXIT_BITMAP_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_MSR_EXIT_BITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_MSR_EXIT_BITMAP_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_MSR_EXIT_BITMAP_0 {
    pub fn UnhandledMsrs(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_UnhandledMsrs(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn TscMsrWrite(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_TscMsrWrite(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn TscMsrRead(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_TscMsrRead(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn ApicBaseMsrWrite(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_ApicBaseMsrWrite(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn MiscEnableMsrRead(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_MiscEnableMsrRead(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn McUpdatePatchLevelMsrRead(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_McUpdatePatchLevelMsrRead(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u64) << 5);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(288230376151711743 << 6)) | ((value & 288230376151711743) << 6);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_NESTED_GUEST_STATE {
    pub Anonymous: WHV_X64_NESTED_GUEST_STATE_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_NESTED_GUEST_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_NESTED_GUEST_STATE_0 {
    pub _bitfield: u64,
    pub Reserved1: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_NESTED_GUEST_STATE_0 {
    pub fn NestedVirtActive(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_NestedVirtActive(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn NestedGuestMode(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_NestedGuestMode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn VmEntryPending(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_VmEntryPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn Reserved0(&self) -> u64 {
        self._bitfield >> 3
    }
    pub fn set_Reserved0(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(2305843009213693951 << 3)) | ((value & 2305843009213693951) << 3);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_NESTED_INVEPT_REGISTER {
    pub Anonymous: WHV_X64_NESTED_INVEPT_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_NESTED_INVEPT_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_X64_NESTED_INVEPT_REGISTER_0 {
    pub Type: u8,
    pub Reserved: [u8; 7],
    pub Eptp: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_NESTED_INVEPT_REGISTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_NESTED_INVVPID_REGISTER {
    pub Anonymous: WHV_X64_NESTED_INVVPID_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_NESTED_INVVPID_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_X64_NESTED_INVVPID_REGISTER_0 {
    pub Type: u8,
    pub Reserved: [u8; 3],
    pub Vpid: u32,
    pub LinearAddress: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_NESTED_INVVPID_REGISTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_NESTED_STATE {
    pub Vmx: WHV_X64_VMX_NESTED_STATE,
    pub Svm: WHV_X64_SVM_NESTED_STATE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_NESTED_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_DEBUG_EXCEPTION {
    pub AsUINT64: u64,
    pub Anonymous: WHV_X64_PENDING_DEBUG_EXCEPTION_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_DEBUG_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_DEBUG_EXCEPTION_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_PENDING_DEBUG_EXCEPTION_0 {
    pub fn Breakpoint0(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Breakpoint0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn Breakpoint1(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_Breakpoint1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn Breakpoint2(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Breakpoint2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn Breakpoint3(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Breakpoint3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u64) << 3);
    }
    pub fn SingleStep(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_SingleStep(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
    pub fn Reserved0(&self) -> u64 {
        self._bitfield >> 5
    }
    pub fn set_Reserved0(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(576460752303423487 << 5)) | ((value & 576460752303423487) << 5);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_PENDING_EVENT_TYPE = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_EXCEPTION_EVENT {
    pub Anonymous: WHV_X64_PENDING_EXCEPTION_EVENT_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_EXCEPTION_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_EXCEPTION_EVENT_0 {
    pub _bitfield: u32,
    pub ErrorCode: u32,
    pub ExceptionParameter: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_PENDING_EXCEPTION_EVENT_0 {
    pub fn EventPending(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EventPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn EventType(&self) -> u32 {
        (self._bitfield << 28) >> 29
    }
    pub fn set_EventType(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 24) >> 28
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn DeliverErrorCode(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_DeliverErrorCode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u32) << 8);
    }
    pub fn Reserved1(&self) -> u32 {
        (self._bitfield << 16) >> 25
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(127 << 9)) | ((value & 127) << 9);
    }
    pub fn Vector(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Vector(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_EXT_INT_EVENT {
    pub Anonymous: WHV_X64_PENDING_EXT_INT_EVENT_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_EXT_INT_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_EXT_INT_EVENT_0 {
    pub _bitfield: u64,
    pub Reserved2: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_PENDING_EXT_INT_EVENT_0 {
    pub fn EventPending(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EventPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn EventType(&self) -> u64 {
        (self._bitfield << 60) >> 61
    }
    pub fn set_EventType(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn Reserved0(&self) -> u64 {
        (self._bitfield << 56) >> 60
    }
    pub fn set_Reserved0(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn Vector(&self) -> u64 {
        (self._bitfield << 48) >> 56
    }
    pub fn set_Vector(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Reserved1(&self) -> u64 {
        self._bitfield >> 16
    }
    pub fn set_Reserved1(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(281474976710655 << 16)) | ((value & 281474976710655) << 16);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_INTERRUPTION_REGISTER {
    pub Anonymous: WHV_X64_PENDING_INTERRUPTION_REGISTER_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_INTERRUPTION_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {
    pub _bitfield: u32,
    pub ErrorCode: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {
    pub fn InterruptionPending(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_InterruptionPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn InterruptionType(&self) -> u32 {
        (self._bitfield << 28) >> 29
    }
    pub fn set_InterruptionType(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn DeliverErrorCode(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_DeliverErrorCode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn InstructionLength(&self) -> u32 {
        (self._bitfield << 23) >> 28
    }
    pub fn set_InstructionLength(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 5)) | ((value & 15) << 5);
    }
    pub fn NestedEvent(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_NestedEvent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u32) << 9);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 16) >> 26
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(63 << 10)) | ((value & 63) << 10);
    }
    pub fn InterruptionVector(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_InterruptionVector(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_PENDING_INTERRUPTION_TYPE = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT0 {
    pub Anonymous: WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT0_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT0_0 {
    pub _bitfield: u64,
    pub ExitCode: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT0_0 {
    pub fn EventPending(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EventPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn EventType(&self) -> u64 {
        (self._bitfield << 59) >> 60
    }
    pub fn set_EventType(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(15 << 1)) | ((value & 15) << 1);
    }
    pub fn Reserved0(&self) -> u64 {
        (self._bitfield << 56) >> 61
    }
    pub fn set_Reserved0(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn InstructionBytesValid(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_InstructionBytesValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u64) << 8);
    }
    pub fn Reserved1(&self) -> u64 {
        self._bitfield >> 9
    }
    pub fn set_Reserved1(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(36028797018963967 << 9)) | ((value & 36028797018963967) << 9);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT1 {
    pub Anonymous: WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT1_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT1_0 {
    pub ExitInfo1: u64,
    pub ExitInfo2: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT2 {
    pub Anonymous: WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT2_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT2_0 {
    pub NextRip: u64,
    pub InstructionBytesFetchedCount: u8,
    pub InstructionBytes: [u8; 7],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT3 {
    pub Anonymous: WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT3_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT3_0 {
    pub InstructionBytes: [u8; 8],
    pub Reserved2: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_SVM_NESTED_EXIT_EVENT3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT0 {
    pub Anonymous: WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT0_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT0_0 {
    pub _bitfield: u32,
    pub ExitReason: u32,
    pub ExitQualification: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT0_0 {
    pub fn EventPending(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EventPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn EventType(&self) -> u32 {
        (self._bitfield << 27) >> 28
    }
    pub fn set_EventType(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 1)) | ((value & 15) << 1);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 5
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(134217727 << 5)) | ((value & 134217727) << 5);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT1 {
    pub Anonymous: WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT1_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT1_0 {
    pub InstructionLength: u32,
    pub InstructionInfo: u32,
    pub ExitInterruptionInfo: u32,
    pub ExitExceptionErrorCode: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT2 {
    pub Anonymous: WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT2_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT2_0 {
    pub GuestLinearAddress: u64,
    pub GuestPhysicalAddress: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT3 {
    pub Anonymous: WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT3_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_PENDING_VMX_NESTED_EXIT_EVENT3_0 {
    pub MsrData: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_PROCESSOR_FEATURES = WHV_PROCESSOR_FEATURES;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_PROCESSOR_FEATURES1 = WHV_PROCESSOR_FEATURES1;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_X64_RDTSC_CONTEXT {
    pub TscAux: u64,
    pub VirtualOffset: u64,
    pub Tsc: u64,
    pub ReferenceTime: u64,
    pub RdtscInfo: WHV_X64_RDTSC_INFO,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_RDTSC_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_RDTSC_INFO {
    pub Anonymous: WHV_X64_RDTSC_INFO_0,
    pub AsUINT64: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_RDTSC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_RDTSC_INFO_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_RDTSC_INFO_0 {
    pub fn IsRdtscp(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_IsRdtscp(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(9223372036854775807 << 1)) | ((value & 9223372036854775807) << 1);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_X64_SEGMENT_REGISTER {
    pub Base: u64,
    pub Limit: u32,
    pub Selector: u16,
    pub Anonymous: WHV_X64_SEGMENT_REGISTER_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_SEGMENT_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_SEGMENT_REGISTER_0 {
    pub Anonymous: WHV_X64_SEGMENT_REGISTER_0_0,
    pub Attributes: u16,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_SEGMENT_REGISTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_SEGMENT_REGISTER_0_0 {
    pub _bitfield: u16,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_SEGMENT_REGISTER_0_0 {
    pub fn SegmentType(&self) -> u16 {
        (self._bitfield << 12) >> 12
    }
    pub fn set_SegmentType(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn NonSystemSegment(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_NonSystemSegment(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn DescriptorPrivilegeLevel(&self) -> u16 {
        (self._bitfield << 9) >> 14
    }
    pub fn set_DescriptorPrivilegeLevel(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(3 << 5)) | ((value & 3) << 5);
    }
    pub fn Present(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Present(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 4) >> 12
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn Available(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_Available(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u16) << 12);
    }
    pub fn Long(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_Long(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u16) << 13);
    }
    pub fn Default(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_Default(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u16) << 14);
    }
    pub fn Granularity(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_Granularity(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C, align(4096))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_X64_SVM_NESTED_STATE {
    pub Vendor: WHV_NESTED_STATE_TYPE,
    pub Flags: WHV_X64_SVM_NESTED_STATE_0,
    pub NestedEnlightenmentsControl: WHV_NESTED_ENLIGHTENMENTS_CONTROL,
    pub HostSaveGpa: u64,
    pub VmControlMsr: u64,
    pub VirtualTscRatioMsr: u64,
    pub VmcbGpa: u64,
    pub HostState: WHV_SVM_NESTED_HOST_STATE,
    pub Reserved: [u8; 3832],
    pub VmcbBytes: [u8; 4096],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_SVM_NESTED_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_SVM_NESTED_STATE_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_SVM_NESTED_STATE_0 {
    pub fn GuestMode(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_GuestMode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn VmEntryPending(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_VmEntryPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn HostSaveGpaValid(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_HostSaveGpaValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn CurrentVmcbValid(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_CurrentVmcbValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_X64_TABLE_REGISTER {
    pub Pad: [u16; 3],
    pub Limit: u16,
    pub Base: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_TABLE_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_UNSUPPORTED_FEATURE_CODE = i32;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    pub FeatureCode: WHV_X64_UNSUPPORTED_FEATURE_CODE,
    pub Reserved: u32,
    pub FeatureParameter: u64,
}
#[repr(C, align(4096))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_X64_VMX_NESTED_STATE {
    pub Vendor: WHV_NESTED_STATE_TYPE,
    pub Flags: WHV_X64_VMX_NESTED_STATE_0,
    pub NestedEnlightenmentsControl: WHV_NESTED_ENLIGHTENMENTS_CONTROL,
    pub Pdpt: [u64; 4],
    pub VmxonRegionGpa: u64,
    pub VmcsGpa: u64,
    pub CurrentEnlightenedVmcs: u64,
    pub VirtualApicRegs: WHV_X64_VMX_NESTED_STATE_1,
    pub Reserved: [u8; 3944],
    pub VmcsBytes: [u8; 4096],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_VMX_NESTED_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_VMX_NESTED_STATE_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_VMX_NESTED_STATE_0 {
    pub fn GuestMode(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_GuestMode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Vmxon(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_Vmxon(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn CurrentVmcsValid(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_CurrentVmcsValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn VmEntryPending(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_VmEntryPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn VmcsEnlightened(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_VmcsEnlightened(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn EnlightenedVmEntry(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_EnlightenedVmEntry(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u32) << 5);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(67108863 << 6)) | ((value & 67108863) << 6);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WHV_X64_VMX_NESTED_STATE_1 {
    pub Tpr: u32,
    pub Ppr: u32,
    pub Isr: [u32; 8],
    pub Irr: [u32; 8],
    pub IcrLow: u32,
    pub IcrHigh: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_VMX_NESTED_STATE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_VP_EXECUTION_STATE {
    pub Anonymous: WHV_X64_VP_EXECUTION_STATE_0,
    pub AsUINT16: u16,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_VP_EXECUTION_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_VP_EXECUTION_STATE_0 {
    pub _bitfield: u16,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl WHV_X64_VP_EXECUTION_STATE_0 {
    pub fn Cpl(&self) -> u16 {
        (self._bitfield << 14) >> 14
    }
    pub fn set_Cpl(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn Cr0Pe(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Cr0Pe(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn Cr0Am(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Cr0Am(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn EferLma(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_EferLma(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn DebugActive(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_DebugActive(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn InterruptionPending(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_InterruptionPending(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn Reserved0(&self) -> u16 {
        (self._bitfield << 4) >> 11
    }
    pub fn set_Reserved0(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(31 << 7)) | ((value & 31) << 7);
    }
    pub fn InterruptShadow(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_InterruptShadow(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u16) << 12);
    }
    pub fn Reserved1(&self) -> u16 {
        self._bitfield >> 13
    }
    pub fn set_Reserved1(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(7 << 13)) | ((value & 7) << 13);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WHV_X64_VP_EXIT_CONTEXT = WHV_VP_EXIT_CONTEXT;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_XMM_CONTROL_STATUS_REGISTER {
    pub Anonymous: WHV_X64_XMM_CONTROL_STATUS_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_XMM_CONTROL_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {
    pub Anonymous: WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0,
    pub XmmStatusControl: u32,
    pub XmmStatusControlMask: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {
    pub LastFpRdp: u64,
    pub Anonymous: WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {
    pub LastFpDp: u32,
    pub LastFpDs: u16,
    pub Reserved: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvAdviseGpaRangeCodePin: WHV_ADVISE_GPA_RANGE_CODE = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvAdviseGpaRangeCodePopulate: WHV_ADVISE_GPA_RANGE_CODE = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvAdviseGpaRangeCodeUnpin: WHV_ADVISE_GPA_RANGE_CODE = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvAllocateVpciResourceFlagAllowDirectP2P: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvAllocateVpciResourceFlagNone: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64IcEmulationModeGicV3: WHV_ARM64_IC_EMULATION_MODE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64IcEmulationModeNone: WHV_ARM64_IC_EMULATION_MODE = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64InterruptTypeFixed: WHV_INTERRUPT_TYPE = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64InterruptTypeMaximum: WHV_INTERRUPT_TYPE = 8;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64PendingEventException: WHV_ARM64_PENDING_EVENT_TYPE = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64PendingEventSyntheticException: WHV_ARM64_PENDING_EVENT_TYPE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterActlrEl1: WHV_REGISTER_NAME = 262147;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApdAKeyHiEl1: WHV_REGISTER_NAME = 262182;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApdAKeyLoEl1: WHV_REGISTER_NAME = 262183;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApdBKeyHiEl1: WHV_REGISTER_NAME = 262184;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApdBKeyLoEl1: WHV_REGISTER_NAME = 262185;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApgAKeyHiEl1: WHV_REGISTER_NAME = 262186;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApgAKeyLoEl1: WHV_REGISTER_NAME = 262187;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApiAKeyHiEl1: WHV_REGISTER_NAME = 262188;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApiAKeyLoEl1: WHV_REGISTER_NAME = 262189;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApiBKeyHiEl1: WHV_REGISTER_NAME = 262190;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterApiBKeyLoEl1: WHV_REGISTER_NAME = 262191;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterCntkctlEl1: WHV_REGISTER_NAME = 360456;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterCntvCtlEl0: WHV_REGISTER_NAME = 360462;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterCntvCvalEl0: WHV_REGISTER_NAME = 360463;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterCntvctEl0: WHV_REGISTER_NAME = 360465;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterContextidrEl1: WHV_REGISTER_NAME = 262157;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterCpacrEl1: WHV_REGISTER_NAME = 262148;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterCsselrEl1: WHV_REGISTER_NAME = 262197;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr0El1: WHV_REGISTER_NAME = 327680;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr10El1: WHV_REGISTER_NAME = 327690;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr11El1: WHV_REGISTER_NAME = 327691;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr12El1: WHV_REGISTER_NAME = 327692;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr13El1: WHV_REGISTER_NAME = 327693;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr14El1: WHV_REGISTER_NAME = 327694;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr15El1: WHV_REGISTER_NAME = 327695;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr1El1: WHV_REGISTER_NAME = 327681;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr2El1: WHV_REGISTER_NAME = 327682;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr3El1: WHV_REGISTER_NAME = 327683;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr4El1: WHV_REGISTER_NAME = 327684;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr5El1: WHV_REGISTER_NAME = 327685;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr6El1: WHV_REGISTER_NAME = 327686;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr7El1: WHV_REGISTER_NAME = 327687;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr8El1: WHV_REGISTER_NAME = 327688;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbcr9El1: WHV_REGISTER_NAME = 327689;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr0El1: WHV_REGISTER_NAME = 327712;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr10El1: WHV_REGISTER_NAME = 327722;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr11El1: WHV_REGISTER_NAME = 327723;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr12El1: WHV_REGISTER_NAME = 327724;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr13El1: WHV_REGISTER_NAME = 327725;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr14El1: WHV_REGISTER_NAME = 327726;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr15El1: WHV_REGISTER_NAME = 327727;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr1El1: WHV_REGISTER_NAME = 327713;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr2El1: WHV_REGISTER_NAME = 327714;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr3El1: WHV_REGISTER_NAME = 327715;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr4El1: WHV_REGISTER_NAME = 327716;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr5El1: WHV_REGISTER_NAME = 327717;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr6El1: WHV_REGISTER_NAME = 327718;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr7El1: WHV_REGISTER_NAME = 327719;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr8El1: WHV_REGISTER_NAME = 327720;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgbvr9El1: WHV_REGISTER_NAME = 327721;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgprcrEl1: WHV_REGISTER_NAME = 327749;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr0El1: WHV_REGISTER_NAME = 327696;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr10El1: WHV_REGISTER_NAME = 327706;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr11El1: WHV_REGISTER_NAME = 327707;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr12El1: WHV_REGISTER_NAME = 327708;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr13El1: WHV_REGISTER_NAME = 327709;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr14El1: WHV_REGISTER_NAME = 327710;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr15El1: WHV_REGISTER_NAME = 327711;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr1El1: WHV_REGISTER_NAME = 327697;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr2El1: WHV_REGISTER_NAME = 327698;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr3El1: WHV_REGISTER_NAME = 327699;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr4El1: WHV_REGISTER_NAME = 327700;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr5El1: WHV_REGISTER_NAME = 327701;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr6El1: WHV_REGISTER_NAME = 327702;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr7El1: WHV_REGISTER_NAME = 327703;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr8El1: WHV_REGISTER_NAME = 327704;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwcr9El1: WHV_REGISTER_NAME = 327705;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr0El1: WHV_REGISTER_NAME = 327728;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr10El1: WHV_REGISTER_NAME = 327738;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr11El1: WHV_REGISTER_NAME = 327739;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr12El1: WHV_REGISTER_NAME = 327740;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr13El1: WHV_REGISTER_NAME = 327741;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr14El1: WHV_REGISTER_NAME = 327742;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr15El1: WHV_REGISTER_NAME = 327743;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr1El1: WHV_REGISTER_NAME = 327729;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr2El1: WHV_REGISTER_NAME = 327730;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr3El1: WHV_REGISTER_NAME = 327731;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr4El1: WHV_REGISTER_NAME = 327732;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr5El1: WHV_REGISTER_NAME = 327733;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr6El1: WHV_REGISTER_NAME = 327734;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr7El1: WHV_REGISTER_NAME = 327735;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr8El1: WHV_REGISTER_NAME = 327736;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterDbgwvr9El1: WHV_REGISTER_NAME = 327737;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterElrEl1: WHV_REGISTER_NAME = 262165;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterEsrEl1: WHV_REGISTER_NAME = 262152;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterFarEl1: WHV_REGISTER_NAME = 262153;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterFp: WHV_REGISTER_NAME = 131101;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterFpcr: WHV_REGISTER_NAME = 262162;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterFpsr: WHV_REGISTER_NAME = 262163;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterGicrBaseGpa: WHV_REGISTER_NAME = 405504;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Dfr0El1: WHV_REGISTER_NAME = 139304;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Dfr1El1: WHV_REGISTER_NAME = 139305;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Isar0El1: WHV_REGISTER_NAME = 139312;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Isar1El1: WHV_REGISTER_NAME = 139313;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Isar2El1: WHV_REGISTER_NAME = 139314;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Mmfr0El1: WHV_REGISTER_NAME = 139320;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Mmfr1El1: WHV_REGISTER_NAME = 139321;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Mmfr2El1: WHV_REGISTER_NAME = 139322;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Mmfr3El1: WHV_REGISTER_NAME = 139323;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Mmfr4El1: WHV_REGISTER_NAME = 139324;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Pfr0El1: WHV_REGISTER_NAME = 139296;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Pfr1El1: WHV_REGISTER_NAME = 139297;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Pfr2El1: WHV_REGISTER_NAME = 139298;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Smfr0El1: WHV_REGISTER_NAME = 139301;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdAa64Zfr0El1: WHV_REGISTER_NAME = 139300;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdDfr0El1: WHV_REGISTER_NAME = 139274;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdIsar0El1: WHV_REGISTER_NAME = 139280;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdIsar1El1: WHV_REGISTER_NAME = 139281;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdIsar2El1: WHV_REGISTER_NAME = 139282;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdIsar3El1: WHV_REGISTER_NAME = 139283;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdIsar4El1: WHV_REGISTER_NAME = 139284;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdIsar5El1: WHV_REGISTER_NAME = 139285;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdMidrEl1: WHV_REGISTER_NAME = 139264;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdMmfr0El1: WHV_REGISTER_NAME = 139276;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdMmfr1El1: WHV_REGISTER_NAME = 139277;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdMmfr2El1: WHV_REGISTER_NAME = 139278;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdMmfr3El1: WHV_REGISTER_NAME = 139279;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdMpidrEl1: WHV_REGISTER_NAME = 139269;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdMvfr0El1: WHV_REGISTER_NAME = 139288;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdMvfr1El1: WHV_REGISTER_NAME = 139289;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdMvfr2El1: WHV_REGISTER_NAME = 139290;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdPfr0El1: WHV_REGISTER_NAME = 139272;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdPfr1El1: WHV_REGISTER_NAME = 139273;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdPfr2El1: WHV_REGISTER_NAME = 139292;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes01El1: WHV_REGISTER_NAME = 139265;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes02El1: WHV_REGISTER_NAME = 139266;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes03El1: WHV_REGISTER_NAME = 139267;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes04El1: WHV_REGISTER_NAME = 139268;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes07El1: WHV_REGISTER_NAME = 139271;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes100El1: WHV_REGISTER_NAME = 139344;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes101El1: WHV_REGISTER_NAME = 139345;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes102El1: WHV_REGISTER_NAME = 139346;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes103El1: WHV_REGISTER_NAME = 139347;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes104El1: WHV_REGISTER_NAME = 139348;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes105El1: WHV_REGISTER_NAME = 139349;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes106El1: WHV_REGISTER_NAME = 139350;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes107El1: WHV_REGISTER_NAME = 139351;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes110El1: WHV_REGISTER_NAME = 139352;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes111El1: WHV_REGISTER_NAME = 139353;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes112El1: WHV_REGISTER_NAME = 139354;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes113El1: WHV_REGISTER_NAME = 139355;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes114El1: WHV_REGISTER_NAME = 139356;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes115El1: WHV_REGISTER_NAME = 139357;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes116El1: WHV_REGISTER_NAME = 139358;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes117El1: WHV_REGISTER_NAME = 139359;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes120El1: WHV_REGISTER_NAME = 139360;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes121El1: WHV_REGISTER_NAME = 139361;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes122El1: WHV_REGISTER_NAME = 139362;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes123El1: WHV_REGISTER_NAME = 139363;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes124El1: WHV_REGISTER_NAME = 139364;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes125El1: WHV_REGISTER_NAME = 139365;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes126El1: WHV_REGISTER_NAME = 139366;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes127El1: WHV_REGISTER_NAME = 139367;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes130El1: WHV_REGISTER_NAME = 139368;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes131El1: WHV_REGISTER_NAME = 139369;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes132El1: WHV_REGISTER_NAME = 139370;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes133El1: WHV_REGISTER_NAME = 139371;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes134El1: WHV_REGISTER_NAME = 139372;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes135El1: WHV_REGISTER_NAME = 139373;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes136El1: WHV_REGISTER_NAME = 139374;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes137El1: WHV_REGISTER_NAME = 139375;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes13El1: WHV_REGISTER_NAME = 139275;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes140El1: WHV_REGISTER_NAME = 139376;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes141El1: WHV_REGISTER_NAME = 139377;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes142El1: WHV_REGISTER_NAME = 139378;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes143El1: WHV_REGISTER_NAME = 139379;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes144El1: WHV_REGISTER_NAME = 139380;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes145El1: WHV_REGISTER_NAME = 139381;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes146El1: WHV_REGISTER_NAME = 139382;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes147El1: WHV_REGISTER_NAME = 139383;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes150El1: WHV_REGISTER_NAME = 139384;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes151El1: WHV_REGISTER_NAME = 139385;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes152El1: WHV_REGISTER_NAME = 139386;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes153El1: WHV_REGISTER_NAME = 139387;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes154El1: WHV_REGISTER_NAME = 139388;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes155El1: WHV_REGISTER_NAME = 139389;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes156El1: WHV_REGISTER_NAME = 139390;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes157El1: WHV_REGISTER_NAME = 139391;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes26El1: WHV_REGISTER_NAME = 139286;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes27El1: WHV_REGISTER_NAME = 139287;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes33El1: WHV_REGISTER_NAME = 139291;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes35El1: WHV_REGISTER_NAME = 139293;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes36El1: WHV_REGISTER_NAME = 139294;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes37El1: WHV_REGISTER_NAME = 139295;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes43El1: WHV_REGISTER_NAME = 139299;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes46El1: WHV_REGISTER_NAME = 139302;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes47El1: WHV_REGISTER_NAME = 139303;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes52El1: WHV_REGISTER_NAME = 139306;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes53El1: WHV_REGISTER_NAME = 139307;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes54El1: WHV_REGISTER_NAME = 139308;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes55El1: WHV_REGISTER_NAME = 139309;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes56El1: WHV_REGISTER_NAME = 139310;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes57El1: WHV_REGISTER_NAME = 139311;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes63El1: WHV_REGISTER_NAME = 139315;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes64El1: WHV_REGISTER_NAME = 139316;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes65El1: WHV_REGISTER_NAME = 139317;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes66El1: WHV_REGISTER_NAME = 139318;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes67El1: WHV_REGISTER_NAME = 139319;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes75El1: WHV_REGISTER_NAME = 139325;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes76El1: WHV_REGISTER_NAME = 139326;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes77El1: WHV_REGISTER_NAME = 139327;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes80El1: WHV_REGISTER_NAME = 139328;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes81El1: WHV_REGISTER_NAME = 139329;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes82El1: WHV_REGISTER_NAME = 139330;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes83El1: WHV_REGISTER_NAME = 139331;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes84El1: WHV_REGISTER_NAME = 139332;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes85El1: WHV_REGISTER_NAME = 139333;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes86El1: WHV_REGISTER_NAME = 139334;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes87El1: WHV_REGISTER_NAME = 139335;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes90El1: WHV_REGISTER_NAME = 139336;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes91El1: WHV_REGISTER_NAME = 139337;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes92El1: WHV_REGISTER_NAME = 139338;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes93El1: WHV_REGISTER_NAME = 139339;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes94El1: WHV_REGISTER_NAME = 139340;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes95El1: WHV_REGISTER_NAME = 139341;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes96El1: WHV_REGISTER_NAME = 139342;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRes97El1: WHV_REGISTER_NAME = 139343;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterIdRevidrEl1: WHV_REGISTER_NAME = 139270;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterLr: WHV_REGISTER_NAME = 131102;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterMairEl1: WHV_REGISTER_NAME = 262155;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterMdrarEl1: WHV_REGISTER_NAME = 327756;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterMdscrEl1: WHV_REGISTER_NAME = 327757;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterMidrEl1: WHV_REGISTER_NAME = 262225;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterMpidrEl1: WHV_REGISTER_NAME = 262145;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterOsdlrEl1: WHV_REGISTER_NAME = 327758;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterOslarEl1: WHV_REGISTER_NAME = 327762;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterOslsrEl1: WHV_REGISTER_NAME = 327763;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterParEl1: WHV_REGISTER_NAME = 262154;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPartitionInfoPage: WHV_REGISTER_NAME = 589845;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPc: WHV_REGISTER_NAME = 131106;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmccfiltrEl0: WHV_REGISTER_NAME = 335872;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmccntrEl0: WHV_REGISTER_NAME = 335873;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmceid0El0: WHV_REGISTER_NAME = 335874;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmceid1El0: WHV_REGISTER_NAME = 335875;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmcntenclrEl0: WHV_REGISTER_NAME = 335876;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmcntensetEl0: WHV_REGISTER_NAME = 335877;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmcrEl0: WHV_REGISTER_NAME = 335878;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr0El0: WHV_REGISTER_NAME = 335879;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr10El0: WHV_REGISTER_NAME = 335889;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr11El0: WHV_REGISTER_NAME = 335890;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr12El0: WHV_REGISTER_NAME = 335891;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr13El0: WHV_REGISTER_NAME = 335892;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr14El0: WHV_REGISTER_NAME = 335893;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr15El0: WHV_REGISTER_NAME = 335894;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr16El0: WHV_REGISTER_NAME = 335895;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr17El0: WHV_REGISTER_NAME = 335896;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr18El0: WHV_REGISTER_NAME = 335897;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr19El0: WHV_REGISTER_NAME = 335898;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr1El0: WHV_REGISTER_NAME = 335880;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr20El0: WHV_REGISTER_NAME = 335899;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr21El0: WHV_REGISTER_NAME = 335900;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr22El0: WHV_REGISTER_NAME = 335901;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr23El0: WHV_REGISTER_NAME = 335902;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr24El0: WHV_REGISTER_NAME = 335903;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr25El0: WHV_REGISTER_NAME = 335904;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr26El0: WHV_REGISTER_NAME = 335905;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr27El0: WHV_REGISTER_NAME = 335906;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr28El0: WHV_REGISTER_NAME = 335907;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr29El0: WHV_REGISTER_NAME = 335908;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr2El0: WHV_REGISTER_NAME = 335881;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr30El0: WHV_REGISTER_NAME = 335909;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr3El0: WHV_REGISTER_NAME = 335882;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr4El0: WHV_REGISTER_NAME = 335883;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr5El0: WHV_REGISTER_NAME = 335884;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr6El0: WHV_REGISTER_NAME = 335885;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr7El0: WHV_REGISTER_NAME = 335886;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr8El0: WHV_REGISTER_NAME = 335887;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevcntr9El0: WHV_REGISTER_NAME = 335888;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper0El0: WHV_REGISTER_NAME = 335910;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper10El0: WHV_REGISTER_NAME = 335920;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper11El0: WHV_REGISTER_NAME = 335921;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper12El0: WHV_REGISTER_NAME = 335922;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper13El0: WHV_REGISTER_NAME = 335923;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper14El0: WHV_REGISTER_NAME = 335924;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper15El0: WHV_REGISTER_NAME = 335925;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper16El0: WHV_REGISTER_NAME = 335926;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper17El0: WHV_REGISTER_NAME = 335927;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper18El0: WHV_REGISTER_NAME = 335928;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper19El0: WHV_REGISTER_NAME = 335929;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper1El0: WHV_REGISTER_NAME = 335911;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper20El0: WHV_REGISTER_NAME = 335930;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper21El0: WHV_REGISTER_NAME = 335931;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper22El0: WHV_REGISTER_NAME = 335932;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper23El0: WHV_REGISTER_NAME = 335933;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper24El0: WHV_REGISTER_NAME = 335934;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper25El0: WHV_REGISTER_NAME = 335935;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper26El0: WHV_REGISTER_NAME = 335936;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper27El0: WHV_REGISTER_NAME = 335937;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper28El0: WHV_REGISTER_NAME = 335938;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper29El0: WHV_REGISTER_NAME = 335939;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper2El0: WHV_REGISTER_NAME = 335912;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper30El0: WHV_REGISTER_NAME = 335940;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper3El0: WHV_REGISTER_NAME = 335913;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper4El0: WHV_REGISTER_NAME = 335914;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper5El0: WHV_REGISTER_NAME = 335915;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper6El0: WHV_REGISTER_NAME = 335916;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper7El0: WHV_REGISTER_NAME = 335917;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper8El0: WHV_REGISTER_NAME = 335918;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmevtyper9El0: WHV_REGISTER_NAME = 335919;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmintenclrEl1: WHV_REGISTER_NAME = 335941;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmintensetEl1: WHV_REGISTER_NAME = 335942;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmovsclrEl0: WHV_REGISTER_NAME = 335944;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmovssetEl0: WHV_REGISTER_NAME = 335945;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmselrEl0: WHV_REGISTER_NAME = 335946;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPmuserenrEl0: WHV_REGISTER_NAME = 335948;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterPstate: WHV_REGISTER_NAME = 131107;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ0: WHV_REGISTER_NAME = 196608;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ1: WHV_REGISTER_NAME = 196609;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ10: WHV_REGISTER_NAME = 196618;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ11: WHV_REGISTER_NAME = 196619;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ12: WHV_REGISTER_NAME = 196620;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ13: WHV_REGISTER_NAME = 196621;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ14: WHV_REGISTER_NAME = 196622;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ15: WHV_REGISTER_NAME = 196623;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ16: WHV_REGISTER_NAME = 196624;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ17: WHV_REGISTER_NAME = 196625;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ18: WHV_REGISTER_NAME = 196626;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ19: WHV_REGISTER_NAME = 196627;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ2: WHV_REGISTER_NAME = 196610;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ20: WHV_REGISTER_NAME = 196628;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ21: WHV_REGISTER_NAME = 196629;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ22: WHV_REGISTER_NAME = 196630;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ23: WHV_REGISTER_NAME = 196631;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ24: WHV_REGISTER_NAME = 196632;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ25: WHV_REGISTER_NAME = 196633;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ26: WHV_REGISTER_NAME = 196634;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ27: WHV_REGISTER_NAME = 196635;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ28: WHV_REGISTER_NAME = 196636;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ29: WHV_REGISTER_NAME = 196637;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ3: WHV_REGISTER_NAME = 196611;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ30: WHV_REGISTER_NAME = 196638;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ31: WHV_REGISTER_NAME = 196639;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ4: WHV_REGISTER_NAME = 196612;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ5: WHV_REGISTER_NAME = 196613;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ6: WHV_REGISTER_NAME = 196614;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ7: WHV_REGISTER_NAME = 196615;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ8: WHV_REGISTER_NAME = 196616;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterQ9: WHV_REGISTER_NAME = 196617;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterSctlrEl1: WHV_REGISTER_NAME = 262146;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterSmcrEl1: WHV_REGISTER_NAME = 262237;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterSmidrEl1: WHV_REGISTER_NAME = 262239;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterSmpriEl1: WHV_REGISTER_NAME = 262240;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterSp: WHV_REGISTER_NAME = 131103;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterSpEl0: WHV_REGISTER_NAME = 131104;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterSpEl1: WHV_REGISTER_NAME = 131105;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterSpsrEl1: WHV_REGISTER_NAME = 262164;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterSvcr: WHV_REGISTER_NAME = 262274;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterTcrEl1: WHV_REGISTER_NAME = 262151;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterTpidr2El0: WHV_REGISTER_NAME = 262246;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterTpidrEl0: WHV_REGISTER_NAME = 262161;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterTpidrEl1: WHV_REGISTER_NAME = 262158;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterTpidrroEl0: WHV_REGISTER_NAME = 262160;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterTtbr0El1: WHV_REGISTER_NAME = 262149;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterTtbr1El1: WHV_REGISTER_NAME = 262150;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterVbarEl1: WHV_REGISTER_NAME = 262156;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX0: WHV_REGISTER_NAME = 131072;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX1: WHV_REGISTER_NAME = 131073;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX10: WHV_REGISTER_NAME = 131082;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX11: WHV_REGISTER_NAME = 131083;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX12: WHV_REGISTER_NAME = 131084;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX13: WHV_REGISTER_NAME = 131085;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX14: WHV_REGISTER_NAME = 131086;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX15: WHV_REGISTER_NAME = 131087;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX16: WHV_REGISTER_NAME = 131088;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX17: WHV_REGISTER_NAME = 131089;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX18: WHV_REGISTER_NAME = 131090;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX19: WHV_REGISTER_NAME = 131091;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX2: WHV_REGISTER_NAME = 131074;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX20: WHV_REGISTER_NAME = 131092;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX21: WHV_REGISTER_NAME = 131093;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX22: WHV_REGISTER_NAME = 131094;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX23: WHV_REGISTER_NAME = 131095;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX24: WHV_REGISTER_NAME = 131096;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX25: WHV_REGISTER_NAME = 131097;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX26: WHV_REGISTER_NAME = 131098;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX27: WHV_REGISTER_NAME = 131099;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX28: WHV_REGISTER_NAME = 131100;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX3: WHV_REGISTER_NAME = 131075;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX4: WHV_REGISTER_NAME = 131076;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX5: WHV_REGISTER_NAME = 131077;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX6: WHV_REGISTER_NAME = 131078;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX7: WHV_REGISTER_NAME = 131079;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX8: WHV_REGISTER_NAME = 131080;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterX9: WHV_REGISTER_NAME = 131081;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64RegisterZcrEl1: WHV_REGISTER_NAME = 262257;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64ResetTypePowerOff: WHV_ARM64_RESET_TYPE = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64ResetTypeReboot: WHV_ARM64_RESET_TYPE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64SyntheticExceptionTypeCrashdump: WHV_ARM64_SYNTHETIC_EXCEPTION_TYPE = 2;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64SyntheticExceptionTypeMax: WHV_ARM64_SYNTHETIC_EXCEPTION_TYPE = 64;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64SyntheticExceptionTypeSecure: WHV_ARM64_SYNTHETIC_EXCEPTION_TYPE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64SyntheticExceptionTypeSmc: WHV_ARM64_SYNTHETIC_EXCEPTION_TYPE = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvArm64SyntheticExceptionTypeVirtualizationFault: WHV_ARM64_SYNTHETIC_EXCEPTION_TYPE = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCacheTypeUncached: WHV_CACHE_TYPE = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvCacheTypeUncached: WHV_CACHE_TYPE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCacheTypeWriteBack: WHV_CACHE_TYPE = 6;
#[cfg(target_arch = "aarch64")]
pub const WHvCacheTypeWriteBack: WHV_CACHE_TYPE = 6;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCacheTypeWriteCombining: WHV_CACHE_TYPE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvCacheTypeWriteCombining: WHV_CACHE_TYPE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCacheTypeWriteProtected: WHV_CACHE_TYPE = 5;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCacheTypeWriteThrough: WHV_CACHE_TYPE = 4;
#[cfg(target_arch = "aarch64")]
pub const WHvCacheTypeWriteThrough: WHV_CACHE_TYPE = 4;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeExceptionExitBitmap: WHV_CAPABILITY_CODE = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeExtendedVmExits: WHV_CAPABILITY_CODE = 2;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeExtendedVmExits: WHV_CAPABILITY_CODE = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeFeatures: WHV_CAPABILITY_CODE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeFeatures: WHV_CAPABILITY_CODE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeGicLpiIntIdBits: WHV_CAPABILITY_CODE = 8209;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeGpaRangePopulateFlags: WHV_CAPABILITY_CODE = 5;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeGpaRangePopulateFlags: WHV_CAPABILITY_CODE = 5;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeHypervisorPresent: WHV_CAPABILITY_CODE = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeHypervisorPresent: WHV_CAPABILITY_CODE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeInterruptClockFrequency: WHV_CAPABILITY_CODE = 4101;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeMaxSmeStreamingVectorLength: WHV_CAPABILITY_CODE = 8211;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeMaxSveVectorLength: WHV_CAPABILITY_CODE = 8210;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodePhysicalAddressWidth: WHV_CAPABILITY_CODE = 4106;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodePhysicalAddressWidth: WHV_CAPABILITY_CODE = 4106;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeProcessorClFlushSize: WHV_CAPABILITY_CODE = 4098;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeProcessorClFlushSize: WHV_CAPABILITY_CODE = 4098;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeProcessorClockFrequency: WHV_CAPABILITY_CODE = 4100;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeProcessorClockFrequency: WHV_CAPABILITY_CODE = 4100;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeProcessorFeatures: WHV_CAPABILITY_CODE = 4097;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeProcessorFeatures: WHV_CAPABILITY_CODE = 4097;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeProcessorFeaturesBanks: WHV_CAPABILITY_CODE = 4102;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeProcessorFeaturesBanks: WHV_CAPABILITY_CODE = 4102;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeProcessorFrequencyCap: WHV_CAPABILITY_CODE = 4103;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeProcessorFrequencyCap: WHV_CAPABILITY_CODE = 4103;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeProcessorPerfmonFeatures: WHV_CAPABILITY_CODE = 4105;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeProcessorVendor: WHV_CAPABILITY_CODE = 4096;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeProcessorVendor: WHV_CAPABILITY_CODE = 4096;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeProcessorXsaveFeatures: WHV_CAPABILITY_CODE = 4099;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeSchedulerFeatures: WHV_CAPABILITY_CODE = 6;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeSchedulerFeatures: WHV_CAPABILITY_CODE = 6;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeSyntheticProcessorFeaturesBanks: WHV_CAPABILITY_CODE = 4104;
#[cfg(target_arch = "aarch64")]
pub const WHvCapabilityCodeSyntheticProcessorFeaturesBanks: WHV_CAPABILITY_CODE = 4104;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxBasic: WHV_CAPABILITY_CODE = 8192;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxCr0Fixed0: WHV_CAPABILITY_CODE = 8198;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxCr0Fixed1: WHV_CAPABILITY_CODE = 8199;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxCr4Fixed0: WHV_CAPABILITY_CODE = 8200;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxCr4Fixed1: WHV_CAPABILITY_CODE = 8201;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxEntryCtls: WHV_CAPABILITY_CODE = 8196;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxEptVpidCap: WHV_CAPABILITY_CODE = 8204;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxExitCtls: WHV_CAPABILITY_CODE = 8195;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxMisc: WHV_CAPABILITY_CODE = 8197;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxPinbasedCtls: WHV_CAPABILITY_CODE = 8193;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxProcbasedCtls: WHV_CAPABILITY_CODE = 8194;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxProcbasedCtls2: WHV_CAPABILITY_CODE = 8203;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxTrueEntryCtls: WHV_CAPABILITY_CODE = 8208;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxTrueExitCtls: WHV_CAPABILITY_CODE = 8207;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxTruePinbasedCtls: WHV_CAPABILITY_CODE = 8205;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxTrueProcbasedCtls: WHV_CAPABILITY_CODE = 8206;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeVmxVmcsEnum: WHV_CAPABILITY_CODE = 8202;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCapabilityCodeX64MsrExitBitmap: WHV_CAPABILITY_CODE = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCreateVpciDeviceFlagNone: WHV_CREATE_VPCI_DEVICE_FLAGS = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCreateVpciDeviceFlagPhysicallyBacked: WHV_CREATE_VPCI_DEVICE_FLAGS = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvCreateVpciDeviceFlagUseLogicalInterrupts: WHV_CREATE_VPCI_DEVICE_FLAGS = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMapGpaRangeFlagExecute: WHV_MAP_GPA_RANGE_FLAGS = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMapGpaRangeFlagNone: WHV_MAP_GPA_RANGE_FLAGS = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMapGpaRangeFlagRead: WHV_MAP_GPA_RANGE_FLAGS = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMapGpaRangeFlagTrackDirtyPages: WHV_MAP_GPA_RANGE_FLAGS = 8;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMapGpaRangeFlagWrite: WHV_MAP_GPA_RANGE_FLAGS = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMemoryAccessExecute: WHV_MEMORY_ACCESS_TYPE = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMemoryAccessRead: WHV_MEMORY_ACCESS_TYPE = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMemoryAccessWrite: WHV_MEMORY_ACCESS_TYPE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvMessageTypeRegisterIntercept: WHV_RUN_VP_EXIT_REASON = -2147418106;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMsrActionArchitectureDefault: WHV_MSR_ACTION = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMsrActionExit: WHV_MSR_ACTION = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvMsrActionIgnoreWriteReadZero: WHV_MSR_ACTION = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvNestedStateTypeSvm: WHV_NESTED_STATE_TYPE = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvNestedStateTypeVmx: WHV_NESTED_STATE_TYPE = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvNotificationPortPropertyPreferredTargetDuration: WHV_NOTIFICATION_PORT_PROPERTY_CODE = 5;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvNotificationPortPropertyPreferredTargetVp: WHV_NOTIFICATION_PORT_PROPERTY_CODE = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvNotificationPortTypeDoorbell: WHV_NOTIFICATION_PORT_TYPE = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvNotificationPortTypeEvent: WHV_NOTIFICATION_PORT_TYPE = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionCounterSetMemory: WHV_PARTITION_COUNTER_SET = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeAllowDeviceAssignment: WHV_PARTITION_PROPERTY_CODE = 12;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeAllowDeviceAssignment: WHV_PARTITION_PROPERTY_CODE = 12;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeApicRemoteReadSupport: WHV_PARTITION_PROPERTY_CODE = 4105;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeArm64IcParameters: WHV_PARTITION_PROPERTY_CODE = 4114;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeCpuCap: WHV_PARTITION_PROPERTY_CODE = 8;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeCpuCap: WHV_PARTITION_PROPERTY_CODE = 8;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeCpuGroupId: WHV_PARTITION_PROPERTY_CODE = 10;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeCpuGroupId: WHV_PARTITION_PROPERTY_CODE = 10;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeCpuReserve: WHV_PARTITION_PROPERTY_CODE = 7;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeCpuReserve: WHV_PARTITION_PROPERTY_CODE = 7;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeCpuWeight: WHV_PARTITION_PROPERTY_CODE = 9;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeCpuWeight: WHV_PARTITION_PROPERTY_CODE = 9;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeCpuidExitList: WHV_PARTITION_PROPERTY_CODE = 4099;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeCpuidResultList: WHV_PARTITION_PROPERTY_CODE = 4100;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeCpuidResultList2: WHV_PARTITION_PROPERTY_CODE = 4109;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeDisableSmt: WHV_PARTITION_PROPERTY_CODE = 13;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeDisableSmt: WHV_PARTITION_PROPERTY_CODE = 13;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeExceptionExitBitmap: WHV_PARTITION_PROPERTY_CODE = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeExtendedVmExits: WHV_PARTITION_PROPERTY_CODE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeExtendedVmExits: WHV_PARTITION_PROPERTY_CODE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeInterruptClockFrequency: WHV_PARTITION_PROPERTY_CODE = 4104;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeLocalApicEmulationMode: WHV_PARTITION_PROPERTY_CODE = 4101;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeMsrActionList: WHV_PARTITION_PROPERTY_CODE = 4111;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeNestedVirtualization: WHV_PARTITION_PROPERTY_CODE = 4;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeNestedVirtualization: WHV_PARTITION_PROPERTY_CODE = 4;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodePhysicalAddressWidth: WHV_PARTITION_PROPERTY_CODE = 4113;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodePhysicalAddressWidth: WHV_PARTITION_PROPERTY_CODE = 4113;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodePrimaryNumaNode: WHV_PARTITION_PROPERTY_CODE = 6;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodePrimaryNumaNode: WHV_PARTITION_PROPERTY_CODE = 6;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeProcessorClFlushSize: WHV_PARTITION_PROPERTY_CODE = 4098;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeProcessorClFlushSize: WHV_PARTITION_PROPERTY_CODE = 4098;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeProcessorClockFrequency: WHV_PARTITION_PROPERTY_CODE = 4103;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeProcessorClockFrequency: WHV_PARTITION_PROPERTY_CODE = 4103;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeProcessorCount: WHV_PARTITION_PROPERTY_CODE = 8191;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeProcessorCount: WHV_PARTITION_PROPERTY_CODE = 8191;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeProcessorFeatures: WHV_PARTITION_PROPERTY_CODE = 4097;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeProcessorFeatures: WHV_PARTITION_PROPERTY_CODE = 4097;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeProcessorFeaturesBanks: WHV_PARTITION_PROPERTY_CODE = 4106;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeProcessorFeaturesBanks: WHV_PARTITION_PROPERTY_CODE = 4106;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeProcessorFrequencyCap: WHV_PARTITION_PROPERTY_CODE = 11;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeProcessorFrequencyCap: WHV_PARTITION_PROPERTY_CODE = 11;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeProcessorPerfmonFeatures: WHV_PARTITION_PROPERTY_CODE = 4110;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeProcessorXsaveFeatures: WHV_PARTITION_PROPERTY_CODE = 4102;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeReferenceTime: WHV_PARTITION_PROPERTY_CODE = 4107;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeReferenceTime: WHV_PARTITION_PROPERTY_CODE = 4107;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeSeparateSecurityDomain: WHV_PARTITION_PROPERTY_CODE = 3;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeSeparateSecurityDomain: WHV_PARTITION_PROPERTY_CODE = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeSyntheticProcessorFeaturesBanks: WHV_PARTITION_PROPERTY_CODE = 4108;
#[cfg(target_arch = "aarch64")]
pub const WHvPartitionPropertyCodeSyntheticProcessorFeaturesBanks: WHV_PARTITION_PROPERTY_CODE = 4108;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeUnimplementedMsrAction: WHV_PARTITION_PROPERTY_CODE = 4112;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvPartitionPropertyCodeX64MsrExitBitmap: WHV_PARTITION_PROPERTY_CODE = 5;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvProcessorCounterSetApic: WHV_PROCESSOR_COUNTER_SET = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvProcessorCounterSetEvents: WHV_PROCESSOR_COUNTER_SET = 2;
#[cfg(target_arch = "aarch64")]
pub const WHvProcessorCounterSetEvents: WHV_PROCESSOR_COUNTER_SET = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvProcessorCounterSetIntercepts: WHV_PROCESSOR_COUNTER_SET = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvProcessorCounterSetIntercepts: WHV_PROCESSOR_COUNTER_SET = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvProcessorCounterSetRuntime: WHV_PROCESSOR_COUNTER_SET = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvProcessorCounterSetRuntime: WHV_PROCESSOR_COUNTER_SET = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvProcessorCounterSetSyntheticFeatures: WHV_PROCESSOR_COUNTER_SET = 4;
#[cfg(target_arch = "aarch64")]
pub const WHvProcessorCounterSetSyntheticFeatures: WHV_PROCESSOR_COUNTER_SET = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvProcessorVendorAmd: WHV_PROCESSOR_VENDOR = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvProcessorVendorArm: WHV_PROCESSOR_VENDOR = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvProcessorVendorHygon: WHV_PROCESSOR_VENDOR = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvProcessorVendorIntel: WHV_PROCESSOR_VENDOR = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterCpuManagementFeaturesInfo: WHV_REGISTER_NAME = 516;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterDeliverabilityNotifications: WHV_REGISTER_NAME = -2147483644;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterDeliverabilityNotifications: WHV_REGISTER_NAME = 65542;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterEom: WHV_REGISTER_NAME = 16404;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterEom: WHV_REGISTER_NAME = 655380;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterFeaturesInfo: WHV_REGISTER_NAME = 513;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterGuestCrashCtl: WHV_REGISTER_NAME = 533;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterGuestCrashP0: WHV_REGISTER_NAME = 528;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterGuestCrashP1: WHV_REGISTER_NAME = 529;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterGuestCrashP2: WHV_REGISTER_NAME = 530;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterGuestCrashP3: WHV_REGISTER_NAME = 531;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterGuestCrashP4: WHV_REGISTER_NAME = 532;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterGuestOsId: WHV_REGISTER_NAME = 20482;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterGuestOsId: WHV_REGISTER_NAME = 589826;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterHardwareFeaturesInfo: WHV_REGISTER_NAME = 515;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterHypervisorVersion: WHV_REGISTER_NAME = 256;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterImplementationLimitsInfo: WHV_REGISTER_NAME = 514;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterInternalActivityState: WHV_REGISTER_NAME = -2147483643;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterInternalActivityState: WHV_REGISTER_NAME = 4;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterInterruptState: WHV_REGISTER_NAME = -2147483647;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterPasidFeaturesInfo: WHV_REGISTER_NAME = 517;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterPendingEvent: WHV_REGISTER_NAME = -2147483646;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterPendingEvent: u32 = 65540;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterPendingEvent0: WHV_REGISTER_NAME = 65540;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterPendingEvent1: WHV_REGISTER_NAME = -2147483645;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterPendingEvent1: WHV_REGISTER_NAME = 65541;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterPendingEvent2: WHV_REGISTER_NAME = -2147483641;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterPendingEvent2: WHV_REGISTER_NAME = 65544;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterPendingEvent3: WHV_REGISTER_NAME = -2147483640;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterPendingEvent3: WHV_REGISTER_NAME = 65545;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterPendingInterruption: WHV_REGISTER_NAME = -2147483648;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterPrivilegesAndFeaturesInfo: WHV_REGISTER_NAME = 512;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterReferenceTsc: WHV_REGISTER_NAME = 20503;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterReferenceTsc: WHV_REGISTER_NAME = 589847;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterReferenceTscSequence: WHV_REGISTER_NAME = 20506;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterReferenceTscSequence: WHV_REGISTER_NAME = 589850;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterScontrol: WHV_REGISTER_NAME = 16400;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterScontrol: WHV_REGISTER_NAME = 655376;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSiefp: WHV_REGISTER_NAME = 16402;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSiefp: u32 = 655378;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSifp: WHV_REGISTER_NAME = 655378;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSimp: WHV_REGISTER_NAME = 16403;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSimp: u32 = 655379;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint0: WHV_REGISTER_NAME = 16384;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint0: WHV_REGISTER_NAME = 655360;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint1: WHV_REGISTER_NAME = 16385;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint1: WHV_REGISTER_NAME = 655361;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint10: WHV_REGISTER_NAME = 16394;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint10: WHV_REGISTER_NAME = 655370;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint11: WHV_REGISTER_NAME = 16395;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint11: WHV_REGISTER_NAME = 655371;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint12: WHV_REGISTER_NAME = 16396;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint12: WHV_REGISTER_NAME = 655372;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint13: WHV_REGISTER_NAME = 16397;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint13: WHV_REGISTER_NAME = 655373;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint14: WHV_REGISTER_NAME = 16398;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint14: WHV_REGISTER_NAME = 655374;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint15: WHV_REGISTER_NAME = 16399;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint15: WHV_REGISTER_NAME = 655375;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint2: WHV_REGISTER_NAME = 16386;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint2: WHV_REGISTER_NAME = 655362;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint3: WHV_REGISTER_NAME = 16387;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint3: WHV_REGISTER_NAME = 655363;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint4: WHV_REGISTER_NAME = 16388;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint4: WHV_REGISTER_NAME = 655364;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint5: WHV_REGISTER_NAME = 16389;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint5: WHV_REGISTER_NAME = 655365;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint6: WHV_REGISTER_NAME = 16390;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint6: WHV_REGISTER_NAME = 655366;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint7: WHV_REGISTER_NAME = 16391;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint7: WHV_REGISTER_NAME = 655367;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint8: WHV_REGISTER_NAME = 16392;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint8: WHV_REGISTER_NAME = 655368;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSint9: WHV_REGISTER_NAME = 16393;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSint9: WHV_REGISTER_NAME = 655369;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSipp: WHV_REGISTER_NAME = 655379;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterSversion: WHV_REGISTER_NAME = 16401;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterSversion: WHV_REGISTER_NAME = 655377;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterVpAssistPage: WHV_REGISTER_NAME = 20499;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterVpAssistPage: WHV_REGISTER_NAME = 589843;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRegisterVpRuntime: WHV_REGISTER_NAME = 20480;
#[cfg(target_arch = "aarch64")]
pub const WHvRegisterVpRuntime: WHV_REGISTER_NAME = 589824;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpCancelReasonUser: WHV_RUN_VP_CANCEL_REASON = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonArm64Reset: WHV_RUN_VP_EXIT_REASON = -2147418100;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonCanceled: WHV_RUN_VP_EXIT_REASON = 8193;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonCanceled: WHV_RUN_VP_EXIT_REASON = -1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonException: WHV_RUN_VP_EXIT_REASON = 4098;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonGpaIntercept: WHV_RUN_VP_EXIT_REASON = -2147483647;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonHypercall: WHV_RUN_VP_EXIT_REASON = 4101;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonHypercall: WHV_RUN_VP_EXIT_REASON = -2147483568;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonInvalidVpRegisterValue: WHV_RUN_VP_EXIT_REASON = 5;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonInvalidVpRegisterValue: WHV_RUN_VP_EXIT_REASON = -2147483616;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonMemoryAccess: WHV_RUN_VP_EXIT_REASON = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonNone: WHV_RUN_VP_EXIT_REASON = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonNone: WHV_RUN_VP_EXIT_REASON = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonSynicSintDeliverable: WHV_RUN_VP_EXIT_REASON = 10;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonSynicSintDeliverable: WHV_RUN_VP_EXIT_REASON = -2147483550;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonUnmappedGpa: WHV_RUN_VP_EXIT_REASON = -2147483648;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonUnrecoverableException: WHV_RUN_VP_EXIT_REASON = 4;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonUnrecoverableException: WHV_RUN_VP_EXIT_REASON = -2147483615;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonUnsupportedFeature: WHV_RUN_VP_EXIT_REASON = 6;
#[cfg(target_arch = "aarch64")]
pub const WHvRunVpExitReasonUnsupportedFeature: WHV_RUN_VP_EXIT_REASON = -2147483614;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64ApicEoi: WHV_RUN_VP_EXIT_REASON = 9;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64ApicInitSipiTrap: WHV_RUN_VP_EXIT_REASON = 4102;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64ApicSmiTrap: WHV_RUN_VP_EXIT_REASON = 4100;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64ApicWriteTrap: WHV_RUN_VP_EXIT_REASON = 4103;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64Cpuid: WHV_RUN_VP_EXIT_REASON = 4097;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64Halt: WHV_RUN_VP_EXIT_REASON = 8;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64InterruptWindow: WHV_RUN_VP_EXIT_REASON = 7;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64IoPortAccess: WHV_RUN_VP_EXIT_REASON = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64MsrAccess: WHV_RUN_VP_EXIT_REASON = 4096;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvRunVpExitReasonX64Rdtsc: WHV_RUN_VP_EXIT_REASON = 4099;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaFlagEnforceSmap: WHV_TRANSLATE_GVA_FLAGS = 256;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaFlagNone: WHV_TRANSLATE_GVA_FLAGS = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvTranslateGvaFlagNone: WHV_TRANSLATE_GVA_FLAGS = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaFlagOverrideSmap: WHV_TRANSLATE_GVA_FLAGS = 512;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaFlagPrivilegeExempt: WHV_TRANSLATE_GVA_FLAGS = 8;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaFlagSetPageTableBits: WHV_TRANSLATE_GVA_FLAGS = 16;
#[cfg(target_arch = "aarch64")]
pub const WHvTranslateGvaFlagSetPageTableBits: WHV_TRANSLATE_GVA_FLAGS = 16;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaFlagValidateExecute: WHV_TRANSLATE_GVA_FLAGS = 4;
#[cfg(target_arch = "aarch64")]
pub const WHvTranslateGvaFlagValidateExecute: WHV_TRANSLATE_GVA_FLAGS = 4;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaFlagValidateRead: WHV_TRANSLATE_GVA_FLAGS = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvTranslateGvaFlagValidateRead: WHV_TRANSLATE_GVA_FLAGS = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaFlagValidateWrite: WHV_TRANSLATE_GVA_FLAGS = 2;
#[cfg(target_arch = "aarch64")]
pub const WHvTranslateGvaFlagValidateWrite: WHV_TRANSLATE_GVA_FLAGS = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaResultGpaIllegalOverlayAccess: WHV_TRANSLATE_GVA_RESULT_CODE = 7;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaResultGpaNoReadAccess: WHV_TRANSLATE_GVA_RESULT_CODE = 5;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaResultGpaNoWriteAccess: WHV_TRANSLATE_GVA_RESULT_CODE = 6;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaResultGpaUnmapped: WHV_TRANSLATE_GVA_RESULT_CODE = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaResultIntercept: WHV_TRANSLATE_GVA_RESULT_CODE = 8;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaResultInvalidPageTableFlags: WHV_TRANSLATE_GVA_RESULT_CODE = 3;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaResultPageNotPresent: WHV_TRANSLATE_GVA_RESULT_CODE = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaResultPrivilegeViolation: WHV_TRANSLATE_GVA_RESULT_CODE = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTranslateGvaResultSuccess: WHV_TRANSLATE_GVA_RESULT_CODE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTriggerTypeDeviceInterrupt: WHV_TRIGGER_TYPE = 2;
#[cfg(target_arch = "aarch64")]
pub const WHvTriggerTypeDeviceInterrupt: WHV_TRIGGER_TYPE = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTriggerTypeInterrupt: WHV_TRIGGER_TYPE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvTriggerTypeSynicEvent: WHV_TRIGGER_TYPE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvTriggerTypeSynicEvent: WHV_TRIGGER_TYPE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvUnsupportedFeatureIntercept: WHV_X64_UNSUPPORTED_FEATURE_CODE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvUnsupportedFeatureTaskSwitchTss: WHV_X64_UNSUPPORTED_FEATURE_CODE = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVirtualProcessorPropertyCodeNumaNode: WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvVirtualProcessorStateTypeGlobalInterruptState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = -1073741818;
#[cfg(target_arch = "aarch64")]
pub const WHvVirtualProcessorStateTypeInterruptControllerState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = -2147483648;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVirtualProcessorStateTypeInterruptControllerState2: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = 4096;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVirtualProcessorStateTypeNestedState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = 4098;
#[cfg(target_arch = "aarch64")]
pub const WHvVirtualProcessorStateTypeSmeState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = -2147483640;
#[cfg(target_arch = "aarch64")]
pub const WHvVirtualProcessorStateTypeSveState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = -2147483641;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVirtualProcessorStateTypeSynicEventFlagPage: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = 1;
#[cfg(target_arch = "aarch64")]
pub const WHvVirtualProcessorStateTypeSynicEventFlagPage: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = -2147483645;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVirtualProcessorStateTypeSynicMessagePage: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = 0;
#[cfg(target_arch = "aarch64")]
pub const WHvVirtualProcessorStateTypeSynicMessagePage: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = -2147483646;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVirtualProcessorStateTypeSynicTimerState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = 2;
#[cfg(target_arch = "aarch64")]
pub const WHvVirtualProcessorStateTypeSynicTimerState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = 4;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVirtualProcessorStateTypeXsaveState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE = 4097;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciBar0: WHV_VPCI_DEVICE_REGISTER_SPACE = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciBar1: WHV_VPCI_DEVICE_REGISTER_SPACE = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciBar2: WHV_VPCI_DEVICE_REGISTER_SPACE = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciBar3: WHV_VPCI_DEVICE_REGISTER_SPACE = 3;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciBar4: WHV_VPCI_DEVICE_REGISTER_SPACE = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciBar5: WHV_VPCI_DEVICE_REGISTER_SPACE = 5;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciConfigSpace: WHV_VPCI_DEVICE_REGISTER_SPACE = -1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciDeviceNotificationMmioRemapping: WHV_VPCI_DEVICE_NOTIFICATION_TYPE = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciDeviceNotificationSurpriseRemoval: WHV_VPCI_DEVICE_NOTIFICATION_TYPE = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciDeviceNotificationUndefined: WHV_VPCI_DEVICE_NOTIFICATION_TYPE = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciDevicePropertyCodeHardwareIDs: WHV_VPCI_DEVICE_PROPERTY_CODE = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciDevicePropertyCodeProbedBARs: WHV_VPCI_DEVICE_PROPERTY_CODE = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciDevicePropertyCodeUndefined: WHV_VPCI_DEVICE_PROPERTY_CODE = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciInterruptTargetFlagMulticast: WHV_VPCI_INTERRUPT_TARGET_FLAGS = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciInterruptTargetFlagNone: WHV_VPCI_INTERRUPT_TARGET_FLAGS = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciMmioRangeFlagReadAccess: WHV_VPCI_MMIO_RANGE_FLAGS = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvVpciMmioRangeFlagWriteAccess: WHV_VPCI_MMIO_RANGE_FLAGS = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ApicWriteTypeDfr: WHV_X64_APIC_WRITE_TYPE = 224;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ApicWriteTypeLdr: WHV_X64_APIC_WRITE_TYPE = 208;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ApicWriteTypeLint0: WHV_X64_APIC_WRITE_TYPE = 848;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ApicWriteTypeLint1: WHV_X64_APIC_WRITE_TYPE = 864;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ApicWriteTypeSvr: WHV_X64_APIC_WRITE_TYPE = 240;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64CpuidResult2FlagSubleafSpecific: WHV_X64_CPUID_RESULT2_FLAGS = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64CpuidResult2FlagVpSpecific: WHV_X64_CPUID_RESULT2_FLAGS = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeAlignmentCheckFault: WHV_EXCEPTION_TYPE = 17;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeBoundRangeFault: WHV_EXCEPTION_TYPE = 5;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeBreakpointTrap: WHV_EXCEPTION_TYPE = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeControlProtectionFault: WHV_EXCEPTION_TYPE = 21;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeDebugTrapOrFault: WHV_EXCEPTION_TYPE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeDeviceNotAvailableFault: WHV_EXCEPTION_TYPE = 7;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeDivideErrorFault: WHV_EXCEPTION_TYPE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeDoubleFaultAbort: WHV_EXCEPTION_TYPE = 8;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeFloatingPointErrorFault: WHV_EXCEPTION_TYPE = 16;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeGeneralProtectionFault: WHV_EXCEPTION_TYPE = 13;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeInvalidOpcodeFault: WHV_EXCEPTION_TYPE = 6;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeInvalidTaskStateSegmentFault: WHV_EXCEPTION_TYPE = 10;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeMachineCheckAbort: WHV_EXCEPTION_TYPE = 18;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeOverflowTrap: WHV_EXCEPTION_TYPE = 4;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypePageFault: WHV_EXCEPTION_TYPE = 14;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeSegmentNotPresentFault: WHV_EXCEPTION_TYPE = 11;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeSimdFloatingPointFault: WHV_EXCEPTION_TYPE = 19;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64ExceptionTypeStackFault: WHV_EXCEPTION_TYPE = 12;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptDestinationModeLogical: WHV_INTERRUPT_DESTINATION_MODE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptDestinationModePhysical: WHV_INTERRUPT_DESTINATION_MODE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptTriggerModeEdge: WHV_INTERRUPT_TRIGGER_MODE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptTriggerModeLevel: WHV_INTERRUPT_TRIGGER_MODE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptTypeFixed: WHV_INTERRUPT_TYPE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptTypeInit: WHV_INTERRUPT_TYPE = 5;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptTypeLocalInt1: WHV_INTERRUPT_TYPE = 9;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptTypeLowestPriority: WHV_INTERRUPT_TYPE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptTypeNmi: WHV_INTERRUPT_TYPE = 4;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64InterruptTypeSipi: WHV_INTERRUPT_TYPE = 6;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64LocalApicEmulationModeNone: WHV_X64_LOCAL_APIC_EMULATION_MODE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64LocalApicEmulationModeX2Apic: WHV_X64_LOCAL_APIC_EMULATION_MODE = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64LocalApicEmulationModeXApic: WHV_X64_LOCAL_APIC_EMULATION_MODE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64PendingEventException: WHV_X64_PENDING_EVENT_TYPE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64PendingEventExtInt: WHV_X64_PENDING_EVENT_TYPE = 5;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64PendingEventSvmNestedExit: WHV_X64_PENDING_EVENT_TYPE = 7;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64PendingEventVmxNestedExit: WHV_X64_PENDING_EVENT_TYPE = 8;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64PendingException: WHV_X64_PENDING_INTERRUPTION_TYPE = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64PendingInterrupt: WHV_X64_PENDING_INTERRUPTION_TYPE = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64PendingNmi: WHV_X64_PENDING_INTERRUPTION_TYPE = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterACount: WHV_REGISTER_NAME = 8319;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterAmdVmCr: WHV_REGISTER_NAME = 8372;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterAmdVmHsavePa: WHV_REGISTER_NAME = 8371;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicBase: WHV_REGISTER_NAME = 8195;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicCurrentCount: WHV_REGISTER_NAME = 12345;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicDivide: WHV_REGISTER_NAME = 12350;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicEoi: WHV_REGISTER_NAME = 12299;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicEse: WHV_REGISTER_NAME = 12328;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIcr: WHV_REGISTER_NAME = 12336;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicId: WHV_REGISTER_NAME = 12290;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicInitCount: WHV_REGISTER_NAME = 12344;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIrr0: WHV_REGISTER_NAME = 12320;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIrr1: WHV_REGISTER_NAME = 12321;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIrr2: WHV_REGISTER_NAME = 12322;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIrr3: WHV_REGISTER_NAME = 12323;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIrr4: WHV_REGISTER_NAME = 12324;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIrr5: WHV_REGISTER_NAME = 12325;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIrr6: WHV_REGISTER_NAME = 12326;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIrr7: WHV_REGISTER_NAME = 12327;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIsr0: WHV_REGISTER_NAME = 12304;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIsr1: WHV_REGISTER_NAME = 12305;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIsr2: WHV_REGISTER_NAME = 12306;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIsr3: WHV_REGISTER_NAME = 12307;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIsr4: WHV_REGISTER_NAME = 12308;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIsr5: WHV_REGISTER_NAME = 12309;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIsr6: WHV_REGISTER_NAME = 12310;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicIsr7: WHV_REGISTER_NAME = 12311;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicLdr: WHV_REGISTER_NAME = 12301;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicLvtError: WHV_REGISTER_NAME = 12343;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicLvtLint0: WHV_REGISTER_NAME = 12341;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicLvtLint1: WHV_REGISTER_NAME = 12342;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicLvtPerfmon: WHV_REGISTER_NAME = 12340;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicLvtThermal: WHV_REGISTER_NAME = 12339;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicLvtTimer: WHV_REGISTER_NAME = 12338;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicPpr: WHV_REGISTER_NAME = 12298;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicSelfIpi: WHV_REGISTER_NAME = 12351;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicSpurious: WHV_REGISTER_NAME = 12303;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicTmr0: WHV_REGISTER_NAME = 12312;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicTmr1: WHV_REGISTER_NAME = 12313;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicTmr2: WHV_REGISTER_NAME = 12314;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicTmr3: WHV_REGISTER_NAME = 12315;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicTmr4: WHV_REGISTER_NAME = 12316;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicTmr5: WHV_REGISTER_NAME = 12317;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicTmr6: WHV_REGISTER_NAME = 12318;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicTmr7: WHV_REGISTER_NAME = 12319;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicTpr: WHV_REGISTER_NAME = 12296;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterApicVersion: WHV_REGISTER_NAME = 12291;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterBndcfgs: WHV_REGISTER_NAME = 8316;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterCr0: WHV_REGISTER_NAME = 28;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterCr2: WHV_REGISTER_NAME = 29;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterCr3: WHV_REGISTER_NAME = 30;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterCr4: WHV_REGISTER_NAME = 31;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterCr8: WHV_REGISTER_NAME = 32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterCs: WHV_REGISTER_NAME = 19;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterCstar: WHV_REGISTER_NAME = 8202;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterDeliverabilityNotifications: WHV_REGISTER_NAME = -2147483644;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterDr0: WHV_REGISTER_NAME = 33;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterDr1: WHV_REGISTER_NAME = 34;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterDr2: WHV_REGISTER_NAME = 35;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterDr3: WHV_REGISTER_NAME = 36;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterDr6: WHV_REGISTER_NAME = 37;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterDr7: WHV_REGISTER_NAME = 38;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterDs: WHV_REGISTER_NAME = 21;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterEfer: WHV_REGISTER_NAME = 8193;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterEs: WHV_REGISTER_NAME = 18;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFpControlStatus: WHV_REGISTER_NAME = 4120;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFpMmx0: WHV_REGISTER_NAME = 4112;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFpMmx1: WHV_REGISTER_NAME = 4113;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFpMmx2: WHV_REGISTER_NAME = 4114;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFpMmx3: WHV_REGISTER_NAME = 4115;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFpMmx4: WHV_REGISTER_NAME = 4116;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFpMmx5: WHV_REGISTER_NAME = 4117;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFpMmx6: WHV_REGISTER_NAME = 4118;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFpMmx7: WHV_REGISTER_NAME = 4119;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterFs: WHV_REGISTER_NAME = 22;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterGdtr: WHV_REGISTER_NAME = 27;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterGs: WHV_REGISTER_NAME = 23;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterHypercall: WHV_REGISTER_NAME = 20481;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32FeatureControl: WHV_REGISTER_NAME = 8353;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxBasic: WHV_REGISTER_NAME = 8354;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxCr0Fixed0: WHV_REGISTER_NAME = 8360;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxCr0Fixed1: WHV_REGISTER_NAME = 8361;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxCr4Fixed0: WHV_REGISTER_NAME = 8362;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxCr4Fixed1: WHV_REGISTER_NAME = 8363;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxEntryCtls: WHV_REGISTER_NAME = 8358;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxEptVpidCap: WHV_REGISTER_NAME = 8366;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxExitCtls: WHV_REGISTER_NAME = 8357;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxMisc: WHV_REGISTER_NAME = 8359;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxPinbasedCtls: WHV_REGISTER_NAME = 8355;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxProcbasedCtls: WHV_REGISTER_NAME = 8356;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxProcbasedCtls2: WHV_REGISTER_NAME = 8365;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxTrueEntryCtls: WHV_REGISTER_NAME = 8370;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxTrueExitCtls: WHV_REGISTER_NAME = 8369;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxTruePinbasedCtls: WHV_REGISTER_NAME = 8367;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxTrueProcbasedCtls: WHV_REGISTER_NAME = 8368;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIa32VmxVmcsEnum: WHV_REGISTER_NAME = 8364;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterIdtr: WHV_REGISTER_NAME = 26;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterInitialApicId: WHV_REGISTER_NAME = 8204;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterInterruptSspTableAddr: WHV_REGISTER_NAME = 8339;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterKernelGsBase: WHV_REGISTER_NAME = 8194;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterLdtr: WHV_REGISTER_NAME = 24;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterLstar: WHV_REGISTER_NAME = 8201;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMCount: WHV_REGISTER_NAME = 8318;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrIa32MiscEnable: WHV_REGISTER_NAME = 8352;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrCap: WHV_REGISTER_NAME = 8205;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrDefType: WHV_REGISTER_NAME = 8206;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix16k80000: WHV_REGISTER_NAME = 8305;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix16kA0000: WHV_REGISTER_NAME = 8306;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix4kC0000: WHV_REGISTER_NAME = 8307;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix4kC8000: WHV_REGISTER_NAME = 8308;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix4kD0000: WHV_REGISTER_NAME = 8309;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix4kD8000: WHV_REGISTER_NAME = 8310;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix4kE0000: WHV_REGISTER_NAME = 8311;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix4kE8000: WHV_REGISTER_NAME = 8312;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix4kF0000: WHV_REGISTER_NAME = 8313;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix4kF8000: WHV_REGISTER_NAME = 8314;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrFix64k00000: WHV_REGISTER_NAME = 8304;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase0: WHV_REGISTER_NAME = 8208;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase1: WHV_REGISTER_NAME = 8209;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase2: WHV_REGISTER_NAME = 8210;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase3: WHV_REGISTER_NAME = 8211;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase4: WHV_REGISTER_NAME = 8212;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase5: WHV_REGISTER_NAME = 8213;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase6: WHV_REGISTER_NAME = 8214;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase7: WHV_REGISTER_NAME = 8215;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase8: WHV_REGISTER_NAME = 8216;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBase9: WHV_REGISTER_NAME = 8217;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBaseA: WHV_REGISTER_NAME = 8218;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBaseB: WHV_REGISTER_NAME = 8219;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBaseC: WHV_REGISTER_NAME = 8220;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBaseD: WHV_REGISTER_NAME = 8221;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBaseE: WHV_REGISTER_NAME = 8222;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysBaseF: WHV_REGISTER_NAME = 8223;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask0: WHV_REGISTER_NAME = 8256;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask1: WHV_REGISTER_NAME = 8257;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask2: WHV_REGISTER_NAME = 8258;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask3: WHV_REGISTER_NAME = 8259;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask4: WHV_REGISTER_NAME = 8260;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask5: WHV_REGISTER_NAME = 8261;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask6: WHV_REGISTER_NAME = 8262;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask7: WHV_REGISTER_NAME = 8263;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask8: WHV_REGISTER_NAME = 8264;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMask9: WHV_REGISTER_NAME = 8265;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMaskA: WHV_REGISTER_NAME = 8266;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMaskB: WHV_REGISTER_NAME = 8267;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMaskC: WHV_REGISTER_NAME = 8268;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMaskD: WHV_REGISTER_NAME = 8269;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMaskE: WHV_REGISTER_NAME = 8270;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterMsrMtrrPhysMaskF: WHV_REGISTER_NAME = 8271;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterNestedCurrentVmGpa: WHV_REGISTER_NAME = 20561;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterNestedGuestState: WHV_REGISTER_NAME = 20560;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterNestedVmxInvEpt: WHV_REGISTER_NAME = 20562;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterNestedVmxInvVpid: WHV_REGISTER_NAME = 20563;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterPat: WHV_REGISTER_NAME = 8196;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterPendingDebugException: WHV_REGISTER_NAME = -2147483642;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterPl0Ssp: WHV_REGISTER_NAME = 8335;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterPl1Ssp: WHV_REGISTER_NAME = 8336;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterPl2Ssp: WHV_REGISTER_NAME = 8337;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterPl3Ssp: WHV_REGISTER_NAME = 8338;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterPredCmd: WHV_REGISTER_NAME = 8325;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterR10: WHV_REGISTER_NAME = 10;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterR11: WHV_REGISTER_NAME = 11;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterR12: WHV_REGISTER_NAME = 12;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterR13: WHV_REGISTER_NAME = 13;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterR14: WHV_REGISTER_NAME = 14;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterR15: WHV_REGISTER_NAME = 15;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterR8: WHV_REGISTER_NAME = 8;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterR9: WHV_REGISTER_NAME = 9;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRax: WHV_REGISTER_NAME = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRbp: WHV_REGISTER_NAME = 5;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRbx: WHV_REGISTER_NAME = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRcx: WHV_REGISTER_NAME = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRdi: WHV_REGISTER_NAME = 7;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRdx: WHV_REGISTER_NAME = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRflags: WHV_REGISTER_NAME = 17;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRip: WHV_REGISTER_NAME = 16;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRsi: WHV_REGISTER_NAME = 6;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterRsp: WHV_REGISTER_NAME = 4;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterSCet: WHV_REGISTER_NAME = 8333;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterSfmask: WHV_REGISTER_NAME = 8203;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterSpecCtrl: WHV_REGISTER_NAME = 8324;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterSs: WHV_REGISTER_NAME = 20;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterSsp: WHV_REGISTER_NAME = 8334;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterStar: WHV_REGISTER_NAME = 8200;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterSysenterCs: WHV_REGISTER_NAME = 8197;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterSysenterEip: WHV_REGISTER_NAME = 8198;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterSysenterEsp: WHV_REGISTER_NAME = 8199;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterTr: WHV_REGISTER_NAME = 25;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterTsc: WHV_REGISTER_NAME = 8192;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterTscAdjust: WHV_REGISTER_NAME = 8342;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterTscAux: WHV_REGISTER_NAME = 8315;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterTscDeadline: WHV_REGISTER_NAME = 8341;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterTscVirtualOffset: WHV_REGISTER_NAME = 8327;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterTsxCtrl: WHV_REGISTER_NAME = 8328;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterUCet: WHV_REGISTER_NAME = 8332;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterUmwaitControl: WHV_REGISTER_NAME = 8344;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterVirtualCr0: WHV_REGISTER_NAME = 40;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterVirtualCr3: WHV_REGISTER_NAME = 41;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterVirtualCr4: WHV_REGISTER_NAME = 42;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterVirtualCr8: WHV_REGISTER_NAME = 43;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXCr0: WHV_REGISTER_NAME = 39;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXfd: WHV_REGISTER_NAME = 8345;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXfdErr: WHV_REGISTER_NAME = 8346;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm0: WHV_REGISTER_NAME = 4096;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm1: WHV_REGISTER_NAME = 4097;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm10: WHV_REGISTER_NAME = 4106;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm11: WHV_REGISTER_NAME = 4107;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm12: WHV_REGISTER_NAME = 4108;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm13: WHV_REGISTER_NAME = 4109;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm14: WHV_REGISTER_NAME = 4110;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm15: WHV_REGISTER_NAME = 4111;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm2: WHV_REGISTER_NAME = 4098;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm3: WHV_REGISTER_NAME = 4099;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm4: WHV_REGISTER_NAME = 4100;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm5: WHV_REGISTER_NAME = 4101;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm6: WHV_REGISTER_NAME = 4102;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm7: WHV_REGISTER_NAME = 4103;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm8: WHV_REGISTER_NAME = 4104;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmm9: WHV_REGISTER_NAME = 4105;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXmmControlStatus: WHV_REGISTER_NAME = 4121;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WHvX64RegisterXss: WHV_REGISTER_NAME = 8331;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const WhvRunVpCancelReasonUser: u32 = 0;
