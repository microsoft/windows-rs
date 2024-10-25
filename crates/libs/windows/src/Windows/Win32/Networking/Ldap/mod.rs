pub const LAPI_MAJOR_VER1: u32 = 1u32;
pub const LAPI_MINOR_VER1: u32 = 1u32;
pub const LBER_DEFAULT: i32 = -1i32;
pub const LBER_ERROR: i32 = -1i32;
pub const LBER_TRANSLATE_STRINGS: u32 = 4u32;
pub const LBER_USE_DER: u32 = 1u32;
pub const LBER_USE_INDEFINITE_LEN: u32 = 2u32;
pub const LDAP_ABANDON_CMD: i32 = 80i32;
pub const LDAP_ADD_CMD: i32 = 104i32;
pub const LDAP_ADMIN_LIMIT_EXCEEDED: LDAP_RETCODE = 11i32;
pub const LDAP_AFFECTS_MULTIPLE_DSAS: LDAP_RETCODE = 71i32;
pub const LDAP_ALIAS_DEREF_PROBLEM: LDAP_RETCODE = 36i32;
pub const LDAP_ALIAS_PROBLEM: LDAP_RETCODE = 33i32;
pub const LDAP_ALREADY_EXISTS: LDAP_RETCODE = 68i32;
pub const LDAP_API_FEATURE_VIRTUAL_LIST_VIEW: u32 = 1001u32;
pub const LDAP_API_INFO_VERSION: u32 = 1u32;
pub const LDAP_API_VERSION: u32 = 2004u32;
pub const LDAP_ATTRIBUTE_OR_VALUE_EXISTS: LDAP_RETCODE = 20i32;
pub const LDAP_AUTH_METHOD_NOT_SUPPORTED: LDAP_RETCODE = 7i32;
pub const LDAP_AUTH_OTHERKIND: i32 = 134i32;
pub const LDAP_AUTH_SASL: i32 = 131i32;
pub const LDAP_AUTH_SIMPLE: i32 = 128i32;
pub const LDAP_AUTH_UNKNOWN: LDAP_RETCODE = 86i32;
pub const LDAP_BIND_CMD: i32 = 96i32;
pub const LDAP_BUSY: LDAP_RETCODE = 51i32;
pub const LDAP_CAP_ACTIVE_DIRECTORY_ADAM_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1851");
pub const LDAP_CAP_ACTIVE_DIRECTORY_ADAM_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1851");
pub const LDAP_CAP_ACTIVE_DIRECTORY_LDAP_INTEG_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1791");
pub const LDAP_CAP_ACTIVE_DIRECTORY_LDAP_INTEG_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1791");
pub const LDAP_CAP_ACTIVE_DIRECTORY_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.800");
pub const LDAP_CAP_ACTIVE_DIRECTORY_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.800");
pub const LDAP_CAP_ACTIVE_DIRECTORY_PARTIAL_SECRETS_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1920");
pub const LDAP_CAP_ACTIVE_DIRECTORY_PARTIAL_SECRETS_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1920");
pub const LDAP_CAP_ACTIVE_DIRECTORY_V51_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1670");
pub const LDAP_CAP_ACTIVE_DIRECTORY_V51_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1670");
pub const LDAP_CAP_ACTIVE_DIRECTORY_V60_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1935");
pub const LDAP_CAP_ACTIVE_DIRECTORY_V60_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1935");
pub const LDAP_CAP_ACTIVE_DIRECTORY_V61_OID: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1935");
pub const LDAP_CAP_ACTIVE_DIRECTORY_V61_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1935");
pub const LDAP_CAP_ACTIVE_DIRECTORY_V61_R2_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2080");
pub const LDAP_CAP_ACTIVE_DIRECTORY_V61_R2_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2080");
pub const LDAP_CAP_ACTIVE_DIRECTORY_W8_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2237");
pub const LDAP_CAP_ACTIVE_DIRECTORY_W8_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2237");
pub const LDAP_CHASE_EXTERNAL_REFERRALS: u32 = 64u32;
pub const LDAP_CHASE_SUBORDINATE_REFERRALS: u32 = 32u32;
pub const LDAP_CLIENT_LOOP: LDAP_RETCODE = 96i32;
pub const LDAP_COMPARE_CMD: i32 = 110i32;
pub const LDAP_COMPARE_FALSE: LDAP_RETCODE = 5i32;
pub const LDAP_COMPARE_TRUE: LDAP_RETCODE = 6i32;
pub const LDAP_CONFIDENTIALITY_REQUIRED: LDAP_RETCODE = 13i32;
pub const LDAP_CONNECT_ERROR: LDAP_RETCODE = 91i32;
pub const LDAP_CONSTRAINT_VIOLATION: LDAP_RETCODE = 19i32;
pub const LDAP_CONTROL_NOT_FOUND: LDAP_RETCODE = 93i32;
pub const LDAP_CONTROL_REFERRALS: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.616");
pub const LDAP_CONTROL_REFERRALS_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.616");
pub const LDAP_CONTROL_VLVREQUEST: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.3.4.9");
pub const LDAP_CONTROL_VLVREQUEST_W: windows_core::PCWSTR = windows_core::w!("2.16.840.1.113730.3.4.9");
pub const LDAP_CONTROL_VLVRESPONSE: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.3.4.10");
pub const LDAP_CONTROL_VLVRESPONSE_W: windows_core::PCWSTR = windows_core::w!("2.16.840.1.113730.3.4.10");
pub const LDAP_DECODING_ERROR: LDAP_RETCODE = 84i32;
pub const LDAP_DELETE_CMD: i32 = 74i32;
pub const LDAP_DEREF_ALWAYS: u32 = 3u32;
pub const LDAP_DEREF_FINDING: u32 = 2u32;
pub const LDAP_DEREF_NEVER: u32 = 0u32;
pub const LDAP_DEREF_SEARCHING: u32 = 1u32;
pub const LDAP_DIRSYNC_ANCESTORS_FIRST_ORDER: u32 = 2048u32;
pub const LDAP_DIRSYNC_INCREMENTAL_VALUES: u32 = 2147483648u32;
pub const LDAP_DIRSYNC_OBJECT_SECURITY: u32 = 1u32;
pub const LDAP_DIRSYNC_PUBLIC_DATA_ONLY: u32 = 8192u32;
pub const LDAP_DIRSYNC_ROPAS_DATA_ONLY: u32 = 1073741824u32;
pub const LDAP_ENCODING_ERROR: LDAP_RETCODE = 83i32;
pub const LDAP_EXTENDED_CMD: i32 = 119i32;
pub const LDAP_FEATURE_INFO_VERSION: u32 = 1u32;
pub const LDAP_FILTER_AND: u32 = 160u32;
pub const LDAP_FILTER_APPROX: u32 = 168u32;
pub const LDAP_FILTER_EQUALITY: u32 = 163u32;
pub const LDAP_FILTER_ERROR: LDAP_RETCODE = 87i32;
pub const LDAP_FILTER_EXTENSIBLE: u32 = 169u32;
pub const LDAP_FILTER_GE: u32 = 165u32;
pub const LDAP_FILTER_LE: u32 = 166u32;
pub const LDAP_FILTER_NOT: u32 = 162u32;
pub const LDAP_FILTER_OR: u32 = 161u32;
pub const LDAP_FILTER_PRESENT: u32 = 135u32;
pub const LDAP_FILTER_SUBSTRINGS: u32 = 164u32;
pub const LDAP_GC_PORT: u32 = 3268u32;
pub const LDAP_INAPPROPRIATE_AUTH: LDAP_RETCODE = 48i32;
pub const LDAP_INAPPROPRIATE_MATCHING: LDAP_RETCODE = 18i32;
pub const LDAP_INSUFFICIENT_RIGHTS: LDAP_RETCODE = 50i32;
pub const LDAP_INVALID_CMD: u32 = 255u32;
pub const LDAP_INVALID_CREDENTIALS: LDAP_RETCODE = 49i32;
pub const LDAP_INVALID_DN_SYNTAX: LDAP_RETCODE = 34i32;
pub const LDAP_INVALID_RES: u32 = 255u32;
pub const LDAP_INVALID_SYNTAX: LDAP_RETCODE = 21i32;
pub const LDAP_IS_LEAF: LDAP_RETCODE = 35i32;
pub const LDAP_LOCAL_ERROR: LDAP_RETCODE = 82i32;
pub const LDAP_LOOP_DETECT: LDAP_RETCODE = 54i32;
pub const LDAP_MATCHING_RULE_BIT_AND: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.803");
pub const LDAP_MATCHING_RULE_BIT_AND_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.803");
pub const LDAP_MATCHING_RULE_BIT_OR: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.804");
pub const LDAP_MATCHING_RULE_BIT_OR_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.804");
pub const LDAP_MATCHING_RULE_DN_BINARY_COMPLEX: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2253");
pub const LDAP_MATCHING_RULE_DN_BINARY_COMPLEX_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2253");
pub const LDAP_MATCHING_RULE_TRANSITIVE_EVALUATION: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1941");
pub const LDAP_MATCHING_RULE_TRANSITIVE_EVALUATION_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1941");
pub const LDAP_MODIFY_CMD: i32 = 102i32;
pub const LDAP_MODRDN_CMD: i32 = 108i32;
pub const LDAP_MOD_ADD: u32 = 0u32;
pub const LDAP_MOD_BVALUES: u32 = 128u32;
pub const LDAP_MOD_DELETE: u32 = 1u32;
pub const LDAP_MOD_REPLACE: u32 = 2u32;
pub const LDAP_MORE_RESULTS_TO_RETURN: LDAP_RETCODE = 95i32;
pub const LDAP_MSG_ALL: u32 = 1u32;
pub const LDAP_MSG_ONE: u32 = 0u32;
pub const LDAP_MSG_RECEIVED: u32 = 2u32;
pub const LDAP_NAMING_VIOLATION: LDAP_RETCODE = 64i32;
pub const LDAP_NOT_ALLOWED_ON_NONLEAF: LDAP_RETCODE = 66i32;
pub const LDAP_NOT_ALLOWED_ON_RDN: LDAP_RETCODE = 67i32;
pub const LDAP_NOT_SUPPORTED: LDAP_RETCODE = 92i32;
pub const LDAP_NO_LIMIT: u32 = 0u32;
pub const LDAP_NO_MEMORY: LDAP_RETCODE = 90i32;
pub const LDAP_NO_OBJECT_CLASS_MODS: LDAP_RETCODE = 69i32;
pub const LDAP_NO_RESULTS_RETURNED: LDAP_RETCODE = 94i32;
pub const LDAP_NO_SUCH_ATTRIBUTE: LDAP_RETCODE = 16i32;
pub const LDAP_NO_SUCH_OBJECT: LDAP_RETCODE = 32i32;
pub const LDAP_OBJECT_CLASS_VIOLATION: LDAP_RETCODE = 65i32;
pub const LDAP_OFFSET_RANGE_ERROR: LDAP_RETCODE = 61i32;
pub const LDAP_OPATT_ABANDON_REPL: windows_core::PCSTR = windows_core::s!("abandonReplication");
pub const LDAP_OPATT_ABANDON_REPL_W: windows_core::PCWSTR = windows_core::w!("abandonReplication");
pub const LDAP_OPATT_BECOME_DOM_MASTER: windows_core::PCSTR = windows_core::s!("becomeDomainMaster");
pub const LDAP_OPATT_BECOME_DOM_MASTER_W: windows_core::PCWSTR = windows_core::w!("becomeDomainMaster");
pub const LDAP_OPATT_BECOME_PDC: windows_core::PCSTR = windows_core::s!("becomePdc");
pub const LDAP_OPATT_BECOME_PDC_W: windows_core::PCWSTR = windows_core::w!("becomePdc");
pub const LDAP_OPATT_BECOME_RID_MASTER: windows_core::PCSTR = windows_core::s!("becomeRidMaster");
pub const LDAP_OPATT_BECOME_RID_MASTER_W: windows_core::PCWSTR = windows_core::w!("becomeRidMaster");
pub const LDAP_OPATT_BECOME_SCHEMA_MASTER: windows_core::PCSTR = windows_core::s!("becomeSchemaMaster");
pub const LDAP_OPATT_BECOME_SCHEMA_MASTER_W: windows_core::PCWSTR = windows_core::w!("becomeSchemaMaster");
pub const LDAP_OPATT_CONFIG_NAMING_CONTEXT: windows_core::PCSTR = windows_core::s!("configurationNamingContext");
pub const LDAP_OPATT_CONFIG_NAMING_CONTEXT_W: windows_core::PCWSTR = windows_core::w!("configurationNamingContext");
pub const LDAP_OPATT_CURRENT_TIME: windows_core::PCSTR = windows_core::s!("currentTime");
pub const LDAP_OPATT_CURRENT_TIME_W: windows_core::PCWSTR = windows_core::w!("currentTime");
pub const LDAP_OPATT_DEFAULT_NAMING_CONTEXT: windows_core::PCSTR = windows_core::s!("defaultNamingContext");
pub const LDAP_OPATT_DEFAULT_NAMING_CONTEXT_W: windows_core::PCWSTR = windows_core::w!("defaultNamingContext");
pub const LDAP_OPATT_DNS_HOST_NAME: windows_core::PCSTR = windows_core::s!("dnsHostName");
pub const LDAP_OPATT_DNS_HOST_NAME_W: windows_core::PCWSTR = windows_core::w!("dnsHostName");
pub const LDAP_OPATT_DO_GARBAGE_COLLECTION: windows_core::PCSTR = windows_core::s!("doGarbageCollection");
pub const LDAP_OPATT_DO_GARBAGE_COLLECTION_W: windows_core::PCWSTR = windows_core::w!("doGarbageCollection");
pub const LDAP_OPATT_DS_SERVICE_NAME: windows_core::PCSTR = windows_core::s!("dsServiceName");
pub const LDAP_OPATT_DS_SERVICE_NAME_W: windows_core::PCWSTR = windows_core::w!("dsServiceName");
pub const LDAP_OPATT_FIXUP_INHERITANCE: windows_core::PCSTR = windows_core::s!("fixupInheritance");
pub const LDAP_OPATT_FIXUP_INHERITANCE_W: windows_core::PCWSTR = windows_core::w!("fixupInheritance");
pub const LDAP_OPATT_HIGHEST_COMMITTED_USN: windows_core::PCSTR = windows_core::s!("highestCommitedUSN");
pub const LDAP_OPATT_HIGHEST_COMMITTED_USN_W: windows_core::PCWSTR = windows_core::w!("highestCommitedUSN");
pub const LDAP_OPATT_INVALIDATE_RID_POOL: windows_core::PCSTR = windows_core::s!("invalidateRidPool");
pub const LDAP_OPATT_INVALIDATE_RID_POOL_W: windows_core::PCWSTR = windows_core::w!("invalidateRidPool");
pub const LDAP_OPATT_LDAP_SERVICE_NAME: windows_core::PCSTR = windows_core::s!("ldapServiceName");
pub const LDAP_OPATT_LDAP_SERVICE_NAME_W: windows_core::PCWSTR = windows_core::w!("ldapServiceName");
pub const LDAP_OPATT_NAMING_CONTEXTS: windows_core::PCSTR = windows_core::s!("namingContexts");
pub const LDAP_OPATT_NAMING_CONTEXTS_W: windows_core::PCWSTR = windows_core::w!("namingContexts");
pub const LDAP_OPATT_RECALC_HIERARCHY: windows_core::PCSTR = windows_core::s!("recalcHierarchy");
pub const LDAP_OPATT_RECALC_HIERARCHY_W: windows_core::PCWSTR = windows_core::w!("recalcHierarchy");
pub const LDAP_OPATT_ROOT_DOMAIN_NAMING_CONTEXT: windows_core::PCSTR = windows_core::s!("rootDomainNamingContext");
pub const LDAP_OPATT_ROOT_DOMAIN_NAMING_CONTEXT_W: windows_core::PCWSTR = windows_core::w!("rootDomainNamingContext");
pub const LDAP_OPATT_SCHEMA_NAMING_CONTEXT: windows_core::PCSTR = windows_core::s!("schemaNamingContext");
pub const LDAP_OPATT_SCHEMA_NAMING_CONTEXT_W: windows_core::PCWSTR = windows_core::w!("schemaNamingContext");
pub const LDAP_OPATT_SCHEMA_UPDATE_NOW: windows_core::PCSTR = windows_core::s!("schemaUpdateNow");
pub const LDAP_OPATT_SCHEMA_UPDATE_NOW_W: windows_core::PCWSTR = windows_core::w!("schemaUpdateNow");
pub const LDAP_OPATT_SERVER_NAME: windows_core::PCSTR = windows_core::s!("serverName");
pub const LDAP_OPATT_SERVER_NAME_W: windows_core::PCWSTR = windows_core::w!("serverName");
pub const LDAP_OPATT_SUBSCHEMA_SUBENTRY: windows_core::PCSTR = windows_core::s!("subschemaSubentry");
pub const LDAP_OPATT_SUBSCHEMA_SUBENTRY_W: windows_core::PCWSTR = windows_core::w!("subschemaSubentry");
pub const LDAP_OPATT_SUPPORTED_CAPABILITIES: windows_core::PCSTR = windows_core::s!("supportedCapabilities");
pub const LDAP_OPATT_SUPPORTED_CAPABILITIES_W: windows_core::PCWSTR = windows_core::w!("supportedCapabilities");
pub const LDAP_OPATT_SUPPORTED_CONTROL: windows_core::PCSTR = windows_core::s!("supportedControl");
pub const LDAP_OPATT_SUPPORTED_CONTROL_W: windows_core::PCWSTR = windows_core::w!("supportedControl");
pub const LDAP_OPATT_SUPPORTED_LDAP_POLICIES: windows_core::PCSTR = windows_core::s!("supportedLDAPPolicies");
pub const LDAP_OPATT_SUPPORTED_LDAP_POLICIES_W: windows_core::PCWSTR = windows_core::w!("supportedLDAPPolicies");
pub const LDAP_OPATT_SUPPORTED_LDAP_VERSION: windows_core::PCSTR = windows_core::s!("supportedLDAPVersion");
pub const LDAP_OPATT_SUPPORTED_LDAP_VERSION_W: windows_core::PCWSTR = windows_core::w!("supportedLDAPVersion");
pub const LDAP_OPATT_SUPPORTED_SASL_MECHANISM: windows_core::PCSTR = windows_core::s!("supportedSASLMechanisms");
pub const LDAP_OPATT_SUPPORTED_SASL_MECHANISM_W: windows_core::PCWSTR = windows_core::w!("supportedSASLMechanisms");
pub const LDAP_OPERATIONS_ERROR: LDAP_RETCODE = 1i32;
pub const LDAP_OPT_API_FEATURE_INFO: u32 = 21u32;
pub const LDAP_OPT_API_INFO: u32 = 0u32;
pub const LDAP_OPT_AREC_EXCLUSIVE: u32 = 152u32;
pub const LDAP_OPT_AUTO_RECONNECT: u32 = 145u32;
pub const LDAP_OPT_CACHE_ENABLE: u32 = 15u32;
pub const LDAP_OPT_CACHE_FN_PTRS: u32 = 13u32;
pub const LDAP_OPT_CACHE_STRATEGY: u32 = 14u32;
pub const LDAP_OPT_CHASE_REFERRALS: u32 = 2u32;
pub const LDAP_OPT_CLDAP_TIMEOUT: u32 = 69u32;
pub const LDAP_OPT_CLDAP_TRIES: u32 = 70u32;
pub const LDAP_OPT_CLIENT_CERTIFICATE: u32 = 128u32;
pub const LDAP_OPT_DEREF: u32 = 2u32;
pub const LDAP_OPT_DESC: u32 = 1u32;
pub const LDAP_OPT_DNS: u32 = 1u32;
pub const LDAP_OPT_DNSDOMAIN_NAME: u32 = 59u32;
pub const LDAP_OPT_ENCRYPT: u32 = 150u32;
pub const LDAP_OPT_ERROR_NUMBER: u32 = 49u32;
pub const LDAP_OPT_ERROR_STRING: u32 = 50u32;
pub const LDAP_OPT_FAST_CONCURRENT_BIND: u32 = 65u32;
pub const LDAP_OPT_GETDSNAME_FLAGS: u32 = 61u32;
pub const LDAP_OPT_HOST_NAME: u32 = 48u32;
pub const LDAP_OPT_HOST_REACHABLE: u32 = 62u32;
pub const LDAP_OPT_IO_FN_PTRS: u32 = 11u32;
pub const LDAP_OPT_PING_KEEP_ALIVE: u32 = 54u32;
pub const LDAP_OPT_PING_LIMIT: u32 = 56u32;
pub const LDAP_OPT_PING_WAIT_TIME: u32 = 55u32;
pub const LDAP_OPT_PROMPT_CREDENTIALS: u32 = 63u32;
pub const LDAP_OPT_PROTOCOL_VERSION: u32 = 17u32;
pub const LDAP_OPT_REBIND_ARG: u32 = 7u32;
pub const LDAP_OPT_REBIND_FN: u32 = 6u32;
pub const LDAP_OPT_REFERRALS: u32 = 8u32;
pub const LDAP_OPT_REFERRAL_CALLBACK: u32 = 112u32;
pub const LDAP_OPT_REFERRAL_HOP_LIMIT: u32 = 16u32;
pub const LDAP_OPT_REF_DEREF_CONN_PER_MSG: u32 = 148u32;
pub const LDAP_OPT_RESTART: u32 = 9u32;
pub const LDAP_OPT_RETURN_REFS: u32 = 4u32;
pub const LDAP_OPT_ROOTDSE_CACHE: u32 = 154u32;
pub const LDAP_OPT_SASL_METHOD: u32 = 151u32;
pub const LDAP_OPT_SCH_FLAGS: u32 = 67u32;
pub const LDAP_OPT_SECURITY_CONTEXT: u32 = 153u32;
pub const LDAP_OPT_SEND_TIMEOUT: u32 = 66u32;
pub const LDAP_OPT_SERVER_CERTIFICATE: u32 = 129u32;
pub const LDAP_OPT_SERVER_ERROR: u32 = 51u32;
pub const LDAP_OPT_SERVER_EXT_ERROR: u32 = 52u32;
pub const LDAP_OPT_SIGN: u32 = 149u32;
pub const LDAP_OPT_SIZELIMIT: u32 = 3u32;
pub const LDAP_OPT_SOCKET_BIND_ADDRESSES: u32 = 68u32;
pub const LDAP_OPT_SSL: u32 = 10u32;
pub const LDAP_OPT_SSL_INFO: u32 = 147u32;
pub const LDAP_OPT_SSPI_FLAGS: u32 = 146u32;
pub const LDAP_OPT_TCP_KEEPALIVE: u32 = 64u32;
pub const LDAP_OPT_THREAD_FN_PTRS: u32 = 5u32;
pub const LDAP_OPT_TIMELIMIT: u32 = 4u32;
pub const LDAP_OPT_TLS: u32 = 10u32;
pub const LDAP_OPT_TLS_INFO: u32 = 147u32;
pub const LDAP_OPT_VERSION: u32 = 17u32;
pub const LDAP_OTHER: LDAP_RETCODE = 80i32;
pub const LDAP_PAGED_RESULT_OID_STRING: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.319");
pub const LDAP_PAGED_RESULT_OID_STRING_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.319");
pub const LDAP_PARAM_ERROR: LDAP_RETCODE = 89i32;
pub const LDAP_PARTIAL_RESULTS: LDAP_RETCODE = 9i32;
pub const LDAP_POLICYHINT_APPLY_FULLPWDPOLICY: u32 = 1u32;
pub const LDAP_PORT: u32 = 389u32;
pub const LDAP_PROTOCOL_ERROR: LDAP_RETCODE = 2i32;
pub const LDAP_REFERRAL: LDAP_RETCODE = 10i32;
pub const LDAP_REFERRAL_LIMIT_EXCEEDED: LDAP_RETCODE = 97i32;
pub const LDAP_REFERRAL_V2: LDAP_RETCODE = 9i32;
pub const LDAP_RESULTS_TOO_LARGE: LDAP_RETCODE = 70i32;
pub const LDAP_RES_ADD: i32 = 105i32;
pub const LDAP_RES_ANY: i32 = -1i32;
pub const LDAP_RES_BIND: i32 = 97i32;
pub const LDAP_RES_COMPARE: i32 = 111i32;
pub const LDAP_RES_DELETE: i32 = 107i32;
pub const LDAP_RES_EXTENDED: i32 = 120i32;
pub const LDAP_RES_MODIFY: i32 = 103i32;
pub const LDAP_RES_MODRDN: i32 = 109i32;
pub const LDAP_RES_REFERRAL: i32 = 115i32;
pub const LDAP_RES_SEARCH_ENTRY: i32 = 100i32;
pub const LDAP_RES_SEARCH_RESULT: i32 = 101i32;
pub const LDAP_RES_SESSION: i32 = 114i32;
pub const LDAP_SASL_BIND_IN_PROGRESS: LDAP_RETCODE = 14i32;
pub const LDAP_SCOPE_BASE: u32 = 0u32;
pub const LDAP_SCOPE_ONELEVEL: u32 = 1u32;
pub const LDAP_SCOPE_SUBTREE: u32 = 2u32;
pub const LDAP_SEARCH_CMD: i32 = 99i32;
pub const LDAP_SEARCH_HINT_INDEX_ONLY_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2207");
pub const LDAP_SEARCH_HINT_INDEX_ONLY_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2207");
pub const LDAP_SEARCH_HINT_REQUIRED_INDEX_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2306");
pub const LDAP_SEARCH_HINT_REQUIRED_INDEX_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2306");
pub const LDAP_SEARCH_HINT_SOFT_SIZE_LIMIT_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2210");
pub const LDAP_SEARCH_HINT_SOFT_SIZE_LIMIT_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2210");
pub const LDAP_SERVER_ASQ_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1504");
pub const LDAP_SERVER_ASQ_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1504");
pub const LDAP_SERVER_BATCH_REQUEST_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2212");
pub const LDAP_SERVER_BATCH_REQUEST_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2212");
pub const LDAP_SERVER_BYPASS_QUOTA_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2256");
pub const LDAP_SERVER_BYPASS_QUOTA_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2256");
pub const LDAP_SERVER_CROSSDOM_MOVE_TARGET_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.521");
pub const LDAP_SERVER_CROSSDOM_MOVE_TARGET_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.521");
pub const LDAP_SERVER_DIRSYNC_EX_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2090");
pub const LDAP_SERVER_DIRSYNC_EX_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2090");
pub const LDAP_SERVER_DIRSYNC_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.841");
pub const LDAP_SERVER_DIRSYNC_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.841");
pub const LDAP_SERVER_DN_INPUT_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2026");
pub const LDAP_SERVER_DN_INPUT_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2026");
pub const LDAP_SERVER_DOMAIN_SCOPE_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1339");
pub const LDAP_SERVER_DOMAIN_SCOPE_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1339");
pub const LDAP_SERVER_DOWN: LDAP_RETCODE = 81i32;
pub const LDAP_SERVER_EXPECTED_ENTRY_COUNT_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2211");
pub const LDAP_SERVER_EXPECTED_ENTRY_COUNT_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2211");
pub const LDAP_SERVER_EXTENDED_DN_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.529");
pub const LDAP_SERVER_EXTENDED_DN_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.529");
pub const LDAP_SERVER_FAST_BIND_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1781");
pub const LDAP_SERVER_FAST_BIND_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1781");
pub const LDAP_SERVER_FORCE_UPDATE_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1974");
pub const LDAP_SERVER_FORCE_UPDATE_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1974");
pub const LDAP_SERVER_GET_STATS_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.970");
pub const LDAP_SERVER_GET_STATS_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.970");
pub const LDAP_SERVER_LAZY_COMMIT_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.619");
pub const LDAP_SERVER_LAZY_COMMIT_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.619");
pub const LDAP_SERVER_LINK_TTL_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2309");
pub const LDAP_SERVER_LINK_TTL_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2309");
pub const LDAP_SERVER_NOTIFICATION_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.528");
pub const LDAP_SERVER_NOTIFICATION_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.528");
pub const LDAP_SERVER_PERMISSIVE_MODIFY_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1413");
pub const LDAP_SERVER_PERMISSIVE_MODIFY_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1413");
pub const LDAP_SERVER_POLICY_HINTS_DEPRECATED_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2066");
pub const LDAP_SERVER_POLICY_HINTS_DEPRECATED_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2066");
pub const LDAP_SERVER_POLICY_HINTS_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2239");
pub const LDAP_SERVER_POLICY_HINTS_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2239");
pub const LDAP_SERVER_QUOTA_CONTROL_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1852");
pub const LDAP_SERVER_QUOTA_CONTROL_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1852");
pub const LDAP_SERVER_RANGE_OPTION_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.802");
pub const LDAP_SERVER_RANGE_OPTION_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.802");
pub const LDAP_SERVER_RANGE_RETRIEVAL_NOERR_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1948");
pub const LDAP_SERVER_RANGE_RETRIEVAL_NOERR_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1948");
pub const LDAP_SERVER_RESP_SORT_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.474");
pub const LDAP_SERVER_RESP_SORT_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.474");
pub const LDAP_SERVER_SD_FLAGS_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.801");
pub const LDAP_SERVER_SD_FLAGS_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.801");
pub const LDAP_SERVER_SEARCH_HINTS_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2206");
pub const LDAP_SERVER_SEARCH_HINTS_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2206");
pub const LDAP_SERVER_SEARCH_OPTIONS_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1340");
pub const LDAP_SERVER_SEARCH_OPTIONS_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1340");
pub const LDAP_SERVER_SET_OWNER_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2255");
pub const LDAP_SERVER_SET_OWNER_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2255");
pub const LDAP_SERVER_SHOW_DEACTIVATED_LINK_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2065");
pub const LDAP_SERVER_SHOW_DEACTIVATED_LINK_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2065");
pub const LDAP_SERVER_SHOW_DELETED_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.417");
pub const LDAP_SERVER_SHOW_DELETED_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.417");
pub const LDAP_SERVER_SHOW_RECYCLED_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2064");
pub const LDAP_SERVER_SHOW_RECYCLED_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2064");
pub const LDAP_SERVER_SHUTDOWN_NOTIFY_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1907");
pub const LDAP_SERVER_SHUTDOWN_NOTIFY_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1907");
pub const LDAP_SERVER_SORT_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.473");
pub const LDAP_SERVER_SORT_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.473");
pub const LDAP_SERVER_TREE_DELETE_EX_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2204");
pub const LDAP_SERVER_TREE_DELETE_EX_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2204");
pub const LDAP_SERVER_TREE_DELETE_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.805");
pub const LDAP_SERVER_TREE_DELETE_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.805");
pub const LDAP_SERVER_UPDATE_STATS_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2205");
pub const LDAP_SERVER_UPDATE_STATS_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2205");
pub const LDAP_SERVER_VERIFY_NAME_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.1338");
pub const LDAP_SERVER_VERIFY_NAME_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.1338");
pub const LDAP_SERVER_WHO_AM_I_OID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.4203.1.11.3");
pub const LDAP_SERVER_WHO_AM_I_OID_W: windows_core::PCWSTR = windows_core::w!("1.3.6.1.4.1.4203.1.11.3");
pub const LDAP_SESSION_CMD: i32 = 113i32;
pub const LDAP_SIZELIMIT_EXCEEDED: LDAP_RETCODE = 4i32;
pub const LDAP_SORT_CONTROL_MISSING: LDAP_RETCODE = 60i32;
pub const LDAP_SSL_GC_PORT: u32 = 3269u32;
pub const LDAP_SSL_PORT: u32 = 636u32;
pub const LDAP_START_TLS_OID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.1466.20037");
pub const LDAP_START_TLS_OID_W: windows_core::PCWSTR = windows_core::w!("1.3.6.1.4.1.1466.20037");
pub const LDAP_STRONG_AUTH_REQUIRED: LDAP_RETCODE = 8i32;
pub const LDAP_SUBSTRING_ANY: i32 = 129i32;
pub const LDAP_SUBSTRING_FINAL: i32 = 130i32;
pub const LDAP_SUBSTRING_INITIAL: i32 = 128i32;
pub const LDAP_SUCCESS: LDAP_RETCODE = 0i32;
pub const LDAP_TIMELIMIT_EXCEEDED: LDAP_RETCODE = 3i32;
pub const LDAP_TIMEOUT: LDAP_RETCODE = 85i32;
pub const LDAP_TTL_EXTENDED_OP_OID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.1466.101.119.1");
pub const LDAP_TTL_EXTENDED_OP_OID_W: windows_core::PCWSTR = windows_core::w!("1.3.6.1.4.1.1466.101.119.1");
pub const LDAP_UNAVAILABLE: LDAP_RETCODE = 52i32;
pub const LDAP_UNAVAILABLE_CRIT_EXTENSION: LDAP_RETCODE = 12i32;
pub const LDAP_UNBIND_CMD: i32 = 66i32;
pub const LDAP_UNDEFINED_TYPE: LDAP_RETCODE = 17i32;
pub const LDAP_UNICODE: u32 = 1u32;
pub const LDAP_UNWILLING_TO_PERFORM: LDAP_RETCODE = 53i32;
pub const LDAP_UPDATE_STATS_INVOCATIONID_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2209");
pub const LDAP_UPDATE_STATS_INVOCATIONID_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2209");
pub const LDAP_UPDATE_STATS_USN_OID: windows_core::PCSTR = windows_core::s!("1.2.840.113556.1.4.2208");
pub const LDAP_UPDATE_STATS_USN_OID_W: windows_core::PCWSTR = windows_core::w!("1.2.840.113556.1.4.2208");
pub const LDAP_USER_CANCELLED: LDAP_RETCODE = 88i32;
pub const LDAP_VENDOR_NAME: windows_core::PCSTR = windows_core::s!("Microsoft Corporation.");
pub const LDAP_VENDOR_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft Corporation.");
pub const LDAP_VENDOR_VERSION: u32 = 510u32;
pub const LDAP_VERSION: u32 = 2u32;
pub const LDAP_VERSION1: u32 = 1u32;
pub const LDAP_VERSION2: u32 = 2u32;
pub const LDAP_VERSION3: u32 = 3u32;
pub const LDAP_VERSION_MAX: u32 = 3u32;
pub const LDAP_VERSION_MIN: u32 = 2u32;
pub const LDAP_VIRTUAL_LIST_VIEW_ERROR: LDAP_RETCODE = 76i32;
pub const LDAP_VLVINFO_VERSION: u32 = 1u32;
pub const SERVER_SEARCH_FLAG_DOMAIN_SCOPE: u32 = 1u32;
pub const SERVER_SEARCH_FLAG_PHANTOM_ROOT: u32 = 2u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LDAP_RETCODE(pub i32);
impl windows_core::TypeKind for LDAP_RETCODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BerElement {
    pub opaque: windows_core::PSTR,
}
impl Default for BerElement {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BerElement {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAP {
    pub ld_sb: LDAP_0,
    pub ld_host: windows_core::PSTR,
    pub ld_version: u32,
    pub ld_lberoptions: u8,
    pub ld_deref: u32,
    pub ld_timelimit: u32,
    pub ld_sizelimit: u32,
    pub ld_errno: u32,
    pub ld_matched: windows_core::PSTR,
    pub ld_error: windows_core::PSTR,
    pub ld_msgid: u32,
    pub Reserved3: [u8; 25],
    pub ld_cldaptries: u32,
    pub ld_cldaptimeout: u32,
    pub ld_refhoplimit: u32,
    pub ld_options: u32,
}
impl Default for LDAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAP {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAP_0 {
    pub sb_sd: usize,
    pub Reserved1: [u8; 41],
    pub sb_naddr: usize,
    pub Reserved2: [u8; 24],
}
impl Default for LDAP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAP_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPAPIFeatureInfoA {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: windows_core::PSTR,
    pub ldapaif_version: i32,
}
impl Default for LDAPAPIFeatureInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPAPIFeatureInfoA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPAPIFeatureInfoW {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: windows_core::PWSTR,
    pub ldapaif_version: i32,
}
impl Default for LDAPAPIFeatureInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPAPIFeatureInfoW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPAPIInfoA {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut *mut i8,
    pub ldapai_vendor_name: windows_core::PSTR,
    pub ldapai_vendor_version: i32,
}
impl Default for LDAPAPIInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPAPIInfoA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPAPIInfoW {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut windows_core::PWSTR,
    pub ldapai_vendor_name: windows_core::PWSTR,
    pub ldapai_vendor_version: i32,
}
impl Default for LDAPAPIInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPAPIInfoW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPControlA {
    pub ldctl_oid: windows_core::PSTR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: super::super::Foundation::BOOLEAN,
}
impl Default for LDAPControlA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPControlA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPControlW {
    pub ldctl_oid: windows_core::PWSTR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: super::super::Foundation::BOOLEAN,
}
impl Default for LDAPControlW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPControlW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPMessage {
    pub lm_msgid: u32,
    pub lm_msgtype: u32,
    pub lm_ber: *mut core::ffi::c_void,
    pub lm_chain: *mut LDAPMessage,
    pub lm_next: *mut LDAPMessage,
    pub lm_time: u32,
    pub Connection: *mut LDAP,
    pub Request: *mut core::ffi::c_void,
    pub lm_returncode: u32,
    pub lm_referral: u16,
    pub lm_chased: super::super::Foundation::BOOLEAN,
    pub lm_eom: super::super::Foundation::BOOLEAN,
    pub ConnectionReferenced: super::super::Foundation::BOOLEAN,
}
impl Default for LDAPMessage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPMessage {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPModA {
    pub mod_op: u32,
    pub mod_type: windows_core::PSTR,
    pub mod_vals: LDAPModA_0,
}
impl Default for LDAPModA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPModA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union LDAPModA_0 {
    pub modv_strvals: *mut windows_core::PSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
impl Default for LDAPModA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPModA_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPModW {
    pub mod_op: u32,
    pub mod_type: windows_core::PWSTR,
    pub mod_vals: LDAPModW_0,
}
impl Default for LDAPModW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPModW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union LDAPModW_0 {
    pub modv_strvals: *mut windows_core::PWSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
impl Default for LDAPModW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPModW_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPSortKeyA {
    pub sk_attrtype: windows_core::PSTR,
    pub sk_matchruleoid: windows_core::PSTR,
    pub sk_reverseorder: super::super::Foundation::BOOLEAN,
}
impl Default for LDAPSortKeyA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPSortKeyA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPSortKeyW {
    pub sk_attrtype: windows_core::PWSTR,
    pub sk_matchruleoid: windows_core::PWSTR,
    pub sk_reverseorder: super::super::Foundation::BOOLEAN,
}
impl Default for LDAPSortKeyW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPSortKeyW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAPVLVInfo {
    pub ldvlv_version: i32,
    pub ldvlv_before_count: u32,
    pub ldvlv_after_count: u32,
    pub ldvlv_offset: u32,
    pub ldvlv_count: u32,
    pub ldvlv_attrvalue: *mut LDAP_BERVAL,
    pub ldvlv_context: *mut LDAP_BERVAL,
    pub ldvlv_extradata: *mut core::ffi::c_void,
}
impl Default for LDAPVLVInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAPVLVInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAP_BERVAL {
    pub bv_len: u32,
    pub bv_val: windows_core::PSTR,
}
impl Default for LDAP_BERVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAP_BERVAL {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAP_REFERRAL_CALLBACK {
    pub SizeOfCallbacks: u32,
    pub QueryForConnection: QUERYFORCONNECTION,
    pub NotifyRoutine: NOTIFYOFNEWCONNECTION,
    pub DereferenceRoutine: DEREFERENCECONNECTION,
}
impl Default for LDAP_REFERRAL_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAP_REFERRAL_CALLBACK {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAP_TIMEVAL {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
impl Default for LDAP_TIMEVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAP_TIMEVAL {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LDAP_VERSION_INFO {
    pub lv_size: u32,
    pub lv_major: u32,
    pub lv_minor: u32,
}
impl Default for LDAP_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LDAP_VERSION_INFO {
    type TypeKind = windows_core::CopyType;
}
pub type DBGPRINT = Option<unsafe extern "system" fn(format: windows_core::PCSTR) -> u32>;
pub type DEREFERENCECONNECTION = Option<unsafe extern "system" fn(primaryconnection: *mut LDAP, connectiontodereference: *mut LDAP) -> u32>;
pub type NOTIFYOFNEWCONNECTION = Option<unsafe extern "system" fn(primaryconnection: *mut LDAP, referralfromconnection: *mut LDAP, newdn: windows_core::PCWSTR, hostname: windows_core::PCSTR, newconnection: *mut LDAP, portnumber: u32, secauthidentity: *mut core::ffi::c_void, currentuser: *mut core::ffi::c_void, errorcodefrombind: u32) -> super::super::Foundation::BOOLEAN>;
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub type QUERYCLIENTCERT = Option<unsafe extern "system" fn(connection: *mut LDAP, trusted_cas: *mut super::super::Security::Authentication::Identity::SecPkgContext_IssuerListInfoEx, ppcertificate: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN>;
pub type QUERYFORCONNECTION = Option<unsafe extern "system" fn(primaryconnection: *mut LDAP, referralfromconnection: *mut LDAP, newdn: windows_core::PCWSTR, hostname: windows_core::PCSTR, portnumber: u32, secauthidentity: *mut core::ffi::c_void, currentusertoken: *mut core::ffi::c_void, connectiontouse: *mut *mut LDAP) -> u32>;
#[cfg(feature = "Win32_Security_Cryptography")]
pub type VERIFYSERVERCERT = Option<unsafe extern "system" fn(connection: *mut LDAP, pservercert: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN>;
