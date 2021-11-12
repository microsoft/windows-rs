#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub fn CloseIMsgSession(lpmsgsess: *mut _MSGSESS);
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn GetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub fn MapStorageSCode(stgscode: i32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OpenIMsgOnIStg(
        lpmsgsess: *mut _MSGSESS,
        lpallocatebuffer: super::super::System::AddressBook::LPALLOCATEBUFFER,
        lpallocatemore: super::super::System::AddressBook::LPALLOCATEMORE,
        lpfreebuffer: super::super::System::AddressBook::LPFREEBUFFER,
        lpmalloc: super::super::System::Com::IMalloc,
        lpmapisup: *mut ::core::ffi::c_void,
        lpstg: super::super::System::Com::StructuredStorage::IStorage,
        lpfmsgcallrelease: *mut MSGCALLRELEASE,
        ulcallerdata: u32,
        ulflags: u32,
        lppmsg: *mut super::super::System::AddressBook::IMessage,
    ) -> i32;
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OpenIMsgSession(lpmalloc: super::super::System::Com::IMalloc, ulflags: u32, lppmsgsess: *mut *mut _MSGSESS) -> i32;
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn SetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> ::windows_sys::core::HRESULT;
}
pub struct BlockRange(i32);
pub struct BlockRangeList(i32);
pub struct BootOptions(i32);
pub const CATID_SMTP_DNSRESOLVERRECORDSINK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3171631974, data2: 36355, data3: 4562, data4: [148, 246, 0, 192, 79, 121, 241, 214] };
pub const CATID_SMTP_DSN: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 582309681,
    data2: 62968,
    data3: 19747,
    data4: [189, 143, 135, 181, 35, 113, 167, 58],
};
pub const CATID_SMTP_GET_AUX_DOMAIN_INFO_FLAGS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2231318154,
    data2: 64179,
    data3: 17367,
    data4: [188, 223, 105, 44, 91, 70, 230, 177],
};
pub const CATID_SMTP_LOG: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2479924536,
    data2: 11294,
    data3: 19304,
    data4: [167, 201, 215, 58, 138, 166, 238, 151],
};
pub const CATID_SMTP_MAXMSGSIZE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3958462942, data2: 42622, data3: 4562, data4: [148, 247, 0, 192, 79, 121, 241, 214] };
pub const CATID_SMTP_MSGTRACKLOG: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3336524458, data2: 32176, data3: 4562, data4: [148, 244, 0, 192, 79, 121, 241, 214] };
pub const CATID_SMTP_ON_BEFORE_DATA: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4133653650, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_INBOUND_COMMAND: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4133653645, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_MESSAGE_START: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4133653648, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_PER_RECIPIENT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4133653649, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_SERVER_RESPONSE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4133653646, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_SESSION_END: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4133653651, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_SESSION_START: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4133653647, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_STORE_DRIVER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1494702160, data2: 58675, data3: 4561, data4: [170, 103, 0, 192, 79, 163, 69, 246] };
pub const CATID_SMTP_TRANSPORT_CATEGORIZE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2516734627, data2: 2618, data3: 4562, data4: [158, 0, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_POSTCATEGORIZE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1987155540, data2: 1446, data3: 4562, data4: [157, 253, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_PRECATEGORIZE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2746022669, data2: 33791, data3: 4562, data4: [158, 20, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_ROUTER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 674509001, data2: 6224, data3: 4562, data4: [158, 3, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_SUBMISSION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4282165795, data2: 185, data3: 4562, data4: [157, 251, 0, 192, 79, 163, 34, 186] };
pub const CLSID_SmtpCat: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2990290359, data2: 37401, data3: 4562, data4: [158, 23, 0, 192, 79, 163, 34, 186] };
#[repr(transparent)]
pub struct DDiscFormat2DataEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DDiscFormat2EraseEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DDiscFormat2RawCDEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DDiscFormat2TrackAtOnceEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DDiscMaster2Events(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DFileSystemImageEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DFileSystemImageImportEvents(pub *mut ::core::ffi::c_void);
pub struct DISC_RECORDER_STATE_FLAGS(i32);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IBLOCKRANGE_ENDLBA: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IBLOCKRANGE_STARTLBA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: u32 = 771u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: u32 = 272u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: u32 = 271u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: u32 = 276u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: u32 = 275u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: u32 = 270u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: u32 = 269u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: u32 = 279u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: u32 = 265u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: u32 = 268u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: u32 = 280u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: u32 = 266u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_RECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: u32 = 274u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: u32 = 273u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: u32 = 267u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 278u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: u32 = 277u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: u32 = 264u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_WRITE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: u32 = 263u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: u32 = 769u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: u32 = 266u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: u32 = 270u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: u32 = 269u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: u32 = 516u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: u32 = 265u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: u32 = 268u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: u32 = 267u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: u32 = 517u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: u32 = 264u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 272u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: u32 = 271u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: u32 = 769u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: u32 = 770u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 771u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: u32 = 772u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: u32 = 270u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: u32 = 267u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: u32 = 274u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: u32 = 273u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: u32 = 263u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: u32 = 266u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_RECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: u32 = 272u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: u32 = 271u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: u32 = 516u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 276u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: u32 = 275u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: u32 = 1793u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: u32 = 1792u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: u32 = 2049u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: u32 = 1794u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_CLOSETRAY: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: u32 = 521u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: u32 = 523u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: u32 = 518u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_DISABLEMCN: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_ENABLEMCN: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: u32 = 525u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: u32 = 519u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_PRODUCTID: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: u32 = 520u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: u32 = 524u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: u32 = 522u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_VENDORID: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_VOLUMENAME: u32 = 516u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: u32 = 517u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_FREESECTORS: u32 = 516u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_IMPORTRECORDER: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_INUSE: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: u32 = 518u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: u32 = 519u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: u32 = 517u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: u32 = 265u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: u32 = 263u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: u32 = 264u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_ISRC: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: u32 = 264u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: u32 = 263u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_CANCELWRITE: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_DISCRECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_WRITESECTION: u32 = 512u32;
#[repr(transparent)]
pub struct DWriteEngine2Events(pub *mut ::core::ffi::c_void);
pub struct EmulationType(i32);
pub struct EnumFsiItems(i32);
pub struct EnumProgressItems(i32);
pub struct FileSystemImageResult(i32);
pub struct FsiDirectoryItem(i32);
pub struct FsiFileItem(i32);
pub struct FsiFileSystems(i32);
pub struct FsiItemType(i32);
pub struct FsiNamedStreams(i32);
pub struct FsiStream(i32);
pub const GUID_SMTPSVC_SOURCE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 456918630, data2: 58480, data3: 4561, data4: [170, 103, 0, 192, 79, 163, 69, 246] };
pub const GUID_SMTP_SOURCE_TYPE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4217750748, data2: 58472, data3: 4561, data4: [170, 103, 0, 192, 79, 163, 69, 246] };
#[repr(transparent)]
pub struct IBlockRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBlockRangeList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBootOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBurnVerification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscFormat2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscFormat2Data(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscFormat2DataEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscFormat2Erase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscFormat2RawCD(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscFormat2RawCDEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscFormat2TrackAtOnce(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscFormat2TrackAtOnceEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscMaster(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscMaster2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscMasterProgressEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscRecorder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscRecorder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscRecorder2Ex(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDiscMasterFormats(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDiscRecorders(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumFsiItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumProgressItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSystemImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSystemImage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSystemImage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSystemImageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSystemImageResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsiDirectoryItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsiDirectoryItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsiFileItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsiFileItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsiItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsiNamedStreams(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsoImageManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJolietDiscMaster(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI2FS_MajorVersion: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI2FS_MinorVersion: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: u32 = 10u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPILib2_MajorVersion: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPILib2_MinorVersion: u32 = 0u32;
pub struct IMAPI_BURN_VERIFICATION_LEVEL(i32);
pub struct IMAPI_CD_SECTOR_TYPE(i32);
pub struct IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(i32);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_ALREADYOPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220958i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_BADJOLIETNAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220963i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_BOOTIMAGE_AND_NONBLANK_DISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220946i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_CANNOT_WRITE_TO_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220948i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_COMPRESSEDSTASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220952i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_INVALIDTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220972i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_NOPROPERTIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220975i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_NOTACCESSIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220974i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_NOTPRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220973i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_STILL_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220954i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DISCFULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220964i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DISCINFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220967i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_ENCRYPTEDSTASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220951i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_FILEACCESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220968i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_FILEEXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220956i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_FILESYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220969i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_GENERIC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220978i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_INITIALIZE_ENDWRITE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220970i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_INITIALIZE_WRITE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220971i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_INVALIDIMAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220962i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_LOSS_OF_STREAMING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220953i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_MEDIUM_INVALIDTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220976i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_MEDIUM_NOTPRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220977i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOACTIVEFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220961i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOACTIVERECORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220960i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOTENOUGHDISKFORSTASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220950i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOTINITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220980i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOTOPENED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220981i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_REMOVABLESTASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220949i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_STASHINUSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220955i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_TRACKNOTOPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220966i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_TRACKOPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220965i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_TRACK_NOT_BIG_ENOUGH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220947i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_USERABORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220979i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_WRONGDISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220957i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_WRONGFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220959i32 as _);
pub struct IMAPI_FEATURE_PAGE_TYPE(i32);
pub struct IMAPI_FORMAT2_DATA_MEDIA_STATE(i32);
pub struct IMAPI_FORMAT2_DATA_WRITE_ACTION(i32);
pub struct IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(i32);
pub struct IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(i32);
pub struct IMAPI_FORMAT2_TAO_WRITE_ACTION(i32);
pub struct IMAPI_MEDIA_PHYSICAL_TYPE(i32);
pub struct IMAPI_MEDIA_WRITE_PROTECT_STATE(i32);
pub struct IMAPI_MODE_PAGE_REQUEST_TYPE(i32);
pub struct IMAPI_MODE_PAGE_TYPE(i32);
pub struct IMAPI_PROFILE_TYPE(i32);
pub struct IMAPI_READ_TRACK_ADDRESS_TYPE(i32);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: u32 = 2195u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: u32 = 75u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: u32 = 680u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: u32 = 4568u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTOR_SIZE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_S_BUFFER_TO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262657i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_S_PROPERTIESIGNORED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262656i32 as _);
pub struct IMMPID_CPV_ENUM(i32);
pub struct IMMPID_MPV_ENUM(i32);
pub struct IMMPID_MP_ENUM(i32);
pub struct IMMPID_NMP_ENUM(i32);
pub struct IMMPID_RPV_ENUM(i32);
pub struct IMMPID_RP_ENUM(i32);
pub struct IMMP_MPV_STORE_DRIVER_HANDLE(i32);
#[repr(transparent)]
pub struct IMultisession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultisessionRandomWrite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultisessionSequential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultisessionSequential2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRawCDImageCreator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRawCDImageTrackInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRedbookDiscMaster(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamConcatenate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamInterleave(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamPseudoRandomBased(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWriteEngine2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWriteEngine2EventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWriteSpeedDescriptor(pub *mut ::core::ffi::c_void);
pub struct MEDIA_FLAGS(i32);
pub struct MEDIA_TYPES(i32);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MPV_INBOUND_CUTOFF_EXCEEDED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MPV_WRITE_CONTENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_MSGCLASS_DELIVERY_REPORT: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_MSGCLASS_NONDELIVERY_REPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_MSGCLASS_REPLICATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_MSGCLASS_SYSTEM: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_ABANDON_DELIVERY: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_ABORT_DELIVERY: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_BAD_MAIL: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_CATEGORIZED: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_RETRY: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_SUBMITTED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_SUCCESS: u32 = 0u32;
pub struct MSDiscMasterObj(i32);
pub struct MSDiscRecorderObj(i32);
pub struct MSEnumDiscRecordersObj(i32);
pub struct MSGCALLRELEASE(i32);
pub struct MsftDiscFormat2Data(i32);
pub struct MsftDiscFormat2Erase(i32);
pub struct MsftDiscFormat2RawCD(i32);
pub struct MsftDiscFormat2TrackAtOnce(i32);
pub struct MsftDiscMaster2(i32);
pub struct MsftDiscRecorder2(i32);
pub struct MsftFileSystemImage(i32);
pub struct MsftIsoImageManager(i32);
pub struct MsftMultisessionRandomWrite(i32);
pub struct MsftMultisessionSequential(i32);
pub struct MsftRawCDImageCreator(i32);
pub struct MsftStreamConcatenate(i32);
pub struct MsftStreamInterleave(i32);
pub struct MsftStreamPrng001(i32);
pub struct MsftStreamZero(i32);
pub struct MsftWriteEngine2(i32);
pub struct MsftWriteSpeedDescriptor(i32);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const NMP_PROCESS_CONTROL: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const NMP_PROCESS_MODERATOR: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const NMP_PROCESS_POST: u32 = 1u32;
pub struct PlatformId(i32);
pub struct ProgressItem(i32);
pub struct ProgressItems(i32);
pub struct RECORDER_TYPES(i32);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DELIVERED: u32 = 272u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_HANDLED: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_DELAY: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_FAILURE: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_MASK: u32 = 251658240u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_NEVER: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_SUCCESS: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_DELAYED: u32 = 16384u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_DELIVERED: u32 = 131136u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_EXPANDED: u32 = 32832u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_NDR: u32 = 1104u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_RELAYED: u32 = 65600u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_ENPANDED: u32 = 8208u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_ERROR_CONTEXT_CAT: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_ERROR_CONTEXT_MTA: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_ERROR_CONTEXT_STORE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_EXPANDED: u32 = 8208u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_FAILED: u32 = 2096u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_GENERAL_FAILURE: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_HANDLED: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_RECIP_FLAGS_RESERVED: u32 = 15u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_REMOTE_MTA_NO_DSN: u32 = 524288u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_UNRESOLVED: u32 = 4144u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_VOLATILE_FLAGS_MASK: u32 = 4026531840u32;
pub struct SPropAttrArray(i32);
pub struct _MSGSESS(i32);
pub struct tagIMMPID_CPV_STRUCT(i32);
pub struct tagIMMPID_GUIDLIST_ITEM(i32);
pub struct tagIMMPID_MPV_STRUCT(i32);
pub struct tagIMMPID_MP_STRUCT(i32);
pub struct tagIMMPID_NMP_STRUCT(i32);
pub struct tagIMMPID_RPV_STRUCT(i32);
pub struct tagIMMPID_RP_STRUCT(i32);
