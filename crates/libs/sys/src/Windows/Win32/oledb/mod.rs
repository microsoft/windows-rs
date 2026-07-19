pub type ACCESS_MASKENUM = i32;
pub const BMK_DURABILITY_INTRANSACTION: u32 = 1;
pub const BMK_DURABILITY_REORGANIZATION: u32 = 3;
pub const BMK_DURABILITY_ROWSET: u32 = 0;
pub const BMK_DURABILITY_XTRANSACTION: u32 = 2;
pub const CRESTRICTIONS_DBSCHEMA_ASSERTIONS: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_CATALOGS: u32 = 1;
pub const CRESTRICTIONS_DBSCHEMA_CHARACTER_SETS: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_CHECK_CONSTRAINTS: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_CHECK_CONSTRAINTS_BY_TABLE: u32 = 6;
pub const CRESTRICTIONS_DBSCHEMA_COLLATIONS: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_COLUMNS: u32 = 4;
pub const CRESTRICTIONS_DBSCHEMA_COLUMN_DOMAIN_USAGE: u32 = 4;
pub const CRESTRICTIONS_DBSCHEMA_COLUMN_PRIVILEGES: u32 = 6;
pub const CRESTRICTIONS_DBSCHEMA_CONSTRAINT_COLUMN_USAGE: u32 = 4;
pub const CRESTRICTIONS_DBSCHEMA_CONSTRAINT_TABLE_USAGE: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_FOREIGN_KEYS: u32 = 6;
pub const CRESTRICTIONS_DBSCHEMA_INDEXES: u32 = 5;
pub const CRESTRICTIONS_DBSCHEMA_KEY_COLUMN_USAGE: u32 = 7;
pub const CRESTRICTIONS_DBSCHEMA_OBJECTS: u32 = 1;
pub const CRESTRICTIONS_DBSCHEMA_OBJECT_ACTIONS: u32 = 1;
pub const CRESTRICTIONS_DBSCHEMA_PRIMARY_KEYS: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURES: u32 = 4;
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURE_COLUMNS: u32 = 4;
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURE_PARAMETERS: u32 = 4;
pub const CRESTRICTIONS_DBSCHEMA_PROVIDER_TYPES: u32 = 2;
pub const CRESTRICTIONS_DBSCHEMA_REFERENTIAL_CONSTRAINTS: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_SCHEMATA: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_SQL_LANGUAGES: u32 = 0;
pub const CRESTRICTIONS_DBSCHEMA_STATISTICS: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_TABLES: u32 = 4;
pub const CRESTRICTIONS_DBSCHEMA_TABLES_INFO: u32 = 4;
pub const CRESTRICTIONS_DBSCHEMA_TABLE_CONSTRAINTS: u32 = 7;
pub const CRESTRICTIONS_DBSCHEMA_TABLE_PRIVILEGES: u32 = 5;
pub const CRESTRICTIONS_DBSCHEMA_TABLE_STATISTICS: u32 = 7;
pub const CRESTRICTIONS_DBSCHEMA_TRANSLATIONS: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_TRUSTEE: u32 = 4;
pub const CRESTRICTIONS_DBSCHEMA_USAGE_PRIVILEGES: u32 = 6;
pub const CRESTRICTIONS_DBSCHEMA_VIEWS: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_VIEW_COLUMN_USAGE: u32 = 3;
pub const CRESTRICTIONS_DBSCHEMA_VIEW_TABLE_USAGE: u32 = 3;
pub const CRESTRICTIONS_MDSCHEMA_ACTIONS: u32 = 8;
pub const CRESTRICTIONS_MDSCHEMA_COMMANDS: u32 = 5;
pub const CRESTRICTIONS_MDSCHEMA_CUBES: u32 = 3;
pub const CRESTRICTIONS_MDSCHEMA_DIMENSIONS: u32 = 5;
pub const CRESTRICTIONS_MDSCHEMA_FUNCTIONS: u32 = 4;
pub const CRESTRICTIONS_MDSCHEMA_HIERARCHIES: u32 = 6;
pub const CRESTRICTIONS_MDSCHEMA_LEVELS: u32 = 7;
pub const CRESTRICTIONS_MDSCHEMA_MEASURES: u32 = 5;
pub const CRESTRICTIONS_MDSCHEMA_MEMBERS: u32 = 12;
pub const CRESTRICTIONS_MDSCHEMA_PROPERTIES: u32 = 9;
pub const CRESTRICTIONS_MDSCHEMA_SETS: u32 = 5;
pub type DBACCESSORFLAGS = u32;
pub type DBACCESSORFLAGSENUM = i32;
pub const DBACCESSOR_INHERITED: DBACCESSORFLAGSENUM = 16;
pub const DBACCESSOR_INVALID: DBACCESSORFLAGSENUM = 0;
pub const DBACCESSOR_OPTIMIZED: DBACCESSORFLAGSENUM = 8;
pub const DBACCESSOR_PARAMETERDATA: DBACCESSORFLAGSENUM = 4;
pub const DBACCESSOR_PASSBYREF: DBACCESSORFLAGSENUM = 1;
pub const DBACCESSOR_ROWDATA: DBACCESSORFLAGSENUM = 2;
pub type DBASYNCHOP = u32;
pub type DBASYNCHOPENUM = i32;
pub const DBASYNCHOP_OPEN: DBASYNCHOPENUM = 0;
pub type DBASYNCHPHASE = u32;
pub type DBASYNCHPHASEENUM = i32;
pub const DBASYNCHPHASE_CANCELED: DBASYNCHPHASEENUM = 3;
pub const DBASYNCHPHASE_COMPLETE: DBASYNCHPHASEENUM = 2;
pub const DBASYNCHPHASE_INITIALIZATION: DBASYNCHPHASEENUM = 0;
pub const DBASYNCHPHASE_POPULATION: DBASYNCHPHASEENUM = 1;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBBINDEXT {
    pub pExtension: *mut u8,
    pub ulExtension: DBCOUNTITEM,
}
#[cfg(target_arch = "x86")]
impl Default for DBBINDEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBBINDEXT {
    pub pExtension: *mut u8,
    pub ulExtension: DBCOUNTITEM,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBBINDEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBBINDFLAG = u32;
pub type DBBINDFLAGENUM = i32;
pub const DBBINDFLAG_HTML: DBBINDFLAGENUM = 1;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy)]
pub struct DBBINDING {
    pub iOrdinal: DBORDINAL,
    pub obValue: DBBYTEOFFSET,
    pub obLength: DBBYTEOFFSET,
    pub obStatus: DBBYTEOFFSET,
    pub pTypeInfo: *mut core::ffi::c_void,
    pub pObject: *mut DBOBJECT,
    pub pBindExt: *mut DBBINDEXT,
    pub dwPart: DBPART,
    pub dwMemOwner: DBMEMOWNER,
    pub eParamIO: DBPARAMIO,
    pub cbMaxLen: DBLENGTH,
    pub dwFlags: u32,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "oaidl")]
impl Default for DBBINDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy)]
pub struct DBBINDING {
    pub iOrdinal: DBORDINAL,
    pub obValue: DBBYTEOFFSET,
    pub obLength: DBBYTEOFFSET,
    pub obStatus: DBBYTEOFFSET,
    pub pTypeInfo: *mut core::ffi::c_void,
    pub pObject: *mut DBOBJECT,
    pub pBindExt: *mut DBBINDEXT,
    pub dwPart: DBPART,
    pub dwMemOwner: DBMEMOWNER,
    pub eParamIO: DBPARAMIO,
    pub cbMaxLen: DBLENGTH,
    pub dwFlags: u32,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
impl Default for DBBINDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBBINDSTATUS = u32;
pub type DBBINDSTATUSENUM = i32;
pub const DBBINDSTATUS_BADBINDINFO: DBBINDSTATUSENUM = 3;
pub const DBBINDSTATUS_BADORDINAL: DBBINDSTATUSENUM = 1;
pub const DBBINDSTATUS_BADSTORAGEFLAGS: DBBINDSTATUSENUM = 4;
pub const DBBINDSTATUS_MULTIPLESTORAGE: DBBINDSTATUSENUM = 6;
pub const DBBINDSTATUS_NOINTERFACE: DBBINDSTATUSENUM = 5;
pub const DBBINDSTATUS_OK: DBBINDSTATUSENUM = 0;
pub const DBBINDSTATUS_UNSUPPORTEDCONVERSION: DBBINDSTATUSENUM = 2;
pub type DBBINDURLFLAG = u32;
pub type DBBINDURLFLAGENUM = i32;
pub const DBBINDURLFLAG_ASYNCHRONOUS: DBBINDURLFLAGENUM = 4096;
pub const DBBINDURLFLAG_COLLECTION: DBBINDURLFLAGENUM = 8192;
pub const DBBINDURLFLAG_DELAYFETCHCOLUMNS: DBBINDURLFLAGENUM = 32768;
pub const DBBINDURLFLAG_DELAYFETCHSTREAM: DBBINDURLFLAGENUM = 16384;
pub const DBBINDURLFLAG_ISSTRUCTUREDDOCUMENT: DBBINDURLFLAGENUM = 134217728;
pub const DBBINDURLFLAG_OPENIFEXISTS: DBBINDURLFLAGENUM = 33554432;
pub const DBBINDURLFLAG_OUTPUT: DBBINDURLFLAGENUM = 8388608;
pub const DBBINDURLFLAG_OVERWRITE: DBBINDURLFLAGENUM = 67108864;
pub const DBBINDURLFLAG_READ: DBBINDURLFLAGENUM = 1;
pub const DBBINDURLFLAG_READWRITE: DBBINDURLFLAGENUM = 3;
pub const DBBINDURLFLAG_RECURSIVE: DBBINDURLFLAGENUM = 4194304;
pub const DBBINDURLFLAG_SHARE_DENY_NONE: DBBINDURLFLAGENUM = 16;
pub const DBBINDURLFLAG_SHARE_DENY_READ: DBBINDURLFLAGENUM = 4;
pub const DBBINDURLFLAG_SHARE_DENY_WRITE: DBBINDURLFLAGENUM = 8;
pub const DBBINDURLFLAG_SHARE_EXCLUSIVE: DBBINDURLFLAGENUM = 12;
pub const DBBINDURLFLAG_WAITFORINIT: DBBINDURLFLAGENUM = 16777216;
pub const DBBINDURLFLAG_WRITE: DBBINDURLFLAGENUM = 2;
pub type DBBINDURLSTATUS = u32;
pub type DBBINDURLSTATUSENUM = i32;
pub const DBBINDURLSTATUS_S_DENYNOTSUPPORTED: DBBINDURLSTATUSENUM = 1;
pub const DBBINDURLSTATUS_S_DENYTYPENOTSUPPORTED: DBBINDURLSTATUSENUM = 4;
pub const DBBINDURLSTATUS_S_OK: DBBINDURLSTATUSENUM = 0;
pub const DBBINDURLSTATUS_S_REDIRECTED: DBBINDURLSTATUSENUM = 8;
#[cfg(target_arch = "x86")]
pub type DBBKMARK = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DBBKMARK = u64;
pub const DBBMK_FIRST: DBBOOKMARK = 1;
pub const DBBMK_INVALID: DBBOOKMARK = 0;
pub const DBBMK_LAST: DBBOOKMARK = 2;
pub type DBBOOKMARK = i32;
#[cfg(target_arch = "x86")]
pub type DBBYTEOFFSET = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DBBYTEOFFSET = u64;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DBCOLUMNACCESS {
    pub pData: *mut core::ffi::c_void,
    pub columnid: DBID,
    pub cbDataLen: DBLENGTH,
    pub dwStatus: DBSTATUS,
    pub cbMaxLen: DBLENGTH,
    pub dwReserved: DB_DWRESERVE,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DBCOLUMNACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DBCOLUMNACCESS {
    pub pData: *mut core::ffi::c_void,
    pub columnid: DBID,
    pub cbDataLen: DBLENGTH,
    pub dwStatus: DBSTATUS,
    pub cbMaxLen: DBLENGTH,
    pub dwReserved: DB_DWRESERVE,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DBCOLUMNACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBCOLUMNDESC {
    pub pwszTypeName: windows_sys::core::PWSTR,
    pub pTypeInfo: *mut core::ffi::c_void,
    pub rgPropertySets: *mut DBPROPSET,
    pub pclsid: *mut windows_sys::core::GUID,
    pub cPropertySets: u32,
    pub ulColumnSize: DBLENGTH,
    pub dbcid: DBID,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBCOLUMNDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBCOLUMNDESC {
    pub pwszTypeName: windows_sys::core::PWSTR,
    pub pTypeInfo: *mut core::ffi::c_void,
    pub rgPropertySets: *mut DBPROPSET,
    pub pclsid: *mut windows_sys::core::GUID,
    pub cPropertySets: u32,
    pub ulColumnSize: DBLENGTH,
    pub dbcid: DBID,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBCOLUMNDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBCOLUMNDESCFLAGS = u32;
pub type DBCOLUMNDESCFLAGSENUM = i32;
pub const DBCOLUMNDESCFLAGS_CLSID: DBCOLUMNDESCFLAGSENUM = 8;
pub const DBCOLUMNDESCFLAGS_COLSIZE: DBCOLUMNDESCFLAGSENUM = 16;
pub const DBCOLUMNDESCFLAGS_DBCID: DBCOLUMNDESCFLAGSENUM = 32;
pub const DBCOLUMNDESCFLAGS_ITYPEINFO: DBCOLUMNDESCFLAGSENUM = 2;
pub const DBCOLUMNDESCFLAGS_PRECISION: DBCOLUMNDESCFLAGSENUM = 128;
pub const DBCOLUMNDESCFLAGS_PROPERTIES: DBCOLUMNDESCFLAGSENUM = 4;
pub const DBCOLUMNDESCFLAGS_SCALE: DBCOLUMNDESCFLAGSENUM = 256;
pub const DBCOLUMNDESCFLAGS_TYPENAME: DBCOLUMNDESCFLAGSENUM = 1;
pub const DBCOLUMNDESCFLAGS_WTYPE: DBCOLUMNDESCFLAGSENUM = 64;
pub type DBCOLUMNFLAGS = u32;
pub type DBCOLUMNFLAGS15ENUM = i32;
pub type DBCOLUMNFLAGSENUM = i32;
pub type DBCOLUMNFLAGSENUM20 = i32;
pub type DBCOLUMNFLAGSENUM21 = i32;
pub type DBCOLUMNFLAGSENUM26 = i32;
pub const DBCOLUMNFLAGS_CACHEDEFERRED: DBCOLUMNFLAGSENUM = 4096;
pub const DBCOLUMNFLAGS_ISBOOKMARK: DBCOLUMNFLAGSENUM = 1;
pub const DBCOLUMNFLAGS_ISCHAPTER: DBCOLUMNFLAGS15ENUM = 8192;
pub const DBCOLUMNFLAGS_ISCOLLECTION: DBCOLUMNFLAGSENUM21 = 262144;
pub const DBCOLUMNFLAGS_ISDEFAULTSTREAM: DBCOLUMNFLAGSENUM21 = 131072;
pub const DBCOLUMNFLAGS_ISFIXEDLENGTH: DBCOLUMNFLAGSENUM = 16;
pub const DBCOLUMNFLAGS_ISLONG: DBCOLUMNFLAGSENUM = 128;
pub const DBCOLUMNFLAGS_ISNULLABLE: DBCOLUMNFLAGSENUM = 32;
pub const DBCOLUMNFLAGS_ISROW: DBCOLUMNFLAGSENUM26 = 2097152;
pub const DBCOLUMNFLAGS_ISROWID: DBCOLUMNFLAGSENUM = 256;
pub const DBCOLUMNFLAGS_ISROWSET: DBCOLUMNFLAGSENUM26 = 1048576;
pub const DBCOLUMNFLAGS_ISROWURL: DBCOLUMNFLAGSENUM21 = 65536;
pub const DBCOLUMNFLAGS_ISROWVER: DBCOLUMNFLAGSENUM = 512;
pub const DBCOLUMNFLAGS_ISSTREAM: DBCOLUMNFLAGSENUM26 = 524288;
pub const DBCOLUMNFLAGS_MAYBENULL: DBCOLUMNFLAGSENUM = 64;
pub const DBCOLUMNFLAGS_MAYDEFER: DBCOLUMNFLAGSENUM = 2;
pub const DBCOLUMNFLAGS_RESERVED: DBCOLUMNFLAGSENUM20 = 32768;
pub const DBCOLUMNFLAGS_ROWSPECIFICCOLUMN: DBCOLUMNFLAGSENUM26 = 4194304;
pub const DBCOLUMNFLAGS_SCALEISNEGATIVE: DBCOLUMNFLAGSENUM20 = 16384;
pub const DBCOLUMNFLAGS_WRITE: DBCOLUMNFLAGSENUM = 4;
pub const DBCOLUMNFLAGS_WRITEUNKNOWN: DBCOLUMNFLAGSENUM = 8;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy)]
pub struct DBCOLUMNINFO {
    pub pwszName: windows_sys::core::PWSTR,
    pub pTypeInfo: *mut core::ffi::c_void,
    pub iOrdinal: DBORDINAL,
    pub dwFlags: DBCOLUMNFLAGS,
    pub ulColumnSize: DBLENGTH,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
    pub columnid: DBID,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "oaidl")]
impl Default for DBCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy)]
pub struct DBCOLUMNINFO {
    pub pwszName: windows_sys::core::PWSTR,
    pub pTypeInfo: *mut core::ffi::c_void,
    pub iOrdinal: DBORDINAL,
    pub dwFlags: DBCOLUMNFLAGS,
    pub ulColumnSize: DBLENGTH,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
    pub columnid: DBID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
impl Default for DBCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBCOMMANDPERSISTFLAG = u32;
pub type DBCOMMANDPERSISTFLAGENUM = i32;
pub type DBCOMMANDPERSISTFLAGENUM21 = i32;
pub const DBCOMMANDPERSISTFLAG_DEFAULT: DBCOMMANDPERSISTFLAGENUM21 = 0;
pub const DBCOMMANDPERSISTFLAG_NOSAVE: DBCOMMANDPERSISTFLAGENUM = 1;
pub const DBCOMMANDPERSISTFLAG_PERSISTPROCEDURE: DBCOMMANDPERSISTFLAGENUM21 = 4;
pub const DBCOMMANDPERSISTFLAG_PERSISTVIEW: DBCOMMANDPERSISTFLAGENUM21 = 2;
pub type DBCOMPARE = u32;
pub type DBCOMPAREENUM = i32;
pub type DBCOMPAREOP = u32;
pub type DBCOMPAREOPSENUM = i32;
pub type DBCOMPAREOPSENUM20 = i32;
pub const DBCOMPAREOPS_BEGINSWITH: DBCOMPAREOPSENUM = 5;
pub const DBCOMPAREOPS_CASEINSENSITIVE: DBCOMPAREOPSENUM = 8192;
pub const DBCOMPAREOPS_CASESENSITIVE: DBCOMPAREOPSENUM = 4096;
pub const DBCOMPAREOPS_CONTAINS: DBCOMPAREOPSENUM = 6;
pub const DBCOMPAREOPS_EQ: DBCOMPAREOPSENUM = 2;
pub const DBCOMPAREOPS_GE: DBCOMPAREOPSENUM = 3;
pub const DBCOMPAREOPS_GT: DBCOMPAREOPSENUM = 4;
pub const DBCOMPAREOPS_IGNORE: DBCOMPAREOPSENUM = 8;
pub const DBCOMPAREOPS_LE: DBCOMPAREOPSENUM = 1;
pub const DBCOMPAREOPS_LT: DBCOMPAREOPSENUM = 0;
pub const DBCOMPAREOPS_NE: DBCOMPAREOPSENUM = 7;
pub const DBCOMPAREOPS_NOTBEGINSWITH: DBCOMPAREOPSENUM20 = 9;
pub const DBCOMPAREOPS_NOTCONTAINS: DBCOMPAREOPSENUM20 = 10;
pub const DBCOMPARE_EQ: DBCOMPAREENUM = 1;
pub const DBCOMPARE_GT: DBCOMPAREENUM = 2;
pub const DBCOMPARE_LT: DBCOMPAREENUM = 0;
pub const DBCOMPARE_NE: DBCOMPAREENUM = 3;
pub const DBCOMPARE_NOTCOMPARABLE: DBCOMPAREENUM = 4;
pub const DBCOMPUTEMODE_COMPUTED: u32 = 1;
pub const DBCOMPUTEMODE_DYNAMIC: u32 = 2;
pub const DBCOMPUTEMODE_NOTCOMPUTED: u32 = 3;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBCONSTRAINTDESC {
    pub pConstraintID: *mut DBID,
    pub ConstraintType: DBCONSTRAINTTYPE,
    pub cColumns: DBORDINAL,
    pub rgColumnList: *mut DBID,
    pub pReferencedTableID: *mut DBID,
    pub cForeignKeyColumns: DBORDINAL,
    pub rgForeignKeyColumnList: *mut DBID,
    pub pwszConstraintText: *mut super::OLECHAR,
    pub UpdateRule: DBUPDELRULE,
    pub DeleteRule: DBUPDELRULE,
    pub MatchType: DBMATCHTYPE,
    pub Deferrability: DBDEFERRABILITY,
    pub cReserved: DB_URESERVE,
    pub rgReserved: *mut DBPROPSET,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBCONSTRAINTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBCONSTRAINTDESC {
    pub pConstraintID: *mut DBID,
    pub ConstraintType: DBCONSTRAINTTYPE,
    pub cColumns: DBORDINAL,
    pub rgColumnList: *mut DBID,
    pub pReferencedTableID: *mut DBID,
    pub cForeignKeyColumns: DBORDINAL,
    pub rgForeignKeyColumnList: *mut DBID,
    pub pwszConstraintText: *mut super::OLECHAR,
    pub UpdateRule: DBUPDELRULE,
    pub DeleteRule: DBUPDELRULE,
    pub MatchType: DBMATCHTYPE,
    pub Deferrability: DBDEFERRABILITY,
    pub cReserved: DB_URESERVE,
    pub rgReserved: *mut DBPROPSET,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBCONSTRAINTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBCONSTRAINTTYPE = u32;
pub type DBCONSTRAINTTYPEENUM = i32;
pub const DBCONSTRAINTTYPE_CHECK: DBCONSTRAINTTYPEENUM = 3;
pub const DBCONSTRAINTTYPE_FOREIGNKEY: DBCONSTRAINTTYPEENUM = 1;
pub const DBCONSTRAINTTYPE_PRIMARYKEY: DBCONSTRAINTTYPEENUM = 2;
pub const DBCONSTRAINTTYPE_UNIQUE: DBCONSTRAINTTYPEENUM = 0;
pub type DBCONVERTFLAGS = u32;
pub type DBCONVERTFLAGSENUM = i32;
pub type DBCONVERTFLAGSENUM20 = i32;
pub const DBCONVERTFLAGS_COLUMN: DBCONVERTFLAGSENUM = 0;
pub const DBCONVERTFLAGS_FROMVARIANT: DBCONVERTFLAGSENUM20 = 8;
pub const DBCONVERTFLAGS_ISFIXEDLENGTH: DBCONVERTFLAGSENUM20 = 4;
pub const DBCONVERTFLAGS_ISLONG: DBCONVERTFLAGSENUM20 = 2;
pub const DBCONVERTFLAGS_PARAMETER: DBCONVERTFLAGSENUM = 1;
pub type DBCOPYFLAGS = u32;
pub type DBCOPYFLAGSENUM = i32;
pub const DBCOPY_ALLOW_EMULATION: DBCOPYFLAGSENUM = 1024;
pub const DBCOPY_ASYNC: DBCOPYFLAGSENUM = 256;
pub const DBCOPY_ATOMIC: DBCOPYFLAGSENUM = 4096;
pub const DBCOPY_NON_RECURSIVE: DBCOPYFLAGSENUM = 2048;
pub const DBCOPY_REPLACE_EXISTING: DBCOPYFLAGSENUM = 512;
#[cfg(target_arch = "x86")]
pub type DBCOUNTITEM = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DBCOUNTITEM = u64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DBDATE {
    pub year: i16,
    pub month: u16,
    pub day: u16,
}
pub type DBDEFERRABILITY = u32;
pub type DBDEFERRABILITYENUM = i32;
pub const DBDEFERRABILITY_DEFERRABLE: DBDEFERRABILITYENUM = 2;
pub const DBDEFERRABILITY_DEFERRED: DBDEFERRABILITYENUM = 1;
pub type DBDELETEFLAGS = u32;
pub type DBDELETEFLAGSENUM = i32;
pub const DBDELETE_ASYNC: DBDELETEFLAGSENUM = 256;
pub const DBDELETE_ATOMIC: DBDELETEFLAGSENUM = 4096;
pub type DBEVENTPHASE = u32;
pub type DBEVENTPHASEENUM = i32;
pub const DBEVENTPHASE_ABOUTTODO: DBEVENTPHASEENUM = 1;
pub const DBEVENTPHASE_DIDEVENT: DBEVENTPHASEENUM = 4;
pub const DBEVENTPHASE_FAILEDTODO: DBEVENTPHASEENUM = 3;
pub const DBEVENTPHASE_OKTODO: DBEVENTPHASEENUM = 0;
pub const DBEVENTPHASE_SYNCHAFTER: DBEVENTPHASEENUM = 2;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct DBFAILUREINFO {
    pub hRow: HROW,
    pub iColumn: DBORDINAL,
    pub failure: windows_sys::core::HRESULT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct DBFAILUREINFO {
    pub hRow: HROW,
    pub iColumn: DBORDINAL,
    pub failure: windows_sys::core::HRESULT,
}
#[cfg(target_arch = "x86")]
pub type DBHASHVALUE = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
pub type DBHASHVALUE = super::DWORDLONG;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBID {
    pub uGuid: DBID_0,
    pub eKind: DBKIND,
    pub uName: DBID_1,
}
#[cfg(target_arch = "x86")]
impl Default for DBID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union DBID_0 {
    pub guid: windows_sys::core::GUID,
    pub pguid: *mut windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
impl Default for DBID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union DBID_1 {
    pub pwszName: windows_sys::core::PWSTR,
    pub ulPropid: u32,
}
#[cfg(target_arch = "x86")]
impl Default for DBID_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBID {
    pub uGuid: DBID_0,
    pub eKind: DBKIND,
    pub uName: DBID_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union DBID_0 {
    pub guid: windows_sys::core::GUID,
    pub pguid: *mut windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union DBID_1 {
    pub pwszName: windows_sys::core::PWSTR,
    pub ulPropid: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBID_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBIMPLICITSESSION {
    pub pUnkOuter: *mut core::ffi::c_void,
    pub piid: *mut windows_sys::core::GUID,
    pub pSession: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for DBIMPLICITSESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBIMPLICITSESSION {
    pub pUnkOuter: *mut core::ffi::c_void,
    pub piid: *mut windows_sys::core::GUID,
    pub pSession: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBIMPLICITSESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBINDEXCOLUMNDESC {
    pub pColumnID: *mut DBID,
    pub eIndexColOrder: DBINDEX_COL_ORDER,
}
#[cfg(target_arch = "x86")]
impl Default for DBINDEXCOLUMNDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBINDEXCOLUMNDESC {
    pub pColumnID: *mut DBID,
    pub eIndexColOrder: DBINDEX_COL_ORDER,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBINDEXCOLUMNDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBINDEX_COL_ORDER = u32;
pub type DBINDEX_COL_ORDERENUM = i32;
pub const DBINDEX_COL_ORDER_ASC: DBINDEX_COL_ORDERENUM = 0;
pub const DBINDEX_COL_ORDER_DESC: DBINDEX_COL_ORDERENUM = 1;
pub type DBKIND = u32;
pub type DBKINDENUM = i32;
pub const DBKIND_GUID: DBKINDENUM = 6;
pub const DBKIND_GUID_NAME: DBKINDENUM = 0;
pub const DBKIND_GUID_PROPID: DBKINDENUM = 1;
pub const DBKIND_NAME: DBKINDENUM = 2;
pub const DBKIND_PGUID_NAME: DBKINDENUM = 3;
pub const DBKIND_PGUID_PROPID: DBKINDENUM = 4;
pub const DBKIND_PROPID: DBKINDENUM = 5;
#[cfg(target_arch = "x86")]
pub type DBLENGTH = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DBLENGTH = u64;
pub type DBLITERAL = u32;
pub type DBLITERALENUM = i32;
pub type DBLITERALENUM20 = i32;
pub type DBLITERALENUM21 = i32;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBLITERALINFO {
    pub pwszLiteralValue: windows_sys::core::PWSTR,
    pub pwszInvalidChars: windows_sys::core::PWSTR,
    pub pwszInvalidStartingChars: windows_sys::core::PWSTR,
    pub lt: DBLITERAL,
    pub fSupported: windows_sys::core::BOOL,
    pub cchMaxLen: u32,
}
#[cfg(target_arch = "x86")]
impl Default for DBLITERALINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBLITERALINFO {
    pub pwszLiteralValue: windows_sys::core::PWSTR,
    pub pwszInvalidChars: windows_sys::core::PWSTR,
    pub pwszInvalidStartingChars: windows_sys::core::PWSTR,
    pub lt: DBLITERAL,
    pub fSupported: windows_sys::core::BOOL,
    pub cchMaxLen: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBLITERALINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DBLITERAL_BINARY_LITERAL: DBLITERALENUM = 1;
pub const DBLITERAL_CATALOG_NAME: DBLITERALENUM = 2;
pub const DBLITERAL_CATALOG_SEPARATOR: DBLITERALENUM = 3;
pub const DBLITERAL_CHAR_LITERAL: DBLITERALENUM = 4;
pub const DBLITERAL_COLUMN_ALIAS: DBLITERALENUM = 5;
pub const DBLITERAL_COLUMN_NAME: DBLITERALENUM = 6;
pub const DBLITERAL_CORRELATION_NAME: DBLITERALENUM = 7;
pub const DBLITERAL_CUBE_NAME: DBLITERALENUM20 = 21;
pub const DBLITERAL_CURSOR_NAME: DBLITERALENUM = 8;
pub const DBLITERAL_DIMENSION_NAME: DBLITERALENUM20 = 22;
pub const DBLITERAL_ESCAPE_PERCENT: DBLITERALENUM = 9;
pub const DBLITERAL_ESCAPE_PERCENT_PREFIX: u32 = 9;
pub const DBLITERAL_ESCAPE_PERCENT_SUFFIX: DBLITERALENUM21 = 29;
pub const DBLITERAL_ESCAPE_UNDERSCORE: DBLITERALENUM = 10;
pub const DBLITERAL_ESCAPE_UNDERSCORE_PREFIX: u32 = 10;
pub const DBLITERAL_ESCAPE_UNDERSCORE_SUFFIX: DBLITERALENUM21 = 30;
pub const DBLITERAL_HIERARCHY_NAME: DBLITERALENUM20 = 23;
pub const DBLITERAL_INDEX_NAME: DBLITERALENUM = 11;
pub const DBLITERAL_INVALID: DBLITERALENUM = 0;
pub const DBLITERAL_LEVEL_NAME: DBLITERALENUM20 = 24;
pub const DBLITERAL_LIKE_PERCENT: DBLITERALENUM = 12;
pub const DBLITERAL_LIKE_UNDERSCORE: DBLITERALENUM = 13;
pub const DBLITERAL_MEMBER_NAME: DBLITERALENUM20 = 25;
pub const DBLITERAL_PROCEDURE_NAME: DBLITERALENUM = 14;
pub const DBLITERAL_PROPERTY_NAME: DBLITERALENUM20 = 26;
pub const DBLITERAL_QUOTE: DBLITERALENUM = 15;
pub const DBLITERAL_QUOTE_PREFIX: u32 = 15;
pub const DBLITERAL_QUOTE_SUFFIX: DBLITERALENUM20 = 28;
pub const DBLITERAL_SCHEMA_NAME: DBLITERALENUM = 16;
pub const DBLITERAL_SCHEMA_SEPARATOR: DBLITERALENUM20 = 27;
pub const DBLITERAL_TABLE_NAME: DBLITERALENUM = 17;
pub const DBLITERAL_TEXT_COMMAND: DBLITERALENUM = 18;
pub const DBLITERAL_USER_NAME: DBLITERALENUM = 19;
pub const DBLITERAL_VIEW_NAME: DBLITERALENUM = 20;
pub type DBMATCHTYPE = u32;
pub type DBMATCHTYPEENUM = i32;
pub const DBMATCHTYPE_FULL: DBMATCHTYPEENUM = 0;
pub const DBMATCHTYPE_NONE: DBMATCHTYPEENUM = 1;
pub const DBMATCHTYPE_PARTIAL: DBMATCHTYPEENUM = 2;
pub type DBMEMOWNER = u32;
pub type DBMEMOWNERENUM = i32;
pub const DBMEMOWNER_CLIENTOWNED: DBMEMOWNERENUM = 0;
pub const DBMEMOWNER_PROVIDEROWNED: DBMEMOWNERENUM = 1;
pub type DBMOVEFLAGS = u32;
pub type DBMOVEFLAGSENUM = i32;
pub const DBMOVE_ALLOW_EMULATION: DBMOVEFLAGSENUM = 1024;
pub const DBMOVE_ASYNC: DBMOVEFLAGSENUM = 256;
pub const DBMOVE_ATOMIC: DBMOVEFLAGSENUM = 4096;
pub const DBMOVE_DONT_UPDATE_LINKS: DBMOVEFLAGSENUM = 512;
pub const DBMOVE_REPLACE_EXISTING: DBMOVEFLAGSENUM = 1;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct DBOBJECT {
    pub dwFlags: u32,
    pub iid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct DBOBJECT {
    pub dwFlags: u32,
    pub iid: windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
pub type DBORDINAL = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DBORDINAL = u64;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBPARAMBINDINFO {
    pub pwszDataSourceType: windows_sys::core::PWSTR,
    pub pwszName: windows_sys::core::PWSTR,
    pub ulParamSize: DBLENGTH,
    pub dwFlags: DBPARAMFLAGS,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
impl Default for DBPARAMBINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBPARAMBINDINFO {
    pub pwszDataSourceType: windows_sys::core::PWSTR,
    pub pwszName: windows_sys::core::PWSTR,
    pub ulParamSize: DBLENGTH,
    pub dwFlags: DBPARAMFLAGS,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBPARAMBINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBPARAMFLAGS = u32;
pub type DBPARAMFLAGSENUM = i32;
pub type DBPARAMFLAGSENUM20 = i32;
pub const DBPARAMFLAGS_ISINPUT: DBPARAMFLAGSENUM = 1;
pub const DBPARAMFLAGS_ISLONG: DBPARAMFLAGSENUM = 128;
pub const DBPARAMFLAGS_ISNULLABLE: DBPARAMFLAGSENUM = 64;
pub const DBPARAMFLAGS_ISOUTPUT: DBPARAMFLAGSENUM = 2;
pub const DBPARAMFLAGS_ISSIGNED: DBPARAMFLAGSENUM = 16;
pub const DBPARAMFLAGS_SCALEISNEGATIVE: DBPARAMFLAGSENUM20 = 256;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy)]
pub struct DBPARAMINFO {
    pub dwFlags: DBPARAMFLAGS,
    pub iOrdinal: DBORDINAL,
    pub pwszName: windows_sys::core::PWSTR,
    pub pTypeInfo: *mut core::ffi::c_void,
    pub ulParamSize: DBLENGTH,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "oaidl")]
impl Default for DBPARAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy)]
pub struct DBPARAMINFO {
    pub dwFlags: DBPARAMFLAGS,
    pub iOrdinal: DBORDINAL,
    pub pwszName: windows_sys::core::PWSTR,
    pub pTypeInfo: *mut core::ffi::c_void,
    pub ulParamSize: DBLENGTH,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
impl Default for DBPARAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBPARAMIO = u32;
pub type DBPARAMIOENUM = i32;
pub const DBPARAMIO_INPUT: DBPARAMIOENUM = 1;
pub const DBPARAMIO_NOTPARAM: DBPARAMIOENUM = 0;
pub const DBPARAMIO_OUTPUT: DBPARAMIOENUM = 2;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBPARAMS {
    pub pData: *mut core::ffi::c_void,
    pub cParamSets: DB_UPARAMS,
    pub hAccessor: HACCESSOR,
}
#[cfg(target_arch = "x86")]
impl Default for DBPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBPARAMS {
    pub pData: *mut core::ffi::c_void,
    pub cParamSets: DB_UPARAMS,
    pub hAccessor: HACCESSOR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DBPARAMTYPE_INPUT: u32 = 1;
pub const DBPARAMTYPE_INPUTOUTPUT: u32 = 2;
pub const DBPARAMTYPE_OUTPUT: u32 = 3;
pub const DBPARAMTYPE_RETURNVALUE: u32 = 4;
pub type DBPART = u32;
pub type DBPARTENUM = i32;
pub const DBPART_INVALID: DBPARTENUM = 0;
pub const DBPART_LENGTH: DBPARTENUM = 2;
pub const DBPART_STATUS: DBPARTENUM = 4;
pub const DBPART_VALUE: DBPARTENUM = 1;
pub type DBPENDINGSTATUS = u32;
pub type DBPENDINGSTATUSENUM = i32;
pub const DBPENDINGSTATUS_CHANGED: DBPENDINGSTATUSENUM = 2;
pub const DBPENDINGSTATUS_DELETED: DBPENDINGSTATUSENUM = 4;
pub const DBPENDINGSTATUS_INVALIDROW: DBPENDINGSTATUSENUM = 16;
pub const DBPENDINGSTATUS_NEW: DBPENDINGSTATUSENUM = 1;
pub const DBPENDINGSTATUS_UNCHANGED: DBPENDINGSTATUSENUM = 8;
pub type DBPOSITIONFLAGS = u32;
pub type DBPOSITIONFLAGSENUM = i32;
pub const DBPOSITION_BOF: DBPOSITIONFLAGSENUM = 2;
pub const DBPOSITION_EOF: DBPOSITIONFLAGSENUM = 3;
pub const DBPOSITION_NOROW: DBPOSITIONFLAGSENUM = 1;
pub const DBPOSITION_OK: DBPOSITIONFLAGSENUM = 0;
pub const DBPROMPT_COMPLETE: u32 = 2;
pub const DBPROMPT_COMPLETEREQUIRED: u32 = 3;
pub const DBPROMPT_NOPROMPT: u32 = 4;
pub const DBPROMPT_PROMPT: u32 = 1;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBPROP {
    pub dwPropertyID: DBPROPID,
    pub dwOptions: DBPROPOPTIONS,
    pub dwStatus: DBPROPSTATUS,
    pub colid: DBID,
    pub vValue: super::VARIANT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBPROP {
    pub dwPropertyID: DBPROPID,
    pub dwOptions: DBPROPOPTIONS,
    pub dwStatus: DBPROPSTATUS,
    pub colid: DBID,
    pub vValue: super::VARIANT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBPROPENUM = i32;
pub type DBPROPENUM15 = i32;
pub type DBPROPENUM20 = i32;
pub type DBPROPENUM21 = i32;
pub type DBPROPENUM25 = i32;
pub type DBPROPENUM26 = i32;
pub type DBPROPFLAGS = u32;
pub type DBPROPFLAGSENUM = i32;
pub type DBPROPFLAGSENUM21 = i32;
pub type DBPROPFLAGSENUM25 = i32;
pub type DBPROPFLAGSENUM26 = i32;
pub const DBPROPFLAGS_COLUMN: DBPROPFLAGSENUM = 1;
pub const DBPROPFLAGS_COLUMNOK: DBPROPFLAGSENUM = 256;
pub const DBPROPFLAGS_DATASOURCE: DBPROPFLAGSENUM = 2;
pub const DBPROPFLAGS_DATASOURCECREATE: DBPROPFLAGSENUM = 4;
pub const DBPROPFLAGS_DATASOURCEINFO: DBPROPFLAGSENUM = 8;
pub const DBPROPFLAGS_DBINIT: DBPROPFLAGSENUM = 16;
pub const DBPROPFLAGS_INDEX: DBPROPFLAGSENUM = 32;
pub const DBPROPFLAGS_NOTSUPPORTED: DBPROPFLAGSENUM = 0;
pub const DBPROPFLAGS_READ: DBPROPFLAGSENUM = 512;
pub const DBPROPFLAGS_REQUIRED: DBPROPFLAGSENUM = 2048;
pub const DBPROPFLAGS_ROWSET: DBPROPFLAGSENUM = 64;
pub const DBPROPFLAGS_SESSION: DBPROPFLAGSENUM = 4096;
pub const DBPROPFLAGS_STREAM: DBPROPFLAGSENUM26 = 32768;
pub const DBPROPFLAGS_TABLE: DBPROPFLAGSENUM = 128;
pub const DBPROPFLAGS_TRUSTEE: DBPROPFLAGSENUM21 = 8192;
pub const DBPROPFLAGS_VIEW: DBPROPFLAGSENUM25 = 16384;
pub const DBPROPFLAGS_WRITE: DBPROPFLAGSENUM = 1024;
pub type DBPROPID = u32;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBPROPIDSET {
    pub rgPropertyIDs: *mut DBPROPID,
    pub cPropertyIDs: u32,
    pub guidPropertySet: windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
impl Default for DBPROPIDSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBPROPIDSET {
    pub rgPropertyIDs: *mut DBPROPID,
    pub cPropertyIDs: u32,
    pub guidPropertySet: windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBPROPIDSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBPROPINFO {
    pub pwszDescription: windows_sys::core::PWSTR,
    pub dwPropertyID: DBPROPID,
    pub dwFlags: DBPROPFLAGS,
    pub vtType: super::VARTYPE,
    pub vValues: super::VARIANT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBPROPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBPROPINFO {
    pub pwszDescription: windows_sys::core::PWSTR,
    pub dwPropertyID: DBPROPID,
    pub dwFlags: DBPROPFLAGS,
    pub vtType: super::VARTYPE,
    pub vValues: super::VARIANT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBPROPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBPROPINFOSET {
    pub rgPropertyInfos: PDBPROPINFO,
    pub cPropertyInfos: u32,
    pub guidPropertySet: windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBPROPINFOSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBPROPINFOSET {
    pub rgPropertyInfos: PDBPROPINFO,
    pub cPropertyInfos: u32,
    pub guidPropertySet: windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBPROPINFOSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBPROPOPTIONS = u32;
pub type DBPROPOPTIONSENUM = i32;
pub const DBPROPOPTIONS_OPTIONAL: DBPROPOPTIONSENUM = 1;
pub const DBPROPOPTIONS_REQUIRED: DBPROPOPTIONSENUM = 0;
pub const DBPROPOPTIONS_SETIFCHEAP: DBPROPOPTIONSENUM = 1;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBPROPSET {
    pub rgProperties: *mut DBPROP,
    pub cProperties: u32,
    pub guidPropertySet: windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBPROPSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DBPROPSET {
    pub rgProperties: *mut DBPROP,
    pub cProperties: u32,
    pub guidPropertySet: windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBPROPSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DBPROPSTATUS = u32;
pub type DBPROPSTATUSENUM = i32;
pub type DBPROPSTATUSENUM21 = i32;
pub const DBPROPSTATUS_BADCOLUMN: DBPROPSTATUSENUM = 4;
pub const DBPROPSTATUS_BADOPTION: DBPROPSTATUSENUM = 3;
pub const DBPROPSTATUS_BADVALUE: DBPROPSTATUSENUM = 2;
pub const DBPROPSTATUS_CONFLICTING: DBPROPSTATUSENUM = 8;
pub const DBPROPSTATUS_NOTALLSETTABLE: DBPROPSTATUSENUM = 5;
pub const DBPROPSTATUS_NOTAVAILABLE: DBPROPSTATUSENUM21 = 9;
pub const DBPROPSTATUS_NOTSET: DBPROPSTATUSENUM = 7;
pub const DBPROPSTATUS_NOTSETTABLE: DBPROPSTATUSENUM = 6;
pub const DBPROPSTATUS_NOTSUPPORTED: DBPROPSTATUSENUM = 1;
pub const DBPROPSTATUS_OK: DBPROPSTATUSENUM = 0;
pub const DBPROPVAL_AO_RANDOM: u32 = 2;
pub const DBPROPVAL_AO_SEQUENTIAL: u32 = 0;
pub const DBPROPVAL_AO_SEQUENTIALSTORAGEOBJECTS: u32 = 1;
pub const DBPROPVAL_ASYNCH_BACKGROUNDPOPULATION: u32 = 8;
pub const DBPROPVAL_ASYNCH_INITIALIZE: u32 = 1;
pub const DBPROPVAL_ASYNCH_POPULATEONDEMAND: u32 = 32;
pub const DBPROPVAL_ASYNCH_PREPOPULATE: u32 = 16;
pub const DBPROPVAL_ASYNCH_RANDOMPOPULATION: u32 = 4;
pub const DBPROPVAL_ASYNCH_SEQUENTIALPOPULATION: u32 = 2;
pub const DBPROPVAL_BD_INTRANSACTION: u32 = 1;
pub const DBPROPVAL_BD_REORGANIZATION: u32 = 3;
pub const DBPROPVAL_BD_ROWSET: u32 = 0;
pub const DBPROPVAL_BD_XTRANSACTION: u32 = 2;
pub const DBPROPVAL_BI_CROSSROWSET: u32 = 1;
pub const DBPROPVAL_BMK_KEY: u32 = 2;
pub const DBPROPVAL_BMK_NUMERIC: u32 = 1;
pub const DBPROPVAL_BO_NOINDEXUPDATE: u32 = 1;
pub const DBPROPVAL_BO_NOLOG: u32 = 0;
pub const DBPROPVAL_BO_REFINTEGRITY: u32 = 2;
pub const DBPROPVAL_CB_DELETE: u32 = 1;
pub const DBPROPVAL_CB_NON_NULL: u32 = 2;
pub const DBPROPVAL_CB_NULL: u32 = 1;
pub const DBPROPVAL_CB_PRESERVE: u32 = 2;
pub const DBPROPVAL_CD_NOTNULL: u32 = 1;
pub const DBPROPVAL_CL_END: u32 = 2;
pub const DBPROPVAL_CL_START: u32 = 1;
pub const DBPROPVAL_CM_TRANSACTIONS: u32 = 1;
pub const DBPROPVAL_CO_BEGINSWITH: u32 = 32;
pub const DBPROPVAL_CO_CASEINSENSITIVE: u32 = 8;
pub const DBPROPVAL_CO_CASESENSITIVE: u32 = 4;
pub const DBPROPVAL_CO_CONTAINS: u32 = 16;
pub const DBPROPVAL_CO_EQUALITY: u32 = 1;
pub const DBPROPVAL_CO_STRING: u32 = 2;
pub const DBPROPVAL_CS_COMMUNICATIONFAILURE: u32 = 2;
pub const DBPROPVAL_CS_INITIALIZED: u32 = 1;
pub const DBPROPVAL_CS_UNINITIALIZED: u32 = 0;
pub const DBPROPVAL_CU_DML_STATEMENTS: u32 = 1;
pub const DBPROPVAL_CU_INDEX_DEFINITION: u32 = 4;
pub const DBPROPVAL_CU_PRIVILEGE_DEFINITION: u32 = 8;
pub const DBPROPVAL_CU_TABLE_DEFINITION: u32 = 2;
pub const DBPROPVAL_DF_INITIALLY_DEFERRED: u32 = 1;
pub const DBPROPVAL_DF_INITIALLY_IMMEDIATE: u32 = 2;
pub const DBPROPVAL_DF_NOT_DEFERRABLE: u32 = 3;
pub const DBPROPVAL_DST_DOCSOURCE: u32 = 4;
pub const DBPROPVAL_DST_MDP: u32 = 2;
pub const DBPROPVAL_DST_TDP: u32 = 1;
pub const DBPROPVAL_DST_TDPANDMDP: u32 = 3;
pub const DBPROPVAL_FU_CATALOG: u32 = 8;
pub const DBPROPVAL_FU_COLUMN: u32 = 2;
pub const DBPROPVAL_FU_NOT_SUPPORTED: u32 = 1;
pub const DBPROPVAL_FU_TABLE: u32 = 4;
pub const DBPROPVAL_GB_COLLATE: u32 = 16;
pub const DBPROPVAL_GB_CONTAINS_SELECT: u32 = 4;
pub const DBPROPVAL_GB_EQUALS_SELECT: u32 = 2;
pub const DBPROPVAL_GB_NOT_SUPPORTED: u32 = 1;
pub const DBPROPVAL_GB_NO_RELATION: u32 = 8;
pub const DBPROPVAL_GU_NOTSUPPORTED: u32 = 1;
pub const DBPROPVAL_GU_SUFFIX: u32 = 2;
pub const DBPROPVAL_HT_DIFFERENT_CATALOGS: u32 = 1;
pub const DBPROPVAL_HT_DIFFERENT_PROVIDERS: u32 = 2;
pub const DBPROPVAL_IC_LOWER: u32 = 2;
pub const DBPROPVAL_IC_MIXED: u32 = 8;
pub const DBPROPVAL_IC_SENSITIVE: u32 = 4;
pub const DBPROPVAL_IC_UPPER: u32 = 1;
pub const DBPROPVAL_IN_ALLOWNULL: u32 = 0;
pub const DBPROPVAL_IN_DISALLOWNULL: u32 = 1;
pub const DBPROPVAL_IN_IGNOREANYNULL: u32 = 4;
pub const DBPROPVAL_IN_IGNORENULL: u32 = 2;
pub const DBPROPVAL_IT_BTREE: u32 = 1;
pub const DBPROPVAL_IT_CONTENT: u32 = 3;
pub const DBPROPVAL_IT_HASH: u32 = 2;
pub const DBPROPVAL_IT_OTHER: u32 = 4;
pub const DBPROPVAL_LM_NONE: u32 = 1;
pub const DBPROPVAL_LM_SINGLEROW: u32 = 2;
pub const DBPROPVAL_MR_CONCURRENT: u32 = 2;
pub const DBPROPVAL_MR_NOTSUPPORTED: u32 = 0;
pub const DBPROPVAL_MR_SUPPORTED: u32 = 1;
pub const DBPROPVAL_NC_END: u32 = 1;
pub const DBPROPVAL_NC_HIGH: u32 = 2;
pub const DBPROPVAL_NC_LOW: u32 = 4;
pub const DBPROPVAL_NC_START: u32 = 8;
pub const DBPROPVAL_NP_ABOUTTODO: u32 = 2;
pub const DBPROPVAL_NP_DIDEVENT: u32 = 16;
pub const DBPROPVAL_NP_FAILEDTODO: u32 = 8;
pub const DBPROPVAL_NP_OKTODO: u32 = 1;
pub const DBPROPVAL_NP_SYNCHAFTER: u32 = 4;
pub const DBPROPVAL_NT_MULTIPLEROWS: u32 = 2;
pub const DBPROPVAL_NT_SINGLEROW: u32 = 1;
pub const DBPROPVAL_OA_ATEXECUTE: u32 = 2;
pub const DBPROPVAL_OA_ATROWRELEASE: u32 = 4;
pub const DBPROPVAL_OA_NOTSUPPORTED: u32 = 1;
pub const DBPROPVAL_OO_BLOB: u32 = 1;
pub const DBPROPVAL_OO_DIRECTBIND: u32 = 16;
pub const DBPROPVAL_OO_IPERSIST: u32 = 2;
pub const DBPROPVAL_OO_ROWOBJECT: u32 = 4;
pub const DBPROPVAL_OO_SCOPED: u32 = 8;
pub const DBPROPVAL_OO_SINGLETON: u32 = 32;
pub const DBPROPVAL_OP_EQUAL: u32 = 1;
pub const DBPROPVAL_OP_RELATIVE: u32 = 2;
pub const DBPROPVAL_OP_STRING: u32 = 4;
pub const DBPROPVAL_ORS_HISTOGRAM: u32 = 8;
pub const DBPROPVAL_ORS_INDEX: u32 = 1;
pub const DBPROPVAL_ORS_INTEGRATEDINDEX: u32 = 2;
pub const DBPROPVAL_ORS_STOREDPROC: u32 = 4;
pub const DBPROPVAL_ORS_TABLE: u32 = 0;
pub const DBPROPVAL_OS_AGR_AFTERSESSION: u32 = 8;
pub const DBPROPVAL_OS_CLIENTCURSOR: u32 = 4;
pub const DBPROPVAL_OS_DISABLEALL: u32 = 0;
pub const DBPROPVAL_OS_ENABLEALL: u32 = 4294967295;
pub const DBPROPVAL_OS_RESOURCEPOOLING: u32 = 1;
pub const DBPROPVAL_OS_TXNENLISTMENT: u32 = 2;
pub const DBPROPVAL_PT_GUID: u32 = 8;
pub const DBPROPVAL_PT_GUID_NAME: u32 = 1;
pub const DBPROPVAL_PT_GUID_PROPID: u32 = 2;
pub const DBPROPVAL_PT_NAME: u32 = 4;
pub const DBPROPVAL_PT_PGUID_NAME: u32 = 32;
pub const DBPROPVAL_PT_PGUID_PROPID: u32 = 64;
pub const DBPROPVAL_PT_PROPID: u32 = 16;
pub const DBPROPVAL_RD_RESETALL: u32 = 4294967295;
pub const DBPROPVAL_RT_APTMTTHREAD: u32 = 2;
pub const DBPROPVAL_RT_FREETHREAD: u32 = 1;
pub const DBPROPVAL_RT_SINGLETHREAD: u32 = 4;
pub const DBPROPVAL_SQL_ANSI89_IEF: u32 = 8;
pub const DBPROPVAL_SQL_ANSI92_ENTRY: u32 = 16;
pub const DBPROPVAL_SQL_ANSI92_FULL: u32 = 128;
pub const DBPROPVAL_SQL_ANSI92_INTERMEDIATE: u32 = 64;
pub const DBPROPVAL_SQL_ESCAPECLAUSES: u32 = 256;
pub const DBPROPVAL_SQL_FIPS_TRANSITIONAL: u32 = 32;
pub const DBPROPVAL_SQL_NONE: u32 = 0;
pub const DBPROPVAL_SQL_ODBC_CORE: u32 = 2;
pub const DBPROPVAL_SQL_ODBC_EXTENDED: u32 = 4;
pub const DBPROPVAL_SQL_ODBC_MINIMUM: u32 = 1;
pub const DBPROPVAL_SQL_SUBMINIMUM: u32 = 512;
pub const DBPROPVAL_SQ_COMPARISON: u32 = 2;
pub const DBPROPVAL_SQ_CORRELATEDSUBQUERIES: u32 = 1;
pub const DBPROPVAL_SQ_EXISTS: u32 = 4;
pub const DBPROPVAL_SQ_IN: u32 = 8;
pub const DBPROPVAL_SQ_QUANTIFIED: u32 = 16;
pub const DBPROPVAL_SQ_TABLE: u32 = 32;
pub const DBPROPVAL_SS_ILOCKBYTES: u32 = 8;
pub const DBPROPVAL_SS_ISEQUENTIALSTREAM: u32 = 1;
pub const DBPROPVAL_SS_ISTORAGE: u32 = 4;
pub const DBPROPVAL_SS_ISTREAM: u32 = 2;
pub const DBPROPVAL_STGM_CONVERT: u32 = 262144;
pub const DBPROPVAL_STGM_CREATE: u32 = 4096;
pub const DBPROPVAL_STGM_DELETEONRELEASE: u32 = 2097152;
pub const DBPROPVAL_STGM_DIRECT: u32 = 65536;
pub const DBPROPVAL_STGM_FAILIFTHERE: u32 = 524288;
pub const DBPROPVAL_STGM_PRIORITY: u32 = 1048576;
pub const DBPROPVAL_STGM_READ: u32 = 0;
pub const DBPROPVAL_STGM_READWRITE: u32 = 2;
pub const DBPROPVAL_STGM_SHARE_DENY_NONE: u32 = 64;
pub const DBPROPVAL_STGM_SHARE_DENY_READ: u32 = 48;
pub const DBPROPVAL_STGM_SHARE_DENY_WRITE: u32 = 32;
pub const DBPROPVAL_STGM_SHARE_EXCLUSIVE: u32 = 16;
pub const DBPROPVAL_STGM_TRANSACTED: u32 = 131072;
pub const DBPROPVAL_STGM_WRITE: u32 = 1;
pub const DBPROPVAL_SU_DML_STATEMENTS: u32 = 1;
pub const DBPROPVAL_SU_INDEX_DEFINITION: u32 = 4;
pub const DBPROPVAL_SU_PRIVILEGE_DEFINITION: u32 = 8;
pub const DBPROPVAL_SU_TABLE_DEFINITION: u32 = 2;
pub const DBPROPVAL_TC_ALL: u32 = 8;
pub const DBPROPVAL_TC_DDL_COMMIT: u32 = 2;
pub const DBPROPVAL_TC_DDL_IGNORE: u32 = 4;
pub const DBPROPVAL_TC_DDL_LOCK: u32 = 16;
pub const DBPROPVAL_TC_DML: u32 = 1;
pub const DBPROPVAL_TC_NONE: u32 = 0;
pub const DBPROPVAL_TI_BROWSE: u32 = 256;
pub const DBPROPVAL_TI_CHAOS: u32 = 16;
pub const DBPROPVAL_TI_CURSORSTABILITY: u32 = 4096;
pub const DBPROPVAL_TI_ISOLATED: u32 = 1048576;
pub const DBPROPVAL_TI_READCOMMITTED: u32 = 4096;
pub const DBPROPVAL_TI_READUNCOMMITTED: u32 = 256;
pub const DBPROPVAL_TI_REPEATABLEREAD: u32 = 65536;
pub const DBPROPVAL_TI_SERIALIZABLE: u32 = 1048576;
pub const DBPROPVAL_TR_ABORT: u32 = 16;
pub const DBPROPVAL_TR_ABORT_DC: u32 = 8;
pub const DBPROPVAL_TR_ABORT_NO: u32 = 32;
pub const DBPROPVAL_TR_BOTH: u32 = 128;
pub const DBPROPVAL_TR_COMMIT: u32 = 2;
pub const DBPROPVAL_TR_COMMIT_DC: u32 = 1;
pub const DBPROPVAL_TR_COMMIT_NO: u32 = 4;
pub const DBPROPVAL_TR_DONTCARE: u32 = 64;
pub const DBPROPVAL_TR_NONE: u32 = 256;
pub const DBPROPVAL_TR_OPTIMISTIC: u32 = 512;
pub const DBPROPVAL_TS_CARDINALITY: u32 = 1;
pub const DBPROPVAL_TS_HISTOGRAM: u32 = 2;
pub const DBPROPVAL_UP_CHANGE: u32 = 1;
pub const DBPROPVAL_UP_DELETE: u32 = 2;
pub const DBPROPVAL_UP_INSERT: u32 = 4;
pub const DBPROP_ABORTPRESERVE: DBPROPENUM = 2;
pub const DBPROP_ACCESSORDER: DBPROPENUM20 = 231;
pub const DBPROP_ACTIVESESSIONS: DBPROPENUM = 3;
pub const DBPROP_ALTERCOLUMN: DBPROPENUM20 = 245;
pub const DBPROP_APPENDONLY: DBPROPENUM = 187;
pub const DBPROP_ASYNCTXNABORT: DBPROPENUM = 168;
pub const DBPROP_ASYNCTXNCOMMIT: DBPROPENUM = 4;
pub const DBPROP_AUTH_CACHE_AUTHINFO: DBPROPENUM = 5;
pub const DBPROP_AUTH_ENCRYPT_PASSWORD: DBPROPENUM = 6;
pub const DBPROP_AUTH_INTEGRATED: DBPROPENUM = 7;
pub const DBPROP_AUTH_MASK_PASSWORD: DBPROPENUM = 8;
pub const DBPROP_AUTH_PASSWORD: DBPROPENUM = 9;
pub const DBPROP_AUTH_PERSIST_ENCRYPTED: DBPROPENUM = 10;
pub const DBPROP_AUTH_PERSIST_SENSITIVE_AUTHINFO: DBPROPENUM = 11;
pub const DBPROP_AUTH_USERID: DBPROPENUM = 12;
pub const DBPROP_BLOCKINGSTORAGEOBJECTS: DBPROPENUM = 13;
pub const DBPROP_BOOKMARKINFO: DBPROPENUM20 = 232;
pub const DBPROP_BOOKMARKS: DBPROPENUM = 14;
pub const DBPROP_BOOKMARKSKIPPED: DBPROPENUM = 15;
pub const DBPROP_BOOKMARKTYPE: DBPROPENUM = 16;
pub const DBPROP_BYREFACCESSORS: DBPROPENUM = 120;
pub const DBPROP_CACHEDEFERRED: DBPROPENUM = 17;
pub const DBPROP_CANFETCHBACKWARDS: DBPROPENUM = 18;
pub const DBPROP_CANHOLDROWS: DBPROPENUM = 19;
pub const DBPROP_CANSCROLLBACKWARDS: DBPROPENUM = 21;
pub const DBPROP_CATALOGLOCATION: DBPROPENUM = 22;
pub const DBPROP_CATALOGTERM: DBPROPENUM = 23;
pub const DBPROP_CATALOGUSAGE: DBPROPENUM = 24;
pub const DBPROP_CHANGEINSERTEDROWS: DBPROPENUM = 188;
pub const DBPROP_CLIENTCURSOR: DBPROPENUM20 = 260;
pub const DBPROP_COLUMNDEFINITION: DBPROPENUM = 32;
pub const DBPROP_COLUMNLCID: DBPROPENUM20 = 246;
pub const DBPROP_COLUMNRESTRICT: DBPROPENUM = 33;
pub const DBPROP_COL_AUTOINCREMENT: DBPROPENUM = 26;
pub const DBPROP_COL_DEFAULT: DBPROPENUM = 27;
pub const DBPROP_COL_DESCRIPTION: DBPROPENUM = 28;
pub const DBPROP_COL_FIXEDLENGTH: DBPROPENUM = 167;
pub const DBPROP_COL_INCREMENT: DBPROPENUM25 = 283;
pub const DBPROP_COL_ISLONG: DBPROPENUM21 = 281;
pub const DBPROP_COL_NULLABLE: DBPROPENUM = 29;
pub const DBPROP_COL_PRIMARYKEY: DBPROPENUM = 30;
pub const DBPROP_COL_SEED: DBPROPENUM25 = 282;
pub const DBPROP_COL_UNIQUE: DBPROPENUM = 31;
pub const DBPROP_COMMANDTIMEOUT: DBPROPENUM = 34;
pub const DBPROP_COMMITPRESERVE: DBPROPENUM = 35;
pub const DBPROP_COMSERVICES: DBPROPENUM25 = 285;
pub const DBPROP_CONCATNULLBEHAVIOR: DBPROPENUM = 36;
pub const DBPROP_CONNECTIONSTATUS: DBPROPENUM20 = 244;
pub const DBPROP_CURRENTCATALOG: DBPROPENUM = 37;
pub const DBPROP_DATASOURCENAME: DBPROPENUM = 38;
pub const DBPROP_DATASOURCEREADONLY: DBPROPENUM = 39;
pub const DBPROP_DATASOURCE_TYPE: DBPROPENUM20 = 251;
pub const DBPROP_DBMSNAME: DBPROPENUM = 40;
pub const DBPROP_DBMSVER: DBPROPENUM = 41;
pub const DBPROP_DEFERRED: DBPROPENUM = 42;
pub const DBPROP_DELAYSTORAGEOBJECTS: DBPROPENUM = 43;
pub const DBPROP_DSOTHREADMODEL: DBPROPENUM = 169;
pub const DBPROP_FILTERCOMPAREOPS: DBPROPENUM15 = 209;
pub const DBPROP_FINDCOMPAREOPS: DBPROPENUM15 = 210;
pub const DBPROP_GENERATEURL: DBPROPENUM21 = 273;
pub const DBPROP_GROUPBY: DBPROPENUM = 44;
pub const DBPROP_HETEROGENEOUSTABLES: DBPROPENUM = 45;
pub const DBPROP_HIDDENCOLUMNS: DBPROPENUM20 = 258;
pub const DBPROP_IAccessor: DBPROPENUM = 121;
pub const DBPROP_IBindResource: DBPROPENUM21 = 268;
pub const DBPROP_IChapteredRowset: DBPROPENUM15 = 202;
pub const DBPROP_IColumnsInfo: DBPROPENUM = 122;
pub const DBPROP_IColumnsInfo2: DBPROPENUM21 = 275;
pub const DBPROP_IColumnsRowset: DBPROPENUM = 123;
pub const DBPROP_IConnectionPointContainer: DBPROPENUM = 124;
pub const DBPROP_IConvertType: DBPROPENUM = 194;
pub const DBPROP_ICreateRow: DBPROPENUM21 = 269;
pub const DBPROP_IDBAsynchStatus: DBPROPENUM15 = 203;
pub const DBPROP_IDBBinderProperties: DBPROPENUM21 = 274;
pub const DBPROP_IDENTIFIERCASE: DBPROPENUM = 46;
pub const DBPROP_IGetRow: DBPROPENUM21 = 266;
pub const DBPROP_IGetSession: DBPROPENUM21 = 277;
pub const DBPROP_IGetSourceRow: DBPROPENUM21 = 278;
pub const DBPROP_ILockBytes: DBPROPENUM = 136;
pub const DBPROP_IMMOBILEROWS: DBPROPENUM = 47;
pub const DBPROP_IMultipleResults: DBPROPENUM20 = 217;
pub const DBPROP_INDEX_AUTOUPDATE: DBPROPENUM = 48;
pub const DBPROP_INDEX_CLUSTERED: DBPROPENUM = 49;
pub const DBPROP_INDEX_FILLFACTOR: DBPROPENUM = 50;
pub const DBPROP_INDEX_INITIALSIZE: DBPROPENUM = 51;
pub const DBPROP_INDEX_NULLCOLLATION: DBPROPENUM = 52;
pub const DBPROP_INDEX_NULLS: DBPROPENUM = 53;
pub const DBPROP_INDEX_PRIMARYKEY: DBPROPENUM = 54;
pub const DBPROP_INDEX_SORTBOOKMARKS: DBPROPENUM = 55;
pub const DBPROP_INDEX_TEMPINDEX: DBPROPENUM = 163;
pub const DBPROP_INDEX_TYPE: DBPROPENUM = 56;
pub const DBPROP_INDEX_UNIQUE: DBPROPENUM = 57;
pub const DBPROP_INIT_ASYNCH: DBPROPENUM15 = 200;
pub const DBPROP_INIT_BINDFLAGS: DBPROPENUM21 = 270;
pub const DBPROP_INIT_CATALOG: DBPROPENUM20 = 233;
pub const DBPROP_INIT_DATASOURCE: DBPROPENUM = 59;
pub const DBPROP_INIT_GENERALTIMEOUT: DBPROPENUM25 = 284;
pub const DBPROP_INIT_HWND: DBPROPENUM = 60;
pub const DBPROP_INIT_IMPERSONATION_LEVEL: DBPROPENUM = 61;
pub const DBPROP_INIT_LCID: DBPROPENUM = 186;
pub const DBPROP_INIT_LOCATION: DBPROPENUM = 62;
pub const DBPROP_INIT_LOCKOWNER: DBPROPENUM21 = 271;
pub const DBPROP_INIT_MODE: DBPROPENUM = 63;
pub const DBPROP_INIT_OLEDBSERVICES: DBPROPENUM20 = 248;
pub const DBPROP_INIT_PROMPT: DBPROPENUM = 64;
pub const DBPROP_INIT_PROTECTION_LEVEL: DBPROPENUM = 65;
pub const DBPROP_INIT_PROVIDERSTRING: DBPROPENUM = 160;
pub const DBPROP_INIT_TIMEOUT: DBPROPENUM = 66;
pub const DBPROP_IParentRowset: DBPROPENUM20 = 257;
pub const DBPROP_IRegisterProvider: DBPROPENUM21 = 276;
pub const DBPROP_IRow: DBPROPENUM21 = 263;
pub const DBPROP_IRowChange: DBPROPENUM21 = 264;
pub const DBPROP_IRowSchemaChange: DBPROPENUM21 = 265;
pub const DBPROP_IRowset: DBPROPENUM = 126;
pub const DBPROP_IRowsetBookmark: DBPROPENUM26 = 292;
pub const DBPROP_IRowsetChange: DBPROPENUM = 127;
pub const DBPROP_IRowsetCurrentIndex: DBPROPENUM21 = 279;
pub const DBPROP_IRowsetFind: DBPROPENUM15 = 204;
pub const DBPROP_IRowsetIdentity: DBPROPENUM = 128;
pub const DBPROP_IRowsetIndex: DBPROPENUM = 159;
pub const DBPROP_IRowsetInfo: DBPROPENUM = 129;
pub const DBPROP_IRowsetLocate: DBPROPENUM = 130;
pub const DBPROP_IRowsetRefresh: DBPROPENUM20 = 249;
pub const DBPROP_IRowsetResynch: DBPROPENUM = 132;
pub const DBPROP_IRowsetScroll: DBPROPENUM = 133;
pub const DBPROP_IRowsetUpdate: DBPROPENUM = 134;
pub const DBPROP_IRowsetView: DBPROPENUM15 = 212;
pub const DBPROP_IScopedOperations: DBPROPENUM21 = 267;
pub const DBPROP_ISequentialStream: DBPROPENUM = 137;
pub const DBPROP_IStorage: DBPROPENUM = 138;
pub const DBPROP_IStream: DBPROPENUM = 139;
pub const DBPROP_ISupportErrorInfo: DBPROPENUM = 135;
pub const DBPROP_IViewChapter: DBPROPENUM15 = 213;
pub const DBPROP_IViewFilter: DBPROPENUM15 = 214;
pub const DBPROP_IViewRowset: DBPROPENUM15 = 215;
pub const DBPROP_IViewSort: DBPROPENUM15 = 216;
pub const DBPROP_LITERALBOOKMARKS: DBPROPENUM = 67;
pub const DBPROP_LITERALIDENTITY: DBPROPENUM = 68;
pub const DBPROP_LOCKMODE: DBPROPENUM20 = 236;
pub const DBPROP_MAXINDEXSIZE: DBPROPENUM = 70;
pub const DBPROP_MAXOPENCHAPTERS: DBPROPENUM15 = 199;
pub const DBPROP_MAXOPENROWS: DBPROPENUM = 71;
pub const DBPROP_MAXORSINFILTER: DBPROPENUM15 = 205;
pub const DBPROP_MAXPENDINGROWS: DBPROPENUM = 72;
pub const DBPROP_MAXROWS: DBPROPENUM = 73;
pub const DBPROP_MAXROWSIZE: DBPROPENUM = 74;
pub const DBPROP_MAXROWSIZEINCLUDESBLOB: DBPROPENUM = 75;
pub const DBPROP_MAXSORTCOLUMNS: DBPROPENUM15 = 206;
pub const DBPROP_MAXTABLESINSELECT: DBPROPENUM = 76;
pub const DBPROP_MAYWRITECOLUMN: DBPROPENUM = 77;
pub const DBPROP_MEMORYUSAGE: DBPROPENUM = 78;
pub const DBPROP_MULTIPLECONNECTIONS: DBPROPENUM20 = 237;
pub const DBPROP_MULTIPLEPARAMSETS: DBPROPENUM = 191;
pub const DBPROP_MULTIPLERESULTS: DBPROPENUM = 196;
pub const DBPROP_MULTIPLESTORAGEOBJECTS: DBPROPENUM = 80;
pub const DBPROP_MULTITABLEUPDATE: DBPROPENUM = 81;
pub const DBPROP_NOTIFICATIONGRANULARITY: DBPROPENUM = 198;
pub const DBPROP_NOTIFICATIONPHASES: DBPROPENUM = 82;
pub const DBPROP_NOTIFYCOLUMNSET: DBPROPENUM = 171;
pub const DBPROP_NOTIFYROWDELETE: DBPROPENUM = 173;
pub const DBPROP_NOTIFYROWFIRSTCHANGE: DBPROPENUM = 174;
pub const DBPROP_NOTIFYROWINSERT: DBPROPENUM = 175;
pub const DBPROP_NOTIFYROWRESYNCH: DBPROPENUM = 177;
pub const DBPROP_NOTIFYROWSETCHANGED: DBPROPENUM = 211;
pub const DBPROP_NOTIFYROWSETFETCHPOSITIONCHANGE: DBPROPENUM = 179;
pub const DBPROP_NOTIFYROWSETRELEASE: DBPROPENUM = 178;
pub const DBPROP_NOTIFYROWUNDOCHANGE: DBPROPENUM = 180;
pub const DBPROP_NOTIFYROWUNDODELETE: DBPROPENUM = 181;
pub const DBPROP_NOTIFYROWUNDOINSERT: DBPROPENUM = 182;
pub const DBPROP_NOTIFYROWUPDATE: DBPROPENUM = 183;
pub const DBPROP_NULLCOLLATION: DBPROPENUM = 83;
pub const DBPROP_OLEOBJECTS: DBPROPENUM = 84;
pub const DBPROP_OPENROWSETSUPPORT: DBPROPENUM21 = 280;
pub const DBPROP_ORDERBYCOLUMNSINSELECT: DBPROPENUM = 85;
pub const DBPROP_ORDEREDBOOKMARKS: DBPROPENUM = 86;
pub const DBPROP_OTHERINSERT: DBPROPENUM = 87;
pub const DBPROP_OTHERUPDATEDELETE: DBPROPENUM = 88;
pub const DBPROP_OUTPUTENCODING: DBPROPENUM26 = 287;
pub const DBPROP_OUTPUTPARAMETERAVAILABILITY: DBPROPENUM = 184;
pub const DBPROP_OUTPUTSTREAM: DBPROPENUM26 = 286;
pub const DBPROP_OWNINSERT: DBPROPENUM = 89;
pub const DBPROP_OWNUPDATEDELETE: DBPROPENUM = 90;
pub const DBPROP_PERSISTENTIDTYPE: DBPROPENUM = 185;
pub const DBPROP_PREPAREABORTBEHAVIOR: DBPROPENUM = 91;
pub const DBPROP_PREPARECOMMITBEHAVIOR: DBPROPENUM = 92;
pub const DBPROP_PROCEDURETERM: DBPROPENUM = 93;
pub const DBPROP_PROVIDERFILENAME: u32 = 96;
pub const DBPROP_PROVIDERFRIENDLYNAME: DBPROPENUM20 = 235;
pub const DBPROP_PROVIDERMEMORY: DBPROPENUM20 = 259;
pub const DBPROP_PROVIDERNAME: DBPROPENUM = 96;
pub const DBPROP_PROVIDEROLEDBVER: DBPROPENUM = 97;
pub const DBPROP_PROVIDERVER: DBPROPENUM = 98;
pub const DBPROP_QUICKRESTART: DBPROPENUM = 99;
pub const DBPROP_QUOTEDIDENTIFIERCASE: DBPROPENUM = 100;
pub const DBPROP_REENTRANTEVENTS: DBPROPENUM = 101;
pub const DBPROP_REMOVEDELETED: DBPROPENUM = 102;
pub const DBPROP_REPORTMULTIPLECHANGES: DBPROPENUM = 103;
pub const DBPROP_RESETDATASOURCE: DBPROPENUM20 = 247;
pub const DBPROP_RETURNPENDINGINSERTS: DBPROPENUM = 189;
pub const DBPROP_ROWRESTRICT: DBPROPENUM = 104;
pub const DBPROP_ROWSETCONVERSIONSONCOMMAND: DBPROPENUM = 192;
pub const DBPROP_ROWSET_ASYNCH: DBPROPENUM15 = 201;
pub const DBPROP_ROWTHREADMODEL: DBPROPENUM = 105;
pub const DBPROP_ROW_BULKOPS: DBPROPENUM20 = 234;
pub const DBPROP_SCHEMATERM: DBPROPENUM = 106;
pub const DBPROP_SCHEMAUSAGE: DBPROPENUM = 107;
pub const DBPROP_SERVERCURSOR: DBPROPENUM = 108;
pub const DBPROP_SERVERDATAONINSERT: DBPROPENUM20 = 239;
pub const DBPROP_SERVERNAME: DBPROPENUM20 = 250;
pub const DBPROP_SERVER_NAME: u32 = 250;
pub const DBPROP_SESS_AUTOCOMMITISOLEVELS: DBPROPENUM = 190;
pub const DBPROP_SKIPROWCOUNTRESULTS: DBPROPENUM26 = 291;
pub const DBPROP_SORTONINDEX: DBPROPENUM15 = 207;
pub const DBPROP_SQLSUPPORT: DBPROPENUM = 109;
pub const DBPROP_STORAGEFLAGS: DBPROPENUM20 = 240;
pub const DBPROP_STRONGIDENTITY: DBPROPENUM = 119;
pub const DBPROP_STRUCTUREDSTORAGE: DBPROPENUM = 111;
pub const DBPROP_SUBQUERIES: DBPROPENUM = 112;
pub const DBPROP_SUPPORTEDTXNDDL: DBPROPENUM = 161;
pub const DBPROP_SUPPORTEDTXNISOLEVELS: DBPROPENUM = 113;
pub const DBPROP_SUPPORTEDTXNISORETAIN: DBPROPENUM = 114;
pub const DBPROP_TABLESTATISTICS: DBPROPENUM26 = 288;
pub const DBPROP_TABLETERM: DBPROPENUM = 115;
pub const DBPROP_TBL_TEMPTABLE: DBPROPENUM = 140;
pub const DBPROP_TRANSACTEDOBJECT: DBPROPENUM = 116;
pub const DBPROP_TRUSTEE_AUTHENTICATION: DBPROPENUM21 = 242;
pub const DBPROP_TRUSTEE_NEWAUTHENTICATION: DBPROPENUM21 = 243;
pub const DBPROP_TRUSTEE_USERNAME: DBPROPENUM21 = 241;
pub const DBPROP_UNIQUEROWS: DBPROPENUM20 = 238;
pub const DBPROP_UPDATABILITY: DBPROPENUM = 117;
pub const DBPROP_USERNAME: DBPROPENUM = 118;
pub type DBRANGE = u32;
pub type DBRANGEENUM = i32;
pub type DBRANGEENUM20 = i32;
pub const DBRANGE_EXCLUDENULLS: DBRANGEENUM = 4;
pub const DBRANGE_EXCLUSIVEEND: DBRANGEENUM = 2;
pub const DBRANGE_EXCLUSIVESTART: DBRANGEENUM = 1;
pub const DBRANGE_INCLUSIVEEND: DBRANGEENUM = 0;
pub const DBRANGE_INCLUSIVESTART: DBRANGEENUM = 0;
pub const DBRANGE_MATCH: DBRANGEENUM = 16;
pub const DBRANGE_MATCH_N_MASK: DBRANGEENUM20 = 255;
pub const DBRANGE_MATCH_N_SHIFT: DBRANGEENUM20 = 24;
pub const DBRANGE_PREFIX: DBRANGEENUM = 8;
pub type DBREASON = u32;
pub type DBREASONENUM = i32;
pub type DBREASONENUM15 = i32;
pub const DBREASON_COLUMN_RECALCULATED: DBREASONENUM = 3;
pub const DBREASON_COLUMN_SET: DBREASONENUM = 2;
pub const DBREASON_ROWPOSITION_CHANGED: DBREASONENUM15 = 15;
pub const DBREASON_ROWPOSITION_CHAPTERCHANGED: DBREASONENUM15 = 16;
pub const DBREASON_ROWPOSITION_CLEARED: DBREASONENUM15 = 17;
pub const DBREASON_ROWSET_CHANGED: DBREASONENUM = 14;
pub const DBREASON_ROWSET_FETCHPOSITIONCHANGE: DBREASONENUM = 0;
pub const DBREASON_ROWSET_RELEASE: DBREASONENUM = 1;
pub const DBREASON_ROW_ACTIVATE: DBREASONENUM = 4;
pub const DBREASON_ROW_ASYNCHINSERT: DBREASONENUM15 = 18;
pub const DBREASON_ROW_DELETE: DBREASONENUM = 6;
pub const DBREASON_ROW_FIRSTCHANGE: DBREASONENUM = 7;
pub const DBREASON_ROW_INSERT: DBREASONENUM = 8;
pub const DBREASON_ROW_RELEASE: DBREASONENUM = 5;
pub const DBREASON_ROW_RESYNCH: DBREASONENUM = 9;
pub const DBREASON_ROW_UNDOCHANGE: DBREASONENUM = 10;
pub const DBREASON_ROW_UNDODELETE: DBREASONENUM = 12;
pub const DBREASON_ROW_UNDOINSERT: DBREASONENUM = 11;
pub const DBREASON_ROW_UPDATE: DBREASONENUM = 13;
pub type DBREFCOUNT = u32;
pub type DBRESULTFLAG = DB_LRESERVE;
pub type DBRESULTFLAGENUM = i32;
pub const DBRESULTFLAG_DEFAULT: DBRESULTFLAGENUM = 0;
pub const DBRESULTFLAG_ROW: DBRESULTFLAGENUM = 2;
pub const DBRESULTFLAG_ROWSET: DBRESULTFLAGENUM = 1;
#[cfg(target_arch = "x86")]
pub type DBROWCOUNT = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DBROWCOUNT = i64;
#[cfg(target_arch = "x86")]
pub type DBROWOFFSET = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DBROWOFFSET = i64;
pub type DBROWOPTIONS = u32;
pub type DBROWSTATUS = u32;
pub type DBROWSTATUSENUM = i32;
pub type DBROWSTATUSENUM20 = i32;
pub const DBROWSTATUS_E_CANCELED: DBROWSTATUSENUM = 4;
pub const DBROWSTATUS_E_CANTRELEASE: DBROWSTATUSENUM = 6;
pub const DBROWSTATUS_E_CONCURRENCYVIOLATION: DBROWSTATUSENUM = 7;
pub const DBROWSTATUS_E_DELETED: DBROWSTATUSENUM = 8;
pub const DBROWSTATUS_E_FAIL: DBROWSTATUSENUM = 19;
pub const DBROWSTATUS_E_INTEGRITYVIOLATION: DBROWSTATUSENUM = 11;
pub const DBROWSTATUS_E_INVALID: DBROWSTATUSENUM = 12;
pub const DBROWSTATUS_E_LIMITREACHED: DBROWSTATUSENUM = 17;
pub const DBROWSTATUS_E_MAXPENDCHANGESEXCEEDED: DBROWSTATUSENUM = 13;
pub const DBROWSTATUS_E_NEWLYINSERTED: DBROWSTATUSENUM = 10;
pub const DBROWSTATUS_E_OBJECTOPEN: DBROWSTATUSENUM = 14;
pub const DBROWSTATUS_E_OUTOFMEMORY: DBROWSTATUSENUM = 15;
pub const DBROWSTATUS_E_PENDINGINSERT: DBROWSTATUSENUM = 9;
pub const DBROWSTATUS_E_PERMISSIONDENIED: DBROWSTATUSENUM = 16;
pub const DBROWSTATUS_E_SCHEMAVIOLATION: DBROWSTATUSENUM = 18;
pub const DBROWSTATUS_S_MULTIPLECHANGES: DBROWSTATUSENUM = 2;
pub const DBROWSTATUS_S_NOCHANGE: DBROWSTATUSENUM20 = 20;
pub const DBROWSTATUS_S_OK: DBROWSTATUSENUM = 0;
pub const DBROWSTATUS_S_PENDINGCHANGES: DBROWSTATUSENUM = 3;
pub type DBSEEK = u32;
pub type DBSEEKENUM = i32;
pub const DBSEEK_AFTER: DBSEEKENUM = 8;
pub const DBSEEK_AFTEREQ: DBSEEKENUM = 4;
pub const DBSEEK_BEFORE: DBSEEKENUM = 32;
pub const DBSEEK_BEFOREEQ: DBSEEKENUM = 16;
pub const DBSEEK_FIRSTEQ: DBSEEKENUM = 1;
pub const DBSEEK_GE: u32 = 4;
pub const DBSEEK_GT: u32 = 8;
pub const DBSEEK_INVALID: DBSEEKENUM = 0;
pub const DBSEEK_LASTEQ: DBSEEKENUM = 2;
pub const DBSEEK_LE: u32 = 16;
pub const DBSEEK_LT: u32 = 32;
pub type DBSORT = u32;
pub type DBSORTENUM = i32;
pub const DBSORT_ASCENDING: DBSORTENUM = 0;
pub const DBSORT_DESCENDING: DBSORTENUM = 1;
pub type DBSOURCETYPE = u32;
pub type DBSOURCETYPEENUM = i32;
pub type DBSOURCETYPEENUM20 = i32;
pub type DBSOURCETYPEENUM25 = i32;
pub const DBSOURCETYPE_BINDER: DBSOURCETYPEENUM25 = 4;
pub const DBSOURCETYPE_DATASOURCE: DBSOURCETYPEENUM = 1;
pub const DBSOURCETYPE_DATASOURCE_MDP: DBSOURCETYPEENUM20 = 3;
pub const DBSOURCETYPE_DATASOURCE_TDP: DBSOURCETYPEENUM20 = 1;
pub const DBSOURCETYPE_ENUMERATOR: DBSOURCETYPEENUM = 2;
pub type DBSTATUS = u32;
pub type DBSTATUSENUM = i32;
pub type DBSTATUSENUM20 = i32;
pub type DBSTATUSENUM21 = i32;
pub type DBSTATUSENUM25 = i32;
pub type DBSTATUSENUM26 = i32;
pub const DBSTATUS_E_BADACCESSOR: DBSTATUSENUM = 1;
pub const DBSTATUS_E_BADSTATUS: DBSTATUSENUM = 12;
pub const DBSTATUS_E_CANCELED: DBSTATUSENUM25 = 27;
pub const DBSTATUS_E_CANNOTCOMPLETE: DBSTATUSENUM21 = 20;
pub const DBSTATUS_E_CANTCONVERTVALUE: DBSTATUSENUM = 2;
pub const DBSTATUS_E_CANTCREATE: DBSTATUSENUM = 7;
pub const DBSTATUS_E_DATAOVERFLOW: DBSTATUSENUM = 6;
pub const DBSTATUS_E_DOESNOTEXIST: DBSTATUSENUM21 = 16;
pub const DBSTATUS_E_INTEGRITYVIOLATION: DBSTATUSENUM = 10;
pub const DBSTATUS_E_INVALIDURL: DBSTATUSENUM21 = 17;
pub const DBSTATUS_E_NOTCOLLECTION: DBSTATUSENUM25 = 28;
pub const DBSTATUS_E_OUTOFSPACE: DBSTATUSENUM21 = 22;
pub const DBSTATUS_E_PERMISSIONDENIED: DBSTATUSENUM = 9;
pub const DBSTATUS_E_READONLY: DBSTATUSENUM21 = 24;
pub const DBSTATUS_E_RESOURCEEXISTS: DBSTATUSENUM21 = 19;
pub const DBSTATUS_E_RESOURCELOCKED: DBSTATUSENUM21 = 18;
pub const DBSTATUS_E_RESOURCEOUTOFSCOPE: DBSTATUSENUM21 = 25;
pub const DBSTATUS_E_SCHEMAVIOLATION: DBSTATUSENUM = 11;
pub const DBSTATUS_E_SIGNMISMATCH: DBSTATUSENUM = 5;
pub const DBSTATUS_E_UNAVAILABLE: DBSTATUSENUM = 8;
pub const DBSTATUS_E_VOLUMENOTFOUND: DBSTATUSENUM21 = 21;
pub const DBSTATUS_S_ALREADYEXISTS: DBSTATUSENUM21 = 26;
pub const DBSTATUS_S_CANNOTDELETESOURCE: DBSTATUSENUM21 = 23;
pub const DBSTATUS_S_DEFAULT: DBSTATUSENUM = 13;
pub const DBSTATUS_S_IGNORE: DBSTATUSENUM20 = 15;
pub const DBSTATUS_S_ISNULL: DBSTATUSENUM = 3;
pub const DBSTATUS_S_OK: DBSTATUSENUM = 0;
pub const DBSTATUS_S_ROWSETCOLUMN: DBSTATUSENUM26 = 29;
pub const DBSTATUS_S_TRUNCATED: DBSTATUSENUM = 4;
pub const DBSTAT_COLUMN_CARDINALITY: DBTABLESTATISTICSTYPE26 = 2;
pub const DBSTAT_HISTOGRAM: DBTABLESTATISTICSTYPE26 = 1;
pub const DBSTAT_TUPLE_CARDINALITY: DBTABLESTATISTICSTYPE26 = 4;
pub type DBTABLESTATISTICSTYPE26 = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DBTIME {
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct DBTIMESTAMP {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub fraction: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct DBTIMESTAMP {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub fraction: u32,
}
pub type DBTYPE = u16;
pub type DBTYPEENUM = i32;
pub type DBTYPEENUM15 = i32;
pub type DBTYPEENUM20 = i32;
#[cfg(target_arch = "x86")]
pub const DBTYPEFOR_DBLENGTH: u32 = 19;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DBTYPEFOR_DBLENGTH: u32 = 21;
#[cfg(target_arch = "x86")]
pub const DBTYPEFOR_DBORDINAL: u32 = 19;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DBTYPEFOR_DBORDINAL: u32 = 21;
#[cfg(target_arch = "x86")]
pub const DBTYPEFOR_DBROWCOUNT: u32 = 3;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DBTYPEFOR_DBROWCOUNT: u32 = 20;
pub const DBTYPE_ARRAY: DBTYPEENUM = 8192;
pub const DBTYPE_BOOL: DBTYPEENUM = 11;
pub const DBTYPE_BSTR: DBTYPEENUM = 8;
pub const DBTYPE_BYREF: DBTYPEENUM = 16384;
pub const DBTYPE_BYTES: DBTYPEENUM = 128;
pub const DBTYPE_CY: DBTYPEENUM = 6;
pub const DBTYPE_DATE: DBTYPEENUM = 7;
pub const DBTYPE_DBDATE: DBTYPEENUM = 133;
pub const DBTYPE_DBTIME: DBTYPEENUM = 134;
pub const DBTYPE_DBTIMESTAMP: DBTYPEENUM = 135;
pub const DBTYPE_DECIMAL: DBTYPEENUM = 14;
pub const DBTYPE_EMPTY: DBTYPEENUM = 0;
pub const DBTYPE_ERROR: DBTYPEENUM = 10;
pub const DBTYPE_FILETIME: DBTYPEENUM20 = 64;
pub const DBTYPE_GUID: DBTYPEENUM = 72;
pub const DBTYPE_HCHAPTER: DBTYPEENUM15 = 136;
pub const DBTYPE_I1: DBTYPEENUM = 16;
pub const DBTYPE_I2: DBTYPEENUM = 2;
pub const DBTYPE_I4: DBTYPEENUM = 3;
pub const DBTYPE_I8: DBTYPEENUM = 20;
pub const DBTYPE_IDISPATCH: DBTYPEENUM = 9;
pub const DBTYPE_IUNKNOWN: DBTYPEENUM = 13;
pub const DBTYPE_NULL: DBTYPEENUM = 1;
pub const DBTYPE_NUMERIC: DBTYPEENUM = 131;
pub const DBTYPE_PROPVARIANT: DBTYPEENUM20 = 138;
pub const DBTYPE_R4: DBTYPEENUM = 4;
pub const DBTYPE_R8: DBTYPEENUM = 5;
pub const DBTYPE_RESERVED: DBTYPEENUM = 32768;
pub const DBTYPE_STR: DBTYPEENUM = 129;
pub const DBTYPE_UDT: DBTYPEENUM = 132;
pub const DBTYPE_UI1: DBTYPEENUM = 17;
pub const DBTYPE_UI2: DBTYPEENUM = 18;
pub const DBTYPE_UI4: DBTYPEENUM = 19;
pub const DBTYPE_UI8: DBTYPEENUM = 21;
pub const DBTYPE_VARIANT: DBTYPEENUM = 12;
pub const DBTYPE_VARNUMERIC: DBTYPEENUM20 = 139;
pub const DBTYPE_VECTOR: DBTYPEENUM = 4096;
pub const DBTYPE_WSTR: DBTYPEENUM = 130;
pub type DBUPDELRULE = u32;
pub type DBUPDELRULEENUM = i32;
pub const DBUPDELRULE_CASCADE: DBUPDELRULEENUM = 1;
pub const DBUPDELRULE_NOACTION: DBUPDELRULEENUM = 0;
pub const DBUPDELRULE_SETDEFAULT: DBUPDELRULEENUM = 3;
pub const DBUPDELRULE_SETNULL: DBUPDELRULEENUM = 2;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBVECTOR {
    pub size: DBLENGTH,
    pub ptr: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for DBVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBVECTOR {
    pub size: DBLENGTH,
    pub ptr: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DBWATCHREGION_NULL: u32 = 0;
pub const DB_ALL_EXCEPT_LIKE: u32 = 3;
pub const DB_BINDFLAGS_COLLECTION: u32 = 16;
pub const DB_BINDFLAGS_DELAYFETCHCOLUMNS: u32 = 1;
pub const DB_BINDFLAGS_DELAYFETCHSTREAM: u32 = 2;
pub const DB_BINDFLAGS_ISSTRUCTUREDDOCUMENT: u32 = 128;
pub const DB_BINDFLAGS_OPENIFEXISTS: u32 = 32;
pub const DB_BINDFLAGS_OUTPUT: u32 = 8;
pub const DB_BINDFLAGS_OVERWRITE: u32 = 64;
pub const DB_BINDFLAGS_RECURSIVE: u32 = 4;
pub const DB_COLLATION_ASC: u32 = 1;
pub const DB_COLLATION_DESC: u32 = 2;
pub const DB_COUNTUNAVAILABLE: i32 = -1;
#[cfg(target_arch = "x86")]
pub type DB_DWRESERVE = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
pub type DB_DWRESERVE = super::DWORDLONG;
pub const DB_IMP_LEVEL_ANONYMOUS: u32 = 0;
pub const DB_IMP_LEVEL_DELEGATE: u32 = 3;
pub const DB_IMP_LEVEL_IDENTIFY: u32 = 1;
pub const DB_IMP_LEVEL_IMPERSONATE: u32 = 2;
pub const DB_INVALIDCOLUMN: i32 = -1;
pub const DB_INVALID_HACCESSOR: u32 = 0;
pub const DB_INVALID_HCHAPTER: u32 = 0;
pub const DB_LIKE_ONLY: u32 = 2;
pub const DB_LOCAL_EXCLUSIVE: u32 = 3;
pub const DB_LOCAL_SHARED: u32 = 2;
#[cfg(target_arch = "x86")]
pub type DB_LORDINAL = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DB_LORDINAL = i64;
#[cfg(target_arch = "x86")]
pub type DB_LPARAMS = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DB_LPARAMS = i64;
#[cfg(target_arch = "x86")]
pub type DB_LRESERVE = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DB_LRESERVE = i64;
pub const DB_MODE_READ: u32 = 1;
pub const DB_MODE_READWRITE: u32 = 3;
pub const DB_MODE_SHARE_DENY_NONE: u32 = 16;
pub const DB_MODE_SHARE_DENY_READ: u32 = 4;
pub const DB_MODE_SHARE_DENY_WRITE: u32 = 8;
pub const DB_MODE_SHARE_EXCLUSIVE: u32 = 12;
pub const DB_MODE_WRITE: u32 = 2;
pub const DB_NULL_HACCESSOR: u32 = 0;
pub const DB_NULL_HCHAPTER: u32 = 0;
pub const DB_NULL_HROW: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DB_NUMERIC {
    pub precision: u8,
    pub scale: u8,
    pub sign: u8,
    pub val: [u8; 16],
}
impl Default for DB_NUMERIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DB_PROT_LEVEL_CALL: u32 = 2;
pub const DB_PROT_LEVEL_CONNECT: u32 = 1;
pub const DB_PROT_LEVEL_NONE: u32 = 0;
pub const DB_PROT_LEVEL_PKT: u32 = 3;
pub const DB_PROT_LEVEL_PKT_INTEGRITY: u32 = 4;
pub const DB_PROT_LEVEL_PKT_PRIVACY: u32 = 5;
pub const DB_PT_FUNCTION: u32 = 3;
pub const DB_PT_PROCEDURE: u32 = 2;
pub const DB_PT_UNKNOWN: u32 = 1;
pub const DB_REMOTE: u32 = 1;
pub const DB_SEARCHABLE: u32 = 4;
pub const DB_UNSEARCHABLE: u32 = 1;
#[cfg(target_arch = "x86")]
pub type DB_UPARAMS = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DB_UPARAMS = u64;
#[cfg(target_arch = "x86")]
pub type DB_URESERVE = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type DB_URESERVE = u64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DB_VARNUMERIC {
    pub precision: u8,
    pub scale: SBYTE,
    pub sign: u8,
    pub val: [u8; 1],
}
impl Default for DB_VARNUMERIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy, Default)]
pub struct ERRORINFO {
    pub hrError: windows_sys::core::HRESULT,
    pub dwMinor: u32,
    pub clsid: windows_sys::core::GUID,
    pub iid: windows_sys::core::GUID,
    pub dispid: super::DISPID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy, Default)]
pub struct ERRORINFO {
    pub hrError: windows_sys::core::HRESULT,
    pub dwMinor: u32,
    pub clsid: windows_sys::core::GUID,
    pub iid: windows_sys::core::GUID,
    pub dispid: super::DISPID,
}
pub type HACCESSOR = usize;
pub type HCHAPTER = usize;
pub type HROW = usize;
pub type HWATCHREGION = usize;
pub const IDENTIFIER_SDK_ERROR: u32 = 268435456;
pub const IDENTIFIER_SDK_MASK: u32 = 4026531840;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IRowsetExactScroll(pub u8);
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct MDAXISINFO {
    pub cbSize: DBLENGTH,
    pub iAxis: DBCOUNTITEM,
    pub cDimensions: DBCOUNTITEM,
    pub cCoordinates: DBCOUNTITEM,
    pub rgcColumns: *mut DBORDINAL,
    pub rgpwszDimensionNames: *mut windows_sys::core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for MDAXISINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct MDAXISINFO {
    pub cbSize: DBLENGTH,
    pub iAxis: DBCOUNTITEM,
    pub cDimensions: DBCOUNTITEM,
    pub cCoordinates: DBCOUNTITEM,
    pub rgcColumns: *mut DBORDINAL,
    pub rgpwszDimensionNames: *mut windows_sys::core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MDAXISINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MDAXIS_CHAPTERS: u32 = 4;
pub const MDAXIS_COLUMNS: u32 = 0;
pub const MDAXIS_PAGES: u32 = 2;
pub const MDAXIS_ROWS: u32 = 1;
pub const MDAXIS_SECTIONS: u32 = 3;
pub const MDAXIS_SLICERS: u32 = 4294967295;
pub const MDDISPINFO_DRILLED_DOWN: u32 = 65536;
pub const MDDISPINFO_PARENT_SAME_AS_PREV: u32 = 131072;
pub const MDFF_BOLD: u32 = 1;
pub const MDFF_ITALIC: u32 = 2;
pub const MDFF_STRIKEOUT: u32 = 8;
pub const MDFF_UNDERLINE: u32 = 4;
pub const MDLEVEL_TYPE_ALL: u32 = 1;
pub const MDLEVEL_TYPE_CALCULATED: u32 = 2;
pub const MDLEVEL_TYPE_REGULAR: u32 = 0;
pub const MDLEVEL_TYPE_RESERVED1: u32 = 8;
pub const MDLEVEL_TYPE_TIME: u32 = 4;
pub const MDLEVEL_TYPE_TIME_DAYS: u32 = 516;
pub const MDLEVEL_TYPE_TIME_HALF_YEAR: u32 = 36;
pub const MDLEVEL_TYPE_TIME_HOURS: u32 = 772;
pub const MDLEVEL_TYPE_TIME_MINUTES: u32 = 1028;
pub const MDLEVEL_TYPE_TIME_MONTHS: u32 = 132;
pub const MDLEVEL_TYPE_TIME_QUARTERS: u32 = 68;
pub const MDLEVEL_TYPE_TIME_SECONDS: u32 = 2052;
pub const MDLEVEL_TYPE_TIME_UNDEFINED: u32 = 4100;
pub const MDLEVEL_TYPE_TIME_WEEKS: u32 = 260;
pub const MDLEVEL_TYPE_TIME_YEARS: u32 = 20;
pub const MDLEVEL_TYPE_UNKNOWN: u32 = 0;
pub const MDMEASURE_AGGR_AVG: u32 = 5;
pub const MDMEASURE_AGGR_CALCULATED: u32 = 127;
pub const MDMEASURE_AGGR_COUNT: u32 = 2;
pub const MDMEASURE_AGGR_MAX: u32 = 4;
pub const MDMEASURE_AGGR_MIN: u32 = 3;
pub const MDMEASURE_AGGR_STD: u32 = 7;
pub const MDMEASURE_AGGR_SUM: u32 = 1;
pub const MDMEASURE_AGGR_UNKNOWN: u32 = 0;
pub const MDMEASURE_AGGR_VAR: u32 = 6;
pub const MDMEMBER_TYPE_ALL: u32 = 2;
pub const MDMEMBER_TYPE_FORMULA: u32 = 4;
pub const MDMEMBER_TYPE_MEASURE: u32 = 3;
pub const MDMEMBER_TYPE_REGULAR: u32 = 1;
pub const MDMEMBER_TYPE_RESERVE1: u32 = 5;
pub const MDMEMBER_TYPE_RESERVE2: u32 = 6;
pub const MDMEMBER_TYPE_RESERVE3: u32 = 7;
pub const MDMEMBER_TYPE_RESERVE4: u32 = 8;
pub const MDMEMBER_TYPE_UNKNOWN: u32 = 0;
pub const MDPROPVAL_AU_UNCHANGED: u32 = 1;
pub const MDPROPVAL_AU_UNKNOWN: u32 = 2;
pub const MDPROPVAL_AU_UNSUPPORTED: u32 = 0;
pub const MDPROPVAL_FS_FULL_SUPPORT: u32 = 1;
pub const MDPROPVAL_FS_GENERATED_COLUMN: u32 = 2;
pub const MDPROPVAL_FS_GENERATED_DIMENSION: u32 = 3;
pub const MDPROPVAL_FS_NO_SUPPORT: u32 = 4;
pub const MDPROPVAL_MC_SEARCHEDCASE: u32 = 2;
pub const MDPROPVAL_MC_SINGLECASE: u32 = 1;
pub const MDPROPVAL_MD_AFTER: u32 = 4;
pub const MDPROPVAL_MD_BEFORE: u32 = 2;
pub const MDPROPVAL_MD_SELF: u32 = 1;
pub const MDPROPVAL_MF_CREATE_CALCMEMBERS: u32 = 4;
pub const MDPROPVAL_MF_CREATE_NAMEDSETS: u32 = 8;
pub const MDPROPVAL_MF_SCOPE_GLOBAL: u32 = 32;
pub const MDPROPVAL_MF_SCOPE_SESSION: u32 = 16;
pub const MDPROPVAL_MF_WITH_CALCMEMBERS: u32 = 1;
pub const MDPROPVAL_MF_WITH_NAMEDSETS: u32 = 2;
pub const MDPROPVAL_MJC_IMPLICITCUBE: u32 = 4;
pub const MDPROPVAL_MJC_MULTICUBES: u32 = 2;
pub const MDPROPVAL_MJC_SINGLECUBE: u32 = 1;
pub const MDPROPVAL_MMF_CLOSINGPERIOD: u32 = 8;
pub const MDPROPVAL_MMF_COUSIN: u32 = 1;
pub const MDPROPVAL_MMF_OPENINGPERIOD: u32 = 4;
pub const MDPROPVAL_MMF_PARALLELPERIOD: u32 = 2;
pub const MDPROPVAL_MNF_AGGREGATE: u32 = 16;
pub const MDPROPVAL_MNF_CORRELATION: u32 = 64;
pub const MDPROPVAL_MNF_COVARIANCE: u32 = 32;
pub const MDPROPVAL_MNF_DRILLDOWNLEVEL: u32 = 2048;
pub const MDPROPVAL_MNF_DRILLDOWNLEVELBOTTOM: u32 = 32768;
pub const MDPROPVAL_MNF_DRILLDOWNLEVELTOP: u32 = 16384;
pub const MDPROPVAL_MNF_DRILLDOWNMEMBERBOTTOM: u32 = 8192;
pub const MDPROPVAL_MNF_DRILLDOWNMEMBERTOP: u32 = 4096;
pub const MDPROPVAL_MNF_DRILLUPLEVEL: u32 = 131072;
pub const MDPROPVAL_MNF_DRILLUPMEMBER: u32 = 65536;
pub const MDPROPVAL_MNF_LINREG2: u32 = 512;
pub const MDPROPVAL_MNF_LINREGPOINT: u32 = 1024;
pub const MDPROPVAL_MNF_LINREGSLOPE: u32 = 128;
pub const MDPROPVAL_MNF_LINREGVARIANCE: u32 = 256;
pub const MDPROPVAL_MNF_MEDIAN: u32 = 1;
pub const MDPROPVAL_MNF_RANK: u32 = 8;
pub const MDPROPVAL_MNF_STDDEV: u32 = 4;
pub const MDPROPVAL_MNF_VAR: u32 = 2;
pub const MDPROPVAL_MOQ_CATALOG_CUBE: u32 = 2;
pub const MDPROPVAL_MOQ_CUBE_DIM: u32 = 8;
pub const MDPROPVAL_MOQ_DATASOURCE_CUBE: u32 = 1;
pub const MDPROPVAL_MOQ_DIMHIER_LEVEL: u32 = 32;
pub const MDPROPVAL_MOQ_DIMHIER_MEMBER: u32 = 256;
pub const MDPROPVAL_MOQ_DIM_HIER: u32 = 16;
pub const MDPROPVAL_MOQ_LEVEL_MEMBER: u32 = 64;
pub const MDPROPVAL_MOQ_MEMBER_MEMBER: u32 = 128;
pub const MDPROPVAL_MOQ_OUTERREFERENCE: u32 = 1;
pub const MDPROPVAL_MOQ_SCHEMA_CUBE: u32 = 4;
pub const MDPROPVAL_MSC_GREATERTHAN: u32 = 2;
pub const MDPROPVAL_MSC_GREATERTHANEQUAL: u32 = 8;
pub const MDPROPVAL_MSC_LESSTHAN: u32 = 1;
pub const MDPROPVAL_MSC_LESSTHANEQUAL: u32 = 4;
pub const MDPROPVAL_MSF_BOTTOMPERCENT: u32 = 2;
pub const MDPROPVAL_MSF_BOTTOMSUM: u32 = 8;
pub const MDPROPVAL_MSF_DRILLDOWNLEVEL: u32 = 2048;
pub const MDPROPVAL_MSF_DRILLDOWNLEVELBOTTOM: u32 = 32768;
pub const MDPROPVAL_MSF_DRILLDOWNLEVELTOP: u32 = 16384;
pub const MDPROPVAL_MSF_DRILLDOWNMEMBBER: u32 = 1024;
pub const MDPROPVAL_MSF_DRILLDOWNMEMBERBOTTOM: u32 = 8192;
pub const MDPROPVAL_MSF_DRILLDOWNMEMBERTOP: u32 = 4096;
pub const MDPROPVAL_MSF_DRILLUPLEVEL: u32 = 131072;
pub const MDPROPVAL_MSF_DRILLUPMEMBER: u32 = 65536;
pub const MDPROPVAL_MSF_LASTPERIODS: u32 = 32;
pub const MDPROPVAL_MSF_MTD: u32 = 256;
pub const MDPROPVAL_MSF_PERIODSTODATE: u32 = 16;
pub const MDPROPVAL_MSF_QTD: u32 = 128;
pub const MDPROPVAL_MSF_TOGGLEDRILLSTATE: u32 = 262144;
pub const MDPROPVAL_MSF_TOPPERCENT: u32 = 1;
pub const MDPROPVAL_MSF_TOPSUM: u32 = 4;
pub const MDPROPVAL_MSF_WTD: u32 = 512;
pub const MDPROPVAL_MSF_YTD: u32 = 64;
pub const MDPROPVAL_MS_MULTIPLETUPLES: u32 = 1;
pub const MDPROPVAL_MS_SINGLETUPLE: u32 = 2;
pub const MDPROPVAL_NL_NAMEDLEVELS: u32 = 1;
pub const MDPROPVAL_NL_NUMBEREDLEVELS: u32 = 2;
pub const MDPROPVAL_NL_SCHEMAONLY: u32 = 4;
pub const MDPROPVAL_NME_ALLDIMENSIONS: u32 = 0;
pub const MDPROPVAL_NME_MEASURESONLY: u32 = 1;
pub const MDPROPVAL_RR_NORANGEROWSET: u32 = 1;
pub const MDPROPVAL_RR_READONLY: u32 = 2;
pub const MDPROPVAL_RR_UPDATE: u32 = 4;
pub const MDPROPVAL_VISUAL_MODE_DEFAULT: u32 = 0;
pub const MDPROPVAL_VISUAL_MODE_VISUAL: u32 = 1;
pub const MDPROPVAL_VISUAL_MODE_VISUAL_OFF: u32 = 2;
pub const MDPROP_AGGREGATECELL_UPDATE: DBPROPENUM20 = 230;
pub const MDPROP_AXES: DBPROPENUM20 = 252;
pub const MDPROP_CELL: u32 = 2;
pub const MDPROP_FLATTENING_SUPPORT: DBPROPENUM20 = 253;
pub const MDPROP_MDX_AGGREGATECELL_UPDATE: DBPROPENUM20 = 230;
pub const MDPROP_MDX_CASESUPPORT: DBPROPENUM20 = 222;
pub const MDPROP_MDX_CUBEQUALIFICATION: DBPROPENUM20 = 219;
pub const MDPROP_MDX_DESCFLAGS: DBPROPENUM20 = 225;
pub const MDPROP_MDX_FORMULAS: DBPROPENUM20 = 229;
pub const MDPROP_MDX_JOINCUBES: DBPROPENUM20 = 254;
pub const MDPROP_MDX_MEMBER_FUNCTIONS: DBPROPENUM20 = 227;
pub const MDPROP_MDX_NONMEASURE_EXPRESSIONS: DBPROPENUM20 = 262;
pub const MDPROP_MDX_NUMERIC_FUNCTIONS: DBPROPENUM20 = 228;
pub const MDPROP_MDX_OBJQUALIFICATION: DBPROPENUM20 = 261;
pub const MDPROP_MDX_OUTERREFERENCE: DBPROPENUM20 = 220;
pub const MDPROP_MDX_QUERYBYPROPERTY: DBPROPENUM20 = 221;
pub const MDPROP_MDX_SET_FUNCTIONS: DBPROPENUM20 = 226;
pub const MDPROP_MDX_SLICER: DBPROPENUM20 = 218;
pub const MDPROP_MDX_STRING_COMPOP: DBPROPENUM20 = 224;
pub const MDPROP_MEMBER: u32 = 1;
pub const MDPROP_NAMED_LEVELS: DBPROPENUM20 = 255;
pub const MDPROP_RANGEROWSET: DBPROPENUM20 = 256;
pub const MDPROP_VISUALMODE: DBPROPENUM26 = 293;
pub const MDSTATUS_S_CELLEMPTY: DBSTATUSENUM20 = 14;
pub const MDTREEOP_ANCESTORS: u32 = 32;
pub const MDTREEOP_CHILDREN: u32 = 1;
pub const MDTREEOP_DESCENDANTS: u32 = 16;
pub const MDTREEOP_PARENT: u32 = 4;
pub const MDTREEOP_SELF: u32 = 8;
pub const MDTREEOP_SIBLINGS: u32 = 2;
pub const MD_DIMTYPE_MEASURE: u32 = 2;
pub const MD_DIMTYPE_OTHER: u32 = 3;
pub const MD_DIMTYPE_TIME: u32 = 1;
pub const MD_DIMTYPE_UNKNOWN: u32 = 0;
pub const OLEDBVER: u32 = 624;
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub type PDBPROPINFO = *mut DBPROPINFO;
pub const PERM_ALL: ACCESS_MASKENUM = 268435456;
pub const PERM_CREATE: ACCESS_MASKENUM = 16384;
pub const PERM_DELETE: ACCESS_MASKENUM = 65536;
pub const PERM_DESIGN: u32 = 2048;
pub const PERM_DROP: ACCESS_MASKENUM = 256;
pub const PERM_EXCLUSIVE: ACCESS_MASKENUM = 512;
pub const PERM_EXECUTE: ACCESS_MASKENUM = 536870912;
pub const PERM_INSERT: ACCESS_MASKENUM = 32768;
pub const PERM_MAXIMUM_ALLOWED: ACCESS_MASKENUM = 33554432;
pub const PERM_READ: ACCESS_MASKENUM = -2147483648;
pub const PERM_READCONTROL: ACCESS_MASKENUM = 131072;
pub const PERM_READDESIGN: ACCESS_MASKENUM = 1024;
pub const PERM_REFERENCE: ACCESS_MASKENUM = 8192;
pub const PERM_UPDATE: ACCESS_MASKENUM = 1073741824;
pub const PERM_WITHGRANT: ACCESS_MASKENUM = 4096;
pub const PERM_WRITEDESIGN: ACCESS_MASKENUM = 2048;
pub const PERM_WRITEOWNER: ACCESS_MASKENUM = 524288;
pub const PERM_WRITEPERMISSIONS: ACCESS_MASKENUM = 262144;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct RMTPACK {
    pub pISeqStream: *mut core::ffi::c_void,
    pub cbData: u32,
    pub cBSTR: u32,
    pub rgBSTR: *mut windows_sys::core::BSTR,
    pub cVARIANT: u32,
    pub rgVARIANT: *mut super::VARIANT,
    pub cIDISPATCH: u32,
    pub rgIDISPATCH: *mut *mut core::ffi::c_void,
    pub cIUNKNOWN: u32,
    pub rgIUNKNOWN: *mut *mut core::ffi::c_void,
    pub cPROPVARIANT: u32,
    pub rgPROPVARIANT: *mut super::PROPVARIANT,
    pub cArray: u32,
    pub rgArray: *mut super::VARIANT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for RMTPACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct RMTPACK {
    pub pISeqStream: *mut core::ffi::c_void,
    pub cbData: u32,
    pub cBSTR: u32,
    pub rgBSTR: *mut windows_sys::core::BSTR,
    pub cVARIANT: u32,
    pub rgVARIANT: *mut super::VARIANT,
    pub cIDISPATCH: u32,
    pub rgIDISPATCH: *mut *mut core::ffi::c_void,
    pub cIUNKNOWN: u32,
    pub rgIUNKNOWN: *mut *mut core::ffi::c_void,
    pub cPROPVARIANT: u32,
    pub rgPROPVARIANT: *mut super::PROPVARIANT,
    pub cArray: u32,
    pub rgArray: *mut super::VARIANT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for RMTPACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SBYTE = i8;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SEC_OBJECT {
    pub cObjects: u32,
    pub prgObjects: *mut SEC_OBJECT_ELEMENT,
}
#[cfg(target_arch = "x86")]
impl Default for SEC_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SEC_OBJECT {
    pub cObjects: u32,
    pub prgObjects: *mut SEC_OBJECT_ELEMENT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SEC_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SEC_OBJECT_ELEMENT {
    pub guidObjectType: windows_sys::core::GUID,
    pub ObjectID: DBID,
}
#[cfg(target_arch = "x86")]
impl Default for SEC_OBJECT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SEC_OBJECT_ELEMENT {
    pub guidObjectType: windows_sys::core::GUID,
    pub ObjectID: DBID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SEC_OBJECT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STD_BOOKMARKLENGTH: u32 = 1;
