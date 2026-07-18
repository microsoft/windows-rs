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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBACCESSORFLAGS(pub u32);
pub type DBACCESSORFLAGSENUM = i32;
pub const DBACCESSOR_INHERITED: DBACCESSORFLAGSENUM = 16;
pub const DBACCESSOR_INVALID: DBACCESSORFLAGSENUM = 0;
pub const DBACCESSOR_OPTIMIZED: DBACCESSORFLAGSENUM = 8;
pub const DBACCESSOR_PARAMETERDATA: DBACCESSORFLAGSENUM = 4;
pub const DBACCESSOR_PASSBYREF: DBACCESSORFLAGSENUM = 1;
pub const DBACCESSOR_ROWDATA: DBACCESSORFLAGSENUM = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBASYNCHOP(pub u32);
pub type DBASYNCHOPENUM = i32;
pub const DBASYNCHOP_OPEN: DBASYNCHOPENUM = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBASYNCHPHASE(pub u32);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBBINDFLAG(pub u32);
pub type DBBINDFLAGENUM = i32;
pub const DBBINDFLAG_HTML: DBBINDFLAGENUM = 1;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "oaidl")]
pub struct DBBINDING {
    pub iOrdinal: DBORDINAL,
    pub obValue: DBBYTEOFFSET,
    pub obLength: DBBYTEOFFSET,
    pub obStatus: DBBYTEOFFSET,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::ITypeInfo>>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DBBINDING {
    pub iOrdinal: DBORDINAL,
    pub obValue: DBBYTEOFFSET,
    pub obLength: DBBYTEOFFSET,
    pub obStatus: DBBYTEOFFSET,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::ITypeInfo>>,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBBINDSTATUS(pub u32);
pub type DBBINDSTATUSENUM = i32;
pub const DBBINDSTATUS_BADBINDINFO: DBBINDSTATUSENUM = 3;
pub const DBBINDSTATUS_BADORDINAL: DBBINDSTATUSENUM = 1;
pub const DBBINDSTATUS_BADSTORAGEFLAGS: DBBINDSTATUSENUM = 4;
pub const DBBINDSTATUS_MULTIPLESTORAGE: DBBINDSTATUSENUM = 6;
pub const DBBINDSTATUS_NOINTERFACE: DBBINDSTATUSENUM = 5;
pub const DBBINDSTATUS_OK: DBBINDSTATUSENUM = 0;
pub const DBBINDSTATUS_UNSUPPORTEDCONVERSION: DBBINDSTATUSENUM = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBBINDURLFLAG(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBBINDURLSTATUS(pub u32);
pub type DBBINDURLSTATUSENUM = i32;
pub const DBBINDURLSTATUS_S_DENYNOTSUPPORTED: DBBINDURLSTATUSENUM = 1;
pub const DBBINDURLSTATUS_S_DENYTYPENOTSUPPORTED: DBBINDURLSTATUSENUM = 4;
pub const DBBINDURLSTATUS_S_OK: DBBINDURLSTATUSENUM = 0;
pub const DBBINDURLSTATUS_S_REDIRECTED: DBBINDURLSTATUSENUM = 8;
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBBKMARK(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBBKMARK(pub u64);
pub const DBBMK_FIRST: DBBOOKMARK = 1;
pub const DBBMK_INVALID: DBBOOKMARK = 0;
pub const DBBMK_LAST: DBBOOKMARK = 2;
pub type DBBOOKMARK = i32;
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBBYTEOFFSET(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBBYTEOFFSET(pub u64);
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
pub struct DBCOLUMNDESC {
    pub pwszTypeName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::ITypeInfo>>,
    pub rgPropertySets: *mut DBPROPSET,
    pub pclsid: *mut windows_core::GUID,
    pub cPropertySets: u32,
    pub ulColumnSize: DBLENGTH,
    pub dbcid: DBID,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for DBCOLUMNDESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
pub struct DBCOLUMNDESC {
    pub pwszTypeName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::ITypeInfo>>,
    pub rgPropertySets: *mut DBPROPSET,
    pub pclsid: *mut windows_core::GUID,
    pub cPropertySets: u32,
    pub ulColumnSize: DBLENGTH,
    pub dbcid: DBID,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for DBCOLUMNDESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBCOLUMNDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCOLUMNDESCFLAGS(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCOLUMNFLAGS(pub u32);
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
pub struct DBCOLUMNINFO {
    pub pwszName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::ITypeInfo>>,
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
impl Clone for DBCOLUMNINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
pub struct DBCOLUMNINFO {
    pub pwszName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::ITypeInfo>>,
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
impl Clone for DBCOLUMNINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
impl Default for DBCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCOMMANDPERSISTFLAG(pub u32);
pub type DBCOMMANDPERSISTFLAGENUM = i32;
pub type DBCOMMANDPERSISTFLAGENUM21 = i32;
pub const DBCOMMANDPERSISTFLAG_DEFAULT: DBCOMMANDPERSISTFLAGENUM21 = 0;
pub const DBCOMMANDPERSISTFLAG_NOSAVE: DBCOMMANDPERSISTFLAGENUM = 1;
pub const DBCOMMANDPERSISTFLAG_PERSISTPROCEDURE: DBCOMMANDPERSISTFLAGENUM21 = 4;
pub const DBCOMMANDPERSISTFLAG_PERSISTVIEW: DBCOMMANDPERSISTFLAGENUM21 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCOMPARE(pub u32);
pub type DBCOMPAREENUM = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCOMPAREOP(pub u32);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCONSTRAINTTYPE(pub u32);
pub type DBCONSTRAINTTYPEENUM = i32;
pub const DBCONSTRAINTTYPE_CHECK: DBCONSTRAINTTYPEENUM = 3;
pub const DBCONSTRAINTTYPE_FOREIGNKEY: DBCONSTRAINTTYPEENUM = 1;
pub const DBCONSTRAINTTYPE_PRIMARYKEY: DBCONSTRAINTTYPEENUM = 2;
pub const DBCONSTRAINTTYPE_UNIQUE: DBCONSTRAINTTYPEENUM = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCONVERTFLAGS(pub u32);
pub type DBCONVERTFLAGSENUM = i32;
pub type DBCONVERTFLAGSENUM20 = i32;
pub const DBCONVERTFLAGS_COLUMN: DBCONVERTFLAGSENUM = 0;
pub const DBCONVERTFLAGS_FROMVARIANT: DBCONVERTFLAGSENUM20 = 8;
pub const DBCONVERTFLAGS_ISFIXEDLENGTH: DBCONVERTFLAGSENUM20 = 4;
pub const DBCONVERTFLAGS_ISLONG: DBCONVERTFLAGSENUM20 = 2;
pub const DBCONVERTFLAGS_PARAMETER: DBCONVERTFLAGSENUM = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCOPYFLAGS(pub u32);
pub type DBCOPYFLAGSENUM = i32;
pub const DBCOPY_ALLOW_EMULATION: DBCOPYFLAGSENUM = 1024;
pub const DBCOPY_ASYNC: DBCOPYFLAGSENUM = 256;
pub const DBCOPY_ATOMIC: DBCOPYFLAGSENUM = 4096;
pub const DBCOPY_NON_RECURSIVE: DBCOPYFLAGSENUM = 2048;
pub const DBCOPY_REPLACE_EXISTING: DBCOPYFLAGSENUM = 512;
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCOUNTITEM(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBCOUNTITEM(pub u64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBDATE {
    pub year: i16,
    pub month: u16,
    pub day: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBDEFERRABILITY(pub u32);
pub type DBDEFERRABILITYENUM = i32;
pub const DBDEFERRABILITY_DEFERRABLE: DBDEFERRABILITYENUM = 2;
pub const DBDEFERRABILITY_DEFERRED: DBDEFERRABILITYENUM = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBDELETEFLAGS(pub u32);
pub type DBDELETEFLAGSENUM = i32;
pub const DBDELETE_ASYNC: DBDELETEFLAGSENUM = 256;
pub const DBDELETE_ATOMIC: DBDELETEFLAGSENUM = 4096;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBEVENTPHASE(pub u32);
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
    pub failure: windows_core::HRESULT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBFAILUREINFO {
    pub hRow: HROW,
    pub iColumn: DBORDINAL,
    pub failure: windows_core::HRESULT,
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBHASHVALUE(pub u32);
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
    pub guid: windows_core::GUID,
    pub pguid: *mut windows_core::GUID,
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
    pub pwszName: windows_core::PWSTR,
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
    pub guid: windows_core::GUID,
    pub pguid: *mut windows_core::GUID,
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
    pub pwszName: windows_core::PWSTR,
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
pub struct DBIMPLICITSESSION {
    pub pUnkOuter: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub piid: *mut windows_core::GUID,
    pub pSession: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[cfg(target_arch = "x86")]
impl Default for DBIMPLICITSESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DBIMPLICITSESSION {
    pub pUnkOuter: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub piid: *mut windows_core::GUID,
    pub pSession: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBINDEX_COL_ORDER(pub u32);
pub type DBINDEX_COL_ORDERENUM = i32;
pub const DBINDEX_COL_ORDER_ASC: DBINDEX_COL_ORDERENUM = 0;
pub const DBINDEX_COL_ORDER_DESC: DBINDEX_COL_ORDERENUM = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBKIND(pub u32);
pub type DBKINDENUM = i32;
pub const DBKIND_GUID: DBKINDENUM = 6;
pub const DBKIND_GUID_NAME: DBKINDENUM = 0;
pub const DBKIND_GUID_PROPID: DBKINDENUM = 1;
pub const DBKIND_NAME: DBKINDENUM = 2;
pub const DBKIND_PGUID_NAME: DBKINDENUM = 3;
pub const DBKIND_PGUID_PROPID: DBKINDENUM = 4;
pub const DBKIND_PROPID: DBKINDENUM = 5;
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBLENGTH(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBLENGTH(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBLITERAL(pub u32);
pub type DBLITERALENUM = i32;
pub type DBLITERALENUM20 = i32;
pub type DBLITERALENUM21 = i32;
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct DBLITERALINFO {
    pub pwszLiteralValue: windows_core::PWSTR,
    pub pwszInvalidChars: windows_core::PWSTR,
    pub pwszInvalidStartingChars: windows_core::PWSTR,
    pub lt: DBLITERAL,
    pub fSupported: windows_core::BOOL,
    pub cchMaxLen: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBLITERALINFO {
    pub pwszLiteralValue: windows_core::PWSTR,
    pub pwszInvalidChars: windows_core::PWSTR,
    pub pwszInvalidStartingChars: windows_core::PWSTR,
    pub lt: DBLITERAL,
    pub fSupported: windows_core::BOOL,
    pub cchMaxLen: u32,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBMATCHTYPE(pub u32);
pub type DBMATCHTYPEENUM = i32;
pub const DBMATCHTYPE_FULL: DBMATCHTYPEENUM = 0;
pub const DBMATCHTYPE_NONE: DBMATCHTYPEENUM = 1;
pub const DBMATCHTYPE_PARTIAL: DBMATCHTYPEENUM = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBMEMOWNER(pub u32);
pub type DBMEMOWNERENUM = i32;
pub const DBMEMOWNER_CLIENTOWNED: DBMEMOWNERENUM = 0;
pub const DBMEMOWNER_PROVIDEROWNED: DBMEMOWNERENUM = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBMOVEFLAGS(pub u32);
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
    pub iid: windows_core::GUID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBOBJECT {
    pub dwFlags: u32,
    pub iid: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBORDINAL(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBORDINAL(pub u64);
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct DBPARAMBINDINFO {
    pub pwszDataSourceType: windows_core::PWSTR,
    pub pwszName: windows_core::PWSTR,
    pub ulParamSize: DBLENGTH,
    pub dwFlags: DBPARAMFLAGS,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBPARAMBINDINFO {
    pub pwszDataSourceType: windows_core::PWSTR,
    pub pwszName: windows_core::PWSTR,
    pub ulParamSize: DBLENGTH,
    pub dwFlags: DBPARAMFLAGS,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBPARAMFLAGS(pub u32);
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
#[derive(Default)]
pub struct DBPARAMINFO {
    pub dwFlags: DBPARAMFLAGS,
    pub iOrdinal: DBORDINAL,
    pub pwszName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::ITypeInfo>>,
    pub ulParamSize: DBLENGTH,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DBPARAMINFO {
    pub dwFlags: DBPARAMFLAGS,
    pub iOrdinal: DBORDINAL,
    pub pwszName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::ITypeInfo>>,
    pub ulParamSize: DBLENGTH,
    pub wType: DBTYPE,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBPARAMIO(pub u32);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBPART(pub u32);
pub type DBPARTENUM = i32;
pub const DBPART_INVALID: DBPARTENUM = 0;
pub const DBPART_LENGTH: DBPARTENUM = 2;
pub const DBPART_STATUS: DBPARTENUM = 4;
pub const DBPART_VALUE: DBPARTENUM = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBPENDINGSTATUS(pub u32);
pub type DBPENDINGSTATUSENUM = i32;
pub const DBPENDINGSTATUS_CHANGED: DBPENDINGSTATUSENUM = 2;
pub const DBPENDINGSTATUS_DELETED: DBPENDINGSTATUSENUM = 4;
pub const DBPENDINGSTATUS_INVALIDROW: DBPENDINGSTATUSENUM = 16;
pub const DBPENDINGSTATUS_NEW: DBPENDINGSTATUSENUM = 1;
pub const DBPENDINGSTATUS_UNCHANGED: DBPENDINGSTATUSENUM = 8;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBPOSITIONFLAGS(pub u32);
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
pub struct DBPROP {
    pub dwPropertyID: DBPROPID,
    pub dwOptions: DBPROPOPTIONS,
    pub dwStatus: DBPROPSTATUS,
    pub colid: DBID,
    pub vValue: super::VARIANT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for DBPROP {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
pub struct DBPROP {
    pub dwPropertyID: DBPROPID,
    pub dwOptions: DBPROPOPTIONS,
    pub dwStatus: DBPROPSTATUS,
    pub colid: DBID,
    pub vValue: super::VARIANT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for DBPROP {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBPROPFLAGS(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBPROPID(pub u32);
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBPROPIDSET {
    pub rgPropertyIDs: *mut DBPROPID,
    pub cPropertyIDs: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl Default for DBPROPIDSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBPROPIDSET {
    pub rgPropertyIDs: *mut DBPROPID,
    pub cPropertyIDs: u32,
    pub guidPropertySet: windows_core::GUID,
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
pub struct DBPROPINFO {
    pub pwszDescription: windows_core::PWSTR,
    pub dwPropertyID: DBPROPID,
    pub dwFlags: DBPROPFLAGS,
    pub vtType: super::VARTYPE,
    pub vValues: super::VARIANT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for DBPROPINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
pub struct DBPROPINFO {
    pub pwszDescription: windows_core::PWSTR,
    pub dwPropertyID: DBPROPID,
    pub dwFlags: DBPROPFLAGS,
    pub vtType: super::VARTYPE,
    pub vValues: super::VARIANT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for DBPROPINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
#[derive(Clone, Copy, Default)]
pub struct DBPROPINFOSET {
    pub rgPropertyInfos: PDBPROPINFO,
    pub cPropertyInfos: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBPROPINFOSET {
    pub rgPropertyInfos: PDBPROPINFO,
    pub cPropertyInfos: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBPROPOPTIONS(pub u32);
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
    pub guidPropertySet: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBPROPSET {
    pub rgProperties: *mut DBPROP,
    pub cProperties: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for DBPROPSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBPROPSTATUS(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBRANGE(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBREASON(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBREFCOUNT(pub u32);
pub type DBRESULTFLAG = DB_LRESERVE;
pub type DBRESULTFLAGENUM = i32;
pub const DBRESULTFLAG_DEFAULT: DBRESULTFLAGENUM = 0;
pub const DBRESULTFLAG_ROW: DBRESULTFLAGENUM = 2;
pub const DBRESULTFLAG_ROWSET: DBRESULTFLAGENUM = 1;
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBROWCOUNT(pub i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBROWCOUNT(pub i64);
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBROWOFFSET(pub i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBROWOFFSET(pub i64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBROWOPTIONS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBROWSTATUS(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBSEEK(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBSORT(pub u32);
pub type DBSORTENUM = i32;
pub const DBSORT_ASCENDING: DBSORTENUM = 0;
pub const DBSORT_DESCENDING: DBSORTENUM = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBSOURCETYPE(pub u32);
pub type DBSOURCETYPEENUM = i32;
pub type DBSOURCETYPEENUM20 = i32;
pub type DBSOURCETYPEENUM25 = i32;
pub const DBSOURCETYPE_BINDER: DBSOURCETYPEENUM25 = 4;
pub const DBSOURCETYPE_DATASOURCE: DBSOURCETYPEENUM = 1;
pub const DBSOURCETYPE_DATASOURCE_MDP: DBSOURCETYPEENUM20 = 3;
pub const DBSOURCETYPE_DATASOURCE_TDP: DBSOURCETYPEENUM20 = 1;
pub const DBSOURCETYPE_ENUMERATOR: DBSOURCETYPEENUM = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBSTATUS(pub u32);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBTIMESTAMP {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub fraction: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBTYPE(pub u16);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBUPDELRULE(pub u32);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_DWRESERVE(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_LORDINAL(pub i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_LORDINAL(pub i64);
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_LPARAMS(pub i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_LPARAMS(pub i64);
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_LRESERVE(pub i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_LRESERVE(pub i64);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_UPARAMS(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_UPARAMS(pub u64);
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_URESERVE(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DB_URESERVE(pub u64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub hrError: windows_core::HRESULT,
    pub dwMinor: u32,
    pub clsid: windows_core::GUID,
    pub iid: windows_core::GUID,
    pub dispid: super::DISPID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "oaidl")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ERRORINFO {
    pub hrError: windows_core::HRESULT,
    pub dwMinor: u32,
    pub clsid: windows_core::GUID,
    pub iid: windows_core::GUID,
    pub dispid: super::DISPID,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HACCESSOR(pub usize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCHAPTER(pub usize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HROW(pub usize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HWATCHREGION(pub usize);
windows_core::imp::define_interface!(IAccessor, IAccessor_Vtbl, 0x0c733a8c_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IAccessor, windows_core::IUnknown);
impl IAccessor {
    pub unsafe fn AddRefAccessor(&self, haccessor: HACCESSOR, pcrefcount: Option<*mut DBREFCOUNT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRefAccessor)(windows_core::Interface::as_raw(self), haccessor, pcrefcount.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CreateAccessor(&self, dwaccessorflags: DBACCESSORFLAGS, cbindings: DBCOUNTITEM, rgbindings: *const DBBINDING, cbrowsize: DBLENGTH, phaccessor: *mut HACCESSOR, rgstatus: Option<*mut DBBINDSTATUS>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateAccessor)(windows_core::Interface::as_raw(self), dwaccessorflags, cbindings, rgbindings, cbrowsize, phaccessor as _, rgstatus.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetBindings(&self, haccessor: HACCESSOR, pdwaccessorflags: *mut DBACCESSORFLAGS, pcbindings: Option<*mut DBCOUNTITEM>, prgbindings: *mut *mut DBBINDING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBindings)(windows_core::Interface::as_raw(self), haccessor, pdwaccessorflags as _, pcbindings.unwrap_or(core::mem::zeroed()) as _, prgbindings as _) }
    }
    pub unsafe fn ReleaseAccessor(&self, haccessor: HACCESSOR, pcrefcount: Option<*mut DBREFCOUNT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseAccessor)(windows_core::Interface::as_raw(self), haccessor, pcrefcount.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, *mut DBREFCOUNT) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub CreateAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, DBACCESSORFLAGS, DBCOUNTITEM, *const DBBINDING, DBLENGTH, *mut HACCESSOR, *mut DBBINDSTATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CreateAccessor: usize,
    #[cfg(feature = "oaidl")]
    pub GetBindings: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, *mut DBACCESSORFLAGS, *mut DBCOUNTITEM, *mut *mut DBBINDING) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetBindings: usize,
    pub ReleaseAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, *mut DBREFCOUNT) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IAccessor_Impl: windows_core::IUnknownImpl {
    fn AddRefAccessor(&self, haccessor: HACCESSOR, pcrefcount: *mut DBREFCOUNT) -> windows_core::Result<()>;
    fn CreateAccessor(&self, dwaccessorflags: DBACCESSORFLAGS, cbindings: DBCOUNTITEM, rgbindings: *const DBBINDING, cbrowsize: DBLENGTH, phaccessor: *mut HACCESSOR, rgstatus: *mut DBBINDSTATUS) -> windows_core::Result<()>;
    fn GetBindings(&self, haccessor: HACCESSOR, pdwaccessorflags: *mut DBACCESSORFLAGS, pcbindings: *mut DBCOUNTITEM, prgbindings: *mut *mut DBBINDING) -> windows_core::Result<()>;
    fn ReleaseAccessor(&self, haccessor: HACCESSOR, pcrefcount: *mut DBREFCOUNT) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IAccessor_Vtbl {
    pub const fn new<Identity: IAccessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRefAccessor<Identity: IAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, pcrefcount: *mut DBREFCOUNT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessor_Impl::AddRefAccessor(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pcrefcount)).into()
            }
        }
        unsafe extern "system" fn CreateAccessor<Identity: IAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaccessorflags: DBACCESSORFLAGS, cbindings: DBCOUNTITEM, rgbindings: *const DBBINDING, cbrowsize: DBLENGTH, phaccessor: *mut HACCESSOR, rgstatus: *mut DBBINDSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessor_Impl::CreateAccessor(this, core::mem::transmute_copy(&dwaccessorflags), core::mem::transmute_copy(&cbindings), core::mem::transmute_copy(&rgbindings), core::mem::transmute_copy(&cbrowsize), core::mem::transmute_copy(&phaccessor), core::mem::transmute_copy(&rgstatus)).into()
            }
        }
        unsafe extern "system" fn GetBindings<Identity: IAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, pdwaccessorflags: *mut DBACCESSORFLAGS, pcbindings: *mut DBCOUNTITEM, prgbindings: *mut *mut DBBINDING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessor_Impl::GetBindings(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdwaccessorflags), core::mem::transmute_copy(&pcbindings), core::mem::transmute_copy(&prgbindings)).into()
            }
        }
        unsafe extern "system" fn ReleaseAccessor<Identity: IAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, pcrefcount: *mut DBREFCOUNT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessor_Impl::ReleaseAccessor(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pcrefcount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefAccessor: AddRefAccessor::<Identity, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, OFFSET>,
            GetBindings: GetBindings::<Identity, OFFSET>,
            ReleaseAccessor: ReleaseAccessor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IAccessor {}
windows_core::imp::define_interface!(IAlterIndex, IAlterIndex_Vtbl, 0x0c733aa6_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IAlterIndex, windows_core::IUnknown);
impl IAlterIndex {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AlterIndex(&self, ptableid: *const DBID, pindexid: *const DBID, pnewindexid: *const DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AlterIndex)(windows_core::Interface::as_raw(self), ptableid, pindexid, pnewindexid, cpropertysets, rgpropertysets as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlterIndex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub AlterIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *const DBID, *const DBID, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    AlterIndex: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAlterIndex_Impl: windows_core::IUnknownImpl {
    fn AlterIndex(&self, ptableid: *const DBID, pindexid: *const DBID, pnewindexid: *const DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IAlterIndex_Vtbl {
    pub const fn new<Identity: IAlterIndex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AlterIndex<Identity: IAlterIndex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pindexid: *const DBID, pnewindexid: *const DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlterIndex_Impl::AlterIndex(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid), core::mem::transmute_copy(&pnewindexid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AlterIndex: AlterIndex::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlterIndex as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAlterIndex {}
windows_core::imp::define_interface!(IAlterTable, IAlterTable_Vtbl, 0x0c733aa5_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IAlterTable, windows_core::IUnknown);
impl IAlterTable {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AlterColumn(&self, ptableid: *const DBID, pcolumnid: *const DBID, dwcolumndescflags: DBCOLUMNDESCFLAGS, pcolumndesc: *const DBCOLUMNDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AlterColumn)(windows_core::Interface::as_raw(self), ptableid, pcolumnid, dwcolumndescflags, pcolumndesc) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AlterTable(&self, ptableid: *const DBID, pnewtableid: *const DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AlterTable)(windows_core::Interface::as_raw(self), ptableid, pnewtableid, cpropertysets, rgpropertysets as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlterTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub AlterColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *const DBID, DBCOLUMNDESCFLAGS, *const DBCOLUMNDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    AlterColumn: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub AlterTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *const DBID, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    AlterTable: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAlterTable_Impl: windows_core::IUnknownImpl {
    fn AlterColumn(&self, ptableid: *const DBID, pcolumnid: *const DBID, dwcolumndescflags: DBCOLUMNDESCFLAGS, pcolumndesc: *const DBCOLUMNDESC) -> windows_core::Result<()>;
    fn AlterTable(&self, ptableid: *const DBID, pnewtableid: *const DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IAlterTable_Vtbl {
    pub const fn new<Identity: IAlterTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AlterColumn<Identity: IAlterTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pcolumnid: *const DBID, dwcolumndescflags: DBCOLUMNDESCFLAGS, pcolumndesc: *const DBCOLUMNDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlterTable_Impl::AlterColumn(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pcolumnid), core::mem::transmute_copy(&dwcolumndescflags), core::mem::transmute_copy(&pcolumndesc)).into()
            }
        }
        unsafe extern "system" fn AlterTable<Identity: IAlterTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pnewtableid: *const DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlterTable_Impl::AlterTable(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pnewtableid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AlterColumn: AlterColumn::<Identity, OFFSET>,
            AlterTable: AlterTable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlterTable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAlterTable {}
windows_core::imp::define_interface!(IBindResource, IBindResource_Vtbl, 0x0c733ab1_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IBindResource, windows_core::IUnknown);
impl IBindResource {
    #[cfg(feature = "urlmon")]
    pub unsafe fn Bind<P0, P1, P5, T>(&self, punkouter: P0, pwszurl: P1, dwbindurlflags: DBBINDURLFLAG, rguid: *const windows_core::GUID, pauthenticate: P5, pimplsession: Option<*mut DBIMPLICITSESSION>, pdwbindstatus: Option<*mut DBBINDURLSTATUS>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<super::IAuthenticate>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), punkouter.param().abi(), pwszurl.param().abi(), dwbindurlflags, rguid, &T::IID, pauthenticate.param().abi(), pimplsession.unwrap_or(core::mem::zeroed()) as _, pdwbindstatus.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "urlmon")]
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, DBBINDURLFLAG, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut DBIMPLICITSESSION, *mut DBBINDURLSTATUS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "urlmon"))]
    Bind: usize,
}
#[cfg(feature = "urlmon")]
pub trait IBindResource_Impl: windows_core::IUnknownImpl {
    fn Bind(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, pwszurl: &windows_core::PCWSTR, dwbindurlflags: DBBINDURLFLAG, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: windows_core::Ref<super::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut DBBINDURLSTATUS, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "urlmon")]
impl IBindResource_Vtbl {
    pub const fn new<Identity: IBindResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Bind<Identity: IBindResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwbindurlflags: DBBINDURLFLAG, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: *mut core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut DBBINDURLSTATUS, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindResource_Impl::Bind(this, core::mem::transmute_copy(&punkouter), core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwbindurlflags), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pauthenticate), core::mem::transmute_copy(&pimplsession), core::mem::transmute_copy(&pdwbindstatus), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Bind: Bind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindResource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "urlmon")]
impl windows_core::RuntimeName for IBindResource {}
windows_core::imp::define_interface!(IChapteredRowset, IChapteredRowset_Vtbl, 0x0c733a93_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IChapteredRowset, windows_core::IUnknown);
impl IChapteredRowset {
    pub unsafe fn AddRefChapter(&self, hchapter: HCHAPTER, pcrefcount: Option<*mut DBREFCOUNT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRefChapter)(windows_core::Interface::as_raw(self), hchapter, pcrefcount.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ReleaseChapter(&self, hchapter: HCHAPTER, pcrefcount: Option<*mut DBREFCOUNT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseChapter)(windows_core::Interface::as_raw(self), hchapter, pcrefcount.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChapteredRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefChapter: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, *mut DBREFCOUNT) -> windows_core::HRESULT,
    pub ReleaseChapter: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, *mut DBREFCOUNT) -> windows_core::HRESULT,
}
pub trait IChapteredRowset_Impl: windows_core::IUnknownImpl {
    fn AddRefChapter(&self, hchapter: HCHAPTER, pcrefcount: *mut DBREFCOUNT) -> windows_core::Result<()>;
    fn ReleaseChapter(&self, hchapter: HCHAPTER, pcrefcount: *mut DBREFCOUNT) -> windows_core::Result<()>;
}
impl IChapteredRowset_Vtbl {
    pub const fn new<Identity: IChapteredRowset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRefChapter<Identity: IChapteredRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, pcrefcount: *mut DBREFCOUNT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChapteredRowset_Impl::AddRefChapter(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&pcrefcount)).into()
            }
        }
        unsafe extern "system" fn ReleaseChapter<Identity: IChapteredRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, pcrefcount: *mut DBREFCOUNT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChapteredRowset_Impl::ReleaseChapter(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&pcrefcount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefChapter: AddRefChapter::<Identity, OFFSET>,
            ReleaseChapter: ReleaseChapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChapteredRowset as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IChapteredRowset {}
windows_core::imp::define_interface!(IColumnsInfo, IColumnsInfo_Vtbl, 0x0c733a11_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IColumnsInfo, windows_core::IUnknown);
impl IColumnsInfo {
    #[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
    pub unsafe fn GetColumnInfo(&self, pccolumns: *mut DBORDINAL, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetColumnInfo)(windows_core::Interface::as_raw(self), pccolumns as _, prginfo as _, ppstringsbuffer as _) }
    }
    pub unsafe fn MapColumnIDs(&self, ccolumnids: DBORDINAL, rgcolumnids: Option<*const DBID>, rgcolumns: Option<*mut DBORDINAL>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MapColumnIDs)(windows_core::Interface::as_raw(self), ccolumnids, rgcolumnids.unwrap_or(core::mem::zeroed()) as _, rgcolumns.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IColumnsInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
    pub GetColumnInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DBORDINAL, *mut *mut DBCOLUMNINFO, *mut *mut super::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypesbase")))]
    GetColumnInfo: usize,
    pub MapColumnIDs: unsafe extern "system" fn(*mut core::ffi::c_void, DBORDINAL, *const DBID, *mut DBORDINAL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
pub trait IColumnsInfo_Impl: windows_core::IUnknownImpl {
    fn GetColumnInfo(&self, pccolumns: *mut DBORDINAL, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::Result<()>;
    fn MapColumnIDs(&self, ccolumnids: DBORDINAL, rgcolumnids: *const DBID, rgcolumns: *mut DBORDINAL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
impl IColumnsInfo_Vtbl {
    pub const fn new<Identity: IColumnsInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColumnInfo<Identity: IColumnsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccolumns: *mut DBORDINAL, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnsInfo_Impl::GetColumnInfo(this, core::mem::transmute_copy(&pccolumns), core::mem::transmute_copy(&prginfo), core::mem::transmute_copy(&ppstringsbuffer)).into()
            }
        }
        unsafe extern "system" fn MapColumnIDs<Identity: IColumnsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumnids: DBORDINAL, rgcolumnids: *const DBID, rgcolumns: *mut DBORDINAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnsInfo_Impl::MapColumnIDs(this, core::mem::transmute_copy(&ccolumnids), core::mem::transmute_copy(&rgcolumnids), core::mem::transmute_copy(&rgcolumns)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumnInfo: GetColumnInfo::<Identity, OFFSET>,
            MapColumnIDs: MapColumnIDs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnsInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IColumnsInfo {}
windows_core::imp::define_interface!(IColumnsInfo2, IColumnsInfo2_Vtbl, 0x0c733ab8_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IColumnsInfo2 {
    type Target = IColumnsInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IColumnsInfo2, windows_core::IUnknown, IColumnsInfo);
impl IColumnsInfo2 {
    #[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
    pub unsafe fn GetRestrictedColumnInfo(&self, rgcolumnidmasks: &[DBID], dwflags: u32, pccolumns: *mut DBORDINAL, prgcolumnids: *mut *mut DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRestrictedColumnInfo)(windows_core::Interface::as_raw(self), DBORDINAL(rgcolumnidmasks.len().try_into().unwrap()), rgcolumnidmasks.as_ptr(), dwflags, pccolumns as _, prgcolumnids as _, prgcolumninfo as _, ppstringsbuffer as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IColumnsInfo2_Vtbl {
    pub base__: IColumnsInfo_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
    pub GetRestrictedColumnInfo: unsafe extern "system" fn(*mut core::ffi::c_void, DBORDINAL, *const DBID, u32, *mut DBORDINAL, *mut *mut DBID, *mut *mut DBCOLUMNINFO, *mut *mut super::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypesbase")))]
    GetRestrictedColumnInfo: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
pub trait IColumnsInfo2_Impl: IColumnsInfo_Impl {
    fn GetRestrictedColumnInfo(&self, ccolumnidmasks: DBORDINAL, rgcolumnidmasks: *const DBID, dwflags: u32, pccolumns: *mut DBORDINAL, prgcolumnids: *mut *mut DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
impl IColumnsInfo2_Vtbl {
    pub const fn new<Identity: IColumnsInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRestrictedColumnInfo<Identity: IColumnsInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumnidmasks: DBORDINAL, rgcolumnidmasks: *const DBID, dwflags: u32, pccolumns: *mut DBORDINAL, prgcolumnids: *mut *mut DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnsInfo2_Impl::GetRestrictedColumnInfo(this, core::mem::transmute_copy(&ccolumnidmasks), core::mem::transmute_copy(&rgcolumnidmasks), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pccolumns), core::mem::transmute_copy(&prgcolumnids), core::mem::transmute_copy(&prgcolumninfo), core::mem::transmute_copy(&ppstringsbuffer)).into()
            }
        }
        Self { base__: IColumnsInfo_Vtbl::new::<Identity, OFFSET>(), GetRestrictedColumnInfo: GetRestrictedColumnInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnsInfo2 as windows_core::Interface>::IID || iid == &<IColumnsInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IColumnsInfo2 {}
windows_core::imp::define_interface!(IColumnsRowset, IColumnsRowset_Vtbl, 0x0c733a10_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IColumnsRowset, windows_core::IUnknown);
impl IColumnsRowset {
    pub unsafe fn GetAvailableColumns(&self, pcoptcolumns: *mut DBORDINAL, prgoptcolumns: *mut *mut DBID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableColumns)(windows_core::Interface::as_raw(self), pcoptcolumns as _, prgoptcolumns as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetColumnsRowset<P0, T>(&self, punkouter: P0, rgoptcolumns: &[DBID], rgpropertysets: Option<&mut [DBPROPSET]>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetColumnsRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), DBORDINAL(rgoptcolumns.len().try_into().unwrap()), rgoptcolumns.as_ptr(), &T::IID, rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertysets.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IColumnsRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableColumns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DBORDINAL, *mut *mut DBID) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetColumnsRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DBORDINAL, *const DBID, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetColumnsRowset: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IColumnsRowset_Impl: windows_core::IUnknownImpl {
    fn GetAvailableColumns(&self, pcoptcolumns: *mut DBORDINAL, prgoptcolumns: *mut *mut DBID) -> windows_core::Result<()>;
    fn GetColumnsRowset(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, coptcolumns: DBORDINAL, rgoptcolumns: *const DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IColumnsRowset_Vtbl {
    pub const fn new<Identity: IColumnsRowset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailableColumns<Identity: IColumnsRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoptcolumns: *mut DBORDINAL, prgoptcolumns: *mut *mut DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnsRowset_Impl::GetAvailableColumns(this, core::mem::transmute_copy(&pcoptcolumns), core::mem::transmute_copy(&prgoptcolumns)).into()
            }
        }
        unsafe extern "system" fn GetColumnsRowset<Identity: IColumnsRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, coptcolumns: DBORDINAL, rgoptcolumns: *const DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnsRowset_Impl::GetColumnsRowset(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&coptcolumns), core::mem::transmute_copy(&rgoptcolumns), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&ppcolrowset)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableColumns: GetAvailableColumns::<Identity, OFFSET>,
            GetColumnsRowset: GetColumnsRowset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnsRowset as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IColumnsRowset {}
windows_core::imp::define_interface!(ICommand, ICommand_Vtbl, 0x0c733a63_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICommand, windows_core::IUnknown);
impl ICommand {
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Execute<P0, T>(&self, punkouter: P0, pparams: Option<*mut DBPARAMS>, pcrowsaffected: Option<*mut DBROWCOUNT>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, pparams.unwrap_or(core::mem::zeroed()) as _, pcrowsaffected.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetDBSession<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDBSession)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut DBPARAMS, *mut DBROWCOUNT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDBSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICommand_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Execute(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut DBROWCOUNT, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDBSession(&self, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICommand_Vtbl {
    pub const fn new<Identity: ICommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: ICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommand_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Execute<Identity: ICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut DBROWCOUNT, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommand_Impl::Execute(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&pcrowsaffected), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        unsafe extern "system" fn GetDBSession<Identity: ICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommand_Impl::GetDBSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppsession)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Execute: Execute::<Identity, OFFSET>,
            GetDBSession: GetDBSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommand as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICommand {}
windows_core::imp::define_interface!(ICommandPersist, ICommandPersist_Vtbl, 0x0c733aa7_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICommandPersist, windows_core::IUnknown);
impl ICommandPersist {
    pub unsafe fn DeleteCommand(&self, pcommandid: *const DBID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteCommand)(windows_core::Interface::as_raw(self), pcommandid) }
    }
    pub unsafe fn GetCurrentCommand(&self) -> windows_core::Result<*mut DBID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentCommand)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LoadCommand(&self, pcommandid: *const DBID, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LoadCommand)(windows_core::Interface::as_raw(self), pcommandid, dwflags) }
    }
    pub unsafe fn SaveCommand(&self, pcommandid: *const DBID, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveCommand)(windows_core::Interface::as_raw(self), pcommandid, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandPersist_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeleteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID) -> windows_core::HRESULT,
    pub GetCurrentCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut DBID) -> windows_core::HRESULT,
    pub LoadCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, u32) -> windows_core::HRESULT,
    pub SaveCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, u32) -> windows_core::HRESULT,
}
pub trait ICommandPersist_Impl: windows_core::IUnknownImpl {
    fn DeleteCommand(&self, pcommandid: *const DBID) -> windows_core::Result<()>;
    fn GetCurrentCommand(&self) -> windows_core::Result<*mut DBID>;
    fn LoadCommand(&self, pcommandid: *const DBID, dwflags: u32) -> windows_core::Result<()>;
    fn SaveCommand(&self, pcommandid: *const DBID, dwflags: u32) -> windows_core::Result<()>;
}
impl ICommandPersist_Vtbl {
    pub const fn new<Identity: ICommandPersist_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteCommand<Identity: ICommandPersist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandid: *const DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandPersist_Impl::DeleteCommand(this, core::mem::transmute_copy(&pcommandid)).into()
            }
        }
        unsafe extern "system" fn GetCurrentCommand<Identity: ICommandPersist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcommandid: *mut *mut DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICommandPersist_Impl::GetCurrentCommand(this) {
                    Ok(ok__) => {
                        ppcommandid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadCommand<Identity: ICommandPersist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandid: *const DBID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandPersist_Impl::LoadCommand(this, core::mem::transmute_copy(&pcommandid), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SaveCommand<Identity: ICommandPersist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandid: *const DBID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandPersist_Impl::SaveCommand(this, core::mem::transmute_copy(&pcommandid), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeleteCommand: DeleteCommand::<Identity, OFFSET>,
            GetCurrentCommand: GetCurrentCommand::<Identity, OFFSET>,
            LoadCommand: LoadCommand::<Identity, OFFSET>,
            SaveCommand: SaveCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICommandPersist {}
windows_core::imp::define_interface!(ICommandPrepare, ICommandPrepare_Vtbl, 0x0c733a26_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICommandPrepare, windows_core::IUnknown);
impl ICommandPrepare {
    pub unsafe fn Prepare(&self, cexpectedruns: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self), cexpectedruns) }
    }
    pub unsafe fn Unprepare(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unprepare)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandPrepare_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unprepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICommandPrepare_Impl: windows_core::IUnknownImpl {
    fn Prepare(&self, cexpectedruns: u32) -> windows_core::Result<()>;
    fn Unprepare(&self) -> windows_core::Result<()>;
}
impl ICommandPrepare_Vtbl {
    pub const fn new<Identity: ICommandPrepare_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Prepare<Identity: ICommandPrepare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexpectedruns: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandPrepare_Impl::Prepare(this, core::mem::transmute_copy(&cexpectedruns)).into()
            }
        }
        unsafe extern "system" fn Unprepare<Identity: ICommandPrepare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandPrepare_Impl::Unprepare(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Prepare: Prepare::<Identity, OFFSET>, Unprepare: Unprepare::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandPrepare as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICommandPrepare {}
windows_core::imp::define_interface!(ICommandProperties, ICommandProperties_Vtbl, 0x0c733a79_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICommandProperties, windows_core::IUnknown);
impl ICommandProperties {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertyidsets.map_or(core::ptr::null(), |slice| slice.as_ptr()), pcpropertysets as _, prgpropertysets as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetProperties(&self, rgpropertysets: &[DBPROPSET]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperties)(windows_core::Interface::as_raw(self), rgpropertysets.len().try_into().unwrap(), rgpropertysets.as_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetProperties: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    SetProperties: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICommandProperties_Impl: windows_core::IUnknownImpl {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn SetProperties(&self, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ICommandProperties_Vtbl {
    pub const fn new<Identity: ICommandProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperties<Identity: ICommandProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandProperties_Impl::GetProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
            }
        }
        unsafe extern "system" fn SetProperties<Identity: ICommandProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandProperties_Impl::SetProperties(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            SetProperties: SetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandProperties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICommandProperties {}
windows_core::imp::define_interface!(ICommandStream, ICommandStream_Vtbl, 0x0c733abf_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICommandStream, windows_core::IUnknown);
impl ICommandStream {
    pub unsafe fn GetCommandStream(&self, piid: Option<*mut windows_core::GUID>, pguiddialect: Option<*mut windows_core::GUID>, ppcommandstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCommandStream)(windows_core::Interface::as_raw(self), piid.unwrap_or(core::mem::zeroed()) as _, pguiddialect.unwrap_or(core::mem::zeroed()) as _, ppcommandstream as _) }
    }
    pub unsafe fn SetCommandStream<P2>(&self, riid: *const windows_core::GUID, rguiddialect: *const windows_core::GUID, pcommandstream: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCommandStream)(windows_core::Interface::as_raw(self), riid, rguiddialect, pcommandstream.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCommandStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommandStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICommandStream_Impl: windows_core::IUnknownImpl {
    fn GetCommandStream(&self, piid: *mut windows_core::GUID, pguiddialect: *mut windows_core::GUID, ppcommandstream: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetCommandStream(&self, riid: *const windows_core::GUID, rguiddialect: *const windows_core::GUID, pcommandstream: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ICommandStream_Vtbl {
    pub const fn new<Identity: ICommandStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCommandStream<Identity: ICommandStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID, pguiddialect: *mut windows_core::GUID, ppcommandstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandStream_Impl::GetCommandStream(this, core::mem::transmute_copy(&piid), core::mem::transmute_copy(&pguiddialect), core::mem::transmute_copy(&ppcommandstream)).into()
            }
        }
        unsafe extern "system" fn SetCommandStream<Identity: ICommandStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, rguiddialect: *const windows_core::GUID, pcommandstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandStream_Impl::SetCommandStream(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rguiddialect), core::mem::transmute_copy(&pcommandstream)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCommandStream: GetCommandStream::<Identity, OFFSET>,
            SetCommandStream: SetCommandStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICommandStream {}
windows_core::imp::define_interface!(ICommandText, ICommandText_Vtbl, 0x0c733a27_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommandText {
    type Target = ICommand;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommandText, windows_core::IUnknown, ICommand);
impl ICommandText {
    pub unsafe fn GetCommandText(&self, pguiddialect: Option<*mut windows_core::GUID>, ppwszcommand: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCommandText)(windows_core::Interface::as_raw(self), pguiddialect.unwrap_or(core::mem::zeroed()) as _, ppwszcommand as _) }
    }
    pub unsafe fn SetCommandText<P1>(&self, rguiddialect: *const windows_core::GUID, pwszcommand: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCommandText)(windows_core::Interface::as_raw(self), rguiddialect, pwszcommand.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandText_Vtbl {
    pub base__: ICommand_Vtbl,
    pub GetCommandText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCommandText: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ICommandText_Impl: ICommand_Impl {
    fn GetCommandText(&self, pguiddialect: *mut windows_core::GUID, ppwszcommand: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetCommandText(&self, rguiddialect: *const windows_core::GUID, pwszcommand: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ICommandText_Vtbl {
    pub const fn new<Identity: ICommandText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCommandText<Identity: ICommandText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguiddialect: *mut windows_core::GUID, ppwszcommand: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandText_Impl::GetCommandText(this, core::mem::transmute_copy(&pguiddialect), core::mem::transmute_copy(&ppwszcommand)).into()
            }
        }
        unsafe extern "system" fn SetCommandText<Identity: ICommandText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguiddialect: *const windows_core::GUID, pwszcommand: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandText_Impl::SetCommandText(this, core::mem::transmute_copy(&rguiddialect), core::mem::transmute(&pwszcommand)).into()
            }
        }
        Self {
            base__: ICommand_Vtbl::new::<Identity, OFFSET>(),
            GetCommandText: GetCommandText::<Identity, OFFSET>,
            SetCommandText: SetCommandText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandText as windows_core::Interface>::IID || iid == &<ICommand as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICommandText {}
windows_core::imp::define_interface!(ICommandWithParameters, ICommandWithParameters_Vtbl, 0x0c733a64_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICommandWithParameters, windows_core::IUnknown);
impl ICommandWithParameters {
    #[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
    pub unsafe fn GetParameterInfo(&self, pcparams: *mut DB_UPARAMS, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetParameterInfo)(windows_core::Interface::as_raw(self), pcparams as _, prgparaminfo as _, ppnamesbuffer as _) }
    }
    pub unsafe fn MapParameterNames(&self, cparamnames: DB_UPARAMS, rgparamnames: *const windows_core::PCWSTR, rgparamordinals: *mut DB_LPARAMS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MapParameterNames)(windows_core::Interface::as_raw(self), cparamnames, rgparamnames, rgparamordinals as _) }
    }
    pub unsafe fn SetParameterInfo(&self, cparams: DB_UPARAMS, rgparamordinals: Option<*const DB_UPARAMS>, rgparambindinfo: Option<*const DBPARAMBINDINFO>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameterInfo)(windows_core::Interface::as_raw(self), cparams, rgparamordinals.unwrap_or(core::mem::zeroed()) as _, rgparambindinfo.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandWithParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
    pub GetParameterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DB_UPARAMS, *mut *mut DBPARAMINFO, *mut *mut super::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypesbase")))]
    GetParameterInfo: usize,
    pub MapParameterNames: unsafe extern "system" fn(*mut core::ffi::c_void, DB_UPARAMS, *const windows_core::PCWSTR, *mut DB_LPARAMS) -> windows_core::HRESULT,
    pub SetParameterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, DB_UPARAMS, *const DB_UPARAMS, *const DBPARAMBINDINFO) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
pub trait ICommandWithParameters_Impl: windows_core::IUnknownImpl {
    fn GetParameterInfo(&self, pcparams: *mut DB_UPARAMS, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut super::OLECHAR) -> windows_core::Result<()>;
    fn MapParameterNames(&self, cparamnames: DB_UPARAMS, rgparamnames: *const windows_core::PCWSTR, rgparamordinals: *mut DB_LPARAMS) -> windows_core::Result<()>;
    fn SetParameterInfo(&self, cparams: DB_UPARAMS, rgparamordinals: *const DB_UPARAMS, rgparambindinfo: *const DBPARAMBINDINFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
impl ICommandWithParameters_Vtbl {
    pub const fn new<Identity: ICommandWithParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParameterInfo<Identity: ICommandWithParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcparams: *mut DB_UPARAMS, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandWithParameters_Impl::GetParameterInfo(this, core::mem::transmute_copy(&pcparams), core::mem::transmute_copy(&prgparaminfo), core::mem::transmute_copy(&ppnamesbuffer)).into()
            }
        }
        unsafe extern "system" fn MapParameterNames<Identity: ICommandWithParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cparamnames: DB_UPARAMS, rgparamnames: *const windows_core::PCWSTR, rgparamordinals: *mut DB_LPARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandWithParameters_Impl::MapParameterNames(this, core::mem::transmute_copy(&cparamnames), core::mem::transmute_copy(&rgparamnames), core::mem::transmute_copy(&rgparamordinals)).into()
            }
        }
        unsafe extern "system" fn SetParameterInfo<Identity: ICommandWithParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cparams: DB_UPARAMS, rgparamordinals: *const DB_UPARAMS, rgparambindinfo: *const DBPARAMBINDINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandWithParameters_Impl::SetParameterInfo(this, core::mem::transmute_copy(&cparams), core::mem::transmute_copy(&rgparamordinals), core::mem::transmute_copy(&rgparambindinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameterInfo: GetParameterInfo::<Identity, OFFSET>,
            MapParameterNames: MapParameterNames::<Identity, OFFSET>,
            SetParameterInfo: SetParameterInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandWithParameters as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICommandWithParameters {}
windows_core::imp::define_interface!(IConvertType, IConvertType_Vtbl, 0x0c733a88_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IConvertType, windows_core::IUnknown);
impl IConvertType {
    pub unsafe fn CanConvert(&self, wfromtype: DBTYPE, wtotype: DBTYPE, dwconvertflags: DBCONVERTFLAGS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CanConvert)(windows_core::Interface::as_raw(self), wfromtype, wtotype, dwconvertflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConvertType_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CanConvert: unsafe extern "system" fn(*mut core::ffi::c_void, DBTYPE, DBTYPE, DBCONVERTFLAGS) -> windows_core::HRESULT,
}
pub trait IConvertType_Impl: windows_core::IUnknownImpl {
    fn CanConvert(&self, wfromtype: DBTYPE, wtotype: DBTYPE, dwconvertflags: DBCONVERTFLAGS) -> windows_core::Result<()>;
}
impl IConvertType_Vtbl {
    pub const fn new<Identity: IConvertType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanConvert<Identity: IConvertType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wfromtype: DBTYPE, wtotype: DBTYPE, dwconvertflags: DBCONVERTFLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConvertType_Impl::CanConvert(this, core::mem::transmute_copy(&wfromtype), core::mem::transmute_copy(&wtotype), core::mem::transmute_copy(&dwconvertflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CanConvert: CanConvert::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConvertType as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConvertType {}
windows_core::imp::define_interface!(ICreateRow, ICreateRow_Vtbl, 0x0c733ab2_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICreateRow, windows_core::IUnknown);
impl ICreateRow {
    #[cfg(feature = "urlmon")]
    pub unsafe fn CreateRow<P0, P1, P5, T>(&self, punkouter: P0, pwszurl: P1, dwbindurlflags: DBBINDURLFLAG, rguid: *const windows_core::GUID, pauthenticate: P5, pimplsession: Option<*mut DBIMPLICITSESSION>, pdwbindstatus: *mut DBBINDURLSTATUS, ppwsznewurl: *mut windows_core::PWSTR) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<super::IAuthenticate>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateRow)(windows_core::Interface::as_raw(self), punkouter.param().abi(), pwszurl.param().abi(), dwbindurlflags, rguid, &T::IID, pauthenticate.param().abi(), pimplsession.unwrap_or(core::mem::zeroed()) as _, pdwbindstatus as _, ppwsznewurl as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateRow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "urlmon")]
    pub CreateRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, DBBINDURLFLAG, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut DBIMPLICITSESSION, *mut DBBINDURLSTATUS, *mut windows_core::PWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "urlmon"))]
    CreateRow: usize,
}
#[cfg(feature = "urlmon")]
pub trait ICreateRow_Impl: windows_core::IUnknownImpl {
    fn CreateRow(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, pwszurl: &windows_core::PCWSTR, dwbindurlflags: DBBINDURLFLAG, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: windows_core::Ref<super::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut DBBINDURLSTATUS, ppwsznewurl: *mut windows_core::PWSTR, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "urlmon")]
impl ICreateRow_Vtbl {
    pub const fn new<Identity: ICreateRow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateRow<Identity: ICreateRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwbindurlflags: DBBINDURLFLAG, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: *mut core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut DBBINDURLSTATUS, ppwsznewurl: *mut windows_core::PWSTR, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateRow_Impl::CreateRow(this, core::mem::transmute_copy(&punkouter), core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwbindurlflags), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pauthenticate), core::mem::transmute_copy(&pimplsession), core::mem::transmute_copy(&pdwbindstatus), core::mem::transmute_copy(&ppwsznewurl), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRow: CreateRow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateRow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "urlmon")]
impl windows_core::RuntimeName for ICreateRow {}
windows_core::imp::define_interface!(IDBAsynchNotify, IDBAsynchNotify_Vtbl, 0x0c733a96_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IDBAsynchNotify, windows_core::IUnknown);
impl IDBAsynchNotify {
    #[cfg(feature = "winnt")]
    pub unsafe fn OnLowResource(&self, dwreserved: DB_DWRESERVE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLowResource)(windows_core::Interface::as_raw(self), dwreserved) }
    }
    pub unsafe fn OnProgress<P5>(&self, hchapter: HCHAPTER, eoperation: DBASYNCHOP, ulprogress: DBCOUNTITEM, ulprogressmax: DBCOUNTITEM, easynchphase: DBASYNCHPHASE, pwszstatustext: P5) -> windows_core::HRESULT
    where
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), hchapter, eoperation, ulprogress, ulprogressmax, easynchphase, pwszstatustext.param().abi()) }
    }
    pub unsafe fn OnStop<P3>(&self, hchapter: HCHAPTER, eoperation: DBASYNCHOP, hrstatus: windows_core::HRESULT, pwszstatustext: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStop)(windows_core::Interface::as_raw(self), hchapter, eoperation, hrstatus, pwszstatustext.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBAsynchNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub OnLowResource: unsafe extern "system" fn(*mut core::ffi::c_void, DB_DWRESERVE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    OnLowResource: usize,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBASYNCHOP, DBCOUNTITEM, DBCOUNTITEM, DBASYNCHPHASE, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnStop: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBASYNCHOP, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IDBAsynchNotify_Impl: windows_core::IUnknownImpl {
    fn OnLowResource(&self, dwreserved: DB_DWRESERVE) -> windows_core::Result<()>;
    fn OnProgress(&self, hchapter: HCHAPTER, eoperation: DBASYNCHOP, ulprogress: DBCOUNTITEM, ulprogressmax: DBCOUNTITEM, easynchphase: DBASYNCHPHASE, pwszstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnStop(&self, hchapter: HCHAPTER, eoperation: DBASYNCHOP, hrstatus: windows_core::HRESULT, pwszstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IDBAsynchNotify_Vtbl {
    pub const fn new<Identity: IDBAsynchNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLowResource<Identity: IDBAsynchNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: DB_DWRESERVE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBAsynchNotify_Impl::OnLowResource(this, core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn OnProgress<Identity: IDBAsynchNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, eoperation: DBASYNCHOP, ulprogress: DBCOUNTITEM, ulprogressmax: DBCOUNTITEM, easynchphase: DBASYNCHPHASE, pwszstatustext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBAsynchNotify_Impl::OnProgress(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&eoperation), core::mem::transmute_copy(&ulprogress), core::mem::transmute_copy(&ulprogressmax), core::mem::transmute_copy(&easynchphase), core::mem::transmute(&pwszstatustext)).into()
            }
        }
        unsafe extern "system" fn OnStop<Identity: IDBAsynchNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, eoperation: DBASYNCHOP, hrstatus: windows_core::HRESULT, pwszstatustext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBAsynchNotify_Impl::OnStop(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&eoperation), core::mem::transmute_copy(&hrstatus), core::mem::transmute(&pwszstatustext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLowResource: OnLowResource::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnStop: OnStop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBAsynchNotify as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IDBAsynchNotify {}
windows_core::imp::define_interface!(IDBAsynchStatus, IDBAsynchStatus_Vtbl, 0x0c733a95_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IDBAsynchStatus, windows_core::IUnknown);
impl IDBAsynchStatus {
    pub unsafe fn Abort(&self, hchapter: HCHAPTER, eoperation: DBASYNCHOP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), hchapter, eoperation) }
    }
    pub unsafe fn GetStatus(&self, hchapter: HCHAPTER, eoperation: DBASYNCHOP, pulprogress: Option<*mut DBCOUNTITEM>, pulprogressmax: Option<*mut DBCOUNTITEM>, peasynchphase: *mut DBASYNCHPHASE, ppwszstatustext: Option<*mut windows_core::PWSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), hchapter, eoperation, pulprogress.unwrap_or(core::mem::zeroed()) as _, pulprogressmax.unwrap_or(core::mem::zeroed()) as _, peasynchphase as _, ppwszstatustext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBAsynchStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBASYNCHOP) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBASYNCHOP, *mut DBCOUNTITEM, *mut DBCOUNTITEM, *mut DBASYNCHPHASE, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IDBAsynchStatus_Impl: windows_core::IUnknownImpl {
    fn Abort(&self, hchapter: HCHAPTER, eoperation: DBASYNCHOP) -> windows_core::Result<()>;
    fn GetStatus(&self, hchapter: HCHAPTER, eoperation: DBASYNCHOP, pulprogress: *mut DBCOUNTITEM, pulprogressmax: *mut DBCOUNTITEM, peasynchphase: *mut DBASYNCHPHASE, ppwszstatustext: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl IDBAsynchStatus_Vtbl {
    pub const fn new<Identity: IDBAsynchStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Abort<Identity: IDBAsynchStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, eoperation: DBASYNCHOP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBAsynchStatus_Impl::Abort(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&eoperation)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDBAsynchStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, eoperation: DBASYNCHOP, pulprogress: *mut DBCOUNTITEM, pulprogressmax: *mut DBCOUNTITEM, peasynchphase: *mut DBASYNCHPHASE, ppwszstatustext: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBAsynchStatus_Impl::GetStatus(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&eoperation), core::mem::transmute_copy(&pulprogress), core::mem::transmute_copy(&pulprogressmax), core::mem::transmute_copy(&peasynchphase), core::mem::transmute_copy(&ppwszstatustext)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Abort: Abort::<Identity, OFFSET>, GetStatus: GetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBAsynchStatus as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDBAsynchStatus {}
windows_core::imp::define_interface!(IDBBinderProperties, IDBBinderProperties_Vtbl, 0x0c733ab3_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBBinderProperties {
    type Target = IDBProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBBinderProperties, windows_core::IUnknown, IDBProperties);
impl IDBBinderProperties {
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBBinderProperties_Vtbl {
    pub base__: IDBProperties_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDBBinderProperties_Impl: IDBProperties_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IDBBinderProperties_Vtbl {
    pub const fn new<Identity: IDBBinderProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IDBBinderProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBBinderProperties_Impl::Reset(this).into()
            }
        }
        Self { base__: IDBProperties_Vtbl::new::<Identity, OFFSET>(), Reset: Reset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBBinderProperties as windows_core::Interface>::IID || iid == &<IDBProperties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDBBinderProperties {}
windows_core::imp::define_interface!(IDBCreateCommand, IDBCreateCommand_Vtbl, 0x0c733a1d_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IDBCreateCommand, windows_core::IUnknown);
impl IDBCreateCommand {
    pub unsafe fn CreateCommand<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateCommand)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBCreateCommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDBCreateCommand_Impl: windows_core::IUnknownImpl {
    fn CreateCommand(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppcommand: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDBCreateCommand_Vtbl {
    pub const fn new<Identity: IDBCreateCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateCommand<Identity: IDBCreateCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcommand: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBCreateCommand_Impl::CreateCommand(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcommand)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateCommand: CreateCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBCreateCommand as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDBCreateCommand {}
windows_core::imp::define_interface!(IDBCreateSession, IDBCreateSession_Vtbl, 0x0c733a5d_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IDBCreateSession, windows_core::IUnknown);
impl IDBCreateSession {
    pub unsafe fn CreateSession<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateSession)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBCreateSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDBCreateSession_Impl: windows_core::IUnknownImpl {
    fn CreateSession(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppdbsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDBCreateSession_Vtbl {
    pub const fn new<Identity: IDBCreateSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSession<Identity: IDBCreateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdbsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBCreateSession_Impl::CreateSession(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdbsession)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateSession: CreateSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBCreateSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDBCreateSession {}
windows_core::imp::define_interface!(IDBDataSourceAdmin, IDBDataSourceAdmin_Vtbl, 0x0c733a7a_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IDBDataSourceAdmin, windows_core::IUnknown);
impl IDBDataSourceAdmin {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreateDataSource<P2, T>(&self, rgpropertysets: Option<&mut [DBPROPSET]>, punkouter: P2) -> windows_core::Result<T>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateDataSource)(windows_core::Interface::as_raw(self), rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertysets.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn DestroyDataSource(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DestroyDataSource)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCreationProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCreationProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertyidsets.map_or(core::ptr::null(), |slice| slice.as_ptr()), pcpropertyinfosets as _, prgpropertyinfosets as _, ppdescbuffer as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ModifyDataSource(&self, rgpropertysets: Option<&mut [DBPROPSET]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ModifyDataSource)(windows_core::Interface::as_raw(self), rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertysets.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBDataSourceAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub CreateDataSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DBPROPSET, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    CreateDataSource: usize,
    pub DestroyDataSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCreationProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPINFOSET, *mut *mut super::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetCreationProperties: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub ModifyDataSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    ModifyDataSource: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDBDataSourceAdmin_Impl: windows_core::IUnknownImpl {
    fn CreateDataSource(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppdbsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DestroyDataSource(&self) -> windows_core::Result<()>;
    fn GetCreationProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut super::OLECHAR) -> windows_core::Result<()>;
    fn ModifyDataSource(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IDBDataSourceAdmin_Vtbl {
    pub const fn new<Identity: IDBDataSourceAdmin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDataSource<Identity: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdbsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBDataSourceAdmin_Impl::CreateDataSource(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdbsession)).into()
            }
        }
        unsafe extern "system" fn DestroyDataSource<Identity: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBDataSourceAdmin_Impl::DestroyDataSource(this).into()
            }
        }
        unsafe extern "system" fn GetCreationProperties<Identity: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBDataSourceAdmin_Impl::GetCreationProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertyinfosets), core::mem::transmute_copy(&prgpropertyinfosets), core::mem::transmute_copy(&ppdescbuffer)).into()
            }
        }
        unsafe extern "system" fn ModifyDataSource<Identity: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBDataSourceAdmin_Impl::ModifyDataSource(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDataSource: CreateDataSource::<Identity, OFFSET>,
            DestroyDataSource: DestroyDataSource::<Identity, OFFSET>,
            GetCreationProperties: GetCreationProperties::<Identity, OFFSET>,
            ModifyDataSource: ModifyDataSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBDataSourceAdmin as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDBDataSourceAdmin {}
windows_core::imp::define_interface!(IDBInfo, IDBInfo_Vtbl, 0x0c733a89_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IDBInfo, windows_core::IUnknown);
impl IDBInfo {
    pub unsafe fn GetKeywords(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeywords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetLiteralInfo(&self, rgliterals: Option<&[DBLITERAL]>, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLiteralInfo)(windows_core::Interface::as_raw(self), rgliterals.map_or(0, |slice| slice.len().try_into().unwrap()), rgliterals.map_or(core::ptr::null(), |slice| slice.as_ptr()), pcliteralinfo as _, prgliteralinfo as _, ppcharbuffer as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub GetLiteralInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBLITERAL, *mut u32, *mut *mut DBLITERALINFO, *mut *mut super::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetLiteralInfo: usize,
}
#[cfg(feature = "wtypesbase")]
pub trait IDBInfo_Impl: windows_core::IUnknownImpl {
    fn GetKeywords(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetLiteralInfo(&self, cliterals: u32, rgliterals: *const DBLITERAL, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut super::OLECHAR) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypesbase")]
impl IDBInfo_Vtbl {
    pub const fn new<Identity: IDBInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetKeywords<Identity: IDBInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszkeywords: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDBInfo_Impl::GetKeywords(this) {
                    Ok(ok__) => {
                        ppwszkeywords.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLiteralInfo<Identity: IDBInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cliterals: u32, rgliterals: *const DBLITERAL, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBInfo_Impl::GetLiteralInfo(this, core::mem::transmute_copy(&cliterals), core::mem::transmute_copy(&rgliterals), core::mem::transmute_copy(&pcliteralinfo), core::mem::transmute_copy(&prgliteralinfo), core::mem::transmute_copy(&ppcharbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetKeywords: GetKeywords::<Identity, OFFSET>,
            GetLiteralInfo: GetLiteralInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypesbase")]
impl windows_core::RuntimeName for IDBInfo {}
windows_core::imp::define_interface!(IDBInitialize, IDBInitialize_Vtbl, 0x0c733a8b_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IDBInitialize, windows_core::IUnknown);
impl IDBInitialize {
    pub unsafe fn Initialize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Uninitialize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Uninitialize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBInitialize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDBInitialize_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
}
impl IDBInitialize_Vtbl {
    pub const fn new<Identity: IDBInitialize_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDBInitialize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBInitialize_Impl::Initialize(this).into()
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: IDBInitialize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBInitialize_Impl::Uninitialize(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBInitialize as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDBInitialize {}
windows_core::imp::define_interface!(IDBProperties, IDBProperties_Vtbl, 0x0c733a8a_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IDBProperties, windows_core::IUnknown);
impl IDBProperties {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertyidsets.map_or(core::ptr::null(), |slice| slice.as_ptr()), pcpropertysets as _, prgpropertysets as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPropertyInfo(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), rgpropertyidsets.map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertyidsets.map_or(core::ptr::null(), |slice| slice.as_ptr()), pcpropertyinfosets as _, prgpropertyinfosets as _, ppdescbuffer as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetProperties(&self, rgpropertysets: Option<&mut [DBPROPSET]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperties)(windows_core::Interface::as_raw(self), rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertysets.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetProperties: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPINFOSET, *mut *mut super::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetPropertyInfo: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    SetProperties: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDBProperties_Impl: windows_core::IUnknownImpl {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn GetPropertyInfo(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut super::OLECHAR) -> windows_core::Result<()>;
    fn SetProperties(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IDBProperties_Vtbl {
    pub const fn new<Identity: IDBProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperties<Identity: IDBProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBProperties_Impl::GetProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: IDBProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBProperties_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertyinfosets), core::mem::transmute_copy(&prgpropertyinfosets), core::mem::transmute_copy(&ppdescbuffer)).into()
            }
        }
        unsafe extern "system" fn SetProperties<Identity: IDBProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBProperties_Impl::SetProperties(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            SetProperties: SetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBProperties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDBProperties {}
windows_core::imp::define_interface!(IDBSchemaRowset, IDBSchemaRowset_Vtbl, 0x0c733a7b_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IDBSchemaRowset, windows_core::IUnknown);
impl IDBSchemaRowset {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetRowset<P0, T>(&self, punkouter: P0, rguidschema: *const windows_core::GUID, rgrestrictions: Option<&[super::VARIANT]>, rgpropertysets: Option<&mut [DBPROPSET]>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), rguidschema, rgrestrictions.map_or(0, |slice| slice.len().try_into().unwrap()), rgrestrictions.map_or(core::ptr::null(), |slice| slice.as_ptr()), &T::IID, rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertysets.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetSchemas(&self, pcschemas: *mut u32, prgschemas: *mut *mut windows_core::GUID, prgrestrictionsupport: *mut *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSchemas)(windows_core::Interface::as_raw(self), pcschemas as _, prgschemas as _, prgrestrictionsupport as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDBSchemaRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, *const super::VARIANT, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetRowset: usize,
    pub GetSchemas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID, *mut *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDBSchemaRowset_Impl: windows_core::IUnknownImpl {
    fn GetRowset(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, rguidschema: *const windows_core::GUID, crestrictions: u32, rgrestrictions: *const super::VARIANT, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSchemas(&self, pcschemas: *mut u32, prgschemas: *mut *mut windows_core::GUID, prgrestrictionsupport: *mut *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IDBSchemaRowset_Vtbl {
    pub const fn new<Identity: IDBSchemaRowset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRowset<Identity: IDBSchemaRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, rguidschema: *const windows_core::GUID, crestrictions: u32, rgrestrictions: *const super::VARIANT, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBSchemaRowset_Impl::GetRowset(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&rguidschema), core::mem::transmute_copy(&crestrictions), core::mem::transmute_copy(&rgrestrictions), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        unsafe extern "system" fn GetSchemas<Identity: IDBSchemaRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut windows_core::GUID, prgrestrictionsupport: *mut *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDBSchemaRowset_Impl::GetSchemas(this, core::mem::transmute_copy(&pcschemas), core::mem::transmute_copy(&prgschemas), core::mem::transmute_copy(&prgrestrictionsupport)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowset: GetRowset::<Identity, OFFSET>,
            GetSchemas: GetSchemas::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBSchemaRowset as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDBSchemaRowset {}
pub const IDENTIFIER_SDK_ERROR: u32 = 268435456;
pub const IDENTIFIER_SDK_MASK: u32 = 4026531840;
windows_core::imp::define_interface!(IErrorLookup, IErrorLookup_Vtbl, 0x0c733a66_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IErrorLookup, windows_core::IUnknown);
impl IErrorLookup {
    #[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetErrorDescription(&self, hrerror: windows_core::HRESULT, dwlookupid: u32, pdispparams: *const super::DISPPARAMS, lcid: super::LCID, pbstrsource: *mut windows_core::BSTR, pbstrdescription: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), hrerror, dwlookupid, pdispparams, lcid, core::mem::transmute(pbstrsource), core::mem::transmute(pbstrdescription)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetHelpInfo(&self, hrerror: windows_core::HRESULT, dwlookupid: u32, lcid: super::LCID, pbstrhelpfile: *mut windows_core::BSTR, pdwhelpcontext: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHelpInfo)(windows_core::Interface::as_raw(self), hrerror, dwlookupid, lcid, core::mem::transmute(pbstrhelpfile), pdwhelpcontext as _) }
    }
    pub unsafe fn ReleaseErrors(&self, dwdynamicerrorid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseErrors)(windows_core::Interface::as_raw(self), dwdynamicerrorid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorLookup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *const super::DISPPARAMS, super::LCID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetErrorDescription: usize,
    #[cfg(feature = "winnt")]
    pub GetHelpInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, super::LCID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetHelpInfo: usize,
    pub ReleaseErrors: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IErrorLookup_Impl: windows_core::IUnknownImpl {
    fn GetErrorDescription(&self, hrerror: windows_core::HRESULT, dwlookupid: u32, pdispparams: *const super::DISPPARAMS, lcid: super::LCID, pbstrsource: *mut windows_core::BSTR, pbstrdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetHelpInfo(&self, hrerror: windows_core::HRESULT, dwlookupid: u32, lcid: super::LCID, pbstrhelpfile: *mut windows_core::BSTR, pdwhelpcontext: *mut u32) -> windows_core::Result<()>;
    fn ReleaseErrors(&self, dwdynamicerrorid: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IErrorLookup_Vtbl {
    pub const fn new<Identity: IErrorLookup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetErrorDescription<Identity: IErrorLookup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, dwlookupid: u32, pdispparams: *const super::DISPPARAMS, lcid: super::LCID, pbstrsource: *mut *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IErrorLookup_Impl::GetErrorDescription(this, core::mem::transmute_copy(&hrerror), core::mem::transmute_copy(&dwlookupid), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrsource), core::mem::transmute_copy(&pbstrdescription)).into()
            }
        }
        unsafe extern "system" fn GetHelpInfo<Identity: IErrorLookup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, dwlookupid: u32, lcid: super::LCID, pbstrhelpfile: *mut *mut core::ffi::c_void, pdwhelpcontext: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IErrorLookup_Impl::GetHelpInfo(this, core::mem::transmute_copy(&hrerror), core::mem::transmute_copy(&dwlookupid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrhelpfile), core::mem::transmute_copy(&pdwhelpcontext)).into()
            }
        }
        unsafe extern "system" fn ReleaseErrors<Identity: IErrorLookup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdynamicerrorid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IErrorLookup_Impl::ReleaseErrors(this, core::mem::transmute_copy(&dwdynamicerrorid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, OFFSET>,
            ReleaseErrors: ReleaseErrors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorLookup as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IErrorLookup {}
windows_core::imp::define_interface!(IErrorRecords, IErrorRecords_Vtbl, 0x0c733a67_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IErrorRecords, windows_core::IUnknown);
impl IErrorRecords {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddErrorRecord<P3>(&self, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: Option<*const super::DISPPARAMS>, punkcustomerror: P3, dwdynamicerrorid: u32) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddErrorRecord)(windows_core::Interface::as_raw(self), perrorinfo, dwlookupid, pdispparams.unwrap_or(core::mem::zeroed()) as _, punkcustomerror.param().abi(), dwdynamicerrorid) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetBasicErrorInfo(&self, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBasicErrorInfo)(windows_core::Interface::as_raw(self), ulrecordnum, perrorinfo as _) }
    }
    pub unsafe fn GetCustomErrorObject<T>(&self, ulrecordnum: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCustomErrorObject)(windows_core::Interface::as_raw(self), ulrecordnum, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "oaidl", feature = "winnt"))]
    pub unsafe fn GetErrorInfo(&self, ulrecordnum: u32, lcid: super::LCID) -> windows_core::Result<super::IErrorInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorInfo)(windows_core::Interface::as_raw(self), ulrecordnum, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetErrorParameters(&self, ulrecordnum: u32) -> windows_core::Result<super::DISPPARAMS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorParameters)(windows_core::Interface::as_raw(self), ulrecordnum, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRecordCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecordCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorRecords_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub AddErrorRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const ERRORINFO, u32, *const super::DISPPARAMS, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    AddErrorRecord: usize,
    #[cfg(feature = "oaidl")]
    pub GetBasicErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ERRORINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetBasicErrorInfo: usize,
    pub GetCustomErrorObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "winnt"))]
    pub GetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "winnt")))]
    GetErrorInfo: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetErrorParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::DISPPARAMS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetErrorParameters: usize,
    pub GetRecordCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IErrorRecords_Impl: windows_core::IUnknownImpl {
    fn AddErrorRecord(&self, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::DISPPARAMS, punkcustomerror: windows_core::Ref<windows_core::IUnknown>, dwdynamicerrorid: u32) -> windows_core::Result<()>;
    fn GetBasicErrorInfo(&self, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> windows_core::Result<()>;
    fn GetCustomErrorObject(&self, ulrecordnum: u32, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetErrorInfo(&self, ulrecordnum: u32, lcid: super::LCID) -> windows_core::Result<super::IErrorInfo>;
    fn GetErrorParameters(&self, ulrecordnum: u32) -> windows_core::Result<super::DISPPARAMS>;
    fn GetRecordCount(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IErrorRecords_Vtbl {
    pub const fn new<Identity: IErrorRecords_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddErrorRecord<Identity: IErrorRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::DISPPARAMS, punkcustomerror: *mut core::ffi::c_void, dwdynamicerrorid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IErrorRecords_Impl::AddErrorRecord(this, core::mem::transmute_copy(&perrorinfo), core::mem::transmute_copy(&dwlookupid), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&punkcustomerror), core::mem::transmute_copy(&dwdynamicerrorid)).into()
            }
        }
        unsafe extern "system" fn GetBasicErrorInfo<Identity: IErrorRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IErrorRecords_Impl::GetBasicErrorInfo(this, core::mem::transmute_copy(&ulrecordnum), core::mem::transmute_copy(&perrorinfo)).into()
            }
        }
        unsafe extern "system" fn GetCustomErrorObject<Identity: IErrorRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecordnum: u32, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IErrorRecords_Impl::GetCustomErrorObject(this, core::mem::transmute_copy(&ulrecordnum), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn GetErrorInfo<Identity: IErrorRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecordnum: u32, lcid: super::LCID, pperrorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorRecords_Impl::GetErrorInfo(this, core::mem::transmute_copy(&ulrecordnum), core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        pperrorinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorParameters<Identity: IErrorRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecordnum: u32, pdispparams: *mut super::DISPPARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorRecords_Impl::GetErrorParameters(this, core::mem::transmute_copy(&ulrecordnum)) {
                    Ok(ok__) => {
                        pdispparams.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRecordCount<Identity: IErrorRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrecords: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorRecords_Impl::GetRecordCount(this) {
                    Ok(ok__) => {
                        pcrecords.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddErrorRecord: AddErrorRecord::<Identity, OFFSET>,
            GetBasicErrorInfo: GetBasicErrorInfo::<Identity, OFFSET>,
            GetCustomErrorObject: GetCustomErrorObject::<Identity, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, OFFSET>,
            GetErrorParameters: GetErrorParameters::<Identity, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorRecords as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IErrorRecords {}
windows_core::imp::define_interface!(IGetDataSource, IGetDataSource_Vtbl, 0x0c733a75_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IGetDataSource, windows_core::IUnknown);
impl IGetDataSource {
    pub unsafe fn GetDataSource<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDataSource)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetDataSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDataSource: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IGetDataSource_Impl: windows_core::IUnknownImpl {
    fn GetDataSource(&self, riid: *const windows_core::GUID, ppdatasource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IGetDataSource_Vtbl {
    pub const fn new<Identity: IGetDataSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDataSource<Identity: IGetDataSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdatasource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetDataSource_Impl::GetDataSource(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdatasource)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDataSource: GetDataSource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetDataSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetDataSource {}
windows_core::imp::define_interface!(IGetRow, IGetRow_Vtbl, 0x0c733aaf_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IGetRow, windows_core::IUnknown);
impl IGetRow {
    pub unsafe fn GetRowFromHROW<P0, T>(&self, punkouter: P0, hrow: HROW) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetRowFromHROW)(windows_core::Interface::as_raw(self), punkouter.param().abi(), hrow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetURLFromHROW(&self, hrow: HROW) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURLFromHROW)(windows_core::Interface::as_raw(self), hrow, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetRow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRowFromHROW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, HROW, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetURLFromHROW: unsafe extern "system" fn(*mut core::ffi::c_void, HROW, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IGetRow_Impl: windows_core::IUnknownImpl {
    fn GetRowFromHROW(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, hrow: HROW, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetURLFromHROW(&self, hrow: HROW) -> windows_core::Result<windows_core::PWSTR>;
}
impl IGetRow_Vtbl {
    pub const fn new<Identity: IGetRow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRowFromHROW<Identity: IGetRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, hrow: HROW, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetRow_Impl::GetRowFromHROW(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn GetURLFromHROW<Identity: IGetRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: HROW, ppwszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGetRow_Impl::GetURLFromHROW(this, core::mem::transmute_copy(&hrow)) {
                    Ok(ok__) => {
                        ppwszurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowFromHROW: GetRowFromHROW::<Identity, OFFSET>,
            GetURLFromHROW: GetURLFromHROW::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetRow as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetRow {}
windows_core::imp::define_interface!(IGetSession, IGetSession_Vtbl, 0x0c733aba_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IGetSession, windows_core::IUnknown);
impl IGetSession {
    pub unsafe fn GetSession<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSession)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IGetSession_Impl: windows_core::IUnknownImpl {
    fn GetSession(&self, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IGetSession_Vtbl {
    pub const fn new<Identity: IGetSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSession<Identity: IGetSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetSession_Impl::GetSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppsession)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSession: GetSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetSession {}
windows_core::imp::define_interface!(IGetSourceRow, IGetSourceRow_Vtbl, 0x0c733abb_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IGetSourceRow, windows_core::IUnknown);
impl IGetSourceRow {
    pub unsafe fn GetSourceRow<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSourceRow)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetSourceRow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSourceRow: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IGetSourceRow_Impl: windows_core::IUnknownImpl {
    fn GetSourceRow(&self, riid: *const windows_core::GUID, pprow: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IGetSourceRow_Vtbl {
    pub const fn new<Identity: IGetSourceRow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSourceRow<Identity: IGetSourceRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetSourceRow_Impl::GetSourceRow(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pprow)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSourceRow: GetSourceRow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetSourceRow as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetSourceRow {}
windows_core::imp::define_interface!(IIndexDefinition, IIndexDefinition_Vtbl, 0x0c733a68_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IIndexDefinition, windows_core::IUnknown);
impl IIndexDefinition {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreateIndex(&self, ptableid: *const DBID, pindexid: Option<*const DBID>, rgindexcolumndescs: &[DBINDEXCOLUMNDESC], rgpropertysets: &mut [DBPROPSET], ppindexid: *mut *mut DBID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateIndex)(windows_core::Interface::as_raw(self), ptableid, pindexid.unwrap_or(core::mem::zeroed()) as _, DBORDINAL(rgindexcolumndescs.len().try_into().unwrap()), rgindexcolumndescs.as_ptr(), rgpropertysets.len().try_into().unwrap(), rgpropertysets.as_mut_ptr(), ppindexid as _) }
    }
    pub unsafe fn DropIndex(&self, ptableid: *const DBID, pindexid: Option<*const DBID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DropIndex)(windows_core::Interface::as_raw(self), ptableid, pindexid.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexDefinition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub CreateIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *const DBID, DBORDINAL, *const DBINDEXCOLUMNDESC, u32, *mut DBPROPSET, *mut *mut DBID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    CreateIndex: usize,
    pub DropIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *const DBID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IIndexDefinition_Impl: windows_core::IUnknownImpl {
    fn CreateIndex(&self, ptableid: *const DBID, pindexid: *const DBID, cindexcolumndescs: DBORDINAL, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut DBID) -> windows_core::Result<()>;
    fn DropIndex(&self, ptableid: *const DBID, pindexid: *const DBID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IIndexDefinition_Vtbl {
    pub const fn new<Identity: IIndexDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateIndex<Identity: IIndexDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pindexid: *const DBID, cindexcolumndescs: DBORDINAL, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIndexDefinition_Impl::CreateIndex(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid), core::mem::transmute_copy(&cindexcolumndescs), core::mem::transmute_copy(&rgindexcolumndescs), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&ppindexid)).into()
            }
        }
        unsafe extern "system" fn DropIndex<Identity: IIndexDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pindexid: *const DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIndexDefinition_Impl::DropIndex(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateIndex: CreateIndex::<Identity, OFFSET>,
            DropIndex: DropIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIndexDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IIndexDefinition {}
windows_core::imp::define_interface!(IMDDataset, IMDDataset_Vtbl, 0xa07cccd1_8148_11d0_87bb_00c04fc33942);
windows_core::imp::interface_hierarchy!(IMDDataset, windows_core::IUnknown);
impl IMDDataset {
    pub unsafe fn FreeAxisInfo(&self, caxes: DBCOUNTITEM, rgaxisinfo: *const MDAXISINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeAxisInfo)(windows_core::Interface::as_raw(self), caxes, rgaxisinfo) }
    }
    pub unsafe fn GetAxisInfo(&self, pcaxes: *mut DBCOUNTITEM, prgaxisinfo: *mut *mut MDAXISINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAxisInfo)(windows_core::Interface::as_raw(self), pcaxes as _, prgaxisinfo as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAxisRowset<P0, T>(&self, punkouter: P0, iaxis: DBCOUNTITEM, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAxisRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), iaxis, &T::IID, cpropertysets, rgpropertysets as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetCellData(&self, haccessor: HACCESSOR, ulstartcell: DBORDINAL, ulendcell: DBORDINAL, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCellData)(windows_core::Interface::as_raw(self), haccessor, ulstartcell, ulendcell, pdata as _) }
    }
    pub unsafe fn GetSpecification<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSpecification)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDDataset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FreeAxisInfo: unsafe extern "system" fn(*mut core::ffi::c_void, DBCOUNTITEM, *const MDAXISINFO) -> windows_core::HRESULT,
    pub GetAxisInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DBCOUNTITEM, *mut *mut MDAXISINFO) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAxisRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DBCOUNTITEM, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetAxisRowset: usize,
    pub GetCellData: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, DBORDINAL, DBORDINAL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpecification: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMDDataset_Impl: windows_core::IUnknownImpl {
    fn FreeAxisInfo(&self, caxes: DBCOUNTITEM, rgaxisinfo: *const MDAXISINFO) -> windows_core::Result<()>;
    fn GetAxisInfo(&self, pcaxes: *mut DBCOUNTITEM, prgaxisinfo: *mut *mut MDAXISINFO) -> windows_core::Result<()>;
    fn GetAxisRowset(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, iaxis: DBCOUNTITEM, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCellData(&self, haccessor: HACCESSOR, ulstartcell: DBORDINAL, ulendcell: DBORDINAL, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSpecification(&self, riid: *const windows_core::GUID, ppspecification: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IMDDataset_Vtbl {
    pub const fn new<Identity: IMDDataset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FreeAxisInfo<Identity: IMDDataset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, caxes: DBCOUNTITEM, rgaxisinfo: *const MDAXISINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDDataset_Impl::FreeAxisInfo(this, core::mem::transmute_copy(&caxes), core::mem::transmute_copy(&rgaxisinfo)).into()
            }
        }
        unsafe extern "system" fn GetAxisInfo<Identity: IMDDataset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaxes: *mut DBCOUNTITEM, prgaxisinfo: *mut *mut MDAXISINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDDataset_Impl::GetAxisInfo(this, core::mem::transmute_copy(&pcaxes), core::mem::transmute_copy(&prgaxisinfo)).into()
            }
        }
        unsafe extern "system" fn GetAxisRowset<Identity: IMDDataset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, iaxis: DBCOUNTITEM, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDDataset_Impl::GetAxisRowset(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&iaxis), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        unsafe extern "system" fn GetCellData<Identity: IMDDataset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, ulstartcell: DBORDINAL, ulendcell: DBORDINAL, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDDataset_Impl::GetCellData(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&ulstartcell), core::mem::transmute_copy(&ulendcell), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetSpecification<Identity: IMDDataset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppspecification: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDDataset_Impl::GetSpecification(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppspecification)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FreeAxisInfo: FreeAxisInfo::<Identity, OFFSET>,
            GetAxisInfo: GetAxisInfo::<Identity, OFFSET>,
            GetAxisRowset: GetAxisRowset::<Identity, OFFSET>,
            GetCellData: GetCellData::<Identity, OFFSET>,
            GetSpecification: GetSpecification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDDataset as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMDDataset {}
windows_core::imp::define_interface!(IMDFind, IMDFind_Vtbl, 0xa07cccd2_8148_11d0_87bb_00c04fc33942);
windows_core::imp::interface_hierarchy!(IMDFind, windows_core::IUnknown);
impl IMDFind {
    pub unsafe fn FindCell(&self, ulstartingordinal: DBORDINAL, cmembers: DBCOUNTITEM, rgpwszmember: *const windows_core::PCWSTR) -> windows_core::Result<DBORDINAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindCell)(windows_core::Interface::as_raw(self), ulstartingordinal, cmembers, rgpwszmember, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FindTuple(&self, ulaxisidentifier: u32, ulstartingordinal: DBORDINAL, cmembers: DBCOUNTITEM, rgpwszmember: *const windows_core::PCWSTR) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindTuple)(windows_core::Interface::as_raw(self), ulaxisidentifier, ulstartingordinal, cmembers, rgpwszmember, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDFind_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindCell: unsafe extern "system" fn(*mut core::ffi::c_void, DBORDINAL, DBCOUNTITEM, *const windows_core::PCWSTR, *mut DBORDINAL) -> windows_core::HRESULT,
    pub FindTuple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DBORDINAL, DBCOUNTITEM, *const windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait IMDFind_Impl: windows_core::IUnknownImpl {
    fn FindCell(&self, ulstartingordinal: DBORDINAL, cmembers: DBCOUNTITEM, rgpwszmember: *const windows_core::PCWSTR) -> windows_core::Result<DBORDINAL>;
    fn FindTuple(&self, ulaxisidentifier: u32, ulstartingordinal: DBORDINAL, cmembers: DBCOUNTITEM, rgpwszmember: *const windows_core::PCWSTR) -> windows_core::Result<u32>;
}
impl IMDFind_Vtbl {
    pub const fn new<Identity: IMDFind_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindCell<Identity: IMDFind_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstartingordinal: DBORDINAL, cmembers: DBCOUNTITEM, rgpwszmember: *const windows_core::PCWSTR, pulcellordinal: *mut DBORDINAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDFind_Impl::FindCell(this, core::mem::transmute_copy(&ulstartingordinal), core::mem::transmute_copy(&cmembers), core::mem::transmute_copy(&rgpwszmember)) {
                    Ok(ok__) => {
                        pulcellordinal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindTuple<Identity: IMDFind_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulaxisidentifier: u32, ulstartingordinal: DBORDINAL, cmembers: DBCOUNTITEM, rgpwszmember: *const windows_core::PCWSTR, pultupleordinal: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDFind_Impl::FindTuple(this, core::mem::transmute_copy(&ulaxisidentifier), core::mem::transmute_copy(&ulstartingordinal), core::mem::transmute_copy(&cmembers), core::mem::transmute_copy(&rgpwszmember)) {
                    Ok(ok__) => {
                        pultupleordinal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindCell: FindCell::<Identity, OFFSET>, FindTuple: FindTuple::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDFind as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMDFind {}
windows_core::imp::define_interface!(IMDRangeRowset, IMDRangeRowset_Vtbl, 0x0c733aa0_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IMDRangeRowset, windows_core::IUnknown);
impl IMDRangeRowset {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetRangeRowset<P0, T>(&self, punkouter: P0, ulstartcell: DBORDINAL, ulendcell: DBORDINAL, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetRangeRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), ulstartcell, ulendcell, &T::IID, cpropertysets, rgpropertysets as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDRangeRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetRangeRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DBORDINAL, DBORDINAL, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetRangeRowset: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMDRangeRowset_Impl: windows_core::IUnknownImpl {
    fn GetRangeRowset(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, ulstartcell: DBORDINAL, ulendcell: DBORDINAL, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IMDRangeRowset_Vtbl {
    pub const fn new<Identity: IMDRangeRowset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRangeRowset<Identity: IMDRangeRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ulstartcell: DBORDINAL, ulendcell: DBORDINAL, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDRangeRowset_Impl::GetRangeRowset(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&ulstartcell), core::mem::transmute_copy(&ulendcell), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRangeRowset: GetRangeRowset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDRangeRowset as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMDRangeRowset {}
windows_core::imp::define_interface!(IMultipleResults, IMultipleResults_Vtbl, 0x0c733a90_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IMultipleResults, windows_core::IUnknown);
impl IMultipleResults {
    pub unsafe fn GetResult<P0, T>(&self, punkouter: P0, lresultflag: DBRESULTFLAG, pcrowsaffected: Option<*mut DBROWCOUNT>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self), punkouter.param().abi(), lresultflag, &T::IID, pcrowsaffected.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleResults_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DBRESULTFLAG, *const windows_core::GUID, *mut DBROWCOUNT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMultipleResults_Impl: windows_core::IUnknownImpl {
    fn GetResult(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, lresultflag: DBRESULTFLAG, riid: *const windows_core::GUID, pcrowsaffected: *mut DBROWCOUNT, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMultipleResults_Vtbl {
    pub const fn new<Identity: IMultipleResults_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResult<Identity: IMultipleResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, lresultflag: DBRESULTFLAG, riid: *const windows_core::GUID, pcrowsaffected: *mut DBROWCOUNT, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultipleResults_Impl::GetResult(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&lresultflag), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pcrowsaffected), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetResult: GetResult::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultipleResults as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMultipleResults {}
windows_core::imp::define_interface!(IObjectAccessControl, IObjectAccessControl_Vtbl, 0x0c733aa3_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IObjectAccessControl, windows_core::IUnknown);
impl IObjectAccessControl {
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn GetObjectAccessRights(&self, pobject: *const SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::EXPLICIT_ACCESS_W) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectAccessRights)(windows_core::Interface::as_raw(self), pobject, pcaccessentries as _, prgaccessentries as _) }
    }
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn GetObjectOwner(&self, pobject: *const SEC_OBJECT) -> windows_core::Result<*mut super::TRUSTEE_W> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectOwner)(windows_core::Interface::as_raw(self), pobject, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn IsObjectAccessAllowed(&self, pobject: *const SEC_OBJECT, paccessentry: *const super::EXPLICIT_ACCESS_W) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsObjectAccessAllowed)(windows_core::Interface::as_raw(self), pobject, paccessentry, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn SetObjectAccessRights(&self, pobject: *const SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::EXPLICIT_ACCESS_W) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjectAccessRights)(windows_core::Interface::as_raw(self), pobject, caccessentries, prgaccessentries as _) }
    }
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn SetObjectOwner(&self, pobject: *const SEC_OBJECT, powner: *const super::TRUSTEE_W) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjectOwner)(windows_core::Interface::as_raw(self), pobject, powner) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectAccessControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub GetObjectAccessRights: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, *mut u32, *mut *mut super::EXPLICIT_ACCESS_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    GetObjectAccessRights: usize,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub GetObjectOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    GetObjectOwner: usize,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub IsObjectAccessAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, *const super::EXPLICIT_ACCESS_W, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    IsObjectAccessAllowed: usize,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub SetObjectAccessRights: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, u32, *mut super::EXPLICIT_ACCESS_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    SetObjectAccessRights: usize,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub SetObjectOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, *const super::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    SetObjectOwner: usize,
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
pub trait IObjectAccessControl_Impl: windows_core::IUnknownImpl {
    fn GetObjectAccessRights(&self, pobject: *const SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::EXPLICIT_ACCESS_W) -> windows_core::Result<()>;
    fn GetObjectOwner(&self, pobject: *const SEC_OBJECT) -> windows_core::Result<*mut super::TRUSTEE_W>;
    fn IsObjectAccessAllowed(&self, pobject: *const SEC_OBJECT, paccessentry: *const super::EXPLICIT_ACCESS_W) -> windows_core::Result<windows_core::BOOL>;
    fn SetObjectAccessRights(&self, pobject: *const SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::EXPLICIT_ACCESS_W) -> windows_core::Result<()>;
    fn SetObjectOwner(&self, pobject: *const SEC_OBJECT, powner: *const super::TRUSTEE_W) -> windows_core::Result<()>;
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
impl IObjectAccessControl_Vtbl {
    pub const fn new<Identity: IObjectAccessControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectAccessRights<Identity: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::EXPLICIT_ACCESS_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectAccessControl_Impl::GetObjectAccessRights(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&pcaccessentries), core::mem::transmute_copy(&prgaccessentries)).into()
            }
        }
        unsafe extern "system" fn GetObjectOwner<Identity: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, ppowner: *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IObjectAccessControl_Impl::GetObjectOwner(this, core::mem::transmute_copy(&pobject)) {
                    Ok(ok__) => {
                        ppowner.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsObjectAccessAllowed<Identity: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, paccessentry: *const super::EXPLICIT_ACCESS_W, pfresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IObjectAccessControl_Impl::IsObjectAccessAllowed(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&paccessentry)) {
                    Ok(ok__) => {
                        pfresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetObjectAccessRights<Identity: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::EXPLICIT_ACCESS_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectAccessControl_Impl::SetObjectAccessRights(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&caccessentries), core::mem::transmute_copy(&prgaccessentries)).into()
            }
        }
        unsafe extern "system" fn SetObjectOwner<Identity: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, powner: *const super::TRUSTEE_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectAccessControl_Impl::SetObjectOwner(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&powner)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectAccessRights: GetObjectAccessRights::<Identity, OFFSET>,
            GetObjectOwner: GetObjectOwner::<Identity, OFFSET>,
            IsObjectAccessAllowed: IsObjectAccessAllowed::<Identity, OFFSET>,
            SetObjectAccessRights: SetObjectAccessRights::<Identity, OFFSET>,
            SetObjectOwner: SetObjectOwner::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectAccessControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
impl windows_core::RuntimeName for IObjectAccessControl {}
windows_core::imp::define_interface!(IOpenRowset, IOpenRowset_Vtbl, 0x0c733a69_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IOpenRowset, windows_core::IUnknown);
impl IOpenRowset {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn OpenRowset<P0, T>(&self, punkouter: P0, ptableid: Option<*const DBID>, pindexid: Option<*const DBID>, rgpropertysets: Option<&mut [DBPROPSET]>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OpenRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), ptableid.unwrap_or(core::mem::zeroed()) as _, pindexid.unwrap_or(core::mem::zeroed()) as _, &T::IID, rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertysets.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub OpenRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DBID, *const DBID, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    OpenRowset: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IOpenRowset_Impl: windows_core::IUnknownImpl {
    fn OpenRowset(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, ptableid: *const DBID, pindexid: *const DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IOpenRowset_Vtbl {
    pub const fn new<Identity: IOpenRowset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenRowset<Identity: IOpenRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ptableid: *const DBID, pindexid: *const DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpenRowset_Impl::OpenRowset(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenRowset: OpenRowset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenRowset as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IOpenRowset {}
windows_core::imp::define_interface!(IParentRowset, IParentRowset_Vtbl, 0x0c733aaa_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IParentRowset, windows_core::IUnknown);
impl IParentRowset {
    pub unsafe fn GetChildRowset<P0, T>(&self, punkouter: P0, iordinal: DBORDINAL) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetChildRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), iordinal, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IParentRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChildRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DBORDINAL, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IParentRowset_Impl: windows_core::IUnknownImpl {
    fn GetChildRowset(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, iordinal: DBORDINAL, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IParentRowset_Vtbl {
    pub const fn new<Identity: IParentRowset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChildRowset<Identity: IParentRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, iordinal: DBORDINAL, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IParentRowset_Impl::GetChildRowset(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&iordinal), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetChildRowset: GetChildRowset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IParentRowset as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IParentRowset {}
windows_core::imp::define_interface!(IRegisterProvider, IRegisterProvider_Vtbl, 0x0c733ab9_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRegisterProvider, windows_core::IUnknown);
impl IRegisterProvider {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetURLMapping<P0>(&self, pwszurl: P0, dwreserved: DB_DWRESERVE) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURLMapping)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwreserved, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetURLMapping<P0>(&self, pwszurl: P0, dwreserved: DB_DWRESERVE, rclsidprovider: *const windows_core::GUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetURLMapping)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwreserved, rclsidprovider) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn UnregisterProvider<P0>(&self, pwszurl: P0, dwreserved: DB_DWRESERVE, rclsidprovider: *const windows_core::GUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterProvider)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwreserved, rclsidprovider) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisterProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetURLMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DB_DWRESERVE, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetURLMapping: usize,
    #[cfg(feature = "winnt")]
    pub SetURLMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DB_DWRESERVE, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetURLMapping: usize,
    #[cfg(feature = "winnt")]
    pub UnregisterProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DB_DWRESERVE, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    UnregisterProvider: usize,
}
#[cfg(feature = "winnt")]
pub trait IRegisterProvider_Impl: windows_core::IUnknownImpl {
    fn GetURLMapping(&self, pwszurl: &windows_core::PCWSTR, dwreserved: DB_DWRESERVE) -> windows_core::Result<windows_core::GUID>;
    fn SetURLMapping(&self, pwszurl: &windows_core::PCWSTR, dwreserved: DB_DWRESERVE, rclsidprovider: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnregisterProvider(&self, pwszurl: &windows_core::PCWSTR, dwreserved: DB_DWRESERVE, rclsidprovider: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IRegisterProvider_Vtbl {
    pub const fn new<Identity: IRegisterProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetURLMapping<Identity: IRegisterProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwreserved: DB_DWRESERVE, pclsidprovider: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisterProvider_Impl::GetURLMapping(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        pclsidprovider.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetURLMapping<Identity: IRegisterProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwreserved: DB_DWRESERVE, rclsidprovider: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisterProvider_Impl::SetURLMapping(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&rclsidprovider)).into()
            }
        }
        unsafe extern "system" fn UnregisterProvider<Identity: IRegisterProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwreserved: DB_DWRESERVE, rclsidprovider: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisterProvider_Impl::UnregisterProvider(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&rclsidprovider)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetURLMapping: GetURLMapping::<Identity, OFFSET>,
            SetURLMapping: SetURLMapping::<Identity, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisterProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IRegisterProvider {}
windows_core::imp::define_interface!(IRow, IRow_Vtbl, 0x0c733ab4_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRow, windows_core::IUnknown);
impl IRow {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetColumns(&self, rgcolumns: &mut [DBCOLUMNACCESS]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetColumns)(windows_core::Interface::as_raw(self), DBORDINAL(rgcolumns.len().try_into().unwrap()), rgcolumns.as_mut_ptr()) }
    }
    pub unsafe fn GetSourceRowset<T>(&self, phrow: Option<*mut HROW>) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSourceRowset)(windows_core::Interface::as_raw(self), &T::IID, &mut result__, phrow.unwrap_or(core::mem::zeroed()) as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Open<P0, T>(&self, punkouter: P0, pcolumnid: *const DBID, rguidcolumntype: *const windows_core::GUID, dwbindflags: u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), punkouter.param().abi(), pcolumnid, rguidcolumntype, dwbindflags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetColumns: unsafe extern "system" fn(*mut core::ffi::c_void, DBORDINAL, *mut DBCOLUMNACCESS) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetColumns: usize,
    pub GetSourceRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut HROW) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DBID, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IRow_Impl: windows_core::IUnknownImpl {
    fn GetColumns(&self, ccolumns: DBORDINAL, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::Result<()>;
    fn GetSourceRowset(&self, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void, phrow: *mut HROW) -> windows_core::Result<()>;
    fn Open(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, pcolumnid: *const DBID, rguidcolumntype: *const windows_core::GUID, dwbindflags: u32, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IRow_Vtbl {
    pub const fn new<Identity: IRow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColumns<Identity: IRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumns: DBORDINAL, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRow_Impl::GetColumns(this, core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgcolumns)).into()
            }
        }
        unsafe extern "system" fn GetSourceRowset<Identity: IRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void, phrow: *mut HROW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRow_Impl::GetSourceRowset(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pprowset), core::mem::transmute_copy(&phrow)).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pcolumnid: *const DBID, rguidcolumntype: *const windows_core::GUID, dwbindflags: u32, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRow_Impl::Open(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&pcolumnid), core::mem::transmute_copy(&rguidcolumntype), core::mem::transmute_copy(&dwbindflags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumns: GetColumns::<Identity, OFFSET>,
            GetSourceRowset: GetSourceRowset::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IRow {}
windows_core::imp::define_interface!(IRowChange, IRowChange_Vtbl, 0x0c733ab5_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowChange, windows_core::IUnknown);
impl IRowChange {
    #[cfg(feature = "winnt")]
    pub unsafe fn SetColumns(&self, rgcolumns: &[DBCOLUMNACCESS]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColumns)(windows_core::Interface::as_raw(self), DBORDINAL(rgcolumns.len().try_into().unwrap()), rgcolumns.as_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub SetColumns: unsafe extern "system" fn(*mut core::ffi::c_void, DBORDINAL, *const DBCOLUMNACCESS) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetColumns: usize,
}
#[cfg(feature = "winnt")]
pub trait IRowChange_Impl: windows_core::IUnknownImpl {
    fn SetColumns(&self, ccolumns: DBORDINAL, rgcolumns: *const DBCOLUMNACCESS) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IRowChange_Vtbl {
    pub const fn new<Identity: IRowChange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetColumns<Identity: IRowChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumns: DBORDINAL, rgcolumns: *const DBCOLUMNACCESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowChange_Impl::SetColumns(this, core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgcolumns)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetColumns: SetColumns::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowChange as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IRowChange {}
windows_core::imp::define_interface!(IRowPosition, IRowPosition_Vtbl, 0x0c733a94_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowPosition, windows_core::IUnknown);
impl IRowPosition {
    pub unsafe fn ClearRowPosition(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearRowPosition)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRowPosition(&self, phchapter: Option<*mut HCHAPTER>, phrow: *mut HROW, pdwpositionflags: Option<*mut DBPOSITIONFLAGS>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRowPosition)(windows_core::Interface::as_raw(self), phchapter.unwrap_or(core::mem::zeroed()) as _, phrow as _, pdwpositionflags.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetRowset<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetRowset)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Initialize<P0>(&self, prowset: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), prowset.param().abi()) }
    }
    pub unsafe fn SetRowPosition(&self, hchapter: HCHAPTER, hrow: HROW, dwpositionflags: DBPOSITIONFLAGS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRowPosition)(windows_core::Interface::as_raw(self), hchapter, hrow, dwpositionflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowPosition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ClearRowPosition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRowPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HCHAPTER, *mut HROW, *mut DBPOSITIONFLAGS) -> windows_core::HRESULT,
    pub GetRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRowPosition: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, HROW, DBPOSITIONFLAGS) -> windows_core::HRESULT,
}
pub trait IRowPosition_Impl: windows_core::IUnknownImpl {
    fn ClearRowPosition(&self) -> windows_core::Result<()>;
    fn GetRowPosition(&self, phchapter: *mut HCHAPTER, phrow: *mut HROW, pdwpositionflags: *mut DBPOSITIONFLAGS) -> windows_core::Result<()>;
    fn GetRowset(&self, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Initialize(&self, prowset: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetRowPosition(&self, hchapter: HCHAPTER, hrow: HROW, dwpositionflags: DBPOSITIONFLAGS) -> windows_core::Result<()>;
}
impl IRowPosition_Vtbl {
    pub const fn new<Identity: IRowPosition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClearRowPosition<Identity: IRowPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowPosition_Impl::ClearRowPosition(this).into()
            }
        }
        unsafe extern "system" fn GetRowPosition<Identity: IRowPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phchapter: *mut HCHAPTER, phrow: *mut HROW, pdwpositionflags: *mut DBPOSITIONFLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowPosition_Impl::GetRowPosition(this, core::mem::transmute_copy(&phchapter), core::mem::transmute_copy(&phrow), core::mem::transmute_copy(&pdwpositionflags)).into()
            }
        }
        unsafe extern "system" fn GetRowset<Identity: IRowPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowPosition_Impl::GetRowset(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IRowPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowPosition_Impl::Initialize(this, core::mem::transmute_copy(&prowset)).into()
            }
        }
        unsafe extern "system" fn SetRowPosition<Identity: IRowPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, hrow: HROW, dwpositionflags: DBPOSITIONFLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowPosition_Impl::SetRowPosition(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&dwpositionflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ClearRowPosition: ClearRowPosition::<Identity, OFFSET>,
            GetRowPosition: GetRowPosition::<Identity, OFFSET>,
            GetRowset: GetRowset::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            SetRowPosition: SetRowPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowPosition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowPosition {}
windows_core::imp::define_interface!(IRowPositionChange, IRowPositionChange_Vtbl, 0x0997a571_126e_11d0_9f8a_00a0c9a0631e);
windows_core::imp::interface_hierarchy!(IRowPositionChange, windows_core::IUnknown);
impl IRowPositionChange {
    pub unsafe fn OnRowPositionChange(&self, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnRowPositionChange)(windows_core::Interface::as_raw(self), ereason, ephase, fcantdeny.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowPositionChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnRowPositionChange: unsafe extern "system" fn(*mut core::ffi::c_void, DBREASON, DBEVENTPHASE, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IRowPositionChange_Impl: windows_core::IUnknownImpl {
    fn OnRowPositionChange(&self, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IRowPositionChange_Vtbl {
    pub const fn new<Identity: IRowPositionChange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnRowPositionChange<Identity: IRowPositionChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowPositionChange_Impl::OnRowPositionChange(this, core::mem::transmute_copy(&ereason), core::mem::transmute_copy(&ephase), core::mem::transmute_copy(&fcantdeny)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnRowPositionChange: OnRowPositionChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowPositionChange as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowPositionChange {}
windows_core::imp::define_interface!(IRowSchemaChange, IRowSchemaChange_Vtbl, 0x0c733aae_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowSchemaChange {
    type Target = IRowChange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowSchemaChange, windows_core::IUnknown, IRowChange);
impl IRowSchemaChange {
    pub unsafe fn DeleteColumns(&self, ccolumns: DBORDINAL, rgcolumnids: *const DBID, rgdwstatus: *mut DBSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteColumns)(windows_core::Interface::as_raw(self), ccolumns, rgcolumnids, rgdwstatus as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "winnt"))]
    pub unsafe fn AddColumns(&self, ccolumns: DBORDINAL, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddColumns)(windows_core::Interface::as_raw(self), ccolumns, rgnewcolumninfo, rgcolumns as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowSchemaChange_Vtbl {
    pub base__: IRowChange_Vtbl,
    pub DeleteColumns: unsafe extern "system" fn(*mut core::ffi::c_void, DBORDINAL, *const DBID, *mut DBSTATUS) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "winnt"))]
    pub AddColumns: unsafe extern "system" fn(*mut core::ffi::c_void, DBORDINAL, *const DBCOLUMNINFO, *mut DBCOLUMNACCESS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "winnt")))]
    AddColumns: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
pub trait IRowSchemaChange_Impl: IRowChange_Impl {
    fn DeleteColumns(&self, ccolumns: DBORDINAL, rgcolumnids: *const DBID, rgdwstatus: *mut DBSTATUS) -> windows_core::Result<()>;
    fn AddColumns(&self, ccolumns: DBORDINAL, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
impl IRowSchemaChange_Vtbl {
    pub const fn new<Identity: IRowSchemaChange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteColumns<Identity: IRowSchemaChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumns: DBORDINAL, rgcolumnids: *const DBID, rgdwstatus: *mut DBSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowSchemaChange_Impl::DeleteColumns(this, core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgcolumnids), core::mem::transmute_copy(&rgdwstatus)).into()
            }
        }
        unsafe extern "system" fn AddColumns<Identity: IRowSchemaChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumns: DBORDINAL, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowSchemaChange_Impl::AddColumns(this, core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgnewcolumninfo), core::mem::transmute_copy(&rgcolumns)).into()
            }
        }
        Self { base__: IRowChange_Vtbl::new::<Identity, OFFSET>(), DeleteColumns: DeleteColumns::<Identity, OFFSET>, AddColumns: AddColumns::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowSchemaChange as windows_core::Interface>::IID || iid == &<IRowChange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
impl windows_core::RuntimeName for IRowSchemaChange {}
windows_core::imp::define_interface!(IRowset, IRowset_Vtbl, 0x0c733a7c_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowset, windows_core::IUnknown);
impl IRowset {
    pub unsafe fn AddRefRows(&self, crows: DBCOUNTITEM, rghrows: *const HROW, rgrefcounts: *mut DBREFCOUNT, rgrowstatus: *mut DBROWSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRefRows)(windows_core::Interface::as_raw(self), crows, rghrows, rgrefcounts as _, rgrowstatus as _) }
    }
    pub unsafe fn GetData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), hrow, haccessor, pdata as _) }
    }
    pub unsafe fn GetNextRows(&self, hreserved: HCHAPTER, lrowsoffset: DBROWOFFSET, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNextRows)(windows_core::Interface::as_raw(self), hreserved, lrowsoffset, crows, pcrowsobtained as _, prghrows as _) }
    }
    pub unsafe fn ReleaseRows(&self, crows: DBCOUNTITEM, rghrows: *const HROW, rgrowoptions: *const DBROWOPTIONS, rgrefcounts: *mut DBREFCOUNT, rgrowstatus: *mut DBROWSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseRows)(windows_core::Interface::as_raw(self), crows, rghrows, rgrowoptions, rgrefcounts as _, rgrowstatus as _) }
    }
    pub unsafe fn RestartPosition(&self, hreserved: HCHAPTER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestartPosition)(windows_core::Interface::as_raw(self), hreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefRows: unsafe extern "system" fn(*mut core::ffi::c_void, DBCOUNTITEM, *const HROW, *mut DBREFCOUNT, *mut DBROWSTATUS) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, HROW, HACCESSOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextRows: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBROWOFFSET, DBROWCOUNT, *mut DBCOUNTITEM, *mut *mut HROW) -> windows_core::HRESULT,
    pub ReleaseRows: unsafe extern "system" fn(*mut core::ffi::c_void, DBCOUNTITEM, *const HROW, *const DBROWOPTIONS, *mut DBREFCOUNT, *mut DBROWSTATUS) -> windows_core::HRESULT,
    pub RestartPosition: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER) -> windows_core::HRESULT,
}
pub trait IRowset_Impl: windows_core::IUnknownImpl {
    fn AddRefRows(&self, crows: DBCOUNTITEM, rghrows: *const HROW, rgrefcounts: *mut DBREFCOUNT, rgrowstatus: *mut DBROWSTATUS) -> windows_core::Result<()>;
    fn GetData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetNextRows(&self, hreserved: HCHAPTER, lrowsoffset: DBROWOFFSET, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::Result<()>;
    fn ReleaseRows(&self, crows: DBCOUNTITEM, rghrows: *const HROW, rgrowoptions: *const DBROWOPTIONS, rgrefcounts: *mut DBREFCOUNT, rgrowstatus: *mut DBROWSTATUS) -> windows_core::Result<()>;
    fn RestartPosition(&self, hreserved: HCHAPTER) -> windows_core::Result<()>;
}
impl IRowset_Vtbl {
    pub const fn new<Identity: IRowset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRefRows<Identity: IRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: DBCOUNTITEM, rghrows: *const HROW, rgrefcounts: *mut DBREFCOUNT, rgrowstatus: *mut DBROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowset_Impl::AddRefRows(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&rgrefcounts), core::mem::transmute_copy(&rgrowstatus)).into()
            }
        }
        unsafe extern "system" fn GetData<Identity: IRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowset_Impl::GetData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetNextRows<Identity: IRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, lrowsoffset: DBROWOFFSET, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowset_Impl::GetNextRows(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&lrowsoffset), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&pcrowsobtained), core::mem::transmute_copy(&prghrows)).into()
            }
        }
        unsafe extern "system" fn ReleaseRows<Identity: IRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: DBCOUNTITEM, rghrows: *const HROW, rgrowoptions: *const DBROWOPTIONS, rgrefcounts: *mut DBREFCOUNT, rgrowstatus: *mut DBROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowset_Impl::ReleaseRows(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&rgrowoptions), core::mem::transmute_copy(&rgrefcounts), core::mem::transmute_copy(&rgrowstatus)).into()
            }
        }
        unsafe extern "system" fn RestartPosition<Identity: IRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowset_Impl::RestartPosition(this, core::mem::transmute_copy(&hreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefRows: AddRefRows::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            GetNextRows: GetNextRows::<Identity, OFFSET>,
            ReleaseRows: ReleaseRows::<Identity, OFFSET>,
            RestartPosition: RestartPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowset as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowset {}
windows_core::imp::define_interface!(IRowsetBookmark, IRowsetBookmark_Vtbl, 0x0c733ac2_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetBookmark, windows_core::IUnknown);
impl IRowsetBookmark {
    pub unsafe fn PositionOnBookmark(&self, hchapter: HCHAPTER, pbookmark: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PositionOnBookmark)(windows_core::Interface::as_raw(self), hchapter, DBBKMARK(pbookmark.len().try_into().unwrap()), pbookmark.as_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetBookmark_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PositionOnBookmark: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBBKMARK, *const u8) -> windows_core::HRESULT,
}
pub trait IRowsetBookmark_Impl: windows_core::IUnknownImpl {
    fn PositionOnBookmark(&self, hchapter: HCHAPTER, cbbookmark: DBBKMARK, pbookmark: *const u8) -> windows_core::Result<()>;
}
impl IRowsetBookmark_Vtbl {
    pub const fn new<Identity: IRowsetBookmark_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PositionOnBookmark<Identity: IRowsetBookmark_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, cbbookmark: DBBKMARK, pbookmark: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetBookmark_Impl::PositionOnBookmark(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PositionOnBookmark: PositionOnBookmark::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetBookmark as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetBookmark {}
windows_core::imp::define_interface!(IRowsetChange, IRowsetChange_Vtbl, 0x0c733a05_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetChange, windows_core::IUnknown);
impl IRowsetChange {
    pub unsafe fn DeleteRows(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW) -> windows_core::Result<DBROWSTATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeleteRows)(windows_core::Interface::as_raw(self), hreserved, crows, rghrows, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), hrow, haccessor, pdata) }
    }
    pub unsafe fn InsertRow(&self, hreserved: HCHAPTER, haccessor: HACCESSOR, pdata: *const core::ffi::c_void) -> windows_core::Result<HROW> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertRow)(windows_core::Interface::as_raw(self), hreserved, haccessor, pdata, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeleteRows: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBCOUNTITEM, *const HROW, *mut DBROWSTATUS) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, HROW, HACCESSOR, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertRow: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, HACCESSOR, *const core::ffi::c_void, *mut HROW) -> windows_core::HRESULT,
}
pub trait IRowsetChange_Impl: windows_core::IUnknownImpl {
    fn DeleteRows(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW) -> windows_core::Result<DBROWSTATUS>;
    fn SetData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn InsertRow(&self, hreserved: HCHAPTER, haccessor: HACCESSOR, pdata: *const core::ffi::c_void) -> windows_core::Result<HROW>;
}
impl IRowsetChange_Vtbl {
    pub const fn new<Identity: IRowsetChange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteRows<Identity: IRowsetChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, rgrowstatus: *mut DBROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRowsetChange_Impl::DeleteRows(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows)) {
                    Ok(ok__) => {
                        rgrowstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetData<Identity: IRowsetChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: HROW, haccessor: HACCESSOR, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetChange_Impl::SetData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn InsertRow<Identity: IRowsetChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, haccessor: HACCESSOR, pdata: *const core::ffi::c_void, phrow: *mut HROW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRowsetChange_Impl::InsertRow(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)) {
                    Ok(ok__) => {
                        phrow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeleteRows: DeleteRows::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            InsertRow: InsertRow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetChange as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetChange {}
windows_core::imp::define_interface!(IRowsetChapterMember, IRowsetChapterMember_Vtbl, 0x0c733aa8_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetChapterMember, windows_core::IUnknown);
impl IRowsetChapterMember {
    pub unsafe fn IsRowInChapter(&self, hchapter: HCHAPTER, hrow: HROW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsRowInChapter)(windows_core::Interface::as_raw(self), hchapter, hrow) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetChapterMember_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsRowInChapter: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, HROW) -> windows_core::HRESULT,
}
pub trait IRowsetChapterMember_Impl: windows_core::IUnknownImpl {
    fn IsRowInChapter(&self, hchapter: HCHAPTER, hrow: HROW) -> windows_core::Result<()>;
}
impl IRowsetChapterMember_Vtbl {
    pub const fn new<Identity: IRowsetChapterMember_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsRowInChapter<Identity: IRowsetChapterMember_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, hrow: HROW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetChapterMember_Impl::IsRowInChapter(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&hrow)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsRowInChapter: IsRowInChapter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetChapterMember as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetChapterMember {}
windows_core::imp::define_interface!(IRowsetCurrentIndex, IRowsetCurrentIndex_Vtbl, 0x0c733abd_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetCurrentIndex {
    type Target = IRowsetIndex;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetCurrentIndex, windows_core::IUnknown, IRowsetIndex);
impl IRowsetCurrentIndex {
    pub unsafe fn GetIndex(&self) -> windows_core::Result<*mut DBID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndex(&self, pindexid: *const DBID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIndex)(windows_core::Interface::as_raw(self), pindexid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetCurrentIndex_Vtbl {
    pub base__: IRowsetIndex_Vtbl,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut DBID) -> windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRowsetCurrentIndex_Impl: IRowsetIndex_Impl {
    fn GetIndex(&self) -> windows_core::Result<*mut DBID>;
    fn SetIndex(&self, pindexid: *const DBID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IRowsetCurrentIndex_Vtbl {
    pub const fn new<Identity: IRowsetCurrentIndex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIndex<Identity: IRowsetCurrentIndex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppindexid: *mut *mut DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRowsetCurrentIndex_Impl::GetIndex(this) {
                    Ok(ok__) => {
                        ppindexid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndex<Identity: IRowsetCurrentIndex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexid: *const DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetCurrentIndex_Impl::SetIndex(this, core::mem::transmute_copy(&pindexid)).into()
            }
        }
        Self { base__: IRowsetIndex_Vtbl::new::<Identity, OFFSET>(), GetIndex: GetIndex::<Identity, OFFSET>, SetIndex: SetIndex::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetCurrentIndex as windows_core::Interface>::IID || iid == &<IRowsetIndex as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRowsetCurrentIndex {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IRowsetExactScroll(pub u8);
windows_core::imp::define_interface!(IRowsetFind, IRowsetFind_Vtbl, 0x0c733a9d_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetFind, windows_core::IUnknown);
impl IRowsetFind {
    pub unsafe fn FindNextRow(&self, hchapter: HCHAPTER, haccessor: HACCESSOR, pfindvalue: *const core::ffi::c_void, compareop: DBCOMPAREOP, cbbookmark: DBBKMARK, pbookmark: *const u8, lrowsoffset: DBROWOFFSET, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindNextRow)(windows_core::Interface::as_raw(self), hchapter, haccessor, pfindvalue, compareop, cbbookmark, pbookmark, lrowsoffset, crows, pcrowsobtained as _, prghrows as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetFind_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindNextRow: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, HACCESSOR, *const core::ffi::c_void, DBCOMPAREOP, DBBKMARK, *const u8, DBROWOFFSET, DBROWCOUNT, *mut DBCOUNTITEM, *mut *mut HROW) -> windows_core::HRESULT,
}
pub trait IRowsetFind_Impl: windows_core::IUnknownImpl {
    fn FindNextRow(&self, hchapter: HCHAPTER, haccessor: HACCESSOR, pfindvalue: *const core::ffi::c_void, compareop: DBCOMPAREOP, cbbookmark: DBBKMARK, pbookmark: *const u8, lrowsoffset: DBROWOFFSET, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::Result<()>;
}
impl IRowsetFind_Vtbl {
    pub const fn new<Identity: IRowsetFind_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindNextRow<Identity: IRowsetFind_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, haccessor: HACCESSOR, pfindvalue: *const core::ffi::c_void, compareop: DBCOMPAREOP, cbbookmark: DBBKMARK, pbookmark: *const u8, lrowsoffset: DBROWOFFSET, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetFind_Impl::FindNextRow(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pfindvalue), core::mem::transmute_copy(&compareop), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark), core::mem::transmute_copy(&lrowsoffset), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&pcrowsobtained), core::mem::transmute_copy(&prghrows)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindNextRow: FindNextRow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetFind as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetFind {}
windows_core::imp::define_interface!(IRowsetIdentity, IRowsetIdentity_Vtbl, 0x0c733a09_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetIdentity, windows_core::IUnknown);
impl IRowsetIdentity {
    pub unsafe fn IsSameRow(&self, hthisrow: HROW, hthatrow: HROW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsSameRow)(windows_core::Interface::as_raw(self), hthisrow, hthatrow) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsSameRow: unsafe extern "system" fn(*mut core::ffi::c_void, HROW, HROW) -> windows_core::HRESULT,
}
pub trait IRowsetIdentity_Impl: windows_core::IUnknownImpl {
    fn IsSameRow(&self, hthisrow: HROW, hthatrow: HROW) -> windows_core::Result<()>;
}
impl IRowsetIdentity_Vtbl {
    pub const fn new<Identity: IRowsetIdentity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsSameRow<Identity: IRowsetIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hthisrow: HROW, hthatrow: HROW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetIdentity_Impl::IsSameRow(this, core::mem::transmute_copy(&hthisrow), core::mem::transmute_copy(&hthatrow)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsSameRow: IsSameRow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetIdentity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetIdentity {}
windows_core::imp::define_interface!(IRowsetIndex, IRowsetIndex_Vtbl, 0x0c733a82_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetIndex, windows_core::IUnknown);
impl IRowsetIndex {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetIndexInfo(&self, pckeycolumns: *mut DBORDINAL, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIndexInfo)(windows_core::Interface::as_raw(self), pckeycolumns as _, prgindexcolumndesc as _, pcindexpropertysets as _, prgindexpropertysets as _) }
    }
    pub unsafe fn Seek(&self, haccessor: HACCESSOR, ckeyvalues: DBORDINAL, pdata: *const core::ffi::c_void, dwseekoptions: DBSEEK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), haccessor, ckeyvalues, pdata, dwseekoptions) }
    }
    pub unsafe fn SetRange(&self, haccessor: HACCESSOR, cstartkeycolumns: DBORDINAL, pstartdata: *const core::ffi::c_void, cendkeycolumns: DBORDINAL, penddata: *const core::ffi::c_void, dwrangeoptions: DBRANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRange)(windows_core::Interface::as_raw(self), haccessor, cstartkeycolumns, pstartdata, cendkeycolumns, penddata, dwrangeoptions) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetIndex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetIndexInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DBORDINAL, *mut *mut DBINDEXCOLUMNDESC, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetIndexInfo: usize,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, DBORDINAL, *const core::ffi::c_void, DBSEEK) -> windows_core::HRESULT,
    pub SetRange: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, DBORDINAL, *const core::ffi::c_void, DBORDINAL, *const core::ffi::c_void, DBRANGE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRowsetIndex_Impl: windows_core::IUnknownImpl {
    fn GetIndexInfo(&self, pckeycolumns: *mut DBORDINAL, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn Seek(&self, haccessor: HACCESSOR, ckeyvalues: DBORDINAL, pdata: *const core::ffi::c_void, dwseekoptions: DBSEEK) -> windows_core::Result<()>;
    fn SetRange(&self, haccessor: HACCESSOR, cstartkeycolumns: DBORDINAL, pstartdata: *const core::ffi::c_void, cendkeycolumns: DBORDINAL, penddata: *const core::ffi::c_void, dwrangeoptions: DBRANGE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IRowsetIndex_Vtbl {
    pub const fn new<Identity: IRowsetIndex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIndexInfo<Identity: IRowsetIndex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pckeycolumns: *mut DBORDINAL, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetIndex_Impl::GetIndexInfo(this, core::mem::transmute_copy(&pckeycolumns), core::mem::transmute_copy(&prgindexcolumndesc), core::mem::transmute_copy(&pcindexpropertysets), core::mem::transmute_copy(&prgindexpropertysets)).into()
            }
        }
        unsafe extern "system" fn Seek<Identity: IRowsetIndex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, ckeyvalues: DBORDINAL, pdata: *const core::ffi::c_void, dwseekoptions: DBSEEK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetIndex_Impl::Seek(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&ckeyvalues), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwseekoptions)).into()
            }
        }
        unsafe extern "system" fn SetRange<Identity: IRowsetIndex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, cstartkeycolumns: DBORDINAL, pstartdata: *const core::ffi::c_void, cendkeycolumns: DBORDINAL, penddata: *const core::ffi::c_void, dwrangeoptions: DBRANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetIndex_Impl::SetRange(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&cstartkeycolumns), core::mem::transmute_copy(&pstartdata), core::mem::transmute_copy(&cendkeycolumns), core::mem::transmute_copy(&penddata), core::mem::transmute_copy(&dwrangeoptions)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIndexInfo: GetIndexInfo::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            SetRange: SetRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetIndex as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRowsetIndex {}
windows_core::imp::define_interface!(IRowsetInfo, IRowsetInfo_Vtbl, 0x0c733a55_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetInfo, windows_core::IUnknown);
impl IRowsetInfo {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertyidsets.map_or(core::ptr::null(), |slice| slice.as_ptr()), pcpropertysets as _, prgpropertysets as _) }
    }
    pub unsafe fn GetReferencedRowset<T>(&self, iordinal: DBORDINAL) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetReferencedRowset)(windows_core::Interface::as_raw(self), iordinal, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetSpecification<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSpecification)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetProperties: usize,
    pub GetReferencedRowset: unsafe extern "system" fn(*mut core::ffi::c_void, DBORDINAL, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpecification: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRowsetInfo_Impl: windows_core::IUnknownImpl {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn GetReferencedRowset(&self, iordinal: DBORDINAL, riid: *const windows_core::GUID, ppreferencedrowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSpecification(&self, riid: *const windows_core::GUID, ppspecification: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IRowsetInfo_Vtbl {
    pub const fn new<Identity: IRowsetInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperties<Identity: IRowsetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetInfo_Impl::GetProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
            }
        }
        unsafe extern "system" fn GetReferencedRowset<Identity: IRowsetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iordinal: DBORDINAL, riid: *const windows_core::GUID, ppreferencedrowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetInfo_Impl::GetReferencedRowset(this, core::mem::transmute_copy(&iordinal), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppreferencedrowset)).into()
            }
        }
        unsafe extern "system" fn GetSpecification<Identity: IRowsetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppspecification: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetInfo_Impl::GetSpecification(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppspecification)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetReferencedRowset: GetReferencedRowset::<Identity, OFFSET>,
            GetSpecification: GetSpecification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRowsetInfo {}
windows_core::imp::define_interface!(IRowsetLocate, IRowsetLocate_Vtbl, 0x0c733a7d_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetLocate {
    type Target = IRowset;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetLocate, windows_core::IUnknown, IRowset);
impl IRowsetLocate {
    pub unsafe fn Compare(&self, hreserved: HCHAPTER, cbbookmark1: DBBKMARK, pbookmark1: *const u8, cbbookmark2: DBBKMARK, pbookmark2: *const u8) -> windows_core::Result<DBCOMPARE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), hreserved, cbbookmark1, pbookmark1, cbbookmark2, pbookmark2, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRowsAt(&self, hreserved1: HWATCHREGION, hreserved2: HCHAPTER, cbbookmark: DBBKMARK, pbookmark: *const u8, lrowsoffset: DBROWOFFSET, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRowsAt)(windows_core::Interface::as_raw(self), hreserved1, hreserved2, cbbookmark, pbookmark, lrowsoffset, crows, pcrowsobtained as _, prghrows as _) }
    }
    pub unsafe fn GetRowsByBookmark(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rgcbbookmarks: *const DBBKMARK, rgpbookmarks: *const *const u8, rghrows: *mut HROW, rgrowstatus: *mut DBROWSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRowsByBookmark)(windows_core::Interface::as_raw(self), hreserved, crows, rgcbbookmarks, rgpbookmarks, rghrows as _, rgrowstatus as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn Hash(&self, hreserved: HCHAPTER, cbookmarks: DBBKMARK, rgcbbookmarks: *const DBBKMARK, rgpbookmarks: *const *const u8, rghashedvalues: *mut DBHASHVALUE, rgbookmarkstatus: *mut DBROWSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Hash)(windows_core::Interface::as_raw(self), hreserved, cbookmarks, rgcbbookmarks, rgpbookmarks, rghashedvalues as _, rgbookmarkstatus as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetLocate_Vtbl {
    pub base__: IRowset_Vtbl,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBBKMARK, *const u8, DBBKMARK, *const u8, *mut DBCOMPARE) -> windows_core::HRESULT,
    pub GetRowsAt: unsafe extern "system" fn(*mut core::ffi::c_void, HWATCHREGION, HCHAPTER, DBBKMARK, *const u8, DBROWOFFSET, DBROWCOUNT, *mut DBCOUNTITEM, *mut *mut HROW) -> windows_core::HRESULT,
    pub GetRowsByBookmark: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBCOUNTITEM, *const DBBKMARK, *const *const u8, *mut HROW, *mut DBROWSTATUS) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub Hash: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBBKMARK, *const DBBKMARK, *const *const u8, *mut DBHASHVALUE, *mut DBROWSTATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Hash: usize,
}
#[cfg(feature = "winnt")]
pub trait IRowsetLocate_Impl: IRowset_Impl {
    fn Compare(&self, hreserved: HCHAPTER, cbbookmark1: DBBKMARK, pbookmark1: *const u8, cbbookmark2: DBBKMARK, pbookmark2: *const u8) -> windows_core::Result<DBCOMPARE>;
    fn GetRowsAt(&self, hreserved1: HWATCHREGION, hreserved2: HCHAPTER, cbbookmark: DBBKMARK, pbookmark: *const u8, lrowsoffset: DBROWOFFSET, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::Result<()>;
    fn GetRowsByBookmark(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rgcbbookmarks: *const DBBKMARK, rgpbookmarks: *const *const u8, rghrows: *mut HROW, rgrowstatus: *mut DBROWSTATUS) -> windows_core::Result<()>;
    fn Hash(&self, hreserved: HCHAPTER, cbookmarks: DBBKMARK, rgcbbookmarks: *const DBBKMARK, rgpbookmarks: *const *const u8, rghashedvalues: *mut DBHASHVALUE, rgbookmarkstatus: *mut DBROWSTATUS) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IRowsetLocate_Vtbl {
    pub const fn new<Identity: IRowsetLocate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compare<Identity: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, cbbookmark1: DBBKMARK, pbookmark1: *const u8, cbbookmark2: DBBKMARK, pbookmark2: *const u8, pcomparison: *mut DBCOMPARE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRowsetLocate_Impl::Compare(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&cbbookmark1), core::mem::transmute_copy(&pbookmark1), core::mem::transmute_copy(&cbbookmark2), core::mem::transmute_copy(&pbookmark2)) {
                    Ok(ok__) => {
                        pcomparison.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRowsAt<Identity: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved1: HWATCHREGION, hreserved2: HCHAPTER, cbbookmark: DBBKMARK, pbookmark: *const u8, lrowsoffset: DBROWOFFSET, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetLocate_Impl::GetRowsAt(this, core::mem::transmute_copy(&hreserved1), core::mem::transmute_copy(&hreserved2), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark), core::mem::transmute_copy(&lrowsoffset), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&pcrowsobtained), core::mem::transmute_copy(&prghrows)).into()
            }
        }
        unsafe extern "system" fn GetRowsByBookmark<Identity: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, crows: DBCOUNTITEM, rgcbbookmarks: *const DBBKMARK, rgpbookmarks: *const *const u8, rghrows: *mut HROW, rgrowstatus: *mut DBROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetLocate_Impl::GetRowsByBookmark(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rgcbbookmarks), core::mem::transmute_copy(&rgpbookmarks), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&rgrowstatus)).into()
            }
        }
        unsafe extern "system" fn Hash<Identity: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, cbookmarks: DBBKMARK, rgcbbookmarks: *const DBBKMARK, rgpbookmarks: *const *const u8, rghashedvalues: *mut DBHASHVALUE, rgbookmarkstatus: *mut DBROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetLocate_Impl::Hash(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&cbookmarks), core::mem::transmute_copy(&rgcbbookmarks), core::mem::transmute_copy(&rgpbookmarks), core::mem::transmute_copy(&rghashedvalues), core::mem::transmute_copy(&rgbookmarkstatus)).into()
            }
        }
        Self {
            base__: IRowset_Vtbl::new::<Identity, OFFSET>(),
            Compare: Compare::<Identity, OFFSET>,
            GetRowsAt: GetRowsAt::<Identity, OFFSET>,
            GetRowsByBookmark: GetRowsByBookmark::<Identity, OFFSET>,
            Hash: Hash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetLocate as windows_core::Interface>::IID || iid == &<IRowset as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IRowsetLocate {}
windows_core::imp::define_interface!(IRowsetNotify, IRowsetNotify_Vtbl, 0x0c733a83_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetNotify, windows_core::IUnknown);
impl IRowsetNotify {
    pub unsafe fn OnFieldChange<P0>(&self, prowset: P0, hrow: HROW, rgcolumns: &[DBORDINAL], ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRowset>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnFieldChange)(windows_core::Interface::as_raw(self), prowset.param().abi(), hrow, DBORDINAL(rgcolumns.len().try_into().unwrap()), rgcolumns.as_ptr(), ereason, ephase, fcantdeny.into()) }
    }
    pub unsafe fn OnRowChange<P0>(&self, prowset: P0, rghrows: &[HROW], ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRowset>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnRowChange)(windows_core::Interface::as_raw(self), prowset.param().abi(), DBCOUNTITEM(rghrows.len().try_into().unwrap()), rghrows.as_ptr(), ereason, ephase, fcantdeny.into()) }
    }
    pub unsafe fn OnRowsetChange<P0>(&self, prowset: P0, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRowset>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnRowsetChange)(windows_core::Interface::as_raw(self), prowset.param().abi(), ereason, ephase, fcantdeny.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnFieldChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, HROW, DBORDINAL, *const DBORDINAL, DBREASON, DBEVENTPHASE, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnRowChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DBCOUNTITEM, *const HROW, DBREASON, DBEVENTPHASE, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnRowsetChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DBREASON, DBEVENTPHASE, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IRowsetNotify_Impl: windows_core::IUnknownImpl {
    fn OnFieldChange(&self, prowset: windows_core::Ref<IRowset>, hrow: HROW, ccolumns: DBORDINAL, rgcolumns: *const DBORDINAL, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnRowChange(&self, prowset: windows_core::Ref<IRowset>, crows: DBCOUNTITEM, rghrows: *const HROW, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnRowsetChange(&self, prowset: windows_core::Ref<IRowset>, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IRowsetNotify_Vtbl {
    pub const fn new<Identity: IRowsetNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnFieldChange<Identity: IRowsetNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowset: *mut core::ffi::c_void, hrow: HROW, ccolumns: DBORDINAL, rgcolumns: *const DBORDINAL, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetNotify_Impl::OnFieldChange(this, core::mem::transmute_copy(&prowset), core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgcolumns), core::mem::transmute_copy(&ereason), core::mem::transmute_copy(&ephase), core::mem::transmute_copy(&fcantdeny)).into()
            }
        }
        unsafe extern "system" fn OnRowChange<Identity: IRowsetNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowset: *mut core::ffi::c_void, crows: DBCOUNTITEM, rghrows: *const HROW, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetNotify_Impl::OnRowChange(this, core::mem::transmute_copy(&prowset), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&ereason), core::mem::transmute_copy(&ephase), core::mem::transmute_copy(&fcantdeny)).into()
            }
        }
        unsafe extern "system" fn OnRowsetChange<Identity: IRowsetNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowset: *mut core::ffi::c_void, ereason: DBREASON, ephase: DBEVENTPHASE, fcantdeny: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetNotify_Impl::OnRowsetChange(this, core::mem::transmute_copy(&prowset), core::mem::transmute_copy(&ereason), core::mem::transmute_copy(&ephase), core::mem::transmute_copy(&fcantdeny)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnFieldChange: OnFieldChange::<Identity, OFFSET>,
            OnRowChange: OnRowChange::<Identity, OFFSET>,
            OnRowsetChange: OnRowsetChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetNotify {}
windows_core::imp::define_interface!(IRowsetRefresh, IRowsetRefresh_Vtbl, 0x0c733aa9_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetRefresh, windows_core::IUnknown);
impl IRowsetRefresh {
    pub unsafe fn RefreshVisibleData(&self, hchapter: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, foverwrite: bool, pcrowsrefreshed: *mut DBCOUNTITEM, prghrowsrefreshed: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshVisibleData)(windows_core::Interface::as_raw(self), hchapter, crows, rghrows, foverwrite.into(), pcrowsrefreshed as _, prghrowsrefreshed as _, prgrowstatus as _) }
    }
    pub unsafe fn GetLastVisibleData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLastVisibleData)(windows_core::Interface::as_raw(self), hrow, haccessor, pdata as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetRefresh_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RefreshVisibleData: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBCOUNTITEM, *const HROW, windows_core::BOOL, *mut DBCOUNTITEM, *mut *mut HROW, *mut *mut DBROWSTATUS) -> windows_core::HRESULT,
    pub GetLastVisibleData: unsafe extern "system" fn(*mut core::ffi::c_void, HROW, HACCESSOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRowsetRefresh_Impl: windows_core::IUnknownImpl {
    fn RefreshVisibleData(&self, hchapter: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, foverwrite: windows_core::BOOL, pcrowsrefreshed: *mut DBCOUNTITEM, prghrowsrefreshed: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::Result<()>;
    fn GetLastVisibleData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IRowsetRefresh_Vtbl {
    pub const fn new<Identity: IRowsetRefresh_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RefreshVisibleData<Identity: IRowsetRefresh_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, foverwrite: windows_core::BOOL, pcrowsrefreshed: *mut DBCOUNTITEM, prghrowsrefreshed: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetRefresh_Impl::RefreshVisibleData(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&foverwrite), core::mem::transmute_copy(&pcrowsrefreshed), core::mem::transmute_copy(&prghrowsrefreshed), core::mem::transmute_copy(&prgrowstatus)).into()
            }
        }
        unsafe extern "system" fn GetLastVisibleData<Identity: IRowsetRefresh_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetRefresh_Impl::GetLastVisibleData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RefreshVisibleData: RefreshVisibleData::<Identity, OFFSET>,
            GetLastVisibleData: GetLastVisibleData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetRefresh as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetRefresh {}
windows_core::imp::define_interface!(IRowsetResynch, IRowsetResynch_Vtbl, 0x0c733a84_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetResynch, windows_core::IUnknown);
impl IRowsetResynch {
    pub unsafe fn GetVisibleData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVisibleData)(windows_core::Interface::as_raw(self), hrow, haccessor, pdata as _) }
    }
    pub unsafe fn ResynchRows(&self, crows: DBCOUNTITEM, rghrows: *const HROW, pcrowsresynched: *mut DBCOUNTITEM, prghrowsresynched: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResynchRows)(windows_core::Interface::as_raw(self), crows, rghrows, pcrowsresynched as _, prghrowsresynched as _, prgrowstatus as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetResynch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVisibleData: unsafe extern "system" fn(*mut core::ffi::c_void, HROW, HACCESSOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResynchRows: unsafe extern "system" fn(*mut core::ffi::c_void, DBCOUNTITEM, *const HROW, *mut DBCOUNTITEM, *mut *mut HROW, *mut *mut DBROWSTATUS) -> windows_core::HRESULT,
}
pub trait IRowsetResynch_Impl: windows_core::IUnknownImpl {
    fn GetVisibleData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ResynchRows(&self, crows: DBCOUNTITEM, rghrows: *const HROW, pcrowsresynched: *mut DBCOUNTITEM, prghrowsresynched: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::Result<()>;
}
impl IRowsetResynch_Vtbl {
    pub const fn new<Identity: IRowsetResynch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVisibleData<Identity: IRowsetResynch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetResynch_Impl::GetVisibleData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn ResynchRows<Identity: IRowsetResynch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: DBCOUNTITEM, rghrows: *const HROW, pcrowsresynched: *mut DBCOUNTITEM, prghrowsresynched: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetResynch_Impl::ResynchRows(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&pcrowsresynched), core::mem::transmute_copy(&prghrowsresynched), core::mem::transmute_copy(&prgrowstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVisibleData: GetVisibleData::<Identity, OFFSET>,
            ResynchRows: ResynchRows::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetResynch as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetResynch {}
windows_core::imp::define_interface!(IRowsetScroll, IRowsetScroll_Vtbl, 0x0c733a7e_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetScroll {
    type Target = IRowsetLocate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetScroll, windows_core::IUnknown, IRowset, IRowsetLocate);
impl IRowsetScroll {
    pub unsafe fn GetApproximatePosition(&self, hreserved: HCHAPTER, cbbookmark: DBBKMARK, pbookmark: *const u8, pulposition: *mut DBCOUNTITEM, pcrows: *mut DBCOUNTITEM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetApproximatePosition)(windows_core::Interface::as_raw(self), hreserved, cbbookmark, pbookmark, pulposition as _, pcrows as _) }
    }
    pub unsafe fn GetRowsAtRatio(&self, hreserved1: HWATCHREGION, hreserved2: HCHAPTER, ulnumerator: DBCOUNTITEM, uldenominator: DBCOUNTITEM, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRowsAtRatio)(windows_core::Interface::as_raw(self), hreserved1, hreserved2, ulnumerator, uldenominator, crows, pcrowsobtained as _, prghrows as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetScroll_Vtbl {
    pub base__: IRowsetLocate_Vtbl,
    pub GetApproximatePosition: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBBKMARK, *const u8, *mut DBCOUNTITEM, *mut DBCOUNTITEM) -> windows_core::HRESULT,
    pub GetRowsAtRatio: unsafe extern "system" fn(*mut core::ffi::c_void, HWATCHREGION, HCHAPTER, DBCOUNTITEM, DBCOUNTITEM, DBROWCOUNT, *mut DBCOUNTITEM, *mut *mut HROW) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IRowsetScroll_Impl: IRowsetLocate_Impl {
    fn GetApproximatePosition(&self, hreserved: HCHAPTER, cbbookmark: DBBKMARK, pbookmark: *const u8, pulposition: *mut DBCOUNTITEM, pcrows: *mut DBCOUNTITEM) -> windows_core::Result<()>;
    fn GetRowsAtRatio(&self, hreserved1: HWATCHREGION, hreserved2: HCHAPTER, ulnumerator: DBCOUNTITEM, uldenominator: DBCOUNTITEM, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IRowsetScroll_Vtbl {
    pub const fn new<Identity: IRowsetScroll_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetApproximatePosition<Identity: IRowsetScroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, cbbookmark: DBBKMARK, pbookmark: *const u8, pulposition: *mut DBCOUNTITEM, pcrows: *mut DBCOUNTITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetScroll_Impl::GetApproximatePosition(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark), core::mem::transmute_copy(&pulposition), core::mem::transmute_copy(&pcrows)).into()
            }
        }
        unsafe extern "system" fn GetRowsAtRatio<Identity: IRowsetScroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved1: HWATCHREGION, hreserved2: HCHAPTER, ulnumerator: DBCOUNTITEM, uldenominator: DBCOUNTITEM, crows: DBROWCOUNT, pcrowsobtained: *mut DBCOUNTITEM, prghrows: *mut *mut HROW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetScroll_Impl::GetRowsAtRatio(this, core::mem::transmute_copy(&hreserved1), core::mem::transmute_copy(&hreserved2), core::mem::transmute_copy(&ulnumerator), core::mem::transmute_copy(&uldenominator), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&pcrowsobtained), core::mem::transmute_copy(&prghrows)).into()
            }
        }
        Self {
            base__: IRowsetLocate_Vtbl::new::<Identity, OFFSET>(),
            GetApproximatePosition: GetApproximatePosition::<Identity, OFFSET>,
            GetRowsAtRatio: GetRowsAtRatio::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetScroll as windows_core::Interface>::IID || iid == &<IRowset as windows_core::Interface>::IID || iid == &<IRowsetLocate as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IRowsetScroll {}
windows_core::imp::define_interface!(IRowsetUpdate, IRowsetUpdate_Vtbl, 0x0c733a6d_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetUpdate {
    type Target = IRowsetChange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetUpdate, windows_core::IUnknown, IRowsetChange);
impl IRowsetUpdate {
    pub unsafe fn GetOriginalData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOriginalData)(windows_core::Interface::as_raw(self), hrow, haccessor, pdata as _) }
    }
    pub unsafe fn GetPendingRows(&self, hreserved: HCHAPTER, dwrowstatus: DBPENDINGSTATUS, pcpendingrows: *mut DBCOUNTITEM, prgpendingrows: *mut *mut HROW, prgpendingstatus: *mut *mut DBPENDINGSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPendingRows)(windows_core::Interface::as_raw(self), hreserved, dwrowstatus, pcpendingrows as _, prgpendingrows as _, prgpendingstatus as _) }
    }
    pub unsafe fn GetRowStatus(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW) -> windows_core::Result<DBPENDINGSTATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRowStatus)(windows_core::Interface::as_raw(self), hreserved, crows, rghrows, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Undo(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, pcrowsundone: *mut DBCOUNTITEM, prgrowsundone: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Undo)(windows_core::Interface::as_raw(self), hreserved, crows, rghrows, pcrowsundone as _, prgrowsundone as _, prgrowstatus as _) }
    }
    pub unsafe fn Update(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, pcrows: *mut DBCOUNTITEM, prgrows: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), hreserved, crows, rghrows, pcrows as _, prgrows as _, prgrowstatus as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetUpdate_Vtbl {
    pub base__: IRowsetChange_Vtbl,
    pub GetOriginalData: unsafe extern "system" fn(*mut core::ffi::c_void, HROW, HACCESSOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPendingRows: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBPENDINGSTATUS, *mut DBCOUNTITEM, *mut *mut HROW, *mut *mut DBPENDINGSTATUS) -> windows_core::HRESULT,
    pub GetRowStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBCOUNTITEM, *const HROW, *mut DBPENDINGSTATUS) -> windows_core::HRESULT,
    pub Undo: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBCOUNTITEM, *const HROW, *mut DBCOUNTITEM, *mut *mut HROW, *mut *mut DBROWSTATUS) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, DBCOUNTITEM, *const HROW, *mut DBCOUNTITEM, *mut *mut HROW, *mut *mut DBROWSTATUS) -> windows_core::HRESULT,
}
pub trait IRowsetUpdate_Impl: IRowsetChange_Impl {
    fn GetOriginalData(&self, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPendingRows(&self, hreserved: HCHAPTER, dwrowstatus: DBPENDINGSTATUS, pcpendingrows: *mut DBCOUNTITEM, prgpendingrows: *mut *mut HROW, prgpendingstatus: *mut *mut DBPENDINGSTATUS) -> windows_core::Result<()>;
    fn GetRowStatus(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW) -> windows_core::Result<DBPENDINGSTATUS>;
    fn Undo(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, pcrowsundone: *mut DBCOUNTITEM, prgrowsundone: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::Result<()>;
    fn Update(&self, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, pcrows: *mut DBCOUNTITEM, prgrows: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::Result<()>;
}
impl IRowsetUpdate_Vtbl {
    pub const fn new<Identity: IRowsetUpdate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOriginalData<Identity: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: HROW, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetUpdate_Impl::GetOriginalData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetPendingRows<Identity: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, dwrowstatus: DBPENDINGSTATUS, pcpendingrows: *mut DBCOUNTITEM, prgpendingrows: *mut *mut HROW, prgpendingstatus: *mut *mut DBPENDINGSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetUpdate_Impl::GetPendingRows(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&dwrowstatus), core::mem::transmute_copy(&pcpendingrows), core::mem::transmute_copy(&prgpendingrows), core::mem::transmute_copy(&prgpendingstatus)).into()
            }
        }
        unsafe extern "system" fn GetRowStatus<Identity: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, rgpendingstatus: *mut DBPENDINGSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRowsetUpdate_Impl::GetRowStatus(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows)) {
                    Ok(ok__) => {
                        rgpendingstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Undo<Identity: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, pcrowsundone: *mut DBCOUNTITEM, prgrowsundone: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetUpdate_Impl::Undo(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&pcrowsundone), core::mem::transmute_copy(&prgrowsundone), core::mem::transmute_copy(&prgrowstatus)).into()
            }
        }
        unsafe extern "system" fn Update<Identity: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: HCHAPTER, crows: DBCOUNTITEM, rghrows: *const HROW, pcrows: *mut DBCOUNTITEM, prgrows: *mut *mut HROW, prgrowstatus: *mut *mut DBROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetUpdate_Impl::Update(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&pcrows), core::mem::transmute_copy(&prgrows), core::mem::transmute_copy(&prgrowstatus)).into()
            }
        }
        Self {
            base__: IRowsetChange_Vtbl::new::<Identity, OFFSET>(),
            GetOriginalData: GetOriginalData::<Identity, OFFSET>,
            GetPendingRows: GetPendingRows::<Identity, OFFSET>,
            GetRowStatus: GetRowStatus::<Identity, OFFSET>,
            Undo: Undo::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetUpdate as windows_core::Interface>::IID || iid == &<IRowsetChange as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetUpdate {}
windows_core::imp::define_interface!(IRowsetView, IRowsetView_Vtbl, 0x0c733a99_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IRowsetView, windows_core::IUnknown);
impl IRowsetView {
    pub unsafe fn CreateView<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateView)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetView<T>(&self, hchapter: HCHAPTER, phchaptersource: *mut HCHAPTER) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetView)(windows_core::Interface::as_raw(self), hchapter, &T::IID, phchaptersource as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, *const windows_core::GUID, *mut HCHAPTER, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRowsetView_Impl: windows_core::IUnknownImpl {
    fn CreateView(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppview: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetView(&self, hchapter: HCHAPTER, riid: *const windows_core::GUID, phchaptersource: *mut HCHAPTER, ppview: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IRowsetView_Vtbl {
    pub const fn new<Identity: IRowsetView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateView<Identity: IRowsetView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetView_Impl::CreateView(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppview)).into()
            }
        }
        unsafe extern "system" fn GetView<Identity: IRowsetView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: HCHAPTER, riid: *const windows_core::GUID, phchaptersource: *mut HCHAPTER, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetView_Impl::GetView(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&phchaptersource), core::mem::transmute_copy(&ppview)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateView: CreateView::<Identity, OFFSET>, GetView: GetView::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetView as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetView {}
windows_core::imp::define_interface!(ISQLErrorInfo, ISQLErrorInfo_Vtbl, 0x0c733a74_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ISQLErrorInfo, windows_core::IUnknown);
impl ISQLErrorInfo {
    pub unsafe fn GetSQLInfo(&self, pbstrsqlstate: *mut windows_core::BSTR, plnativeerror: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSQLInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrsqlstate), plnativeerror as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISQLErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSQLInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait ISQLErrorInfo_Impl: windows_core::IUnknownImpl {
    fn GetSQLInfo(&self, pbstrsqlstate: *mut windows_core::BSTR, plnativeerror: *mut i32) -> windows_core::Result<()>;
}
impl ISQLErrorInfo_Vtbl {
    pub const fn new<Identity: ISQLErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSQLInfo<Identity: ISQLErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsqlstate: *mut *mut core::ffi::c_void, plnativeerror: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISQLErrorInfo_Impl::GetSQLInfo(this, core::mem::transmute_copy(&pbstrsqlstate), core::mem::transmute_copy(&plnativeerror)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSQLInfo: GetSQLInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISQLErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISQLErrorInfo {}
windows_core::imp::define_interface!(IScopedOperations, IScopedOperations_Vtbl, 0x0c733ab0_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IScopedOperations {
    type Target = IBindResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IScopedOperations, windows_core::IUnknown, IBindResource);
impl IScopedOperations {
    #[cfg(all(feature = "urlmon", feature = "wtypesbase"))]
    pub unsafe fn Copy<P4>(&self, crows: DBCOUNTITEM, rgpwszsourceurls: Option<*const windows_core::PCWSTR>, rgpwszdesturls: *const windows_core::PCWSTR, dwcopyflags: u32, pauthenticate: P4, rgdwstatus: *mut DBSTATUS, rgpwsznewurls: Option<*mut windows_core::PWSTR>, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::IAuthenticate>,
    {
        unsafe { (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), crows, rgpwszsourceurls.unwrap_or(core::mem::zeroed()) as _, rgpwszdesturls, dwcopyflags, pauthenticate.param().abi(), rgdwstatus as _, rgpwsznewurls.unwrap_or(core::mem::zeroed()) as _, ppstringsbuffer as _) }
    }
    #[cfg(all(feature = "urlmon", feature = "wtypesbase"))]
    pub unsafe fn Move<P4>(&self, crows: DBCOUNTITEM, rgpwszsourceurls: Option<*const windows_core::PCWSTR>, rgpwszdesturls: *const windows_core::PCWSTR, dwmoveflags: u32, pauthenticate: P4, rgdwstatus: *mut DBSTATUS, rgpwsznewurls: Option<*mut windows_core::PWSTR>, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::IAuthenticate>,
    {
        unsafe { (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), crows, rgpwszsourceurls.unwrap_or(core::mem::zeroed()) as _, rgpwszdesturls, dwmoveflags, pauthenticate.param().abi(), rgdwstatus as _, rgpwsznewurls.unwrap_or(core::mem::zeroed()) as _, ppstringsbuffer as _) }
    }
    pub unsafe fn Delete(&self, crows: DBCOUNTITEM, rgpwszurls: *const windows_core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut DBSTATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), crows, rgpwszurls, dwdeleteflags, rgdwstatus as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn OpenRowset<P0, T>(&self, punkouter: P0, ptableid: Option<*const DBID>, pindexid: Option<*const DBID>, rgpropertysets: &mut [DBPROPSET], result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), ptableid.unwrap_or(core::mem::zeroed()) as _, pindexid.unwrap_or(core::mem::zeroed()) as _, &T::IID, rgpropertysets.len().try_into().unwrap(), rgpropertysets.as_mut_ptr(), result__ as *mut _ as *mut _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScopedOperations_Vtbl {
    pub base__: IBindResource_Vtbl,
    #[cfg(all(feature = "urlmon", feature = "wtypesbase"))]
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, DBCOUNTITEM, *const windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut DBSTATUS, *mut windows_core::PWSTR, *mut *mut super::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "urlmon", feature = "wtypesbase")))]
    Copy: usize,
    #[cfg(all(feature = "urlmon", feature = "wtypesbase"))]
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, DBCOUNTITEM, *const windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut DBSTATUS, *mut windows_core::PWSTR, *mut *mut super::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "urlmon", feature = "wtypesbase")))]
    Move: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, DBCOUNTITEM, *const windows_core::PCWSTR, u32, *mut DBSTATUS) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub OpenRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DBID, *const DBID, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    OpenRowset: usize,
}
#[cfg(all(feature = "oaidl", feature = "urlmon", feature = "wtypes", feature = "wtypesbase"))]
pub trait IScopedOperations_Impl: IBindResource_Impl {
    fn Copy(&self, crows: DBCOUNTITEM, rgpwszsourceurls: *const windows_core::PCWSTR, rgpwszdesturls: *const windows_core::PCWSTR, dwcopyflags: u32, pauthenticate: windows_core::Ref<super::IAuthenticate>, rgdwstatus: *mut DBSTATUS, rgpwsznewurls: *mut windows_core::PWSTR, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::Result<()>;
    fn Move(&self, crows: DBCOUNTITEM, rgpwszsourceurls: *const windows_core::PCWSTR, rgpwszdesturls: *const windows_core::PCWSTR, dwmoveflags: u32, pauthenticate: windows_core::Ref<super::IAuthenticate>, rgdwstatus: *mut DBSTATUS, rgpwsznewurls: *mut windows_core::PWSTR, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::Result<()>;
    fn Delete(&self, crows: DBCOUNTITEM, rgpwszurls: *const windows_core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut DBSTATUS) -> windows_core::Result<()>;
    fn OpenRowset(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, ptableid: *const DBID, pindexid: *const DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "urlmon", feature = "wtypes", feature = "wtypesbase"))]
impl IScopedOperations_Vtbl {
    pub const fn new<Identity: IScopedOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Copy<Identity: IScopedOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: DBCOUNTITEM, rgpwszsourceurls: *const windows_core::PCWSTR, rgpwszdesturls: *const windows_core::PCWSTR, dwcopyflags: u32, pauthenticate: *mut core::ffi::c_void, rgdwstatus: *mut DBSTATUS, rgpwsznewurls: *mut windows_core::PWSTR, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScopedOperations_Impl::Copy(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rgpwszsourceurls), core::mem::transmute_copy(&rgpwszdesturls), core::mem::transmute_copy(&dwcopyflags), core::mem::transmute_copy(&pauthenticate), core::mem::transmute_copy(&rgdwstatus), core::mem::transmute_copy(&rgpwsznewurls), core::mem::transmute_copy(&ppstringsbuffer)).into()
            }
        }
        unsafe extern "system" fn Move<Identity: IScopedOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: DBCOUNTITEM, rgpwszsourceurls: *const windows_core::PCWSTR, rgpwszdesturls: *const windows_core::PCWSTR, dwmoveflags: u32, pauthenticate: *mut core::ffi::c_void, rgdwstatus: *mut DBSTATUS, rgpwsznewurls: *mut windows_core::PWSTR, ppstringsbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScopedOperations_Impl::Move(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rgpwszsourceurls), core::mem::transmute_copy(&rgpwszdesturls), core::mem::transmute_copy(&dwmoveflags), core::mem::transmute_copy(&pauthenticate), core::mem::transmute_copy(&rgdwstatus), core::mem::transmute_copy(&rgpwsznewurls), core::mem::transmute_copy(&ppstringsbuffer)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IScopedOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: DBCOUNTITEM, rgpwszurls: *const windows_core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut DBSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScopedOperations_Impl::Delete(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rgpwszurls), core::mem::transmute_copy(&dwdeleteflags), core::mem::transmute_copy(&rgdwstatus)).into()
            }
        }
        unsafe extern "system" fn OpenRowset<Identity: IScopedOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ptableid: *const DBID, pindexid: *const DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScopedOperations_Impl::OpenRowset(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        Self {
            base__: IBindResource_Vtbl::new::<Identity, OFFSET>(),
            Copy: Copy::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            OpenRowset: OpenRowset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScopedOperations as windows_core::Interface>::IID || iid == &<IBindResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "urlmon", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IScopedOperations {}
windows_core::imp::define_interface!(ISecurityInfo, ISecurityInfo_Vtbl, 0x0c733aa4_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ISecurityInfo, windows_core::IUnknown);
impl ISecurityInfo {
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn GetCurrentTrustee(&self) -> windows_core::Result<*mut super::TRUSTEE_W> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentTrustee)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetObjectTypes(&self, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectTypes)(windows_core::Interface::as_raw(self), cobjecttypes as _, rgobjecttypes as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetPermissions(&self, objecttype: windows_core::GUID) -> windows_core::Result<super::ACCESS_MASK> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPermissions)(windows_core::Interface::as_raw(self), objecttype, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub GetCurrentTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    GetCurrentTrustee: usize,
    pub GetObjectTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut super::ACCESS_MASK) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetPermissions: usize,
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
pub trait ISecurityInfo_Impl: windows_core::IUnknownImpl {
    fn GetCurrentTrustee(&self) -> windows_core::Result<*mut super::TRUSTEE_W>;
    fn GetObjectTypes(&self, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetPermissions(&self, objecttype: &windows_core::GUID) -> windows_core::Result<super::ACCESS_MASK>;
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
impl ISecurityInfo_Vtbl {
    pub const fn new<Identity: ISecurityInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentTrustee<Identity: ISecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptrustee: *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityInfo_Impl::GetCurrentTrustee(this) {
                    Ok(ok__) => {
                        pptrustee.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectTypes<Identity: ISecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISecurityInfo_Impl::GetObjectTypes(this, core::mem::transmute_copy(&cobjecttypes), core::mem::transmute_copy(&rgobjecttypes)).into()
            }
        }
        unsafe extern "system" fn GetPermissions<Identity: ISecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objecttype: windows_core::GUID, ppermissions: *mut super::ACCESS_MASK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityInfo_Impl::GetPermissions(this, core::mem::transmute(&objecttype)) {
                    Ok(ok__) => {
                        ppermissions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentTrustee: GetCurrentTrustee::<Identity, OFFSET>,
            GetObjectTypes: GetObjectTypes::<Identity, OFFSET>,
            GetPermissions: GetPermissions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
impl windows_core::RuntimeName for ISecurityInfo {}
windows_core::imp::define_interface!(ISessionProperties, ISessionProperties_Vtbl, 0x0c733a85_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ISessionProperties, windows_core::IUnknown);
impl ISessionProperties {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertyidsets.map_or(core::ptr::null(), |slice| slice.as_ptr()), pcpropertysets as _, prgpropertysets as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetProperties(&self, rgpropertysets: Option<&mut [DBPROPSET]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperties)(windows_core::Interface::as_raw(self), rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertysets.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISessionProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetProperties: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    SetProperties: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISessionProperties_Impl: windows_core::IUnknownImpl {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn SetProperties(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ISessionProperties_Vtbl {
    pub const fn new<Identity: ISessionProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperties<Identity: ISessionProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionProperties_Impl::GetProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
            }
        }
        unsafe extern "system" fn SetProperties<Identity: ISessionProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionProperties_Impl::SetProperties(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            SetProperties: SetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISessionProperties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISessionProperties {}
windows_core::imp::define_interface!(ISourcesRowset, ISourcesRowset_Vtbl, 0x0c733a1e_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ISourcesRowset, windows_core::IUnknown);
impl ISourcesRowset {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetSourcesRowset<P0, T>(&self, punkouter: P0, rgproperties: Option<&mut [DBPROPSET]>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSourcesRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, rgproperties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), rgproperties.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourcesRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetSourcesRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetSourcesRowset: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISourcesRowset_Impl: windows_core::IUnknownImpl {
    fn GetSourcesRowset(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ISourcesRowset_Vtbl {
    pub const fn new<Identity: ISourcesRowset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSourcesRowset<Identity: ISourcesRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISourcesRowset_Impl::GetSourcesRowset(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgproperties), core::mem::transmute_copy(&ppsourcesrowset)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSourcesRowset: GetSourcesRowset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISourcesRowset as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISourcesRowset {}
windows_core::imp::define_interface!(ITableCreation, ITableCreation_Vtbl, 0x0c733abc_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITableCreation {
    type Target = ITableDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITableCreation, windows_core::IUnknown, ITableDefinition);
impl ITableCreation {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetTableDefinition(&self, ptableid: *const DBID, pccolumndescs: Option<*mut DBORDINAL>, prgcolumndescs: Option<*mut *mut DBCOLUMNDESC>, pcpropertysets: Option<*mut u32>, prgpropertysets: Option<*mut *mut DBPROPSET>, pcconstraintdescs: Option<*mut u32>, prgconstraintdescs: Option<*mut *mut DBCONSTRAINTDESC>, ppwszstringbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTableDefinition)(windows_core::Interface::as_raw(self), ptableid, pccolumndescs.unwrap_or(core::mem::zeroed()) as _, prgcolumndescs.unwrap_or(core::mem::zeroed()) as _, pcpropertysets.unwrap_or(core::mem::zeroed()) as _, prgpropertysets.unwrap_or(core::mem::zeroed()) as _, pcconstraintdescs.unwrap_or(core::mem::zeroed()) as _, prgconstraintdescs.unwrap_or(core::mem::zeroed()) as _, ppwszstringbuffer as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableCreation_Vtbl {
    pub base__: ITableDefinition_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetTableDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *mut DBORDINAL, *mut *mut DBCOLUMNDESC, *mut u32, *mut *mut DBPROPSET, *mut u32, *mut *mut DBCONSTRAINTDESC, *mut *mut super::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetTableDefinition: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITableCreation_Impl: ITableDefinition_Impl {
    fn GetTableDefinition(&self, ptableid: *const DBID, pccolumndescs: *mut DBORDINAL, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut super::OLECHAR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ITableCreation_Vtbl {
    pub const fn new<Identity: ITableCreation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTableDefinition<Identity: ITableCreation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pccolumndescs: *mut DBORDINAL, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut super::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableCreation_Impl::GetTableDefinition(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pccolumndescs), core::mem::transmute_copy(&prgcolumndescs), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets), core::mem::transmute_copy(&pcconstraintdescs), core::mem::transmute_copy(&prgconstraintdescs), core::mem::transmute_copy(&ppwszstringbuffer)).into()
            }
        }
        Self { base__: ITableDefinition_Vtbl::new::<Identity, OFFSET>(), GetTableDefinition: GetTableDefinition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableCreation as windows_core::Interface>::IID || iid == &<ITableDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITableCreation {}
windows_core::imp::define_interface!(ITableDefinition, ITableDefinition_Vtbl, 0x0c733a86_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ITableDefinition, windows_core::IUnknown);
impl ITableDefinition {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreateTable<P0, T>(&self, punkouter: P0, ptableid: Option<*const DBID>, rgcolumndescs: Option<&[DBCOLUMNDESC]>, rgpropertysets: Option<&mut [DBPROPSET]>, pptableid: *mut *mut DBID) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateTable)(windows_core::Interface::as_raw(self), punkouter.param().abi(), ptableid.unwrap_or(core::mem::zeroed()) as _, rgcolumndescs.map_or(DBORDINAL(0), |slice| DBORDINAL(slice.len().try_into().unwrap())), rgcolumndescs.map_or(core::ptr::null(), |slice| slice.as_ptr()), &T::IID, rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), rgpropertysets.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), pptableid as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn DropTable(&self, ptableid: *const DBID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DropTable)(windows_core::Interface::as_raw(self), ptableid) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddColumn(&self, ptableid: *const DBID, pcolumndesc: *const DBCOLUMNDESC) -> windows_core::Result<*mut DBID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddColumn)(windows_core::Interface::as_raw(self), ptableid, pcolumndesc, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DropColumn(&self, ptableid: *const DBID, pcolumnid: *const DBID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DropColumn)(windows_core::Interface::as_raw(self), ptableid, pcolumnid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableDefinition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub CreateTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DBID, DBORDINAL, *const DBCOLUMNDESC, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut DBID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    CreateTable: usize,
    pub DropTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub AddColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *const DBCOLUMNDESC, *mut *mut DBID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    AddColumn: usize,
    pub DropColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *const DBID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITableDefinition_Impl: windows_core::IUnknownImpl {
    fn CreateTable(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, ptableid: *const DBID, ccolumndescs: DBORDINAL, rgcolumndescs: *const DBCOLUMNDESC, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut DBID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DropTable(&self, ptableid: *const DBID) -> windows_core::Result<()>;
    fn AddColumn(&self, ptableid: *const DBID, pcolumndesc: *const DBCOLUMNDESC) -> windows_core::Result<*mut DBID>;
    fn DropColumn(&self, ptableid: *const DBID, pcolumnid: *const DBID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ITableDefinition_Vtbl {
    pub const fn new<Identity: ITableDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTable<Identity: ITableDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ptableid: *const DBID, ccolumndescs: DBORDINAL, rgcolumndescs: *const DBCOLUMNDESC, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut DBID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableDefinition_Impl::CreateTable(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&ccolumndescs), core::mem::transmute_copy(&rgcolumndescs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pptableid), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        unsafe extern "system" fn DropTable<Identity: ITableDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableDefinition_Impl::DropTable(this, core::mem::transmute_copy(&ptableid)).into()
            }
        }
        unsafe extern "system" fn AddColumn<Identity: ITableDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITableDefinition_Impl::AddColumn(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pcolumndesc)) {
                    Ok(ok__) => {
                        ppcolumnid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DropColumn<Identity: ITableDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pcolumnid: *const DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableDefinition_Impl::DropColumn(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pcolumnid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, OFFSET>,
            DropTable: DropTable::<Identity, OFFSET>,
            AddColumn: AddColumn::<Identity, OFFSET>,
            DropColumn: DropColumn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITableDefinition {}
windows_core::imp::define_interface!(ITableDefinitionWithConstraints, ITableDefinitionWithConstraints_Vtbl, 0x0c733aab_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITableDefinitionWithConstraints {
    type Target = ITableCreation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITableDefinitionWithConstraints, windows_core::IUnknown, ITableDefinition, ITableCreation);
impl ITableDefinitionWithConstraints {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddConstraint(&self, ptableid: *const DBID, pconstraintdesc: *const DBCONSTRAINTDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddConstraint)(windows_core::Interface::as_raw(self), ptableid, pconstraintdesc) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreateTableWithConstraints<P0, T>(&self, punkouter: P0, ptableid: *const DBID, ccolumndescs: DBORDINAL, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *const DBCONSTRAINTDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut DBID) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateTableWithConstraints)(windows_core::Interface::as_raw(self), punkouter.param().abi(), ptableid, ccolumndescs, rgcolumndescs, cconstraintdescs, rgconstraintdescs, &T::IID, cpropertysets, rgpropertysets as _, pptableid as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn DropConstraint(&self, ptableid: *const DBID, pconstraintid: *const DBID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DropConstraint)(windows_core::Interface::as_raw(self), ptableid, pconstraintid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableDefinitionWithConstraints_Vtbl {
    pub base__: ITableCreation_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub AddConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *const DBCONSTRAINTDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    AddConstraint: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub CreateTableWithConstraints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DBID, DBORDINAL, *mut DBCOLUMNDESC, u32, *const DBCONSTRAINTDESC, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut DBID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    CreateTableWithConstraints: usize,
    pub DropConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBID, *const DBID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITableDefinitionWithConstraints_Impl: ITableCreation_Impl {
    fn AddConstraint(&self, ptableid: *const DBID, pconstraintdesc: *const DBCONSTRAINTDESC) -> windows_core::Result<()>;
    fn CreateTableWithConstraints(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, ptableid: *const DBID, ccolumndescs: DBORDINAL, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *const DBCONSTRAINTDESC, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut DBID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DropConstraint(&self, ptableid: *const DBID, pconstraintid: *const DBID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ITableDefinitionWithConstraints_Vtbl {
    pub const fn new<Identity: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddConstraint<Identity: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pconstraintdesc: *const DBCONSTRAINTDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableDefinitionWithConstraints_Impl::AddConstraint(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pconstraintdesc)).into()
            }
        }
        unsafe extern "system" fn CreateTableWithConstraints<Identity: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ptableid: *const DBID, ccolumndescs: DBORDINAL, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *const DBCONSTRAINTDESC, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut DBID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableDefinitionWithConstraints_Impl::CreateTableWithConstraints(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&ccolumndescs), core::mem::transmute_copy(&rgcolumndescs), core::mem::transmute_copy(&cconstraintdescs), core::mem::transmute_copy(&rgconstraintdescs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pptableid), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        unsafe extern "system" fn DropConstraint<Identity: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const DBID, pconstraintid: *const DBID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITableDefinitionWithConstraints_Impl::DropConstraint(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pconstraintid)).into()
            }
        }
        Self {
            base__: ITableCreation_Vtbl::new::<Identity, OFFSET>(),
            AddConstraint: AddConstraint::<Identity, OFFSET>,
            CreateTableWithConstraints: CreateTableWithConstraints::<Identity, OFFSET>,
            DropConstraint: DropConstraint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableDefinitionWithConstraints as windows_core::Interface>::IID || iid == &<ITableDefinition as windows_core::Interface>::IID || iid == &<ITableCreation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITableDefinitionWithConstraints {}
windows_core::imp::define_interface!(ITransactionJoin, ITransactionJoin_Vtbl, 0x0c733a5e_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ITransactionJoin, windows_core::IUnknown);
impl ITransactionJoin {
    #[cfg(feature = "transact")]
    pub unsafe fn GetOptionsObject(&self) -> windows_core::Result<super::ITransactionOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionsObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "transact")]
    pub unsafe fn JoinTransaction<P0, P3>(&self, punktransactioncoord: P0, isolevel: super::ISOLEVEL, isoflags: u32, potheroptions: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<super::ITransactionOptions>,
    {
        unsafe { (windows_core::Interface::vtable(self).JoinTransaction)(windows_core::Interface::as_raw(self), punktransactioncoord.param().abi(), isolevel, isoflags, potheroptions.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionJoin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "transact")]
    pub GetOptionsObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    GetOptionsObject: usize,
    #[cfg(feature = "transact")]
    pub JoinTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::ISOLEVEL, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    JoinTransaction: usize,
}
#[cfg(feature = "transact")]
pub trait ITransactionJoin_Impl: windows_core::IUnknownImpl {
    fn GetOptionsObject(&self) -> windows_core::Result<super::ITransactionOptions>;
    fn JoinTransaction(&self, punktransactioncoord: windows_core::Ref<windows_core::IUnknown>, isolevel: super::ISOLEVEL, isoflags: u32, potheroptions: windows_core::Ref<super::ITransactionOptions>) -> windows_core::Result<()>;
}
#[cfg(feature = "transact")]
impl ITransactionJoin_Vtbl {
    pub const fn new<Identity: ITransactionJoin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionsObject<Identity: ITransactionJoin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionJoin_Impl::GetOptionsObject(this) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn JoinTransaction<Identity: ITransactionJoin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punktransactioncoord: *mut core::ffi::c_void, isolevel: super::ISOLEVEL, isoflags: u32, potheroptions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionJoin_Impl::JoinTransaction(this, core::mem::transmute_copy(&punktransactioncoord), core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), core::mem::transmute_copy(&potheroptions)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, OFFSET>,
            JoinTransaction: JoinTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionJoin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "transact")]
impl windows_core::RuntimeName for ITransactionJoin {}
#[cfg(feature = "transact")]
windows_core::imp::define_interface!(ITransactionLocal, ITransactionLocal_Vtbl, 0x0c733a5f_2a1c_11ce_ade5_00aa0044773d);
#[cfg(feature = "transact")]
impl core::ops::Deref for ITransactionLocal {
    type Target = super::ITransaction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "transact")]
windows_core::imp::interface_hierarchy!(ITransactionLocal, windows_core::IUnknown, super::ITransaction);
#[cfg(feature = "transact")]
impl ITransactionLocal {
    pub unsafe fn GetOptionsObject(&self) -> windows_core::Result<super::ITransactionOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionsObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn StartTransaction<P2>(&self, isolevel: super::ISOLEVEL, isoflags: u32, potheroptions: P2, pultransactionlevel: Option<*mut u32>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::ITransactionOptions>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartTransaction)(windows_core::Interface::as_raw(self), isolevel, isoflags, potheroptions.param().abi(), pultransactionlevel.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[cfg(feature = "transact")]
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLocal_Vtbl {
    pub base__: super::ITransaction_Vtbl,
    pub GetOptionsObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, super::ISOLEVEL, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "transact"))]
pub trait ITransactionLocal_Impl: super::ITransaction_Impl {
    fn GetOptionsObject(&self) -> windows_core::Result<super::ITransactionOptions>;
    fn StartTransaction(&self, isolevel: super::ISOLEVEL, isoflags: u32, potheroptions: windows_core::Ref<super::ITransactionOptions>, pultransactionlevel: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "transact"))]
impl ITransactionLocal_Vtbl {
    pub const fn new<Identity: ITransactionLocal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionsObject<Identity: ITransactionLocal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionLocal_Impl::GetOptionsObject(this) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartTransaction<Identity: ITransactionLocal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isolevel: super::ISOLEVEL, isoflags: u32, potheroptions: *mut core::ffi::c_void, pultransactionlevel: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionLocal_Impl::StartTransaction(this, core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), core::mem::transmute_copy(&potheroptions), core::mem::transmute_copy(&pultransactionlevel)).into()
            }
        }
        Self {
            base__: super::ITransaction_Vtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, OFFSET>,
            StartTransaction: StartTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionLocal as windows_core::Interface>::IID || iid == &<super::ITransaction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionLocal {}
windows_core::imp::define_interface!(ITransactionObject, ITransactionObject_Vtbl, 0x0c733a60_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ITransactionObject, windows_core::IUnknown);
impl ITransactionObject {
    #[cfg(feature = "transact")]
    pub unsafe fn GetTransactionObject(&self, ultransactionlevel: u32) -> windows_core::Result<super::ITransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransactionObject)(windows_core::Interface::as_raw(self), ultransactionlevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "transact")]
    pub GetTransactionObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    GetTransactionObject: usize,
}
#[cfg(feature = "transact")]
pub trait ITransactionObject_Impl: windows_core::IUnknownImpl {
    fn GetTransactionObject(&self, ultransactionlevel: u32) -> windows_core::Result<super::ITransaction>;
}
#[cfg(feature = "transact")]
impl ITransactionObject_Vtbl {
    pub const fn new<Identity: ITransactionObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransactionObject<Identity: ITransactionObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultransactionlevel: u32, pptransactionobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionObject_Impl::GetTransactionObject(this, core::mem::transmute_copy(&ultransactionlevel)) {
                    Ok(ok__) => {
                        pptransactionobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTransactionObject: GetTransactionObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "transact")]
impl windows_core::RuntimeName for ITransactionObject {}
windows_core::imp::define_interface!(ITrusteeAdmin, ITrusteeAdmin_Vtbl, 0x0c733aa1_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ITrusteeAdmin, windows_core::IUnknown);
impl ITrusteeAdmin {
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn CompareTrustees(&self, ptrustee1: *const super::TRUSTEE_W, ptrustee2: *const super::TRUSTEE_W) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CompareTrustees)(windows_core::Interface::as_raw(self), ptrustee1, ptrustee2) }
    }
    #[cfg(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreateTrustee(&self, ptrustee: *const super::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTrustee)(windows_core::Interface::as_raw(self), ptrustee, cpropertysets, rgpropertysets as _) }
    }
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn DeleteTrustee(&self, ptrustee: *const super::TRUSTEE_W) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteTrustee)(windows_core::Interface::as_raw(self), ptrustee) }
    }
    #[cfg(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetTrusteeProperties(&self, ptrustee: *const super::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTrusteeProperties)(windows_core::Interface::as_raw(self), ptrustee, cpropertysets, rgpropertysets as _) }
    }
    #[cfg(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetTrusteeProperties(&self, ptrustee: *const super::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTrusteeProperties)(windows_core::Interface::as_raw(self), ptrustee, cpropertyidsets, rgpropertyidsets, pcpropertysets as _, prgpropertysets as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrusteeAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub CompareTrustees: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W, *const super::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    CompareTrustees: usize,
    #[cfg(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub CreateTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    CreateTrustee: usize,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub DeleteTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    DeleteTrustee: usize,
    #[cfg(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetTrusteeProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetTrusteeProperties: usize,
    #[cfg(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetTrusteeProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetTrusteeProperties: usize,
}
#[cfg(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITrusteeAdmin_Impl: windows_core::IUnknownImpl {
    fn CompareTrustees(&self, ptrustee1: *const super::TRUSTEE_W, ptrustee2: *const super::TRUSTEE_W) -> windows_core::Result<()>;
    fn CreateTrustee(&self, ptrustee: *const super::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
    fn DeleteTrustee(&self, ptrustee: *const super::TRUSTEE_W) -> windows_core::Result<()>;
    fn SetTrusteeProperties(&self, ptrustee: *const super::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
    fn GetTrusteeProperties(&self, ptrustee: *const super::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITrusteeAdmin_Vtbl {
    pub const fn new<Identity: ITrusteeAdmin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompareTrustees<Identity: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee1: *const super::TRUSTEE_W, ptrustee2: *const super::TRUSTEE_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrusteeAdmin_Impl::CompareTrustees(this, core::mem::transmute_copy(&ptrustee1), core::mem::transmute_copy(&ptrustee2)).into()
            }
        }
        unsafe extern "system" fn CreateTrustee<Identity: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrusteeAdmin_Impl::CreateTrustee(this, core::mem::transmute_copy(&ptrustee), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
            }
        }
        unsafe extern "system" fn DeleteTrustee<Identity: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::TRUSTEE_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrusteeAdmin_Impl::DeleteTrustee(this, core::mem::transmute_copy(&ptrustee)).into()
            }
        }
        unsafe extern "system" fn SetTrusteeProperties<Identity: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrusteeAdmin_Impl::SetTrusteeProperties(this, core::mem::transmute_copy(&ptrustee), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
            }
        }
        unsafe extern "system" fn GetTrusteeProperties<Identity: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrusteeAdmin_Impl::GetTrusteeProperties(this, core::mem::transmute_copy(&ptrustee), core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompareTrustees: CompareTrustees::<Identity, OFFSET>,
            CreateTrustee: CreateTrustee::<Identity, OFFSET>,
            DeleteTrustee: DeleteTrustee::<Identity, OFFSET>,
            SetTrusteeProperties: SetTrusteeProperties::<Identity, OFFSET>,
            GetTrusteeProperties: GetTrusteeProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrusteeAdmin as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "accctrl", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITrusteeAdmin {}
windows_core::imp::define_interface!(ITrusteeGroupAdmin, ITrusteeGroupAdmin_Vtbl, 0x0c733aa2_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ITrusteeGroupAdmin, windows_core::IUnknown);
impl ITrusteeGroupAdmin {
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn AddMember(&self, pmembershiptrustee: *const super::TRUSTEE_W, pmembertrustee: *const super::TRUSTEE_W) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddMember)(windows_core::Interface::as_raw(self), pmembershiptrustee, pmembertrustee) }
    }
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn DeleteMember(&self, pmembershiptrustee: *const super::TRUSTEE_W, pmembertrustee: *const super::TRUSTEE_W) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteMember)(windows_core::Interface::as_raw(self), pmembershiptrustee, pmembertrustee) }
    }
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn IsMember(&self, pmembershiptrustee: *const super::TRUSTEE_W, pmembertrustee: *const super::TRUSTEE_W) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMember)(windows_core::Interface::as_raw(self), pmembershiptrustee, pmembertrustee, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn GetMembers(&self, pmembershiptrustee: *const super::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMembers)(windows_core::Interface::as_raw(self), pmembershiptrustee, pcmembers as _, prgmembers as _) }
    }
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub unsafe fn GetMemberships(&self, ptrustee: *const super::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMemberships)(windows_core::Interface::as_raw(self), ptrustee, pcmemberships as _, prgmemberships as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrusteeGroupAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub AddMember: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W, *const super::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    AddMember: usize,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub DeleteMember: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W, *const super::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    DeleteMember: usize,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub IsMember: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W, *const super::TRUSTEE_W, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    IsMember: usize,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub GetMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W, *mut u32, *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    GetMembers: usize,
    #[cfg(all(feature = "accctrl", feature = "winnt"))]
    pub GetMemberships: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TRUSTEE_W, *mut u32, *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "accctrl", feature = "winnt")))]
    GetMemberships: usize,
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
pub trait ITrusteeGroupAdmin_Impl: windows_core::IUnknownImpl {
    fn AddMember(&self, pmembershiptrustee: *const super::TRUSTEE_W, pmembertrustee: *const super::TRUSTEE_W) -> windows_core::Result<()>;
    fn DeleteMember(&self, pmembershiptrustee: *const super::TRUSTEE_W, pmembertrustee: *const super::TRUSTEE_W) -> windows_core::Result<()>;
    fn IsMember(&self, pmembershiptrustee: *const super::TRUSTEE_W, pmembertrustee: *const super::TRUSTEE_W) -> windows_core::Result<windows_core::BOOL>;
    fn GetMembers(&self, pmembershiptrustee: *const super::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::TRUSTEE_W) -> windows_core::Result<()>;
    fn GetMemberships(&self, ptrustee: *const super::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::TRUSTEE_W) -> windows_core::Result<()>;
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
impl ITrusteeGroupAdmin_Vtbl {
    pub const fn new<Identity: ITrusteeGroupAdmin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddMember<Identity: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmembershiptrustee: *const super::TRUSTEE_W, pmembertrustee: *const super::TRUSTEE_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrusteeGroupAdmin_Impl::AddMember(this, core::mem::transmute_copy(&pmembershiptrustee), core::mem::transmute_copy(&pmembertrustee)).into()
            }
        }
        unsafe extern "system" fn DeleteMember<Identity: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmembershiptrustee: *const super::TRUSTEE_W, pmembertrustee: *const super::TRUSTEE_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrusteeGroupAdmin_Impl::DeleteMember(this, core::mem::transmute_copy(&pmembershiptrustee), core::mem::transmute_copy(&pmembertrustee)).into()
            }
        }
        unsafe extern "system" fn IsMember<Identity: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmembershiptrustee: *const super::TRUSTEE_W, pmembertrustee: *const super::TRUSTEE_W, pfstatus: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrusteeGroupAdmin_Impl::IsMember(this, core::mem::transmute_copy(&pmembershiptrustee), core::mem::transmute_copy(&pmembertrustee)) {
                    Ok(ok__) => {
                        pfstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMembers<Identity: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmembershiptrustee: *const super::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrusteeGroupAdmin_Impl::GetMembers(this, core::mem::transmute_copy(&pmembershiptrustee), core::mem::transmute_copy(&pcmembers), core::mem::transmute_copy(&prgmembers)).into()
            }
        }
        unsafe extern "system" fn GetMemberships<Identity: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::TRUSTEE_W) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrusteeGroupAdmin_Impl::GetMemberships(this, core::mem::transmute_copy(&ptrustee), core::mem::transmute_copy(&pcmemberships), core::mem::transmute_copy(&prgmemberships)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddMember: AddMember::<Identity, OFFSET>,
            DeleteMember: DeleteMember::<Identity, OFFSET>,
            IsMember: IsMember::<Identity, OFFSET>,
            GetMembers: GetMembers::<Identity, OFFSET>,
            GetMemberships: GetMemberships::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrusteeGroupAdmin as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
impl windows_core::RuntimeName for ITrusteeGroupAdmin {}
windows_core::imp::define_interface!(IViewChapter, IViewChapter_Vtbl, 0x0c733a98_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IViewChapter, windows_core::IUnknown);
impl IViewChapter {
    pub unsafe fn GetSpecification<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSpecification)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn OpenViewChapter(&self, hsource: HCHAPTER, phviewchapter: Option<*mut HCHAPTER>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenViewChapter)(windows_core::Interface::as_raw(self), hsource, phviewchapter.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewChapter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSpecification: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenViewChapter: unsafe extern "system" fn(*mut core::ffi::c_void, HCHAPTER, *mut HCHAPTER) -> windows_core::HRESULT,
}
pub trait IViewChapter_Impl: windows_core::IUnknownImpl {
    fn GetSpecification(&self, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenViewChapter(&self, hsource: HCHAPTER, phviewchapter: *mut HCHAPTER) -> windows_core::Result<()>;
}
impl IViewChapter_Vtbl {
    pub const fn new<Identity: IViewChapter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSpecification<Identity: IViewChapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewChapter_Impl::GetSpecification(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        unsafe extern "system" fn OpenViewChapter<Identity: IViewChapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsource: HCHAPTER, phviewchapter: *mut HCHAPTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewChapter_Impl::OpenViewChapter(this, core::mem::transmute_copy(&hsource), core::mem::transmute_copy(&phviewchapter)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSpecification: GetSpecification::<Identity, OFFSET>,
            OpenViewChapter: OpenViewChapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewChapter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IViewChapter {}
windows_core::imp::define_interface!(IViewFilter, IViewFilter_Vtbl, 0x0c733a9b_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IViewFilter, windows_core::IUnknown);
impl IViewFilter {
    pub unsafe fn GetFilter(&self, haccessor: HACCESSOR, pcrows: *mut DBCOUNTITEM, pcompareops: *mut *mut DBCOMPAREOP, pcriteriadata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFilter)(windows_core::Interface::as_raw(self), haccessor, pcrows as _, pcompareops as _, pcriteriadata as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetFilterBindings(&self, pcbindings: *mut DBCOUNTITEM, prgbindings: *mut *mut DBBINDING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFilterBindings)(windows_core::Interface::as_raw(self), pcbindings as _, prgbindings as _) }
    }
    pub unsafe fn SetFilter(&self, haccessor: HACCESSOR, compareops: &[DBCOMPAREOP], pcriteriadata: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), haccessor, DBCOUNTITEM(compareops.len().try_into().unwrap()), compareops.as_ptr(), pcriteriadata) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, *mut DBCOUNTITEM, *mut *mut DBCOMPAREOP, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetFilterBindings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DBCOUNTITEM, *mut *mut DBBINDING) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetFilterBindings: usize,
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, DBCOUNTITEM, *const DBCOMPAREOP, *const core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IViewFilter_Impl: windows_core::IUnknownImpl {
    fn GetFilter(&self, haccessor: HACCESSOR, pcrows: *mut DBCOUNTITEM, pcompareops: *mut *mut DBCOMPAREOP, pcriteriadata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetFilterBindings(&self, pcbindings: *mut DBCOUNTITEM, prgbindings: *mut *mut DBBINDING) -> windows_core::Result<()>;
    fn SetFilter(&self, haccessor: HACCESSOR, crows: DBCOUNTITEM, compareops: *const DBCOMPAREOP, pcriteriadata: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IViewFilter_Vtbl {
    pub const fn new<Identity: IViewFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFilter<Identity: IViewFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, pcrows: *mut DBCOUNTITEM, pcompareops: *mut *mut DBCOMPAREOP, pcriteriadata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewFilter_Impl::GetFilter(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pcrows), core::mem::transmute_copy(&pcompareops), core::mem::transmute_copy(&pcriteriadata)).into()
            }
        }
        unsafe extern "system" fn GetFilterBindings<Identity: IViewFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbindings: *mut DBCOUNTITEM, prgbindings: *mut *mut DBBINDING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewFilter_Impl::GetFilterBindings(this, core::mem::transmute_copy(&pcbindings), core::mem::transmute_copy(&prgbindings)).into()
            }
        }
        unsafe extern "system" fn SetFilter<Identity: IViewFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, crows: DBCOUNTITEM, compareops: *const DBCOMPAREOP, pcriteriadata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewFilter_Impl::SetFilter(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&compareops), core::mem::transmute_copy(&pcriteriadata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilter: GetFilter::<Identity, OFFSET>,
            GetFilterBindings: GetFilterBindings::<Identity, OFFSET>,
            SetFilter: SetFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IViewFilter {}
windows_core::imp::define_interface!(IViewRowset, IViewRowset_Vtbl, 0x0c733a97_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IViewRowset, windows_core::IUnknown);
impl IViewRowset {
    pub unsafe fn GetSpecification<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSpecification)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn OpenViewRowset<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OpenViewRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSpecification: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenViewRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IViewRowset_Impl: windows_core::IUnknownImpl {
    fn GetSpecification(&self, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenViewRowset(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IViewRowset_Vtbl {
    pub const fn new<Identity: IViewRowset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSpecification<Identity: IViewRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewRowset_Impl::GetSpecification(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn OpenViewRowset<Identity: IViewRowset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewRowset_Impl::OpenViewRowset(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pprowset)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSpecification: GetSpecification::<Identity, OFFSET>,
            OpenViewRowset: OpenViewRowset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewRowset as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IViewRowset {}
windows_core::imp::define_interface!(IViewSort, IViewSort_Vtbl, 0x0c733a9a_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(IViewSort, windows_core::IUnknown);
impl IViewSort {
    pub unsafe fn GetSortOrder(&self, pcvalues: *mut DBORDINAL, prgcolumns: *mut *mut DBORDINAL, prgorders: *mut *mut DBSORT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSortOrder)(windows_core::Interface::as_raw(self), pcvalues as _, prgcolumns as _, prgorders as _) }
    }
    pub unsafe fn SetSortOrder(&self, cvalues: DBORDINAL, rgcolumns: *const DBORDINAL, rgorders: *const DBSORT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSortOrder)(windows_core::Interface::as_raw(self), cvalues, rgcolumns, rgorders) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSort_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSortOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DBORDINAL, *mut *mut DBORDINAL, *mut *mut DBSORT) -> windows_core::HRESULT,
    pub SetSortOrder: unsafe extern "system" fn(*mut core::ffi::c_void, DBORDINAL, *const DBORDINAL, *const DBSORT) -> windows_core::HRESULT,
}
pub trait IViewSort_Impl: windows_core::IUnknownImpl {
    fn GetSortOrder(&self, pcvalues: *mut DBORDINAL, prgcolumns: *mut *mut DBORDINAL, prgorders: *mut *mut DBSORT) -> windows_core::Result<()>;
    fn SetSortOrder(&self, cvalues: DBORDINAL, rgcolumns: *const DBORDINAL, rgorders: *const DBSORT) -> windows_core::Result<()>;
}
impl IViewSort_Vtbl {
    pub const fn new<Identity: IViewSort_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSortOrder<Identity: IViewSort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcvalues: *mut DBORDINAL, prgcolumns: *mut *mut DBORDINAL, prgorders: *mut *mut DBSORT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewSort_Impl::GetSortOrder(this, core::mem::transmute_copy(&pcvalues), core::mem::transmute_copy(&prgcolumns), core::mem::transmute_copy(&prgorders)).into()
            }
        }
        unsafe extern "system" fn SetSortOrder<Identity: IViewSort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cvalues: DBORDINAL, rgcolumns: *const DBORDINAL, rgorders: *const DBSORT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewSort_Impl::SetSortOrder(this, core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&rgcolumns), core::mem::transmute_copy(&rgorders)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSortOrder: GetSortOrder::<Identity, OFFSET>,
            SetSortOrder: SetSortOrder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewSort as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IViewSort {}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct MDAXISINFO {
    pub cbSize: DBLENGTH,
    pub iAxis: DBCOUNTITEM,
    pub cDimensions: DBCOUNTITEM,
    pub cCoordinates: DBCOUNTITEM,
    pub rgcColumns: *mut DBORDINAL,
    pub rgpwszDimensionNames: *mut windows_core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for MDAXISINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MDAXISINFO {
    pub cbSize: DBLENGTH,
    pub iAxis: DBCOUNTITEM,
    pub cDimensions: DBCOUNTITEM,
    pub cCoordinates: DBCOUNTITEM,
    pub rgcColumns: *mut DBORDINAL,
    pub rgpwszDimensionNames: *mut windows_core::PWSTR,
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
pub struct RMTPACK {
    pub pISeqStream: core::mem::ManuallyDrop<Option<super::ISequentialStream>>,
    pub cbData: u32,
    pub cBSTR: u32,
    pub rgBSTR: *mut windows_core::BSTR,
    pub cVARIANT: u32,
    pub rgVARIANT: *mut super::VARIANT,
    pub cIDISPATCH: u32,
    pub rgIDISPATCH: *mut Option<super::IDispatch>,
    pub cIUNKNOWN: u32,
    pub rgIUNKNOWN: *mut Option<windows_core::IUnknown>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RMTPACK {
    pub pISeqStream: core::mem::ManuallyDrop<Option<super::ISequentialStream>>,
    pub cbData: u32,
    pub cBSTR: u32,
    pub rgBSTR: *mut windows_core::BSTR,
    pub cVARIANT: u32,
    pub rgVARIANT: *mut super::VARIANT,
    pub cIDISPATCH: u32,
    pub rgIDISPATCH: *mut Option<super::IDispatch>,
    pub cIUNKNOWN: u32,
    pub rgIUNKNOWN: *mut Option<windows_core::IUnknown>,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SBYTE(pub i8);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub guidObjectType: windows_core::GUID,
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
    pub guidObjectType: windows_core::GUID,
    pub ObjectID: DBID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SEC_OBJECT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STD_BOOKMARKLENGTH: u32 = 1;
