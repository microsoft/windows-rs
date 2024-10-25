pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = 0i32;
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = 1i32;
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = 2i32;
pub const GPS_BESTEFFORT: GETPROPERTYSTOREFLAGS = 64i32;
pub const GPS_DEFAULT: GETPROPERTYSTOREFLAGS = 0i32;
pub const GPS_DELAYCREATION: GETPROPERTYSTOREFLAGS = 32i32;
pub const GPS_EXTRINSICPROPERTIES: GETPROPERTYSTOREFLAGS = 512i32;
pub const GPS_EXTRINSICPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 1024i32;
pub const GPS_FASTPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 8i32;
pub const GPS_HANDLERPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 1i32;
pub const GPS_MASK_VALID: GETPROPERTYSTOREFLAGS = 8191i32;
pub const GPS_NO_OPLOCK: GETPROPERTYSTOREFLAGS = 128i32;
pub const GPS_OPENSLOWITEM: GETPROPERTYSTOREFLAGS = 16i32;
pub const GPS_PREFERQUERYPROPERTIES: GETPROPERTYSTOREFLAGS = 256i32;
pub const GPS_READWRITE: GETPROPERTYSTOREFLAGS = 2i32;
pub const GPS_TEMPORARY: GETPROPERTYSTOREFLAGS = 4i32;
pub const GPS_VOLATILEPROPERTIES: GETPROPERTYSTOREFLAGS = 2048i32;
pub const GPS_VOLATILEPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 4096i32;
pub const PDAT_AVERAGE: PROPDESC_AGGREGATION_TYPE = 3i32;
pub const PDAT_DATERANGE: PROPDESC_AGGREGATION_TYPE = 4i32;
pub const PDAT_DEFAULT: PROPDESC_AGGREGATION_TYPE = 0i32;
pub const PDAT_FIRST: PROPDESC_AGGREGATION_TYPE = 1i32;
pub const PDAT_MAX: PROPDESC_AGGREGATION_TYPE = 6i32;
pub const PDAT_MIN: PROPDESC_AGGREGATION_TYPE = 7i32;
pub const PDAT_SUM: PROPDESC_AGGREGATION_TYPE = 2i32;
pub const PDAT_UNION: PROPDESC_AGGREGATION_TYPE = 5i32;
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = 2i32;
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = 0i32;
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = 3i32;
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = 1i32;
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = 4i32;
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = 5i32;
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = 4i32;
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = 3i32;
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = 0i32;
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = 5i32;
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = 2i32;
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = 1i32;
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = 2i32;
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = 3i32;
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = 4i32;
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = 1i32;
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = 0i32;
pub const PDEF_ALL: PROPDESC_ENUMFILTER = 0i32;
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = 6i32;
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = 5i32;
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = 2i32;
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = 4i32;
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = 1i32;
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = 3i32;
pub const PDFF_ALWAYSKB: PROPDESC_FORMAT_FLAGS = 4i32;
pub const PDFF_DEFAULT: PROPDESC_FORMAT_FLAGS = 0i32;
pub const PDFF_FILENAME: PROPDESC_FORMAT_FLAGS = 2i32;
pub const PDFF_HIDEDATE: PROPDESC_FORMAT_FLAGS = 512i32;
pub const PDFF_HIDETIME: PROPDESC_FORMAT_FLAGS = 64i32;
pub const PDFF_LONGDATE: PROPDESC_FORMAT_FLAGS = 256i32;
pub const PDFF_LONGTIME: PROPDESC_FORMAT_FLAGS = 32i32;
pub const PDFF_NOAUTOREADINGORDER: PROPDESC_FORMAT_FLAGS = 8192i32;
pub const PDFF_PREFIXNAME: PROPDESC_FORMAT_FLAGS = 1i32;
pub const PDFF_READONLY: PROPDESC_FORMAT_FLAGS = 4096i32;
pub const PDFF_RELATIVEDATE: PROPDESC_FORMAT_FLAGS = 1024i32;
pub const PDFF_RESERVED_RIGHTTOLEFT: PROPDESC_FORMAT_FLAGS = 8i32;
pub const PDFF_SHORTDATE: PROPDESC_FORMAT_FLAGS = 128i32;
pub const PDFF_SHORTTIME: PROPDESC_FORMAT_FLAGS = 16i32;
pub const PDFF_USEEDITINVITATION: PROPDESC_FORMAT_FLAGS = 2048i32;
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = 1i32;
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = 4i32;
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = 0i32;
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = 3i32;
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = 6i32;
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = 5i32;
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = 2i32;
pub const PDOPS_CANCELLED: PDOPSTATUS = 3i32;
pub const PDOPS_ERRORS: PDOPSTATUS = 5i32;
pub const PDOPS_PAUSED: PDOPSTATUS = 2i32;
pub const PDOPS_RUNNING: PDOPSTATUS = 1i32;
pub const PDOPS_STOPPED: PDOPSTATUS = 4i32;
pub const PDRDT_COUNT: PROPDESC_RELATIVEDESCRIPTION_TYPE = 3i32;
pub const PDRDT_DATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 1i32;
pub const PDRDT_DURATION: PROPDESC_RELATIVEDESCRIPTION_TYPE = 6i32;
pub const PDRDT_GENERAL: PROPDESC_RELATIVEDESCRIPTION_TYPE = 0i32;
pub const PDRDT_LENGTH: PROPDESC_RELATIVEDESCRIPTION_TYPE = 5i32;
pub const PDRDT_PRIORITY: PROPDESC_RELATIVEDESCRIPTION_TYPE = 10i32;
pub const PDRDT_RATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 8i32;
pub const PDRDT_RATING: PROPDESC_RELATIVEDESCRIPTION_TYPE = 9i32;
pub const PDRDT_REVISION: PROPDESC_RELATIVEDESCRIPTION_TYPE = 4i32;
pub const PDRDT_SIZE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 2i32;
pub const PDRDT_SPEED: PROPDESC_RELATIVEDESCRIPTION_TYPE = 7i32;
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = 1i32;
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = 0i32;
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = 2i32;
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = 4i32;
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = 3i32;
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = 8i32;
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = 0i32;
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = 1i32;
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = 2i32;
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = 4i32;
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = 16i32;
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: PROPDESC_TYPE_FLAGS = 4096u32;
pub const PDTF_CANBEPURGED: PROPDESC_TYPE_FLAGS = 512u32;
pub const PDTF_CANGROUPBY: PROPDESC_TYPE_FLAGS = 8u32;
pub const PDTF_CANSTACKBY: PROPDESC_TYPE_FLAGS = 16u32;
pub const PDTF_DEFAULT: PROPDESC_TYPE_FLAGS = 0u32;
pub const PDTF_DONTCOERCEEMPTYSTRINGS: PROPDESC_TYPE_FLAGS = 2048u32;
pub const PDTF_INCLUDEINFULLTEXTQUERY: PROPDESC_TYPE_FLAGS = 64u32;
pub const PDTF_ISGROUP: PROPDESC_TYPE_FLAGS = 4u32;
pub const PDTF_ISINNATE: PROPDESC_TYPE_FLAGS = 2u32;
pub const PDTF_ISQUERYABLE: PROPDESC_TYPE_FLAGS = 256u32;
pub const PDTF_ISSYSTEMPROPERTY: PROPDESC_TYPE_FLAGS = 2147483648u32;
pub const PDTF_ISTREEPROPERTY: PROPDESC_TYPE_FLAGS = 32u32;
pub const PDTF_ISVIEWABLE: PROPDESC_TYPE_FLAGS = 128u32;
pub const PDTF_MASK_ALL: PROPDESC_TYPE_FLAGS = 2147491839u32;
pub const PDTF_MULTIPLEVALUES: PROPDESC_TYPE_FLAGS = 1u32;
pub const PDTF_SEARCHRAWVALUE: PROPDESC_TYPE_FLAGS = 1024u32;
pub const PDVF_BEGINNEWGROUP: PROPDESC_VIEW_FLAGS = 4i32;
pub const PDVF_CANWRAP: PROPDESC_VIEW_FLAGS = 4096i32;
pub const PDVF_CENTERALIGN: PROPDESC_VIEW_FLAGS = 1i32;
pub const PDVF_DEFAULT: PROPDESC_VIEW_FLAGS = 0i32;
pub const PDVF_FILLAREA: PROPDESC_VIEW_FLAGS = 8i32;
pub const PDVF_HIDDEN: PROPDESC_VIEW_FLAGS = 2048i32;
pub const PDVF_HIDELABEL: PROPDESC_VIEW_FLAGS = 512i32;
pub const PDVF_MASK_ALL: PROPDESC_VIEW_FLAGS = 7167i32;
pub const PDVF_RIGHTALIGN: PROPDESC_VIEW_FLAGS = 2i32;
pub const PDVF_SHOWBYDEFAULT: PROPDESC_VIEW_FLAGS = 64i32;
pub const PDVF_SHOWINPRIMARYLIST: PROPDESC_VIEW_FLAGS = 128i32;
pub const PDVF_SHOWINSECONDARYLIST: PROPDESC_VIEW_FLAGS = 256i32;
pub const PDVF_SHOWONLYIFPRESENT: PROPDESC_VIEW_FLAGS = 32i32;
pub const PDVF_SORTDESCENDING: PROPDESC_VIEW_FLAGS = 16i32;
pub const PET_DEFAULTVALUE: PROPENUMTYPE = 2i32;
pub const PET_DISCRETEVALUE: PROPENUMTYPE = 0i32;
pub const PET_ENDRANGE: PROPENUMTYPE = 3i32;
pub const PET_RANGEDVALUE: PROPENUMTYPE = 1i32;
pub const PKA_APPEND: PKA_FLAGS = 1i32;
pub const PKA_DELETE: PKA_FLAGS = 2i32;
pub const PKA_SET: PKA_FLAGS = 0i32;
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
pub const PSC_DIRTY: PSC_STATE = 2i32;
pub const PSC_NORMAL: PSC_STATE = 0i32;
pub const PSC_NOTINSOURCE: PSC_STATE = 1i32;
pub const PSC_READONLY: PSC_STATE = 3i32;
pub const PS_ALL: PLACEHOLDER_STATES = 15i32;
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = 8i32;
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = 4i32;
pub const PS_DEFAULT: PLACEHOLDER_STATES = 7i32;
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = 2i32;
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = 1i32;
pub const PS_NONE: PLACEHOLDER_STATES = 0i32;
pub const PUIFFDF_DEFAULT: PROPERTYUI_FORMAT_FLAGS = 0i32;
pub const PUIFFDF_FRIENDLYDATE: PROPERTYUI_FORMAT_FLAGS = 8i32;
pub const PUIFFDF_NOTIME: PROPERTYUI_FORMAT_FLAGS = 4i32;
pub const PUIFFDF_RIGHTTOLEFT: PROPERTYUI_FORMAT_FLAGS = 1i32;
pub const PUIFFDF_SHORTFORMAT: PROPERTYUI_FORMAT_FLAGS = 2i32;
pub const PUIFNF_DEFAULT: PROPERTYUI_NAME_FLAGS = 0i32;
pub const PUIFNF_MNEMONIC: PROPERTYUI_NAME_FLAGS = 1i32;
pub const PUIF_DEFAULT: PROPERTYUI_FLAGS = 0i32;
pub const PUIF_NOLABELININFOTIP: PROPERTYUI_FLAGS = 2i32;
pub const PUIF_RIGHTALIGN: PROPERTYUI_FLAGS = 1i32;
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = 511i32;
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = 4i32;
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = 0i32;
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = 32i32;
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = 16i32;
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = 8i32;
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = 64i32;
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = 256i32;
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = 2i32;
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = 1i32;
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = 128i32;
pub const STS_EXCLUDED: SYNC_TRANSFER_STATUS = 256i32;
pub const STS_FETCHING_METADATA: SYNC_TRANSFER_STATUS = 32i32;
pub const STS_HASERROR: SYNC_TRANSFER_STATUS = 16i32;
pub const STS_HASWARNING: SYNC_TRANSFER_STATUS = 128i32;
pub const STS_INCOMPLETE: SYNC_TRANSFER_STATUS = 512i32;
pub const STS_NEEDSDOWNLOAD: SYNC_TRANSFER_STATUS = 2i32;
pub const STS_NEEDSUPLOAD: SYNC_TRANSFER_STATUS = 1i32;
pub const STS_NONE: SYNC_TRANSFER_STATUS = 0i32;
pub const STS_PAUSED: SYNC_TRANSFER_STATUS = 8i32;
pub const STS_PLACEHOLDER_IFEMPTY: SYNC_TRANSFER_STATUS = 1024i32;
pub const STS_TRANSFERRING: SYNC_TRANSFER_STATUS = 4i32;
pub const STS_USER_REQUESTED_REFRESH: SYNC_TRANSFER_STATUS = 64i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GETPROPERTYSTOREFLAGS(pub i32);
impl windows_core::TypeKind for GETPROPERTYSTOREFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PDOPSTATUS(pub i32);
impl windows_core::TypeKind for PDOPSTATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PKA_FLAGS(pub i32);
impl windows_core::TypeKind for PKA_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PLACEHOLDER_STATES(pub i32);
impl windows_core::TypeKind for PLACEHOLDER_STATES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_AGGREGATION_TYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_AGGREGATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_COLUMNINDEX_TYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_COLUMNINDEX_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_CONDITION_TYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_CONDITION_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_DISPLAYTYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_DISPLAYTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_ENUMFILTER(pub i32);
impl windows_core::TypeKind for PROPDESC_ENUMFILTER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_FORMAT_FLAGS(pub i32);
impl windows_core::TypeKind for PROPDESC_FORMAT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_GROUPING_RANGE(pub i32);
impl windows_core::TypeKind for PROPDESC_GROUPING_RANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_RELATIVEDESCRIPTION_TYPE(pub i32);
impl windows_core::TypeKind for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_SEARCHINFO_FLAGS(pub i32);
impl windows_core::TypeKind for PROPDESC_SEARCHINFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_SORTDESCRIPTION(pub i32);
impl windows_core::TypeKind for PROPDESC_SORTDESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_TYPE_FLAGS(pub u32);
impl windows_core::TypeKind for PROPDESC_TYPE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPDESC_VIEW_FLAGS(pub i32);
impl windows_core::TypeKind for PROPDESC_VIEW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPENUMTYPE(pub i32);
impl windows_core::TypeKind for PROPENUMTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPERTYUI_FLAGS(pub i32);
impl windows_core::TypeKind for PROPERTYUI_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPERTYUI_FORMAT_FLAGS(pub i32);
impl windows_core::TypeKind for PROPERTYUI_FORMAT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROPERTYUI_NAME_FLAGS(pub i32);
impl windows_core::TypeKind for PROPERTYUI_NAME_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PSC_STATE(pub i32);
impl windows_core::TypeKind for PSC_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNC_ENGINE_STATE_FLAGS(pub i32);
impl windows_core::TypeKind for SYNC_ENGINE_STATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYNC_TRANSFER_STATUS(pub i32);
impl windows_core::TypeKind for SYNC_TRANSFER_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _PERSIST_SPROPSTORE_FLAGS(pub i32);
impl windows_core::TypeKind for _PERSIST_SPROPSTORE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
pub const InMemoryPropertyStore: windows_core::GUID = windows_core::GUID::from_u128(0x9a02e012_6303_4e1e_b9a1_630f802592c5);
pub const InMemoryPropertyStoreMarshalByValue: windows_core::GUID = windows_core::GUID::from_u128(0xd4ca0e2d_6da7_4b75_a97c_5f306f0eaedc);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROPERTYKEY {
    pub fmtid: windows_core::GUID,
    pub pid: u32,
}
impl Default for PROPERTYKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROPERTYKEY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [i8; 30],
    pub achCmdLine: [i8; 128],
    pub achWorkDir: [i8; 64],
    pub wHotKey: u16,
    pub achIconFile: [i8; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [i8; 80],
    pub achPIFFile: [i8; 260],
}
impl Default for PROPPRG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROPPRG {
    type TypeKind = windows_core::CopyType;
}
pub const PropertySystem: windows_core::GUID = windows_core::GUID::from_u128(0xb8967f85_58ae_4f46_9fb2_5d7904798f4b);
