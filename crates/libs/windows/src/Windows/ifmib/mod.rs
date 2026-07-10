pub const MAXLEN_IFDESCR: u32 = 256;
pub const MAXLEN_PHYSADDR: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IFNUMBER {
    pub dwValue: u32,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IFROW {
    pub wszName: [u16; 256],
    pub dwIndex: super::ifdef::IF_INDEX,
    pub dwType: super::ipifcons::IFTYPE,
    pub dwMtu: u32,
    pub dwSpeed: u32,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAdminStatus: u32,
    pub dwOperStatus: super::ipifcons::INTERNAL_IF_OPER_STATUS,
    pub dwLastChange: u32,
    pub dwInOctets: u32,
    pub dwInUcastPkts: u32,
    pub dwInNUcastPkts: u32,
    pub dwInDiscards: u32,
    pub dwInErrors: u32,
    pub dwInUnknownProtos: u32,
    pub dwOutOctets: u32,
    pub dwOutUcastPkts: u32,
    pub dwOutNUcastPkts: u32,
    pub dwOutDiscards: u32,
    pub dwOutErrors: u32,
    pub dwOutQLen: u32,
    pub dwDescrLen: u32,
    pub bDescr: [u8; 256],
}
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
impl Default for MIB_IFROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IFTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IFROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
impl Default for MIB_IFTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIB_IFNUMBER(pub *mut MIB_IFNUMBER);
impl PMIB_IFNUMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIB_IFNUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIB_IFROW(pub *mut MIB_IFROW);
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
impl PMIB_IFROW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
impl Default for PMIB_IFROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIB_IFTABLE(pub *mut MIB_IFTABLE);
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
impl PMIB_IFTABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "ifdef", feature = "ipifcons"))]
impl Default for PMIB_IFTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
