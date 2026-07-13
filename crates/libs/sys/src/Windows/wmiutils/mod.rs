#[repr(C)]
#[derive(Clone, Copy)]
pub struct SWbemAnalysisMatrix {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_pszProperty: windows_sys::core::PCWSTR,
    pub m_uPropertyType: u32,
    pub m_uEntries: u32,
    pub m_pValues: *mut *mut core::ffi::c_void,
    pub m_pbTruthTable: *mut windows_sys::core::BOOL,
}
impl Default for SWbemAnalysisMatrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SWbemAnalysisMatrixList {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_uNumMatrices: u32,
    pub m_pMatrices: *mut SWbemAnalysisMatrix,
}
impl Default for SWbemAnalysisMatrixList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SWbemAssocQueryInf {
    pub m_uVersion: u32,
    pub m_uAnalysisType: u32,
    pub m_uFeatureMask: u32,
    pub m_pPath: *mut core::ffi::c_void,
    pub m_pszPath: windows_sys::core::PWSTR,
    pub m_pszQueryText: windows_sys::core::PWSTR,
    pub m_pszResultClass: windows_sys::core::PWSTR,
    pub m_pszAssocClass: windows_sys::core::PWSTR,
    pub m_pszRole: windows_sys::core::PWSTR,
    pub m_pszResultRole: windows_sys::core::PWSTR,
    pub m_pszRequiredQualifier: windows_sys::core::PWSTR,
    pub m_pszRequiredAssocQualifier: windows_sys::core::PWSTR,
}
impl Default for SWbemAssocQueryInf {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SWbemQueryQualifiedName {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNameListSize: u32,
    pub m_ppszNameList: *mut windows_sys::core::PCWSTR,
    pub m_bArraysUsed: windows_sys::core::BOOL,
    pub m_pbArrayElUsed: *mut windows_sys::core::BOOL,
    pub m_puArrayIndex: *mut u32,
}
impl Default for SWbemQueryQualifiedName {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SWbemRpnConst {
    pub m_pszStrVal: windows_sys::core::PCWSTR,
    pub m_bBoolVal: windows_sys::core::BOOL,
    pub m_lLongVal: i32,
    pub m_uLongVal: u32,
    pub m_dblVal: f64,
    pub m_lVal64: i64,
    pub m_uVal64: i64,
}
impl Default for SWbemRpnConst {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SWbemRpnEncodedQuery {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uParsedFeatureMask: u64,
    pub m_uDetectedArraySize: u32,
    pub m_puDetectedFeatures: *mut u32,
    pub m_uSelectListSize: u32,
    pub m_ppSelectList: *mut *mut SWbemQueryQualifiedName,
    pub m_uFromTargetType: u32,
    pub m_pszOptionalFromPath: windows_sys::core::PCWSTR,
    pub m_uFromListSize: u32,
    pub m_ppszFromList: *mut windows_sys::core::PCWSTR,
    pub m_uWhereClauseSize: u32,
    pub m_ppRpnWhereClause: *mut *mut SWbemRpnQueryToken,
    pub m_dblWithinPolling: f64,
    pub m_dblWithinWindow: f64,
    pub m_uOrderByListSize: u32,
    pub m_ppszOrderByList: *mut windows_sys::core::PCWSTR,
    pub m_uOrderDirectionEl: *mut u32,
}
impl Default for SWbemRpnEncodedQuery {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SWbemRpnQueryToken {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uSubexpressionShape: u32,
    pub m_uOperator: u32,
    pub m_pRightIdent: *mut SWbemQueryQualifiedName,
    pub m_pLeftIdent: *mut SWbemQueryQualifiedName,
    pub m_uConstApparentType: u32,
    pub m_Const: SWbemRpnConst,
    pub m_uConst2ApparentType: u32,
    pub m_Const2: SWbemRpnConst,
    pub m_pszRightFunc: windows_sys::core::PCWSTR,
    pub m_pszLeftFunc: windows_sys::core::PCWSTR,
}
impl Default for SWbemRpnQueryToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SWbemRpnTokenList {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNumTokens: u32,
}
pub const WBEMPATH_COMPRESSED: tag_WBEM_GET_TEXT_FLAGS = 1;
pub const WBEMPATH_CREATE_ACCEPT_ABSOLUTE: tag_WBEM_PATH_CREATE_FLAG = 2;
pub const WBEMPATH_CREATE_ACCEPT_ALL: tag_WBEM_PATH_CREATE_FLAG = 4;
pub const WBEMPATH_CREATE_ACCEPT_RELATIVE: tag_WBEM_PATH_CREATE_FLAG = 1;
pub const WBEMPATH_GET_NAMESPACE_ONLY: tag_WBEM_GET_TEXT_FLAGS = 16;
pub const WBEMPATH_GET_ORIGINAL: tag_WBEM_GET_TEXT_FLAGS = 32;
pub const WBEMPATH_GET_RELATIVE_ONLY: tag_WBEM_GET_TEXT_FLAGS = 2;
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: tag_WBEM_GET_TEXT_FLAGS = 8;
pub const WBEMPATH_GET_SERVER_TOO: tag_WBEM_GET_TEXT_FLAGS = 4;
pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: tag_WBEM_PATH_STATUS_FLAG = 1;
pub const WBEMPATH_INFO_CIM_COMPLIANT: tag_WBEM_PATH_STATUS_FLAG = 2048;
pub const WBEMPATH_INFO_CONTAINS_SINGLETON: tag_WBEM_PATH_STATUS_FLAG = 256;
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: tag_WBEM_PATH_STATUS_FLAG = 128;
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: tag_WBEM_PATH_STATUS_FLAG = 2;
pub const WBEMPATH_INFO_HAS_SUBSCOPES: tag_WBEM_PATH_STATUS_FLAG = 16;
pub const WBEMPATH_INFO_HAS_V2_REF_PATHS: tag_WBEM_PATH_STATUS_FLAG = 64;
pub const WBEMPATH_INFO_IS_CLASS_REF: tag_WBEM_PATH_STATUS_FLAG = 4;
pub const WBEMPATH_INFO_IS_COMPOUND: tag_WBEM_PATH_STATUS_FLAG = 32;
pub const WBEMPATH_INFO_IS_INST_REF: tag_WBEM_PATH_STATUS_FLAG = 8;
pub const WBEMPATH_INFO_IS_PARENT: tag_WBEM_PATH_STATUS_FLAG = 8192;
pub const WBEMPATH_INFO_IS_SINGLETON: tag_WBEM_PATH_STATUS_FLAG = 4096;
pub const WBEMPATH_INFO_NATIVE_PATH: tag_WBEM_PATH_STATUS_FLAG = 32768;
pub const WBEMPATH_INFO_PATH_HAD_SERVER: tag_WBEM_PATH_STATUS_FLAG = 131072;
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: tag_WBEM_PATH_STATUS_FLAG = 16384;
pub const WBEMPATH_INFO_V1_COMPLIANT: tag_WBEM_PATH_STATUS_FLAG = 512;
pub const WBEMPATH_INFO_V2_COMPLIANT: tag_WBEM_PATH_STATUS_FLAG = 1024;
pub const WBEMPATH_INFO_WMI_PATH: tag_WBEM_PATH_STATUS_FLAG = 65536;
pub const WBEMPATH_QUOTEDTEXT: tag_WBEM_GET_KEY_FLAGS = 2;
pub const WBEMPATH_TEXT: tag_WBEM_GET_KEY_FLAGS = 1;
pub const WBEMPATH_TREAT_SINGLE_IDENT_AS_NS: tag_WBEM_PATH_CREATE_FLAG = 8;
pub const WMIQ_ANALYSIS_ASSOC_QUERY: WMIQ_ANALYSIS_TYPE = 2;
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: WMIQ_ANALYSIS_TYPE = 3;
pub const WMIQ_ANALYSIS_QUERY_TEXT: WMIQ_ANALYSIS_TYPE = 4;
pub const WMIQ_ANALYSIS_RESERVED: WMIQ_ANALYSIS_TYPE = 134217728;
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: WMIQ_ANALYSIS_TYPE = 1;
pub type WMIQ_ANALYSIS_TYPE = i32;
pub const WMIQ_ASSOCQ_ASSOCCLASS: WMIQ_ASSOCQ_FLAGS = 8;
pub const WMIQ_ASSOCQ_ASSOCIATORS: WMIQ_ASSOCQ_FLAGS = 1;
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: WMIQ_ASSOCQ_FLAGS = 256;
pub const WMIQ_ASSOCQ_CLASSREFSONLY: WMIQ_ASSOCQ_FLAGS = 2048;
pub type WMIQ_ASSOCQ_FLAGS = i32;
pub const WMIQ_ASSOCQ_KEYSONLY: WMIQ_ASSOCQ_FLAGS = 512;
pub const WMIQ_ASSOCQ_REFERENCES: WMIQ_ASSOCQ_FLAGS = 2;
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: WMIQ_ASSOCQ_FLAGS = 128;
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: WMIQ_ASSOCQ_FLAGS = 64;
pub const WMIQ_ASSOCQ_RESULTCLASS: WMIQ_ASSOCQ_FLAGS = 4;
pub const WMIQ_ASSOCQ_RESULTROLE: WMIQ_ASSOCQ_FLAGS = 32;
pub const WMIQ_ASSOCQ_ROLE: WMIQ_ASSOCQ_FLAGS = 16;
pub const WMIQ_ASSOCQ_SCHEMAONLY: WMIQ_ASSOCQ_FLAGS = 1024;
pub type WMIQ_LANGUAGE_FEATURES = i32;
pub const WMIQ_LF10_COMPEX_SUBEXPRESSIONS: WMIQ_LANGUAGE_FEATURES = 10;
pub const WMIQ_LF11_ALIASING: WMIQ_LANGUAGE_FEATURES = 11;
pub const WMIQ_LF12_GROUP_BY_HAVING: WMIQ_LANGUAGE_FEATURES = 12;
pub const WMIQ_LF13_WMI_WITHIN: WMIQ_LANGUAGE_FEATURES = 13;
pub const WMIQ_LF14_SQL_WRITE_OPERATIONS: WMIQ_LANGUAGE_FEATURES = 14;
pub const WMIQ_LF15_GO: WMIQ_LANGUAGE_FEATURES = 15;
pub const WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS: WMIQ_LANGUAGE_FEATURES = 16;
pub const WMIQ_LF17_QUALIFIED_NAMES: WMIQ_LANGUAGE_FEATURES = 17;
pub const WMIQ_LF18_ASSOCIATONS: WMIQ_LANGUAGE_FEATURES = 18;
pub const WMIQ_LF19_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = 19;
pub const WMIQ_LF1_BASIC_SELECT: WMIQ_LANGUAGE_FEATURES = 1;
pub const WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = 20;
pub const WMIQ_LF21_SQL89_JOINS: WMIQ_LANGUAGE_FEATURES = 21;
pub const WMIQ_LF22_SQL92_JOINS: WMIQ_LANGUAGE_FEATURES = 22;
pub const WMIQ_LF23_SUBSELECTS: WMIQ_LANGUAGE_FEATURES = 23;
pub const WMIQ_LF24_UMI_EXTENSIONS: WMIQ_LANGUAGE_FEATURES = 24;
pub const WMIQ_LF25_DATEPART: WMIQ_LANGUAGE_FEATURES = 25;
pub const WMIQ_LF26_LIKE: WMIQ_LANGUAGE_FEATURES = 26;
pub const WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS: WMIQ_LANGUAGE_FEATURES = 27;
pub const WMIQ_LF28_STANDARD_AGGREGATES: WMIQ_LANGUAGE_FEATURES = 28;
pub const WMIQ_LF29_MULTI_LEVEL_ORDER_BY: WMIQ_LANGUAGE_FEATURES = 29;
pub const WMIQ_LF2_CLASS_NAME_IN_QUERY: WMIQ_LANGUAGE_FEATURES = 2;
pub const WMIQ_LF30_WMI_PRAGMAS: WMIQ_LANGUAGE_FEATURES = 30;
pub const WMIQ_LF31_QUALIFIER_TESTS: WMIQ_LANGUAGE_FEATURES = 31;
pub const WMIQ_LF32_SP_EXECUTE: WMIQ_LANGUAGE_FEATURES = 32;
pub const WMIQ_LF33_ARRAY_ACCESS: WMIQ_LANGUAGE_FEATURES = 33;
pub const WMIQ_LF34_UNION: WMIQ_LANGUAGE_FEATURES = 34;
pub const WMIQ_LF35_COMPLEX_SELECT_TARGET: WMIQ_LANGUAGE_FEATURES = 35;
pub const WMIQ_LF36_REFERENCE_TESTS: WMIQ_LANGUAGE_FEATURES = 36;
pub const WMIQ_LF37_SELECT_INTO: WMIQ_LANGUAGE_FEATURES = 37;
pub const WMIQ_LF38_BASIC_DATETIME_TESTS: WMIQ_LANGUAGE_FEATURES = 38;
pub const WMIQ_LF39_COUNT_COLUMN: WMIQ_LANGUAGE_FEATURES = 39;
pub const WMIQ_LF3_STRING_CASE_FUNCTIONS: WMIQ_LANGUAGE_FEATURES = 3;
pub const WMIQ_LF40_BETWEEN: WMIQ_LANGUAGE_FEATURES = 40;
pub const WMIQ_LF4_PROP_TO_PROP_TESTS: WMIQ_LANGUAGE_FEATURES = 4;
pub const WMIQ_LF5_COUNT_STAR: WMIQ_LANGUAGE_FEATURES = 5;
pub const WMIQ_LF6_ORDER_BY: WMIQ_LANGUAGE_FEATURES = 6;
pub const WMIQ_LF7_DISTINCT: WMIQ_LANGUAGE_FEATURES = 7;
pub const WMIQ_LF8_ISA: WMIQ_LANGUAGE_FEATURES = 8;
pub const WMIQ_LF9_THIS: WMIQ_LANGUAGE_FEATURES = 9;
pub const WMIQ_LF_LAST: WMIQ_LANGUAGE_FEATURES = 40;
pub const WMIQ_RPNF_ARRAY_ACCESS_USED: WMIQ_RPNF_FEATURE = 8192;
pub const WMIQ_RPNF_COUNT_STAR: WMIQ_RPNF_FEATURE = 64;
pub const WMIQ_RPNF_EQUALITY_TESTS_ONLY: WMIQ_RPNF_FEATURE = 32;
pub type WMIQ_RPNF_FEATURE = i32;
pub const WMIQ_RPNF_FEATURE_SELECT_STAR: WMIQ_RPNF_FEATURE = 16;
pub const WMIQ_RPNF_GROUP_BY_HAVING: WMIQ_RPNF_FEATURE = 4096;
pub const WMIQ_RPNF_ISA_USED: WMIQ_RPNF_FEATURE = 2048;
pub const WMIQ_RPNF_ORDER_BY: WMIQ_RPNF_FEATURE = 1024;
pub const WMIQ_RPNF_PROJECTION: WMIQ_RPNF_FEATURE = 8;
pub const WMIQ_RPNF_PROP_TO_PROP_TESTS: WMIQ_RPNF_FEATURE = 512;
pub const WMIQ_RPNF_QUALIFIED_NAMES_USED: WMIQ_RPNF_FEATURE = 128;
pub const WMIQ_RPNF_QUERY_IS_CONJUNCTIVE: WMIQ_RPNF_FEATURE = 2;
pub const WMIQ_RPNF_QUERY_IS_DISJUNCTIVE: WMIQ_RPNF_FEATURE = 4;
pub const WMIQ_RPNF_SYSPROP_CLASS_USED: WMIQ_RPNF_FEATURE = 256;
pub const WMIQ_RPNF_WHERE_CLAUSE_PRESENT: WMIQ_RPNF_FEATURE = 1;
pub const WMIQ_RPN_CONST: WMIQ_RPN_TOKEN_FLAGS = 8;
pub const WMIQ_RPN_CONST2: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_FROM_CLASS_LIST: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_FROM_MULTIPLE: WMIQ_RPN_TOKEN_FLAGS = 8;
pub const WMIQ_RPN_FROM_PATH: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_FROM_UNARY: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_GET_EXPR_SHAPE: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_GET_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 3;
pub const WMIQ_RPN_GET_RELOP: WMIQ_RPN_TOKEN_FLAGS = 5;
pub const WMIQ_RPN_GET_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_GET_TOKEN_TYPE: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 32;
pub const WMIQ_RPN_LEFT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_NEXT_TOKEN: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_OP_EQ: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_OP_GE: WMIQ_RPN_TOKEN_FLAGS = 3;
pub const WMIQ_RPN_OP_GT: WMIQ_RPN_TOKEN_FLAGS = 6;
pub const WMIQ_RPN_OP_ISA: WMIQ_RPN_TOKEN_FLAGS = 8;
pub const WMIQ_RPN_OP_ISNOTA: WMIQ_RPN_TOKEN_FLAGS = 9;
pub const WMIQ_RPN_OP_ISNOTNULL: WMIQ_RPN_TOKEN_FLAGS = 11;
pub const WMIQ_RPN_OP_ISNULL: WMIQ_RPN_TOKEN_FLAGS = 10;
pub const WMIQ_RPN_OP_LE: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_OP_LIKE: WMIQ_RPN_TOKEN_FLAGS = 7;
pub const WMIQ_RPN_OP_LT: WMIQ_RPN_TOKEN_FLAGS = 5;
pub const WMIQ_RPN_OP_NE: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_OP_UNDEFINED: WMIQ_RPN_TOKEN_FLAGS = 0;
pub const WMIQ_RPN_RELOP: WMIQ_RPN_TOKEN_FLAGS = 16;
pub const WMIQ_RPN_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 64;
pub const WMIQ_RPN_RIGHT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_TOKEN_AND: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_TOKEN_EXPRESSION: WMIQ_RPN_TOKEN_FLAGS = 1;
pub type WMIQ_RPN_TOKEN_FLAGS = i32;
pub const WMIQ_RPN_TOKEN_NOT: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_TOKEN_OR: WMIQ_RPN_TOKEN_FLAGS = 3;
pub const WbemDefPath: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf4cc405_e2c5_4ddd_b3ce_5e7582d8c9fa);
pub const WbemQuery: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeac8a024_21e2_4523_ad73_a71a0aa2f56a);
pub type tag_WBEM_GET_KEY_FLAGS = i32;
pub type tag_WBEM_GET_TEXT_FLAGS = i32;
pub type tag_WBEM_PATH_CREATE_FLAG = i32;
pub type tag_WBEM_PATH_STATUS_FLAG = i32;
