#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type CMC_EXCEPTION = MCA_EXCEPTION;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type CPE_EXCEPTION = MCA_EXCEPTION;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type ERROR_SEVERITY = super::UCHAR;
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MCA_EXCEPTION {
    pub VersionNumber: super::ULONG,
    pub ExceptionType: MCA_EXCEPTION_TYPE,
    pub TimeStamp: super::LARGE_INTEGER,
    pub ProcessorNumber: super::ULONG,
    pub Reserved1: super::ULONG,
    pub u: MCA_EXCEPTION_0,
    pub ExtCnt: super::ULONG,
    pub Reserved3: super::ULONG,
    pub ExtReg: [super::ULONGLONG; 24],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MCA_EXCEPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MCA_EXCEPTION_0 {
    pub Mca: MCA_EXCEPTION_0_0,
    pub Mce: MCA_EXCEPTION_0_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MCA_EXCEPTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MCA_EXCEPTION_0_0 {
    pub BankNumber: super::UCHAR,
    pub Reserved2: [super::UCHAR; 7],
    pub Status: MCI_STATS,
    pub Address: MCI_ADDR,
    pub Misc: super::ULONGLONG,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MCA_EXCEPTION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MCA_EXCEPTION_0_1 {
    pub Address: super::ULONGLONG,
    pub Type: super::ULONGLONG,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MCA_EXCEPTION {
    pub Dummy: super::ULONG,
}
pub type MCA_EXCEPTION_TYPE = i32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MCA_EXCEPTION_V1_SIZE: super::LONG = 0x38_u32 as _;
#[cfg(target_arch = "x86")]
pub const MCA_EXCEPTION_V2_SIZE: u32 = 256;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MCA_EXCEPTION_V2_SIZE: u64 = 256;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const MCA_EXTREG_V2MAX: i32 = 24;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MCI_ADDR {
    pub Anonymous: MCI_ADDR_0,
    pub QuadPart: super::ULONGLONG,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for MCI_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MCI_ADDR_0 {
    pub Address: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MCI_STATS {
    pub MciStats: MCI_STATS_0,
    pub QuadPart: super::ULONGLONG,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for MCI_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATS_0 {
    pub McaCod: super::USHORT,
    pub MsCod: super::USHORT,
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MCI_STATS {
    pub MciStatus: MCI_STATS_0,
    pub QuadPart: super::ULONG64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for MCI_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATS_0 {
    pub McaErrorCode: super::USHORT,
    pub ModelErrorCode: super::USHORT,
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PERROR_SEVERITY = *mut super::UCHAR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMCA_EXCEPTION = *mut MCA_EXCEPTION;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PMCI_ADDR = *mut MCI_ADDR;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PMCI_STATS = *mut MCI_STATS;
