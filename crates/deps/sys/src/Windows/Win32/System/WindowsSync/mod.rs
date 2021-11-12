#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CONFLICT_RESOLUTION_POLICY(pub i32);
pub const CRP_NONE: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(0i32);
pub const CRP_DESTINATION_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(1i32);
pub const CRP_SOURCE_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(2i32);
pub const CRP_LAST: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(3i32);
#[repr(transparent)]
pub struct CONSTRAINT_CONFLICT_REASON(pub i32);
pub const CCR_OTHER: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(0i32);
pub const CCR_COLLISION: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(1i32);
pub const CCR_NOPARENT: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(2i32);
pub const CCR_IDENTITY: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(3i32);
#[repr(transparent)]
pub struct FILTERING_TYPE(pub i32);
pub const FT_CURRENT_ITEMS_ONLY: FILTERING_TYPE = FILTERING_TYPE(0i32);
pub const FT_CURRENT_ITEMS_AND_VERSIONS_FOR_MOVED_OUT_ITEMS: FILTERING_TYPE = FILTERING_TYPE(1i32);
#[repr(transparent)]
pub struct FILTER_COMBINATION_TYPE(pub i32);
pub const FCT_INTERSECTION: FILTER_COMBINATION_TYPE = FILTER_COMBINATION_TYPE(0i32);
#[repr(transparent)]
pub struct IAsynchronousDataRetriever(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChangeConflict(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChangeUnitException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChangeUnitListFilterInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClockVector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClockVectorElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICombinedFilterInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConstraintConflict(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConstructReplicaKeyMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreFragment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreFragmentInspector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomFilterInfo(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ID_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ID_PARAMETER_PAIR(i32);
#[repr(transparent)]
pub struct IDataRetrieverCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumChangeUnitExceptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumClockVector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumFeedClockVector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumItemIds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumRangeExceptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSingleItemExceptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSyncChangeUnits(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSyncChanges(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSyncProviderConfigUIInfos(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSyncProviderInfos(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFeedClockVector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFeedClockVectorElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterKeyMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterRequestCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterTrackingProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterTrackingRequestCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterTrackingSyncChangeBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IForgottenKnowledge(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnowledgeSyncProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoadChangeContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProviderConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRecoverableError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRecoverableErrorData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegisteredSyncProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReplicaKeyMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRequestFilteredSync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISingleItemException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISupportFilteredSync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISupportLastWriteTime(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncCallback2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeBatch2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeBatchAdvanced(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeBatchBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeBatchBase2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeBatchWithFilterKeyMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeBatchWithPrerequisite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeUnit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeWithFilterKeyMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncChangeWithPrerequisite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncConstraintCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncDataConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncFilterDeserializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncFilterInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncFilterInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncFullEnumerationChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncFullEnumerationChangeBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncFullEnumerationChangeBatch2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncKnowledge(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncKnowledge2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMergeTombstoneChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncProviderConfigUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncProviderConfigUIInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncProviderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncProviderRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncRegistrationChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncSessionExtendedErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncSessionState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncSessionState2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISynchronousDataRetriever(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KNOWLEDGE_COOKIE_COMPARISON_RESULT(pub i32);
pub const KCCR_COOKIE_KNOWLEDGE_EQUAL: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(0i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINED: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(1i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINS: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(2i32);
pub const KCCR_COOKIE_KNOWLEDGE_NOT_COMPARABLE: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(3i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_IS_GLOBAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM_NOUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1430988010,
        data2: 59619,
        data3: 17850,
        data4: [147, 82, 223, 181, 97, 225, 113, 228],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONFIGUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 10u32,
};
pub const SYNC_CHANGE_FLAG_DELETED: u32 = 1u32;
pub const SYNC_CHANGE_FLAG_DOES_NOT_EXIST: u32 = 2u32;
pub const SYNC_CHANGE_FLAG_GHOST: u32 = 4u32;
#[repr(transparent)]
pub struct SYNC_CONSTRAINT_RESOLVE_ACTION(pub i32);
pub const SCRA_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(0i32);
pub const SCRA_ACCEPT_DESTINATION_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(1i32);
pub const SCRA_ACCEPT_SOURCE_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(2i32);
pub const SCRA_TRANSFER_AND_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(3i32);
pub const SCRA_MERGE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(4i32);
pub const SCRA_RENAME_SOURCE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(5i32);
pub const SCRA_RENAME_DESTINATION: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(6i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SYNC_FILTER_CHANGE(i32);
pub const SYNC_FILTER_INFO_COMBINED: u32 = 8u32;
pub const SYNC_FILTER_INFO_FLAG_CHANGE_UNIT_LIST: u32 = 2u32;
pub const SYNC_FILTER_INFO_FLAG_CUSTOM: u32 = 4u32;
pub const SYNC_FILTER_INFO_FLAG_ITEM_LIST: u32 = 1u32;
#[repr(transparent)]
pub struct SYNC_FULL_ENUMERATION_ACTION(pub i32);
pub const SFEA_FULL_ENUMERATION: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(0i32);
pub const SFEA_PARTIAL_SYNC: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(1i32);
pub const SFEA_ABORT: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(2i32);
#[repr(transparent)]
pub struct SYNC_PROGRESS_STAGE(pub i32);
pub const SPS_CHANGE_DETECTION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(0i32);
pub const SPS_CHANGE_ENUMERATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(1i32);
pub const SPS_CHANGE_APPLICATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(2i32);
#[repr(transparent)]
pub struct SYNC_PROVIDER_ROLE(pub i32);
pub const SPR_SOURCE: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(0i32);
pub const SPR_DESTINATION: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(1i32);
#[repr(C)]
pub struct SYNC_RANGE(i32);
#[repr(transparent)]
pub struct SYNC_REGISTRATION_EVENT(pub i32);
pub const SRE_PROVIDER_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(0i32);
pub const SRE_PROVIDER_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(1i32);
pub const SRE_PROVIDER_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(2i32);
pub const SRE_PROVIDER_STATE_CHANGED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(3i32);
pub const SRE_CONFIGUI_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(4i32);
pub const SRE_CONFIGUI_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(5i32);
pub const SRE_CONFIGUI_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(6i32);
#[repr(transparent)]
pub struct SYNC_RESOLVE_ACTION(pub i32);
pub const SRA_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(0i32);
pub const SRA_ACCEPT_DESTINATION_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(1i32);
pub const SRA_ACCEPT_SOURCE_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(2i32);
pub const SRA_MERGE: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(3i32);
pub const SRA_TRANSFER_AND_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(4i32);
pub const SRA_LAST: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(5i32);
#[repr(transparent)]
pub struct SYNC_SERIALIZATION_VERSION(pub i32);
pub const SYNC_SERIALIZATION_VERSION_V1: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(1i32);
pub const SYNC_SERIALIZATION_VERSION_V2: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(4i32);
pub const SYNC_SERIALIZATION_VERSION_V3: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(5i32);
pub const SYNC_SERIALIZE_REPLICA_KEY_MAP: u32 = 1u32;
#[repr(C)]
pub struct SYNC_SESSION_STATISTICS(i32);
#[repr(transparent)]
pub struct SYNC_STATISTICS(pub i32);
pub const SYNC_STATISTICS_RANGE_COUNT: SYNC_STATISTICS = SYNC_STATISTICS(0i32);
#[repr(C)]
pub struct SYNC_TIME(i32);
#[repr(C)]
pub struct SYNC_VERSION(i32);
pub const SYNC_VERSION_FLAG_FROM_FEED: u32 = 1u32;
pub const SYNC_VERSION_FLAG_HAS_BY: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SyncProviderConfigUIConfiguration(i32);
#[repr(C)]
pub struct SyncProviderConfiguration(i32);
pub const SyncProviderRegistration: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4163587825, data2: 37801, data3: 19934, data4: [128, 21, 247, 149, 10, 26, 110, 49] };
