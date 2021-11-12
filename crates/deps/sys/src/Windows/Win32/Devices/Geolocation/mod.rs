#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const BREADCRUMBING_UNSUPPORTED: u32 = 0u32;
pub const BREADCRUMBING_VERSION_1: u32 = 1u32;
pub struct CivicAddressReport(i32);
pub struct CivicAddressReportFactory(i32);
pub struct DefaultLocation(i32);
pub struct DispCivicAddressReport(i32);
pub struct DispLatLongReport(i32);
pub const GNSS_AGNSSFORMAT_LTO: u32 = 4u32;
pub const GNSS_AGNSSFORMAT_XTRA1: u32 = 1u32;
pub const GNSS_AGNSSFORMAT_XTRA2: u32 = 2u32;
pub const GNSS_AGNSSFORMAT_XTRA3: u32 = 8u32;
pub const GNSS_AGNSSFORMAT_XTRA3_1: u32 = 16u32;
pub const GNSS_AGNSSFORMAT_XTRA3_2: u32 = 32u32;
pub const GNSS_AGNSSFORMAT_XTRA_INT: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_AGNSS_INJECT(i32);
pub struct GNSS_AGNSS_INJECTBLOB(i32);
pub struct GNSS_AGNSS_INJECTPOSITION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_AGNSS_INJECTTIME(i32);
pub struct GNSS_AGNSS_REQUEST_PARAM(i32);
pub struct GNSS_AGNSS_REQUEST_TYPE(i32);
pub struct GNSS_BREADCRUMBING_ALERT_DATA(i32);
pub struct GNSS_BREADCRUMBING_PARAM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_BREADCRUMB_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_BREADCRUMB_V1(i32);
pub struct GNSS_CHIPSETINFO(i32);
pub struct GNSS_CONTINUOUSTRACKING_PARAM(i32);
pub struct GNSS_CP_NI_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_CWTESTDATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_DEVICE_CAPABILITY(i32);
pub struct GNSS_DISTANCETRACKING_PARAM(i32);
pub struct GNSS_DRIVERCOMMAND_PARAM(i32);
pub struct GNSS_DRIVERCOMMAND_TYPE(i32);
pub struct GNSS_DRIVER_REQUEST(i32);
pub struct GNSS_DRIVER_REQUEST_DATA(i32);
pub const GNSS_DRIVER_VERSION_1: u32 = 1u32;
pub const GNSS_DRIVER_VERSION_2: u32 = 2u32;
pub const GNSS_DRIVER_VERSION_3: u32 = 3u32;
pub const GNSS_DRIVER_VERSION_4: u32 = 4u32;
pub const GNSS_DRIVER_VERSION_5: u32 = 5u32;
pub const GNSS_DRIVER_VERSION_6: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_ERRORINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_EVENT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_EVENT_2(i32);
pub struct GNSS_EVENT_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA_2(i32);
pub struct GNSS_FIXDATA_ACCURACY(i32);
pub struct GNSS_FIXDATA_ACCURACY_2(i32);
pub struct GNSS_FIXDATA_BASIC(i32);
pub struct GNSS_FIXDATA_BASIC_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA_SATELLITE(i32);
pub const GNSS_FIXDETAIL_ACCURACY: u32 = 2u32;
pub const GNSS_FIXDETAIL_BASIC: u32 = 1u32;
pub const GNSS_FIXDETAIL_SATELLITE: u32 = 4u32;
pub struct GNSS_FIXSESSIONTYPE(i32);
pub struct GNSS_FIXSESSION_PARAM(i32);
pub const GNSS_GEOFENCESUPPORT_CIRCLE: u32 = 2u32;
pub const GNSS_GEOFENCESUPPORT_SUPPORTED: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCES_TRACKINGSTATUS_DATA(i32);
pub struct GNSS_GEOFENCE_ALERT_DATA(i32);
pub struct GNSS_GEOFENCE_CREATE_PARAM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCE_CREATE_RESPONSE(i32);
pub struct GNSS_GEOFENCE_DELETE_PARAM(i32);
pub struct GNSS_GEOFENCE_STATE(i32);
pub struct GNSS_GEOREGION(i32);
pub struct GNSS_GEOREGIONTYPE(i32);
pub struct GNSS_GEOREGION_CIRCLE(i32);
pub struct GNSS_LKGFIX_PARAM(i32);
pub const GNSS_MAXSATELLITE: u32 = 64u32;
pub struct GNSS_NI_NOTIFICATION_TYPE(i32);
pub struct GNSS_NI_PLANE_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_NI_REQUEST_PARAM(i32);
pub struct GNSS_NI_REQUEST_TYPE(i32);
pub struct GNSS_NI_RESPONSE(i32);
pub struct GNSS_NI_USER_RESPONSE(i32);
pub const GNSS_NMEALOGGING_ALL: u32 = 255u32;
pub const GNSS_NMEALOGGING_NONE: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_NMEA_DATA(i32);
pub const GNSS_OPERMODE_AFLT: u32 = 16u32;
pub const GNSS_OPERMODE_ANY: u32 = 0u32;
pub const GNSS_OPERMODE_CELLID: u32 = 8u32;
pub const GNSS_OPERMODE_MSA: u32 = 1u32;
pub const GNSS_OPERMODE_MSB: u32 = 2u32;
pub const GNSS_OPERMODE_MSS: u32 = 4u32;
pub const GNSS_OPERMODE_OTDOA: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_PLATFORM_CAPABILITY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SATELLITEINFO(i32);
pub const GNSS_SATELLITE_ANY: u32 = 0u32;
pub const GNSS_SATELLITE_BEIDOU: u32 = 4u32;
pub const GNSS_SATELLITE_GALILEO: u32 = 8u32;
pub const GNSS_SATELLITE_GLONASS: u32 = 2u32;
pub const GNSS_SATELLITE_GPS: u32 = 1u32;
pub struct GNSS_SELFTESTCONFIG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SELFTESTRESULT(i32);
pub struct GNSS_SINGLESHOT_PARAM(i32);
pub struct GNSS_STOPFIXSESSION_PARAM(i32);
pub struct GNSS_SUPL_CERT_ACTION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_CERT_CONFIG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_HSLP_CONFIG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_NI_INFO(i32);
pub struct GNSS_SUPL_VERSION(i32);
pub struct GNSS_SUPL_VERSION_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_V2UPL_CONFIG(i32);
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
pub struct LOCATION_REPORT_STATUS(i32);
pub struct LatLongReport(i32);
pub struct LatLongReportFactory(i32);
pub struct Location(i32);
pub const MAX_SERVER_URL_NAME: u32 = 260u32;
pub const MIN_BREADCRUMBS_SUPPORTED: u32 = 120u32;
pub const MIN_GEOFENCES_REQUIRED: u32 = 100u32;
#[repr(transparent)]
pub struct _ICivicAddressReportFactoryEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _ILatLongReportFactoryEvents(pub *mut ::core::ffi::c_void);
