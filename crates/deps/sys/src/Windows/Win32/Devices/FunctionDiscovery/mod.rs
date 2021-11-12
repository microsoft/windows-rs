#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const E_FDPAIRING_AUTHFAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1882193917i32 as _);
pub const E_FDPAIRING_AUTHNOTALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1882193914i32 as _);
pub const E_FDPAIRING_CONNECTTIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1882193916i32 as _);
pub const E_FDPAIRING_HWFAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1882193918i32 as _);
pub const E_FDPAIRING_IPBUSDISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1882193913i32 as _);
pub const E_FDPAIRING_NOCONNECTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1882193919i32 as _);
pub const E_FDPAIRING_NOPROFILES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1882193912i32 as _);
pub const E_FDPAIRING_TOOMANYCONNECTIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1882193915i32 as _);
pub const FD_EVENTID: u32 = 1000u32;
pub const FD_EVENTID_ASYNCTHREADEXIT: u32 = 1001u32;
pub const FD_EVENTID_IPADDRESSCHANGE: u32 = 1003u32;
pub const FD_EVENTID_PRIVATE: u32 = 100u32;
pub const FD_EVENTID_QUERYREFRESH: u32 = 1004u32;
pub const FD_EVENTID_SEARCHCOMPLETE: u32 = 1000u32;
pub const FD_EVENTID_SEARCHSTART: u32 = 1002u32;
pub const FD_LONGHORN: u32 = 1u32;
pub const FD_Visibility_Default: u32 = 0u32;
pub const FD_Visibility_Hidden: u32 = 1u32;
pub const FMTID_Device: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] };
pub const FMTID_DeviceInterface: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1400930312, data2: 1979, data3: 18017, data4: [188, 60, 181, 149, 62, 112, 133, 96] };
pub const FMTID_FD: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2420835234, data2: 18205, data3: 16956, data4: [165, 132, 243, 72, 50, 56, 161, 70] };
pub const FMTID_PNPX: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1701460915,
    data2: 60608,
    data3: 17405,
    data4: [132, 119, 74, 224, 64, 74, 150, 205],
};
pub const FMTID_PNPXDynamicProperty: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1338312574,
    data2: 46726,
    data3: 17598,
    data4: [147, 227, 134, 202, 254, 54, 140, 205],
};
pub const FMTID_Pairing: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2282212070,
    data2: 32182,
    data3: 20240,
    data4: [142, 228, 67, 94, 170, 19, 146, 188],
};
pub const FMTID_WSD: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2454742161,
    data2: 65429,
    data3: 18212,
    data4: [160, 90, 91, 129, 136, 90, 124, 146],
};
pub const FunctionDiscovery: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3341542124,
    data2: 36496,
    data3: 17708,
    data4: [178, 154, 171, 143, 241, 192, 113, 252],
};
pub const FunctionInstanceCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3129052389, data2: 46431, data3: 17471, data4: [173, 57, 47, 232, 155, 230, 25, 31] };
#[repr(transparent)]
pub struct IFunctionDiscovery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFunctionDiscoveryNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFunctionDiscoveryProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFunctionDiscoveryProviderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFunctionDiscoveryProviderQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFunctionDiscoveryServiceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFunctionInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFunctionInstanceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFunctionInstanceCollectionQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFunctionInstanceQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPNPXAssociation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPNPXDeviceAssociation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyStoreCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProviderProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProviderPropertyConstraintCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProviderPublishing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProviderQueryConstraintCollection(pub *mut ::core::ffi::c_void);
pub const MAX_FDCONSTRAINTNAME_LENGTH: u32 = 100u32;
pub const MAX_FDCONSTRAINTVALUE_LENGTH: u32 = 1000u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Characteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1126273419,
        data2: 63134,
        data3: 18189,
        data4: [165, 222, 77, 136, 199, 90, 210, 75],
    },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassCoInstallers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1899828995, data2: 41698, data3: 18933, data4: [146, 20, 86, 71, 46, 243, 218, 92] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassInstaller: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_DefaultService: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_DevType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1126273419,
        data2: 63134,
        data3: 18189,
        data4: [165, 222, 77, 136, 199, 90, 210, 75],
    },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Exclusive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1126273419,
        data2: 63134,
        data3: 18189,
        data4: [165, 222, 77, 136, 199, 90, 210, 75],
    },
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_LowerFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1126273419,
        data2: 63134,
        data3: 18189,
        data4: [165, 222, 77, 136, 199, 90, 210, 75],
    },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoDisplayClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoInstallClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoUseClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_PropPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1126273419,
        data2: 63134,
        data3: 18189,
        data4: [165, 222, 77, 136, 199, 90, 210, 75],
    },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_SecuritySDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1126273419,
        data2: 63134,
        data3: 18189,
        data4: [165, 222, 77, 136, 199, 90, 210, 75],
    },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_SilentInstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 630898684, data2: 20647, data3: 18382, data4: [175, 8, 104, 201, 167, 215, 51, 102] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_UpperFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1126273419,
        data2: 63134,
        data3: 18189,
        data4: [165, 222, 77, 136, 199, 90, 210, 75],
    },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Address: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 51u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_AlwaysShowDeviceAsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_AssociationArray: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 80u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_BaselineExperienceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 78u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 90u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_CategoryGroup_Desc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 94u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_CategoryGroup_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 95u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Desc_Plural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 92u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Desc_Singular: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 91u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 93u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceDescription1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 81u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceDescription2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 82u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceFunctionSubRank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DiscoveryMethod: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 52u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ExperienceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 89u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12288u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 57u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_InstallInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2212127526, data2: 38822, data3: 16520, data4: [148, 83, 161, 146, 63, 87, 59, 41] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsAuthenticated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 54u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 55u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsDefaultDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 86u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsDeviceUniquelyIdentifiable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 79u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsEncrypted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 53u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsLocalMachine: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 70u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsMetadataSearchInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 72u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNetworkDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 85u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNotInterestingForDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 74u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNotWorkingProperly: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 83u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 56u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsSharedDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 84u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsShowInDisconnectedState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 68u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Last_Connected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 67u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Last_Seen: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 66u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageFromExplorer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 77u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageOnDeviceConnect: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 76u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8192u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataCabinet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 87u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataChecksum: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 73u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 71u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8194u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ModelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8195u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_PrimaryCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 97u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_RequiresPairingElevation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 88u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_RequiresUninstallElevation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 99u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_UnpairUninstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 98u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 65u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterfaceClass_DefaultInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 348666521, data2: 2879, data3: 17591, data4: [190, 76, 161, 120, 211, 153, 5, 100] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 40784238, data2: 47124, data3: 16715, data4: [131, 205, 133, 109, 111, 239, 72, 34] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Enabled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 40784238, data2: 47124, data3: 16715, data4: [131, 205, 133, 109, 111, 239, 72, 34] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 40784238, data2: 47124, data3: 16715, data4: [131, 205, 133, 109, 111, 239, 72, 34] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_AdditionalSoftwareRequested: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Address: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BIOSVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3941498653, data2: 27187, data3: 17617, data4: [148, 65, 95, 70, 222, 242, 49, 152] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BaseContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusReportedDeviceDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1410045054,
        data2: 35648,
        data3: 17852,
        data4: [168, 162, 106, 11, 137, 76, 189, 162],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusTypeGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Capabilities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Characteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Class: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_CompatibleIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ConfigFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2357121542,
        data2: 16266,
        data3: 18471,
        data4: [179, 171, 174, 158, 31, 174, 252, 108],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DHP_Rebalance_Policy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1410045054,
        data2: 35648,
        data3: 17852,
        data4: [168, 162, 106, 11, 137, 76, 189, 162],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DevNodeStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DevType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DeviceDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Driver: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverCoInstallers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfSection: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfSectionExt: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverLogoLevel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverPropPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverRank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_EjectionRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_EnumeratorName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Exclusive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_FriendlyNameAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2161647270, data2: 29811, data3: 19212, data4: [130, 22, 239, 193, 26, 44, 76, 139] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_GenericDriverInstalled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_HardwareIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstallInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2212127526, data2: 38822, data3: 16520, data4: [148, 83, 161, 146, 63, 87, 59, 41] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstallState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstanceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2026065864, data2: 4170, data3: 19146, data4: [158, 164, 82, 77, 82, 153, 110, 87] },
    pid: 256u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_IsAssociateableByUserAction: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2161647270, data2: 29811, data3: 19212, data4: [130, 22, 239, 193, 26, 44, 76, 139] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Legacy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2152296704,
        data2: 35955,
        data3: 18617,
        data4: [170, 217, 206, 56, 126, 25, 197, 110],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LegacyBusType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LocationInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LocationPaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LowerFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ManufacturerAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2161647270, data2: 29811, data3: 19212, data4: [130, 22, 239, 193, 26, 44, 76, 139] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_MatchingDeviceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ModelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2161647270, data2: 29811, data3: 19212, data4: [130, 22, 239, 193, 26, 44, 76, 139] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_NoConnectSound: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Numa_Node: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1410045054,
        data2: 35648,
        data3: 17852,
        data4: [168, 162, 106, 11, 137, 76, 189, 162],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PDOName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Parent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PowerData: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PowerRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PresenceNotForDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2161647270, data2: 29811, data3: 19212, data4: [130, 22, 239, 193, 26, 44, 76, 139] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ProblemCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicyDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicyOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Reported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2152296704,
        data2: 35955,
        data3: 18617,
        data4: [170, 217, 206, 56, 126, 25, 197, 110],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ResourcePickerExceptions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ResourcePickerTags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2830656989,
        data2: 11837,
        data3: 16532,
        data4: [173, 151, 229, 147, 167, 12, 117, 214],
    },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SafeRemovalRequired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2950264384,
        data2: 34467,
        data3: 16912,
        data4: [182, 124, 40, 156, 65, 170, 190, 85],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SafeRemovalRequiredOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2950264384,
        data2: 34467,
        data3: 16912,
        data4: [182, 124, 40, 156, 65, 170, 190, 85],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SecuritySDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Service: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Siblings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SignalStrength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2161647270, data2: 29811, data3: 19212, data4: [130, 22, 239, 193, 26, 44, 76, 139] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_TransportRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1128310469,
        data2: 37882,
        data3: 18182,
        data4: [151, 44, 123, 100, 128, 8, 165, 167],
    },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UINumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UINumberDescFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UpperFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2757502286,
        data2: 57116,
        data3: 20221,
        data4: [128, 32, 103, 209, 70, 168, 80, 224],
    },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_BrandingIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3480468305,
        data2: 15039,
        data3: 17570,
        data4: [133, 224, 154, 61, 199, 161, 33, 50],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_DetailedDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3480468305,
        data2: 15039,
        data3: 17570,
        data4: [133, 224, 154, 61, 199, 161, 33, 50],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_DocumentationLink: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3480468305,
        data2: 15039,
        data3: 17570,
        data4: [133, 224, 154, 61, 199, 161, 33, 50],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3480468305,
        data2: 15039,
        data3: 17570,
        data4: [133, 224, 154, 61, 199, 161, 33, 50],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_Model: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3480468305,
        data2: 15039,
        data3: 17570,
        data4: [133, 224, 154, 61, 199, 161, 33, 50],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_VendorWebSite: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3480468305,
        data2: 15039,
        data3: 17570,
        data4: [133, 224, 154, 61, 199, 161, 33, 50],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FunctionInstance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 146850387, data2: 41300, data3: 18246, data4: [144, 5, 130, 222, 83, 23, 20, 139] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Devinst: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 4097u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DisplayAttribute: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Function: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 4099u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Image: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 4098u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Model: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_SerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_ShellAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 4100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1588543218, data2: 57546, data3: 17816, data4: [191, 6, 113, 237, 29, 157, 217, 83] },
    pid: 4096u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Numa_Proximity_Domain: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1410045054,
        data2: 35648,
        data3: 17852,
        data4: [168, 162, 106, 11, 137, 76, 189, 162],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Associated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1338312574,
        data2: 46726,
        data3: 17598,
        data4: [147, 227, 134, 202, 254, 54, 140, 205],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Category_Desc_NonPlural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12304u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_CompactSignature: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 28674u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_CompatibleTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1338312574,
        data2: 46726,
        data3: 17598,
        data4: [147, 227, 134, 202, 254, 54, 140, 205],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12292u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCategory_Desc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12293u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCertHash: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 28675u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DomainName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 20480u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_FirmwareVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12289u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_GlobalIdentity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 4096u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 4101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_IPBusEnumerated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 28688u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_InstallState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1338312574,
        data2: 46726,
        data3: 17598,
        data4: [147, 227, 134, 202, 254, 54, 140, 205],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Installable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1338312574,
        data2: 46726,
        data3: 17598,
        data4: [147, 227, 134, 202, 254, 54, 140, 205],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_IpAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12297u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ManufacturerUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8193u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_MetadataVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 4100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ModelUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8196u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_NetworkInterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12296u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_NetworkInterfaceLuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12295u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_PhysicalAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12294u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_PresentationUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8198u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_RemoteAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 4102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Removable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 28672u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_RootProxy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 4103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Scopes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 4098u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_SecureChannel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 28673u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_SerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 12290u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 16384u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceControlUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 16388u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceDescUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 16389u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceEventSubUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 16390u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 16385u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 16386u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ShareName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 20482u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Types: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 4097u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Upc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 8197u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_XAddrs: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 4099u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_IsWifiOnlyDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2282212070,
        data2: 32182,
        data3: 20240,
        data4: [142, 228, 67, 94, 170, 19, 146, 188],
    },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2282212070,
        data2: 32182,
        data3: 20240,
        data4: [142, 228, 67, 94, 170, 19, 146, 188],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2282212070,
        data2: 32182,
        data3: 20240,
        data4: [142, 228, 67, 94, 170, 19, 146, 188],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2282212070,
        data2: 32182,
        data3: 20240,
        data4: [142, 228, 67, 94, 170, 19, 146, 188],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 2282212070,
        data2: 32182,
        data3: 20240,
        data4: [142, 228, 67, 94, 170, 19, 146, 188],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_AltLocationInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 24576u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_DevLifeTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 24577u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_NetworkInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 1701460915,
        data2: 60608,
        data3: 17405,
        data4: [132, 119, 74, 224, 64, 74, 150, 205],
    },
    pid: 24578u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_AssocState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342728, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_AuthType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342722, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigError: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342729, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigMethods: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342725, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342729, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConnType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342724, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_DevicePasswordId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342729, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_EncryptType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342723, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_OSVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342729, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RegistrarType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342731, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RequestType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342721, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RfBand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342727, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_VendorExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342730, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2283342720, data2: 18052, data3: 4570, data4: [162, 106, 0, 2, 179, 152, 142, 129] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Comment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3736970298,
        data2: 14259,
        data3: 17283,
        data4: [145, 231, 68, 152, 218, 41, 149, 171],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_DisplayType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3736970298,
        data2: 14259,
        data3: 17283,
        data4: [145, 231, 68, 152, 218, 41, 149, 171],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_LocalName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3736970298,
        data2: 14259,
        data3: 17283,
        data4: [145, 231, 68, 152, 218, 41, 149, 171],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Provider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3736970298,
        data2: 14259,
        data3: 17283,
        data4: [145, 231, 68, 152, 218, 41, 149, 171],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_RemoteName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3736970298,
        data2: 14259,
        data3: 17283,
        data4: [145, 231, 68, 152, 218, 41, 149, 171],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Scope: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3736970298,
        data2: 14259,
        data3: 17283,
        data4: [145, 231, 68, 152, 218, 41, 149, 171],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Type: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3736970298,
        data2: 14259,
        data3: 17283,
        data4: [145, 231, 68, 152, 218, 41, 149, 171],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Usage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3736970298,
        data2: 14259,
        data3: 17283,
        data4: [145, 231, 68, 152, 218, 41, 149, 171],
    },
    pid: 4u32,
};
pub const PNPXAssociation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3471363273, data2: 20331, data3: 17513, data4: [162, 53, 90, 34, 134, 158, 239, 3] };
pub const PNPXPairingHandler: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3097655618,
    data2: 44519,
    data3: 16517,
    data4: [170, 110, 79, 173, 199, 173, 161, 239],
};
pub const PNPX_INSTALLSTATE_FAILED: u32 = 3u32;
pub const PNPX_INSTALLSTATE_INSTALLED: u32 = 1u32;
pub const PNPX_INSTALLSTATE_INSTALLING: u32 = 2u32;
pub const PNPX_INSTALLSTATE_NOTINSTALLED: u32 = 0u32;
#[repr(transparent)]
pub struct PropertyConstraint(pub i32);
pub const QC_EQUALS: PropertyConstraint = PropertyConstraint(0i32);
pub const QC_NOTEQUAL: PropertyConstraint = PropertyConstraint(1i32);
pub const QC_LESSTHAN: PropertyConstraint = PropertyConstraint(2i32);
pub const QC_LESSTHANOREQUAL: PropertyConstraint = PropertyConstraint(3i32);
pub const QC_GREATERTHAN: PropertyConstraint = PropertyConstraint(4i32);
pub const QC_GREATERTHANOREQUAL: PropertyConstraint = PropertyConstraint(5i32);
pub const QC_STARTSWITH: PropertyConstraint = PropertyConstraint(6i32);
pub const QC_EXISTS: PropertyConstraint = PropertyConstraint(7i32);
pub const QC_DOESNOTEXIST: PropertyConstraint = PropertyConstraint(8i32);
pub const QC_CONTAINS: PropertyConstraint = PropertyConstraint(9i32);
impl ::core::marker::Copy for PropertyConstraint {}
impl ::core::clone::Clone for PropertyConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PropertyStore: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3833161040, data2: 57185, data3: 17547, data4: [145, 147, 19, 252, 19, 65, 177, 99] };
pub const PropertyStoreCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3990052905, data2: 55123, data3: 18530, data4: [170, 91, 91, 204, 173, 42, 77, 41] };
#[repr(transparent)]
pub struct QueryCategoryType(pub i32);
pub const QCT_PROVIDER: QueryCategoryType = QueryCategoryType(0i32);
pub const QCT_LAYERED: QueryCategoryType = QueryCategoryType(1i32);
impl ::core::marker::Copy for QueryCategoryType {}
impl ::core::clone::Clone for QueryCategoryType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QueryUpdateAction(pub i32);
pub const QUA_ADD: QueryUpdateAction = QueryUpdateAction(0i32);
pub const QUA_REMOVE: QueryUpdateAction = QueryUpdateAction(1i32);
pub const QUA_CHANGE: QueryUpdateAction = QueryUpdateAction(2i32);
impl ::core::marker::Copy for QueryUpdateAction {}
impl ::core::clone::Clone for QueryUpdateAction {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SID_DeviceDisplayStatusManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4120552787, data2: 33545, data3: 18122, data4: [151, 54, 26, 195, 198, 45, 96, 49] };
pub const SID_EnumDeviceFunction: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 333507042,
    data2: 50170,
    data3: 20028,
    data4: [144, 110, 100, 80, 47, 164, 220, 149],
};
pub const SID_EnumInterface: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1089122489,
    data2: 19839,
    data3: 19283,
    data4: [163, 52, 21, 129, 221, 144, 65, 244],
};
pub const SID_FDPairingHandler: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 943417850,
    data2: 21638,
    data3: 18906,
    data4: [145, 245, 214, 60, 36, 200, 233, 208],
};
pub const SID_FunctionDiscoveryProviderRefresh: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 726449609, data2: 12740, data3: 16596, data4: [166, 45, 119, 42, 161, 116, 237, 82] };
pub const SID_PNPXAssociation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3471363273, data2: 20331, data3: 17513, data4: [162, 53, 90, 34, 134, 158, 239, 3] };
pub const SID_PNPXPropertyStore: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2825203889,
    data2: 21551,
    data3: 17311,
    data4: [183, 28, 176, 117, 107, 19, 103, 122],
};
pub const SID_PNPXServiceCollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1134461166,
    data2: 41495,
    data3: 18194,
    data4: [159, 166, 222, 171, 217, 194, 167, 39],
};
pub const SID_PnpProvider: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2164340366, data2: 51899, data3: 17446, data4: [172, 255, 150, 196, 16, 129, 32, 0] };
pub const SID_UPnPActivator: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 218982123, data2: 53108, data3: 16740, data4: [181, 47, 8, 52, 70, 114, 221, 70] };
pub const SID_UninstallDeviceFunction: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3374339694,
    data2: 22129,
    data3: 17558,
    data4: [128, 37, 191, 11, 137, 189, 68, 205],
};
pub const SID_UnpairProvider: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2309292796, data2: 34171, data3: 18072, data4: [160, 183, 2, 113, 146, 0, 47, 158] };
#[repr(transparent)]
pub struct SystemVisibilityFlags(pub i32);
pub const SVF_SYSTEM: SystemVisibilityFlags = SystemVisibilityFlags(0i32);
pub const SVF_USER: SystemVisibilityFlags = SystemVisibilityFlags(1i32);
impl ::core::marker::Copy for SystemVisibilityFlags {}
impl ::core::clone::Clone for SystemVisibilityFlags {
    fn clone(&self) -> Self {
        *self
    }
}
