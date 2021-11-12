#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CONFLICT_RESOLUTION_POLICY(i32);
pub struct CONSTRAINT_CONFLICT_REASON(i32);
pub struct FILTERING_TYPE(i32);
pub struct FILTER_COMBINATION_TYPE(i32);
pub struct IAsynchronousDataRetriever(i32);
pub struct IChangeConflict(i32);
pub struct IChangeUnitException(i32);
pub struct IChangeUnitListFilterInfo(i32);
pub struct IClockVector(i32);
pub struct IClockVectorElement(i32);
pub struct ICombinedFilterInfo(i32);
pub struct IConstraintConflict(i32);
pub struct IConstructReplicaKeyMap(i32);
pub struct ICoreFragment(i32);
pub struct ICoreFragmentInspector(i32);
pub struct ICustomFilterInfo(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ID_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ID_PARAMETER_PAIR(i32);
pub struct IDataRetrieverCallback(i32);
pub struct IEnumChangeUnitExceptions(i32);
pub struct IEnumClockVector(i32);
pub struct IEnumFeedClockVector(i32);
pub struct IEnumItemIds(i32);
pub struct IEnumRangeExceptions(i32);
pub struct IEnumSingleItemExceptions(i32);
pub struct IEnumSyncChangeUnits(i32);
pub struct IEnumSyncChanges(i32);
pub struct IEnumSyncProviderConfigUIInfos(i32);
pub struct IEnumSyncProviderInfos(i32);
pub struct IFeedClockVector(i32);
pub struct IFeedClockVectorElement(i32);
pub struct IFilterKeyMap(i32);
pub struct IFilterRequestCallback(i32);
pub struct IFilterTrackingProvider(i32);
pub struct IFilterTrackingRequestCallback(i32);
pub struct IFilterTrackingSyncChangeBuilder(i32);
pub struct IForgottenKnowledge(i32);
pub struct IKnowledgeSyncProvider(i32);
pub struct ILoadChangeContext(i32);
pub struct IProviderConverter(i32);
pub struct IRangeException(i32);
pub struct IRecoverableError(i32);
pub struct IRecoverableErrorData(i32);
pub struct IRegisteredSyncProvider(i32);
pub struct IReplicaKeyMap(i32);
pub struct IRequestFilteredSync(i32);
pub struct ISingleItemException(i32);
pub struct ISupportFilteredSync(i32);
pub struct ISupportLastWriteTime(i32);
pub struct ISyncCallback(i32);
pub struct ISyncCallback2(i32);
pub struct ISyncChange(i32);
pub struct ISyncChangeBatch(i32);
pub struct ISyncChangeBatch2(i32);
pub struct ISyncChangeBatchAdvanced(i32);
pub struct ISyncChangeBatchBase(i32);
pub struct ISyncChangeBatchBase2(i32);
pub struct ISyncChangeBatchWithFilterKeyMap(i32);
pub struct ISyncChangeBatchWithPrerequisite(i32);
pub struct ISyncChangeBuilder(i32);
pub struct ISyncChangeUnit(i32);
pub struct ISyncChangeWithFilterKeyMap(i32);
pub struct ISyncChangeWithPrerequisite(i32);
pub struct ISyncConstraintCallback(i32);
pub struct ISyncDataConverter(i32);
pub struct ISyncFilter(i32);
pub struct ISyncFilterDeserializer(i32);
pub struct ISyncFilterInfo(i32);
pub struct ISyncFilterInfo2(i32);
pub struct ISyncFullEnumerationChange(i32);
pub struct ISyncFullEnumerationChangeBatch(i32);
pub struct ISyncFullEnumerationChangeBatch2(i32);
pub struct ISyncKnowledge(i32);
pub struct ISyncKnowledge2(i32);
pub struct ISyncMergeTombstoneChange(i32);
pub struct ISyncProvider(i32);
pub struct ISyncProviderConfigUI(i32);
pub struct ISyncProviderConfigUIInfo(i32);
pub struct ISyncProviderInfo(i32);
pub struct ISyncProviderRegistration(i32);
pub struct ISyncRegistrationChange(i32);
pub struct ISyncSessionExtendedErrorInfo(i32);
pub struct ISyncSessionState(i32);
pub struct ISyncSessionState2(i32);
pub struct ISynchronousDataRetriever(i32);
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
