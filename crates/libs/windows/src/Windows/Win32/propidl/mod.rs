#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut super::PROPVARIANT, pma: *const PMemoryAllocator) -> bool {
    windows_core::link!("ole32.dll" "system" fn StgConvertPropertyToVariant(pprop : *const SERIALIZEDPROPERTYVALUE, codepage : u16, pvar : *mut super::PROPVARIANT, pma : *const PMemoryAllocator) -> bool);
    unsafe { StgConvertPropertyToVariant(pprop, codepage, pvar, pma) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgConvertVariantToProperty(pvar: *const super::PROPVARIANT, codepage: u16, pprop: Option<*mut SERIALIZEDPROPERTYVALUE>, pcb: *mut u32, pid: super::PROPID, freserved: Option<bool>, pcindirect: Option<*mut u32>) -> *mut SERIALIZEDPROPERTYVALUE {
    windows_core::link!("ole32.dll" "system" fn StgConvertVariantToProperty(pvar : *const super::PROPVARIANT, codepage : u16, pprop : *mut SERIALIZEDPROPERTYVALUE, pcb : *mut u32, pid : super::PROPID, freserved : bool, pcindirect : *mut u32) -> *mut SERIALIZEDPROPERTYVALUE);
    unsafe { StgConvertVariantToProperty(pvar, codepage, pprop.unwrap_or(core::mem::zeroed()) as _, pcb as _, pid, freserved.unwrap_or(core::mem::zeroed()) as _, pcindirect.unwrap_or(core::mem::zeroed()) as _) }
}
pub const PIDDI_THUMBNAIL: i32 = 2;
pub const PIDDSI_BYTECOUNT: i32 = 4;
pub const PIDDSI_CATEGORY: i32 = 2;
pub const PIDDSI_COMPANY: i32 = 15;
pub const PIDDSI_DOCPARTS: i32 = 13;
pub const PIDDSI_HEADINGPAIR: i32 = 12;
pub const PIDDSI_HIDDENCOUNT: i32 = 9;
pub const PIDDSI_LINECOUNT: i32 = 5;
pub const PIDDSI_LINKSDIRTY: i32 = 16;
pub const PIDDSI_MANAGER: i32 = 14;
pub const PIDDSI_MMCLIPCOUNT: i32 = 10;
pub const PIDDSI_NOTECOUNT: i32 = 8;
pub const PIDDSI_PARCOUNT: i32 = 6;
pub const PIDDSI_PRESFORMAT: i32 = 3;
pub const PIDDSI_SCALE: i32 = 11;
pub const PIDDSI_SLIDECOUNT: i32 = 7;
pub const PIDMSI_COPYRIGHT: i32 = 11;
pub const PIDMSI_EDITOR: i32 = 2;
pub const PIDMSI_OWNER: i32 = 8;
pub const PIDMSI_PRODUCTION: i32 = 10;
pub const PIDMSI_PROJECT: i32 = 6;
pub const PIDMSI_RATING: i32 = 9;
pub const PIDMSI_SEQUENCE_NO: i32 = 5;
pub const PIDMSI_SOURCE: i32 = 4;
pub const PIDMSI_STATUS: i32 = 7;
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = 3;
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = 5;
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = 8;
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = 4;
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = 1;
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = 0;
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = 32767;
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = 2;
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = 7;
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = 6;
pub type PIDMSI_STATUS_VALUE = i32;
pub const PIDMSI_SUPPLIER: i32 = 3;
pub const PIDSI_APPNAME: i32 = 18;
pub const PIDSI_AUTHOR: i32 = 4;
pub const PIDSI_CHARCOUNT: i32 = 16;
pub const PIDSI_COMMENTS: i32 = 6;
pub const PIDSI_CREATE_DTM: i32 = 12;
pub const PIDSI_DOC_SECURITY: i32 = 19;
pub const PIDSI_EDITTIME: i32 = 10;
pub const PIDSI_KEYWORDS: i32 = 5;
pub const PIDSI_LASTAUTHOR: i32 = 8;
pub const PIDSI_LASTPRINTED: i32 = 11;
pub const PIDSI_LASTSAVE_DTM: i32 = 13;
pub const PIDSI_PAGECOUNT: i32 = 14;
pub const PIDSI_REVNUMBER: i32 = 9;
pub const PIDSI_SUBJECT: i32 = 3;
pub const PIDSI_TEMPLATE: i32 = 7;
pub const PIDSI_THUMBNAIL: i32 = 17;
pub const PIDSI_TITLE: i32 = 2;
pub const PIDSI_WORDCOUNT: i32 = 15;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PMemoryAllocator(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERIALIZEDPROPERTYVALUE {
    pub dwType: u32,
    pub rgb: [u8; 1],
}
impl Default for SERIALIZEDPROPERTYVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
