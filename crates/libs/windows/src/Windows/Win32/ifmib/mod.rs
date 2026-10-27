pub const MAXLEN_IFDESCR: i32 = 256;
pub const MAXLEN_PHYSADDR: i32 = 8;
pub const MAX_INTERFACE_NAME_LEN: i32 = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IFNUMBER {
    pub dwValue: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_IFROW {
    pub wszName: [super::WCHAR; 256],
    pub dwIndex: super::IF_INDEX,
    pub dwType: super::IFTYPE,
    pub dwMtu: super::DWORD,
    pub dwSpeed: super::DWORD,
    pub dwPhysAddrLen: super::DWORD,
    pub bPhysAddr: [super::UCHAR; 8],
    pub dwAdminStatus: super::DWORD,
    pub dwOperStatus: super::INTERNAL_IF_OPER_STATUS,
    pub dwLastChange: super::DWORD,
    pub dwInOctets: super::DWORD,
    pub dwInUcastPkts: super::DWORD,
    pub dwInNUcastPkts: super::DWORD,
    pub dwInDiscards: super::DWORD,
    pub dwInErrors: super::DWORD,
    pub dwInUnknownProtos: super::DWORD,
    pub dwOutOctets: super::DWORD,
    pub dwOutUcastPkts: super::DWORD,
    pub dwOutNUcastPkts: super::DWORD,
    pub dwOutDiscards: super::DWORD,
    pub dwOutErrors: super::DWORD,
    pub dwOutQLen: super::DWORD,
    pub dwDescrLen: super::DWORD,
    pub bDescr: [super::UCHAR; 256],
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "shared", feature = "winnt"))]
impl Default for MIB_IFROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_IFTABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_IFROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "shared", feature = "winnt"))]
impl Default for MIB_IFTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type PMIB_IFNUMBER = *mut MIB_IFNUMBER;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "shared", feature = "winnt"))]
pub type PMIB_IFROW = *mut MIB_IFROW;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "shared", feature = "winnt"))]
pub type PMIB_IFTABLE = *mut MIB_IFTABLE;
