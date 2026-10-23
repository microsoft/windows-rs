#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_UDP6ROW {
    pub dwLocalAddr: super::IN6_ADDR,
    pub dwLocalScopeId: super::DWORD,
    pub dwLocalPort: super::DWORD,
}
#[cfg(all(feature = "in6addr", feature = "shared"))]
impl Default for MIB_UDP6ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_UDP6ROW2 {
    pub ucLocalAddr: [super::UCHAR; 16],
    pub dwLocalScopeId: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwOwningPid: super::DWORD,
    pub liCreateTimestamp: super::LARGE_INTEGER,
    pub Anonymous: MIB_UDP6ROW2_0,
    pub OwningModuleInfo: [super::ULONGLONG; 16],
    pub ucRemoteAddr: [super::UCHAR; 16],
    pub dwRemoteScopeId: super::DWORD,
    pub dwRemotePort: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDP6ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MIB_UDP6ROW2_0 {
    pub Anonymous: MIB_UDP6ROW2_0_0,
    pub dwFlags: i32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDP6ROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_UDP6ROW2_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_UDP6ROW_OWNER_MODULE {
    pub ucLocalAddr: [super::UCHAR; 16],
    pub dwLocalScopeId: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwOwningPid: super::DWORD,
    pub liCreateTimestamp: super::LARGE_INTEGER,
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [super::ULONGLONG; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MIB_UDP6ROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDP6ROW_OWNER_MODULE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_UDP6ROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_UDP6ROW_OWNER_PID {
    pub ucLocalAddr: [super::UCHAR; 16],
    pub dwLocalScopeId: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwOwningPid: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for MIB_UDP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_UDP6ROW; 1],
}
#[cfg(all(feature = "in6addr", feature = "shared"))]
impl Default for MIB_UDP6TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE2 {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_UDP6ROW2; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDP6TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE_OWNER_MODULE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_UDP6ROW_OWNER_MODULE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE_OWNER_PID {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_UDP6ROW_OWNER_PID; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_UDP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_UDPROW {
    pub dwLocalAddr: super::DWORD,
    pub dwLocalPort: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_UDPROW2 {
    pub dwLocalAddr: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwOwningPid: super::DWORD,
    pub liCreateTimestamp: super::LARGE_INTEGER,
    pub Anonymous: MIB_UDPROW2_0,
    pub OwningModuleInfo: [super::ULONGLONG; 16],
    pub dwRemoteAddr: super::DWORD,
    pub dwRemotePort: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDPROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MIB_UDPROW2_0 {
    pub Anonymous: MIB_UDPROW2_0_0,
    pub dwFlags: i32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDPROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_UDPROW2_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_UDPROW_OWNER_MODULE {
    pub dwLocalAddr: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwOwningPid: super::DWORD,
    pub liCreateTimestamp: super::LARGE_INTEGER,
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [super::ULONGLONG; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MIB_UDPROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDPROW_OWNER_MODULE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_UDPROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_UDPROW_OWNER_PID {
    pub dwLocalAddr: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwOwningPid: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_UDPSTATS {
    pub dwInDatagrams: super::DWORD,
    pub dwNoPorts: super::DWORD,
    pub dwInErrors: super::DWORD,
    pub dwOutDatagrams: super::DWORD,
    pub dwNumAddrs: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_UDPSTATS2 {
    pub dw64InDatagrams: super::DWORD64,
    pub dwNoPorts: super::DWORD,
    pub dwInErrors: super::DWORD,
    pub dw64OutDatagrams: super::DWORD64,
    pub dwNumAddrs: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_UDPTABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_UDPROW; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_UDPTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_UDPTABLE2 {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_UDPROW2; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDPTABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_UDPTABLE_OWNER_MODULE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_UDPROW_OWNER_MODULE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_UDPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_UDPTABLE_OWNER_PID {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_UDPROW_OWNER_PID; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_UDPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "in6addr", feature = "shared"))]
pub type PMIB_UDP6ROW = *mut MIB_UDP6ROW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_UDP6ROW2 = *mut MIB_UDP6ROW2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_UDP6ROW_OWNER_MODULE = *mut MIB_UDP6ROW_OWNER_MODULE;
#[cfg(feature = "shared")]
pub type PMIB_UDP6ROW_OWNER_PID = *mut MIB_UDP6ROW_OWNER_PID;
#[cfg(all(feature = "in6addr", feature = "shared"))]
pub type PMIB_UDP6TABLE = *mut MIB_UDP6TABLE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_UDP6TABLE2 = *mut MIB_UDP6TABLE2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_UDP6TABLE_OWNER_MODULE = *mut MIB_UDP6TABLE_OWNER_MODULE;
#[cfg(feature = "shared")]
pub type PMIB_UDP6TABLE_OWNER_PID = *mut MIB_UDP6TABLE_OWNER_PID;
#[cfg(feature = "shared")]
pub type PMIB_UDPROW = *mut MIB_UDPROW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_UDPROW2 = *mut MIB_UDPROW2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_UDPROW_OWNER_MODULE = *mut MIB_UDPROW_OWNER_MODULE;
#[cfg(feature = "shared")]
pub type PMIB_UDPROW_OWNER_PID = *mut MIB_UDPROW_OWNER_PID;
#[cfg(feature = "shared")]
pub type PMIB_UDPSTATS = *mut MIB_UDPSTATS;
#[cfg(feature = "shared")]
pub type PMIB_UDPSTATS2 = *mut MIB_UDPSTATS2;
#[cfg(feature = "shared")]
pub type PMIB_UDPTABLE = *mut MIB_UDPTABLE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_UDPTABLE2 = *mut MIB_UDPTABLE2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_UDPTABLE_OWNER_MODULE = *mut MIB_UDPTABLE_OWNER_MODULE;
#[cfg(feature = "shared")]
pub type PMIB_UDPTABLE_OWNER_PID = *mut MIB_UDPTABLE_OWNER_PID;
pub const TCPIP_OWNING_MODULE_SIZE: i32 = 16;
