#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn StgConvertPropertyToVariant(pprop : *const SERIALIZEDPROPERTYVALUE, codepage : u16, pvar : *mut super::propidlbase::PROPVARIANT, pma : *const PMemoryAllocator) -> bool);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn StgConvertVariantToProperty(pvar : *const super::propidlbase::PROPVARIANT, codepage : u16, pprop : *mut SERIALIZEDPROPERTYVALUE, pcb : *mut u32, pid : super::wtypes::PROPID, freserved : bool, pcindirect : *mut u32) -> *mut SERIALIZEDPROPERTYVALUE);
pub const PIDDI_THUMBNAIL: u32 = 2;
pub const PIDDSI_BYTECOUNT: u32 = 4;
pub const PIDDSI_CATEGORY: u32 = 2;
pub const PIDDSI_COMPANY: u32 = 15;
pub const PIDDSI_DOCPARTS: u32 = 13;
pub const PIDDSI_HEADINGPAIR: u32 = 12;
pub const PIDDSI_HIDDENCOUNT: u32 = 9;
pub const PIDDSI_LINECOUNT: u32 = 5;
pub const PIDDSI_LINKSDIRTY: u32 = 16;
pub const PIDDSI_MANAGER: u32 = 14;
pub const PIDDSI_MMCLIPCOUNT: u32 = 10;
pub const PIDDSI_NOTECOUNT: u32 = 8;
pub const PIDDSI_PARCOUNT: u32 = 6;
pub const PIDDSI_PRESFORMAT: u32 = 3;
pub const PIDDSI_SCALE: u32 = 11;
pub const PIDDSI_SLIDECOUNT: u32 = 7;
pub const PIDMSI_COPYRIGHT: u32 = 11;
pub const PIDMSI_EDITOR: u32 = 2;
pub const PIDMSI_OWNER: u32 = 8;
pub const PIDMSI_PRODUCTION: u32 = 10;
pub const PIDMSI_PROJECT: u32 = 6;
pub const PIDMSI_RATING: u32 = 9;
pub const PIDMSI_SEQUENCE_NO: u32 = 5;
pub const PIDMSI_SOURCE: u32 = 4;
pub const PIDMSI_STATUS: u32 = 7;
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
pub const PIDMSI_SUPPLIER: u32 = 3;
pub const PIDSI_APPNAME: u32 = 18;
pub const PIDSI_AUTHOR: u32 = 4;
pub const PIDSI_CHARCOUNT: u32 = 16;
pub const PIDSI_COMMENTS: u32 = 6;
pub const PIDSI_CREATE_DTM: u32 = 12;
pub const PIDSI_DOC_SECURITY: u32 = 19;
pub const PIDSI_EDITTIME: u32 = 10;
pub const PIDSI_KEYWORDS: u32 = 5;
pub const PIDSI_LASTAUTHOR: u32 = 8;
pub const PIDSI_LASTPRINTED: u32 = 11;
pub const PIDSI_LASTSAVE_DTM: u32 = 13;
pub const PIDSI_PAGECOUNT: u32 = 14;
pub const PIDSI_REVNUMBER: u32 = 9;
pub const PIDSI_SUBJECT: u32 = 3;
pub const PIDSI_TEMPLATE: u32 = 7;
pub const PIDSI_THUMBNAIL: u32 = 17;
pub const PIDSI_TITLE: u32 = 2;
pub const PIDSI_WORDCOUNT: u32 = 15;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PMemoryAllocator(pub u8);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERIALIZEDPROPERTYVALUE {
    pub dwType: u32,
    pub rgb: [u8; 1],
}
impl Default for SERIALIZEDPROPERTYVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
