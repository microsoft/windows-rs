#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const BREADCRUMBING_UNSUPPORTED: u32 = 0u32;
pub const BREADCRUMBING_VERSION_1: u32 = 1u32;
pub const CivicAddressReport: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3550378973, data2: 32005, data3: 18104, data4: [135, 33, 128, 207, 3, 95, 87, 215] };
pub const CivicAddressReportFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 705819692, data2: 16001, data3: 19156, data4: [156, 190, 69, 87, 157, 137, 103, 26] };
pub const DefaultLocation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2340405216, data2: 23767, data3: 18762, data4: [175, 140, 40, 58, 101, 112, 117, 6] };
pub const DispCivicAddressReport: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1280928492,
    data2: 34116,
    data3: 16514,
    data4: [186, 159, 235, 10, 125, 142, 101, 198],
};
pub const DispLatLongReport: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2054959735,
    data2: 36740,
    data3: 17974,
    data4: [149, 178, 235, 181, 80, 127, 247, 126],
};
pub const GNSS_AGNSSFORMAT_LTO: u32 = 4u32;
pub const GNSS_AGNSSFORMAT_XTRA1: u32 = 1u32;
pub const GNSS_AGNSSFORMAT_XTRA2: u32 = 2u32;
pub const GNSS_AGNSSFORMAT_XTRA3: u32 = 8u32;
pub const GNSS_AGNSSFORMAT_XTRA3_1: u32 = 16u32;
pub const GNSS_AGNSSFORMAT_XTRA3_2: u32 = 32u32;
pub const GNSS_AGNSSFORMAT_XTRA_INT: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_AGNSS_INJECT(i32);
#[repr(C)]
pub struct GNSS_AGNSS_INJECTBLOB(i32);
#[repr(C)]
pub struct GNSS_AGNSS_INJECTPOSITION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_AGNSS_INJECTTIME(i32);
#[repr(C)]
pub struct GNSS_AGNSS_REQUEST_PARAM(i32);
#[repr(transparent)]
pub struct GNSS_AGNSS_REQUEST_TYPE(pub i32);
pub const GNSS_AGNSS_TimeInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(1i32);
pub const GNSS_AGNSS_PositionInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(2i32);
pub const GNSS_AGNSS_BlobInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(3i32);
#[repr(C)]
pub struct GNSS_BREADCRUMBING_ALERT_DATA(i32);
#[repr(C)]
pub struct GNSS_BREADCRUMBING_PARAM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_BREADCRUMB_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_BREADCRUMB_V1(i32);
#[repr(C)]
pub struct GNSS_CHIPSETINFO(i32);
#[repr(C)]
pub struct GNSS_CONTINUOUSTRACKING_PARAM(i32);
#[repr(C)]
pub struct GNSS_CP_NI_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_CWTESTDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_DEVICE_CAPABILITY(i32);
#[repr(C)]
pub struct GNSS_DISTANCETRACKING_PARAM(i32);
#[repr(C)]
pub struct GNSS_DRIVERCOMMAND_PARAM(i32);
#[repr(transparent)]
pub struct GNSS_DRIVERCOMMAND_TYPE(pub i32);
pub const GNSS_SetLocationServiceEnabled: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(1i32);
pub const GNSS_SetLocationNIRequestAllowed: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(2i32);
pub const GNSS_ForceSatelliteSystem: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(3i32);
pub const GNSS_ForceOperationMode: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(4i32);
pub const GNSS_ResetEngine: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(9i32);
pub const GNSS_ClearAgnssData: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(10i32);
pub const GNSS_SetSuplVersion: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(12i32);
pub const GNSS_SetNMEALogging: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(13i32);
pub const GNSS_SetUplServerAccessInterval: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(14i32);
pub const GNSS_SetNiTimeoutInterval: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(15i32);
pub const GNSS_ResetGeofencesTracking: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(16i32);
pub const GNSS_SetSuplVersion2: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(17i32);
pub const GNSS_CustomCommand: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(256i32);
#[repr(transparent)]
pub struct GNSS_DRIVER_REQUEST(pub i32);
pub const SUPL_CONFIG_DATA: GNSS_DRIVER_REQUEST = GNSS_DRIVER_REQUEST(1i32);
#[repr(C)]
pub struct GNSS_DRIVER_REQUEST_DATA(i32);
pub const GNSS_DRIVER_VERSION_1: u32 = 1u32;
pub const GNSS_DRIVER_VERSION_2: u32 = 2u32;
pub const GNSS_DRIVER_VERSION_3: u32 = 3u32;
pub const GNSS_DRIVER_VERSION_4: u32 = 4u32;
pub const GNSS_DRIVER_VERSION_5: u32 = 5u32;
pub const GNSS_DRIVER_VERSION_6: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_ERRORINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_EVENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_EVENT_2(i32);
#[repr(transparent)]
pub struct GNSS_EVENT_TYPE(pub i32);
pub const GNSS_Event_FixAvailable: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(1i32);
pub const GNSS_Event_RequireAgnss: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(2i32);
pub const GNSS_Event_Error: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(3i32);
pub const GNSS_Event_NiRequest: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(12i32);
pub const GNSS_Event_NmeaData: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(13i32);
pub const GNSS_Event_GeofenceAlertData: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(14i32);
pub const GNSS_Event_GeofencesTrackingStatus: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(15i32);
pub const GNSS_Event_DriverRequest: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(16i32);
pub const GNSS_Event_BreadcrumbAlertEvent: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(17i32);
pub const GNSS_Event_FixAvailable_2: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(18i32);
pub const GNSS_Event_Custom: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(32768i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_FIXDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_FIXDATA_2(i32);
#[repr(C)]
pub struct GNSS_FIXDATA_ACCURACY(i32);
#[repr(C)]
pub struct GNSS_FIXDATA_ACCURACY_2(i32);
#[repr(C)]
pub struct GNSS_FIXDATA_BASIC(i32);
#[repr(C)]
pub struct GNSS_FIXDATA_BASIC_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_FIXDATA_SATELLITE(i32);
pub const GNSS_FIXDETAIL_ACCURACY: u32 = 2u32;
pub const GNSS_FIXDETAIL_BASIC: u32 = 1u32;
pub const GNSS_FIXDETAIL_SATELLITE: u32 = 4u32;
#[repr(transparent)]
pub struct GNSS_FIXSESSIONTYPE(pub i32);
pub const GNSS_FixSession_SingleShot: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(1i32);
pub const GNSS_FixSession_DistanceTracking: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(2i32);
pub const GNSS_FixSession_ContinuousTracking: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(3i32);
pub const GNSS_FixSession_LKG: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(4i32);
#[repr(C)]
pub struct GNSS_FIXSESSION_PARAM(i32);
pub const GNSS_GEOFENCESUPPORT_CIRCLE: u32 = 2u32;
pub const GNSS_GEOFENCESUPPORT_SUPPORTED: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_GEOFENCES_TRACKINGSTATUS_DATA(i32);
#[repr(C)]
pub struct GNSS_GEOFENCE_ALERT_DATA(i32);
#[repr(C)]
pub struct GNSS_GEOFENCE_CREATE_PARAM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_GEOFENCE_CREATE_RESPONSE(i32);
#[repr(C)]
pub struct GNSS_GEOFENCE_DELETE_PARAM(i32);
#[repr(transparent)]
pub struct GNSS_GEOFENCE_STATE(pub i32);
pub const GNSS_GeofenceState_Unknown: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(0i32);
pub const GNSS_GeofenceState_Entered: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(1i32);
pub const GNSS_GeofenceState_Exited: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(2i32);
#[repr(C)]
pub struct GNSS_GEOREGION(i32);
#[repr(transparent)]
pub struct GNSS_GEOREGIONTYPE(pub i32);
pub const GNSS_GeoRegion_Circle: GNSS_GEOREGIONTYPE = GNSS_GEOREGIONTYPE(1i32);
#[repr(C)]
pub struct GNSS_GEOREGION_CIRCLE(i32);
#[repr(C)]
pub struct GNSS_LKGFIX_PARAM(i32);
pub const GNSS_MAXSATELLITE: u32 = 64u32;
#[repr(transparent)]
pub struct GNSS_NI_NOTIFICATION_TYPE(pub i32);
pub const GNSS_NI_NoNotifyNoVerify: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(1i32);
pub const GNSS_NI_NotifyOnly: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(2i32);
pub const GNSS_NI_NotifyVerifyDefaultAllow: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(3i32);
pub const GNSS_NI_NotifyVerifyDefaultNotAllow: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(4i32);
pub const GNSS_NI_PrivacyOverride: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(5i32);
#[repr(transparent)]
pub struct GNSS_NI_PLANE_TYPE(pub i32);
pub const GNSS_NI_SUPL: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(1i32);
pub const GNSS_NI_CP: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(2i32);
pub const GNSS_NI_V2UPL: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_NI_REQUEST_PARAM(i32);
#[repr(transparent)]
pub struct GNSS_NI_REQUEST_TYPE(pub i32);
pub const GNSS_NI_Request_SingleShot: GNSS_NI_REQUEST_TYPE = GNSS_NI_REQUEST_TYPE(1i32);
pub const GNSS_NI_Request_AreaTrigger: GNSS_NI_REQUEST_TYPE = GNSS_NI_REQUEST_TYPE(2i32);
#[repr(C)]
pub struct GNSS_NI_RESPONSE(i32);
#[repr(transparent)]
pub struct GNSS_NI_USER_RESPONSE(pub i32);
pub const GNSS_Ni_UserResponseAccept: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(1i32);
pub const GNSS_Ni_UserResponseDeny: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(2i32);
pub const GNSS_Ni_UserResponseTimeout: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(3i32);
pub const GNSS_NMEALOGGING_ALL: u32 = 255u32;
pub const GNSS_NMEALOGGING_NONE: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_NMEA_DATA(i32);
pub const GNSS_OPERMODE_AFLT: u32 = 16u32;
pub const GNSS_OPERMODE_ANY: u32 = 0u32;
pub const GNSS_OPERMODE_CELLID: u32 = 8u32;
pub const GNSS_OPERMODE_MSA: u32 = 1u32;
pub const GNSS_OPERMODE_MSB: u32 = 2u32;
pub const GNSS_OPERMODE_MSS: u32 = 4u32;
pub const GNSS_OPERMODE_OTDOA: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_PLATFORM_CAPABILITY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_SATELLITEINFO(i32);
pub const GNSS_SATELLITE_ANY: u32 = 0u32;
pub const GNSS_SATELLITE_BEIDOU: u32 = 4u32;
pub const GNSS_SATELLITE_GALILEO: u32 = 8u32;
pub const GNSS_SATELLITE_GLONASS: u32 = 2u32;
pub const GNSS_SATELLITE_GPS: u32 = 1u32;
#[repr(C)]
pub struct GNSS_SELFTESTCONFIG(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_SELFTESTRESULT(i32);
#[repr(C)]
pub struct GNSS_SINGLESHOT_PARAM(i32);
#[repr(C)]
pub struct GNSS_STOPFIXSESSION_PARAM(i32);
#[repr(transparent)]
pub struct GNSS_SUPL_CERT_ACTION(pub i32);
pub const GNSS_Supl_Cert_Inject: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(1i32);
pub const GNSS_Supl_Cert_Delete: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(2i32);
pub const GNSS_Supl_Cert_Purge: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_SUPL_CERT_CONFIG(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_SUPL_HSLP_CONFIG(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_SUPL_NI_INFO(i32);
#[repr(C)]
pub struct GNSS_SUPL_VERSION(i32);
#[repr(C)]
pub struct GNSS_SUPL_VERSION_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GNSS_V2UPL_CONFIG(i32);
#[repr(C)]
pub struct GNSS_V2UPL_NI_INFO(i32);
pub const GUID_DEVINTERFACE_GNSS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 859235812, data2: 394, data3: 18025, data4: [132, 197, 189, 5, 243, 189, 54, 139] };
#[repr(transparent)]
pub struct ICivicAddressReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICivicAddressReportFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDefaultLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispCivicAddressReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispLatLongReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILatLongReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILatLongReportFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocationEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocationPower(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocationReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocationReportFactory(pub *mut ::core::ffi::c_void);
pub const IOCTL_GNSS_CONFIG_SUPL_CERT: u32 = 2228488u32;
pub const IOCTL_GNSS_CREATE_GEOFENCE: u32 = 2228544u32;
pub const IOCTL_GNSS_DELETE_GEOFENCE: u32 = 2228548u32;
pub const IOCTL_GNSS_EXECUTE_CWTEST: u32 = 2228496u32;
pub const IOCTL_GNSS_EXECUTE_SELFTEST: u32 = 2228500u32;
pub const IOCTL_GNSS_GET_CHIPSETINFO: u32 = 2228504u32;
pub const IOCTL_GNSS_GET_DEVICE_CAPABILITY: u32 = 2228232u32;
pub const IOCTL_GNSS_GET_FIXDATA: u32 = 2228300u32;
pub const IOCTL_GNSS_INJECT_AGNSS: u32 = 2228352u32;
pub const IOCTL_GNSS_LISTEN_AGNSS: u32 = 2228416u32;
pub const IOCTL_GNSS_LISTEN_BREADCRUMBING_ALERT: u32 = 2228680u32;
pub const IOCTL_GNSS_LISTEN_DRIVER_REQUEST: u32 = 2228608u32;
pub const IOCTL_GNSS_LISTEN_ERROR: u32 = 2228420u32;
pub const IOCTL_GNSS_LISTEN_GEOFENCES_TRACKINGSTATUS: u32 = 2228556u32;
pub const IOCTL_GNSS_LISTEN_GEOFENCE_ALERT: u32 = 2228552u32;
pub const IOCTL_GNSS_LISTEN_NI: u32 = 2228480u32;
pub const IOCTL_GNSS_LISTEN_NMEA: u32 = 2228508u32;
pub const IOCTL_GNSS_MODIFY_FIXSESSION: u32 = 2228292u32;
pub const IOCTL_GNSS_POP_BREADCRUMBS: u32 = 2228684u32;
pub const IOCTL_GNSS_RESPOND_NI: u32 = 2228492u32;
pub const IOCTL_GNSS_SEND_DRIVERCOMMAND: u32 = 2228236u32;
pub const IOCTL_GNSS_SEND_PLATFORM_CAPABILITY: u32 = 2228228u32;
pub const IOCTL_GNSS_SET_SUPL_HSLP: u32 = 2228484u32;
pub const IOCTL_GNSS_SET_V2UPL_CONFIG: u32 = 2228512u32;
pub const IOCTL_GNSS_START_BREADCRUMBING: u32 = 2228672u32;
pub const IOCTL_GNSS_START_FIXSESSION: u32 = 2228288u32;
pub const IOCTL_GNSS_STOP_BREADCRUMBING: u32 = 2228676u32;
pub const IOCTL_GNSS_STOP_FIXSESSION: u32 = 2228296u32;
pub const LOCATION_API_VERSION: u32 = 1u32;
#[repr(transparent)]
pub struct LOCATION_REPORT_STATUS(pub i32);
pub const REPORT_NOT_SUPPORTED: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(0i32);
pub const REPORT_ERROR: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(1i32);
pub const REPORT_ACCESS_DENIED: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(2i32);
pub const REPORT_INITIALIZING: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(3i32);
pub const REPORT_RUNNING: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(4i32);
pub const LatLongReport: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3984703603, data2: 8068, data3: 19624, data4: [161, 97, 24, 60, 119, 107, 198, 81] };
pub const LatLongReportFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2647407816, data2: 34313, data3: 18531, data4: [186, 212, 3, 96, 31, 76, 101, 232] };
pub const Location: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3854098553,
    data2: 61037,
    data3: 20019,
    data4: [164, 56, 200, 127, 46, 149, 146, 84],
};
pub const MAX_SERVER_URL_NAME: u32 = 260u32;
pub const MIN_BREADCRUMBS_SUPPORTED: u32 = 120u32;
pub const MIN_GEOFENCES_REQUIRED: u32 = 100u32;
#[repr(transparent)]
pub struct _ICivicAddressReportFactoryEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _ILatLongReportFactoryEvents(pub *mut ::core::ffi::c_void);
