#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub type CMC_EXCEPTION = MCA_EXCEPTION;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub type CPE_EXCEPTION = MCA_EXCEPTION;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ERROR_SEVERITY(pub u8);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type ERROR_SEVERITY_VALUE = i32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ErrorCorrected: ERROR_SEVERITY_VALUE = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ErrorFatal: ERROR_SEVERITY_VALUE = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ErrorOthers: ERROR_SEVERITY_VALUE = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ErrorRecoverable: ERROR_SEVERITY_VALUE = 0;
pub const HAL_MCA_RECORD: MCA_EXCEPTION_TYPE = 1;
pub const HAL_MCE_RECORD: MCA_EXCEPTION_TYPE = 0;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct MCA_EXCEPTION {
    pub VersionNumber: u32,
    pub ExceptionType: MCA_EXCEPTION_TYPE,
    pub TimeStamp: i64,
    pub ProcessorNumber: u32,
    pub Reserved1: u32,
    pub u: MCA_EXCEPTION_0,
    pub ExtCnt: u32,
    pub Reserved3: u32,
    pub ExtReg: [u64; 24],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for MCA_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union MCA_EXCEPTION_0 {
    pub Mca: MCA_EXCEPTION_0_0,
    pub Mce: MCA_EXCEPTION_0_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for MCA_EXCEPTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct MCA_EXCEPTION_0_0 {
    pub BankNumber: u8,
    pub Reserved2: [u8; 7],
    pub Status: MCI_STATS,
    pub Address: MCI_ADDR,
    pub Misc: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for MCA_EXCEPTION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCA_EXCEPTION_0_1 {
    pub Address: u64,
    pub Type: u64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCA_EXCEPTION {
    pub Dummy: u32,
}
pub type MCA_EXCEPTION_TYPE = i32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const MCA_EXCEPTION_V1_SIZE: u32 = 56;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const MCA_EXTREG_V2MAX: u32 = 24;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union MCI_ADDR {
    pub Anonymous: MCI_ADDR_0,
    pub QuadPart: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for MCI_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCI_ADDR_0 {
    pub Address: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union MCI_STATS {
    pub MciStats: MCI_STATS_0,
    pub QuadPart: u64,
}
#[cfg(target_arch = "x86")]
impl Default for MCI_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCI_STATS_0 {
    pub McaCod: u16,
    pub MsCod: u16,
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union MCI_STATS {
    pub MciStatus: MCI_STATS_0,
    pub QuadPart: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MCI_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCI_STATS_0 {
    pub McaErrorCode: u16,
    pub ModelErrorCode: u16,
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCMC_EXCEPTION(pub *mut MCA_EXCEPTION);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl PCMC_EXCEPTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for PCMC_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCMC_EXCEPTION(pub *mut core::ffi::c_void);
#[cfg(target_arch = "aarch64")]
impl PCMC_EXCEPTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "aarch64")]
impl Default for PCMC_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCPE_EXCEPTION(pub *mut MCA_EXCEPTION);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl PCPE_EXCEPTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for PCPE_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCPE_EXCEPTION(pub *mut core::ffi::c_void);
#[cfg(target_arch = "aarch64")]
impl PCPE_EXCEPTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "aarch64")]
impl Default for PCPE_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PERROR_SEVERITY(pub *mut u8);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl PERROR_SEVERITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PERROR_SEVERITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMCA_EXCEPTION(pub *mut MCA_EXCEPTION);
impl PMCA_EXCEPTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMCA_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMCI_ADDR(pub *mut MCI_ADDR);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl PMCI_ADDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for PMCI_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMCI_STATS(pub *mut MCI_STATS);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl PMCI_STATS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for PMCI_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
