#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct MIB_UDP6ROW {
    pub dwLocalAddr: super::in6addr::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
}
#[cfg(feature = "in6addr")]
impl Default for MIB_UDP6ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDP6ROW2 {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDP6ROW2_0,
    pub OwningModuleInfo: [u64; 16],
    pub ucRemoteAddr: [u8; 16],
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
}
impl Default for MIB_UDP6ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_UDP6ROW2_0 {
    pub Anonymous: MIB_UDP6ROW2_0_0,
    pub dwFlags: i32,
}
impl Default for MIB_UDP6ROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_UDP6ROW2_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDP6ROW_OWNER_MODULE {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [u64; 16],
}
impl Default for MIB_UDP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_UDP6ROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
impl Default for MIB_UDP6ROW_OWNER_MODULE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_UDP6ROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_UDP6ROW_OWNER_PID {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
}
impl Default for MIB_UDP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW; 1],
}
#[cfg(feature = "in6addr")]
impl Default for MIB_UDP6TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW2; 1],
}
impl Default for MIB_UDP6TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW_OWNER_MODULE; 1],
}
impl Default for MIB_UDP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_UDP6TABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW_OWNER_PID; 1],
}
impl Default for MIB_UDP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_UDPROW {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDPROW2 {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDPROW2_0,
    pub OwningModuleInfo: [u64; 16],
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
impl Default for MIB_UDPROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_UDPROW2_0 {
    pub Anonymous: MIB_UDPROW2_0_0,
    pub dwFlags: i32,
}
impl Default for MIB_UDPROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_UDPROW2_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDPROW_OWNER_MODULE {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [u64; 16],
}
impl Default for MIB_UDPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_UDPROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
impl Default for MIB_UDPROW_OWNER_MODULE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_UDPROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_UDPROW_OWNER_PID {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_UDPSTATS {
    pub dwInDatagrams: u32,
    pub dwNoPorts: u32,
    pub dwInErrors: u32,
    pub dwOutDatagrams: u32,
    pub dwNumAddrs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_UDPSTATS2 {
    pub dw64InDatagrams: u64,
    pub dwNoPorts: u32,
    pub dwInErrors: u32,
    pub dw64OutDatagrams: u64,
    pub dwNumAddrs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_UDPTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW; 1],
}
impl Default for MIB_UDPTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDPTABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW2; 1],
}
impl Default for MIB_UDPTABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDPTABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW_OWNER_MODULE; 1],
}
impl Default for MIB_UDPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_UDPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW_OWNER_PID; 1],
}
impl Default for MIB_UDPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "in6addr")]
pub type PMIB_UDP6ROW = *mut MIB_UDP6ROW;
pub type PMIB_UDP6ROW2 = *mut MIB_UDP6ROW2;
pub type PMIB_UDP6ROW_OWNER_MODULE = *mut MIB_UDP6ROW_OWNER_MODULE;
pub type PMIB_UDP6ROW_OWNER_PID = *mut MIB_UDP6ROW_OWNER_PID;
#[cfg(feature = "in6addr")]
pub type PMIB_UDP6TABLE = *mut MIB_UDP6TABLE;
pub type PMIB_UDP6TABLE2 = *mut MIB_UDP6TABLE2;
pub type PMIB_UDP6TABLE_OWNER_MODULE = *mut MIB_UDP6TABLE_OWNER_MODULE;
pub type PMIB_UDP6TABLE_OWNER_PID = *mut MIB_UDP6TABLE_OWNER_PID;
pub type PMIB_UDPROW = *mut MIB_UDPROW;
pub type PMIB_UDPROW2 = *mut MIB_UDPROW2;
pub type PMIB_UDPROW_OWNER_MODULE = *mut MIB_UDPROW_OWNER_MODULE;
pub type PMIB_UDPROW_OWNER_PID = *mut MIB_UDPROW_OWNER_PID;
pub type PMIB_UDPSTATS = *mut MIB_UDPSTATS;
pub type PMIB_UDPSTATS2 = *mut MIB_UDPSTATS2;
pub type PMIB_UDPTABLE = *mut MIB_UDPTABLE;
pub type PMIB_UDPTABLE2 = *mut MIB_UDPTABLE2;
pub type PMIB_UDPTABLE_OWNER_MODULE = *mut MIB_UDPTABLE_OWNER_MODULE;
pub type PMIB_UDPTABLE_OWNER_PID = *mut MIB_UDPTABLE_OWNER_PID;
