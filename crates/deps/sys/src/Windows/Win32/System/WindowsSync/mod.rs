#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CONFLICT_RESOLUTION_POLICY(i32);
pub struct CONSTRAINT_CONFLICT_REASON(i32);
pub struct FILTERING_TYPE(i32);
pub struct FILTER_COMBINATION_TYPE(i32);
pub struct IAsynchronousDataRetriever(pub *mut ::core::ffi::c_void);
pub struct IChangeConflict(pub *mut ::core::ffi::c_void);
pub struct IChangeUnitException(pub *mut ::core::ffi::c_void);
pub struct IChangeUnitListFilterInfo(pub *mut ::core::ffi::c_void);
pub struct IClockVector(pub *mut ::core::ffi::c_void);
pub struct IClockVectorElement(pub *mut ::core::ffi::c_void);
pub struct ICombinedFilterInfo(pub *mut ::core::ffi::c_void);
pub struct IConstraintConflict(pub *mut ::core::ffi::c_void);
pub struct IConstructReplicaKeyMap(pub *mut ::core::ffi::c_void);
pub struct ICoreFragment(pub *mut ::core::ffi::c_void);
pub struct ICoreFragmentInspector(pub *mut ::core::ffi::c_void);
pub struct ICustomFilterInfo(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub struct ID_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ID_PARAMETER_PAIR(i32);
pub struct IDataRetrieverCallback(pub *mut ::core::ffi::c_void);
pub struct IEnumChangeUnitExceptions(pub *mut ::core::ffi::c_void);
pub struct IEnumClockVector(pub *mut ::core::ffi::c_void);
pub struct IEnumFeedClockVector(pub *mut ::core::ffi::c_void);
pub struct IEnumItemIds(pub *mut ::core::ffi::c_void);
pub struct IEnumRangeExceptions(pub *mut ::core::ffi::c_void);
pub struct IEnumSingleItemExceptions(pub *mut ::core::ffi::c_void);
pub struct IEnumSyncChangeUnits(pub *mut ::core::ffi::c_void);
pub struct IEnumSyncChanges(pub *mut ::core::ffi::c_void);
pub struct IEnumSyncProviderConfigUIInfos(pub *mut ::core::ffi::c_void);
pub struct IEnumSyncProviderInfos(pub *mut ::core::ffi::c_void);
pub struct IFeedClockVector(pub *mut ::core::ffi::c_void);
pub struct IFeedClockVectorElement(pub *mut ::core::ffi::c_void);
pub struct IFilterKeyMap(pub *mut ::core::ffi::c_void);
pub struct IFilterRequestCallback(pub *mut ::core::ffi::c_void);
pub struct IFilterTrackingProvider(pub *mut ::core::ffi::c_void);
pub struct IFilterTrackingRequestCallback(pub *mut ::core::ffi::c_void);
pub struct IFilterTrackingSyncChangeBuilder(pub *mut ::core::ffi::c_void);
pub struct IForgottenKnowledge(pub *mut ::core::ffi::c_void);
pub struct IKnowledgeSyncProvider(pub *mut ::core::ffi::c_void);
pub struct ILoadChangeContext(pub *mut ::core::ffi::c_void);
pub struct IProviderConverter(pub *mut ::core::ffi::c_void);
pub struct IRangeException(pub *mut ::core::ffi::c_void);
pub struct IRecoverableError(pub *mut ::core::ffi::c_void);
pub struct IRecoverableErrorData(pub *mut ::core::ffi::c_void);
pub struct IRegisteredSyncProvider(pub *mut ::core::ffi::c_void);
pub struct IReplicaKeyMap(pub *mut ::core::ffi::c_void);
pub struct IRequestFilteredSync(pub *mut ::core::ffi::c_void);
pub struct ISingleItemException(pub *mut ::core::ffi::c_void);
pub struct ISupportFilteredSync(pub *mut ::core::ffi::c_void);
pub struct ISupportLastWriteTime(pub *mut ::core::ffi::c_void);
pub struct ISyncCallback(pub *mut ::core::ffi::c_void);
pub struct ISyncCallback2(pub *mut ::core::ffi::c_void);
pub struct ISyncChange(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeBatch(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeBatch2(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeBatchAdvanced(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeBatchBase(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeBatchBase2(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeBatchWithFilterKeyMap(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeBatchWithPrerequisite(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeBuilder(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeUnit(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeWithFilterKeyMap(pub *mut ::core::ffi::c_void);
pub struct ISyncChangeWithPrerequisite(pub *mut ::core::ffi::c_void);
pub struct ISyncConstraintCallback(pub *mut ::core::ffi::c_void);
pub struct ISyncDataConverter(pub *mut ::core::ffi::c_void);
pub struct ISyncFilter(pub *mut ::core::ffi::c_void);
pub struct ISyncFilterDeserializer(pub *mut ::core::ffi::c_void);
pub struct ISyncFilterInfo(pub *mut ::core::ffi::c_void);
pub struct ISyncFilterInfo2(pub *mut ::core::ffi::c_void);
pub struct ISyncFullEnumerationChange(pub *mut ::core::ffi::c_void);
pub struct ISyncFullEnumerationChangeBatch(pub *mut ::core::ffi::c_void);
pub struct ISyncFullEnumerationChangeBatch2(pub *mut ::core::ffi::c_void);
pub struct ISyncKnowledge(pub *mut ::core::ffi::c_void);
pub struct ISyncKnowledge2(pub *mut ::core::ffi::c_void);
pub struct ISyncMergeTombstoneChange(pub *mut ::core::ffi::c_void);
pub struct ISyncProvider(pub *mut ::core::ffi::c_void);
pub struct ISyncProviderConfigUI(pub *mut ::core::ffi::c_void);
pub struct ISyncProviderConfigUIInfo(pub *mut ::core::ffi::c_void);
pub struct ISyncProviderInfo(pub *mut ::core::ffi::c_void);
pub struct ISyncProviderRegistration(pub *mut ::core::ffi::c_void);
pub struct ISyncRegistrationChange(pub *mut ::core::ffi::c_void);
pub struct ISyncSessionExtendedErrorInfo(pub *mut ::core::ffi::c_void);
pub struct ISyncSessionState(pub *mut ::core::ffi::c_void);
pub struct ISyncSessionState2(pub *mut ::core::ffi::c_void);
pub struct ISynchronousDataRetriever(pub *mut ::core::ffi::c_void);
pub struct KNOWLEDGE_COOKIE_COMPARISON_RESULT(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
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
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PROVIDER_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2216140385,
        data2: 24822,
        data3: 19484,
        data4: [136, 237, 241, 197, 49, 179, 43, 218],
    },
    pid: 10u32,
};
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_CHANGE_FLAG_DELETED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_CHANGE_FLAG_DOES_NOT_EXIST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_CHANGE_FLAG_GHOST: u32 = 4u32;
pub struct SYNC_CONSTRAINT_RESOLVE_ACTION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SYNC_FILTER_CHANGE(i32);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_FILTER_INFO_COMBINED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_FILTER_INFO_FLAG_CHANGE_UNIT_LIST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_FILTER_INFO_FLAG_CUSTOM: u32 = 4u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_FILTER_INFO_FLAG_ITEM_LIST: u32 = 1u32;
pub struct SYNC_FULL_ENUMERATION_ACTION(i32);
pub struct SYNC_PROGRESS_STAGE(i32);
pub struct SYNC_PROVIDER_ROLE(i32);
pub struct SYNC_RANGE(i32);
pub struct SYNC_REGISTRATION_EVENT(i32);
pub struct SYNC_RESOLVE_ACTION(i32);
pub struct SYNC_SERIALIZATION_VERSION(i32);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_SERIALIZE_REPLICA_KEY_MAP: u32 = 1u32;
pub struct SYNC_SESSION_STATISTICS(i32);
pub struct SYNC_STATISTICS(i32);
pub struct SYNC_TIME(i32);
pub struct SYNC_VERSION(i32);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_VERSION_FLAG_FROM_FEED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_VERSION_FLAG_HAS_BY: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct SyncProviderConfigUIConfiguration(i32);
pub struct SyncProviderConfiguration(i32);
pub struct SyncProviderRegistration(i32);
