#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const BREADCRUMBING_UNSUPPORTED: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const BREADCRUMBING_VERSION_1: u32 = 1u32;
pub const CivicAddressReport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd39e7bdd_7d05_46b8_8721_80cf035f57d7);
pub const CivicAddressReportFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a11f42c_3e81_4ad4_9cbe_45579d89671a);
pub const DefaultLocation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b7fbfe0_5cd7_494a_af8c_283a65707506);
pub const DispCivicAddressReport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c596aec_8544_4082_ba9f_eb0a7d8e65c6);
pub const DispLatLongReport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a7c3277_8f84_4636_95b2_ebb5507ff77e);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSSFORMAT_LTO: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSSFORMAT_XTRA1: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSSFORMAT_XTRA2: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSSFORMAT_XTRA3: u32 = 8u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSSFORMAT_XTRA3_1: u32 = 16u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSSFORMAT_XTRA3_2: u32 = 32u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSSFORMAT_XTRA_INT: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_AGNSS_INJECT {
    pub Size: u32,
    pub Version: u32,
    pub InjectionType: GNSS_AGNSS_REQUEST_TYPE,
    pub InjectionStatus: super::super::Foundation::NTSTATUS,
    pub InjectionDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_AGNSS_INJECT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_AGNSS_INJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_AGNSS_INJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_AGNSS_INJECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_AGNSS_INJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_AGNSS_INJECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_AGNSS_INJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_AGNSS_INJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_AGNSS_INJECT_0 {
    pub Time: GNSS_AGNSS_INJECTTIME,
    pub Position: GNSS_AGNSS_INJECTPOSITION,
    pub BlobData: GNSS_AGNSS_INJECTBLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_AGNSS_INJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_AGNSS_INJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_AGNSS_INJECT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_AGNSS_INJECT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_AGNSS_INJECT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_AGNSS_INJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_AGNSS_INJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_AGNSS_INJECTBLOB {
    pub Size: u32,
    pub Version: u32,
    pub BlobOui: u32,
    pub BlobVersion: u32,
    pub AgnssFormat: u32,
    pub BlobSize: u32,
    pub BlobData: [u8; 1],
}
impl ::core::marker::Copy for GNSS_AGNSS_INJECTBLOB {}
impl ::core::clone::Clone for GNSS_AGNSS_INJECTBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_AGNSS_INJECTBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_AGNSS_INJECTBLOB").field("Size", &self.Size).field("Version", &self.Version).field("BlobOui", &self.BlobOui).field("BlobVersion", &self.BlobVersion).field("AgnssFormat", &self.AgnssFormat).field("BlobSize", &self.BlobSize).field("BlobData", &self.BlobData).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_AGNSS_INJECTBLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_AGNSS_INJECTBLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_AGNSS_INJECTBLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_AGNSS_INJECTBLOB {}
impl ::core::default::Default for GNSS_AGNSS_INJECTBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_AGNSS_INJECTPOSITION {
    pub Size: u32,
    pub Version: u32,
    pub Age: u32,
    pub BasicData: GNSS_FIXDATA_BASIC,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY,
}
impl ::core::marker::Copy for GNSS_AGNSS_INJECTPOSITION {}
impl ::core::clone::Clone for GNSS_AGNSS_INJECTPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_AGNSS_INJECTPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_AGNSS_INJECTPOSITION").field("Size", &self.Size).field("Version", &self.Version).field("Age", &self.Age).field("BasicData", &self.BasicData).field("AccuracyData", &self.AccuracyData).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_AGNSS_INJECTPOSITION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_AGNSS_INJECTPOSITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_AGNSS_INJECTPOSITION>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_AGNSS_INJECTPOSITION {}
impl ::core::default::Default for GNSS_AGNSS_INJECTPOSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_AGNSS_INJECTTIME {
    pub Size: u32,
    pub Version: u32,
    pub UtcTime: super::super::Foundation::FILETIME,
    pub TimeUncertainty: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_AGNSS_INJECTTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_AGNSS_INJECTTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_AGNSS_INJECTTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_AGNSS_INJECTTIME").field("Size", &self.Size).field("Version", &self.Version).field("UtcTime", &self.UtcTime).field("TimeUncertainty", &self.TimeUncertainty).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_AGNSS_INJECTTIME {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_AGNSS_INJECTTIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_AGNSS_INJECTTIME>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_AGNSS_INJECTTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_AGNSS_INJECTTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_AGNSS_REQUEST_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub RequestType: GNSS_AGNSS_REQUEST_TYPE,
    pub BlobFormat: u32,
}
impl ::core::marker::Copy for GNSS_AGNSS_REQUEST_PARAM {}
impl ::core::clone::Clone for GNSS_AGNSS_REQUEST_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_AGNSS_REQUEST_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_AGNSS_REQUEST_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("RequestType", &self.RequestType).field("BlobFormat", &self.BlobFormat).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_AGNSS_REQUEST_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_AGNSS_REQUEST_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_AGNSS_REQUEST_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_AGNSS_REQUEST_PARAM {}
impl ::core::default::Default for GNSS_AGNSS_REQUEST_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_AGNSS_REQUEST_TYPE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSS_TimeInjection: GNSS_AGNSS_REQUEST_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSS_PositionInjection: GNSS_AGNSS_REQUEST_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_AGNSS_BlobInjection: GNSS_AGNSS_REQUEST_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_BREADCRUMBING_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_BREADCRUMBING_ALERT_DATA {}
impl ::core::clone::Clone for GNSS_BREADCRUMBING_ALERT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_BREADCRUMBING_ALERT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_BREADCRUMBING_ALERT_DATA").field("Size", &self.Size).field("Version", &self.Version).field("Unused", &self.Unused).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_BREADCRUMBING_ALERT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_BREADCRUMBING_ALERT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_BREADCRUMBING_ALERT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_BREADCRUMBING_ALERT_DATA {}
impl ::core::default::Default for GNSS_BREADCRUMBING_ALERT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_BREADCRUMBING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MaximumHorizontalUncertainty: u32,
    pub MinDistanceBetweenFixes: u32,
    pub MaximumErrorTimeoutMs: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_BREADCRUMBING_PARAM {}
impl ::core::clone::Clone for GNSS_BREADCRUMBING_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_BREADCRUMBING_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_BREADCRUMBING_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("MaximumHorizontalUncertainty", &self.MaximumHorizontalUncertainty).field("MinDistanceBetweenFixes", &self.MinDistanceBetweenFixes).field("MaximumErrorTimeoutMs", &self.MaximumErrorTimeoutMs).field("Unused", &self.Unused).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_BREADCRUMBING_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_BREADCRUMBING_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_BREADCRUMBING_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_BREADCRUMBING_PARAM {}
impl ::core::default::Default for GNSS_BREADCRUMBING_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_BREADCRUMB_LIST {
    pub Size: u32,
    pub Version: u32,
    pub NumCrumbs: u32,
    pub Anonymous: GNSS_BREADCRUMB_LIST_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_BREADCRUMB_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_BREADCRUMB_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_BREADCRUMB_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_BREADCRUMB_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_BREADCRUMB_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_BREADCRUMB_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_BREADCRUMB_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_BREADCRUMB_LIST_0 {
    pub v1: [GNSS_BREADCRUMB_V1; 50],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_BREADCRUMB_LIST_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_BREADCRUMB_LIST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_BREADCRUMB_LIST_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_BREADCRUMB_LIST_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_BREADCRUMB_LIST_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_BREADCRUMB_LIST_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_BREADCRUMB_LIST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_BREADCRUMB_V1 {
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub Latitude: f64,
    pub Longitude: f64,
    pub HorizontalAccuracy: u32,
    pub Speed: u16,
    pub SpeedAccuracy: u16,
    pub Altitude: i16,
    pub AltitudeAccuracy: u16,
    pub Heading: i16,
    pub HeadingAccuracy: u8,
    pub FixSuccess: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_BREADCRUMB_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_BREADCRUMB_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_BREADCRUMB_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_BREADCRUMB_V1")
            .field("FixTimeStamp", &self.FixTimeStamp)
            .field("Latitude", &self.Latitude)
            .field("Longitude", &self.Longitude)
            .field("HorizontalAccuracy", &self.HorizontalAccuracy)
            .field("Speed", &self.Speed)
            .field("SpeedAccuracy", &self.SpeedAccuracy)
            .field("Altitude", &self.Altitude)
            .field("AltitudeAccuracy", &self.AltitudeAccuracy)
            .field("Heading", &self.Heading)
            .field("HeadingAccuracy", &self.HeadingAccuracy)
            .field("FixSuccess", &self.FixSuccess)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_BREADCRUMB_V1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_BREADCRUMB_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_BREADCRUMB_V1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_BREADCRUMB_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_BREADCRUMB_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_CHIPSETINFO {
    pub Size: u32,
    pub Version: u32,
    pub ManufacturerID: [u16; 25],
    pub HardwareID: [u16; 25],
    pub FirmwareVersion: [u16; 20],
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_CHIPSETINFO {}
impl ::core::clone::Clone for GNSS_CHIPSETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_CHIPSETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_CHIPSETINFO").field("Size", &self.Size).field("Version", &self.Version).field("ManufacturerID", &self.ManufacturerID).field("HardwareID", &self.HardwareID).field("FirmwareVersion", &self.FirmwareVersion).field("Unused", &self.Unused).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_CHIPSETINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_CHIPSETINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_CHIPSETINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_CHIPSETINFO {}
impl ::core::default::Default for GNSS_CHIPSETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_CONTINUOUSTRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub PreferredInterval: u32,
}
impl ::core::marker::Copy for GNSS_CONTINUOUSTRACKING_PARAM {}
impl ::core::clone::Clone for GNSS_CONTINUOUSTRACKING_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_CONTINUOUSTRACKING_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_CONTINUOUSTRACKING_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("PreferredInterval", &self.PreferredInterval).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_CONTINUOUSTRACKING_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_CONTINUOUSTRACKING_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_CONTINUOUSTRACKING_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_CONTINUOUSTRACKING_PARAM {}
impl ::core::default::Default for GNSS_CONTINUOUSTRACKING_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_CP_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub NotificationText: [u16; 260],
}
impl ::core::marker::Copy for GNSS_CP_NI_INFO {}
impl ::core::clone::Clone for GNSS_CP_NI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_CP_NI_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_CP_NI_INFO").field("Size", &self.Size).field("Version", &self.Version).field("RequestorId", &self.RequestorId).field("NotificationText", &self.NotificationText).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_CP_NI_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_CP_NI_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_CP_NI_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_CP_NI_INFO {}
impl ::core::default::Default for GNSS_CP_NI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_CWTESTDATA {
    pub Size: u32,
    pub Version: u32,
    pub TestResultStatus: super::super::Foundation::NTSTATUS,
    pub SignalToNoiseRatio: f64,
    pub Frequency: f64,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_CWTESTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_CWTESTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_CWTESTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_CWTESTDATA").field("Size", &self.Size).field("Version", &self.Version).field("TestResultStatus", &self.TestResultStatus).field("SignalToNoiseRatio", &self.SignalToNoiseRatio).field("Frequency", &self.Frequency).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_CWTESTDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_CWTESTDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_CWTESTDATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_CWTESTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_CWTESTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_DEVICE_CAPABILITY {
    pub Size: u32,
    pub Version: u32,
    pub SupportMultipleFixSessions: super::super::Foundation::BOOL,
    pub SupportMultipleAppSessions: super::super::Foundation::BOOL,
    pub RequireAGnssInjection: super::super::Foundation::BOOL,
    pub AgnssFormatSupported: u32,
    pub AgnssFormatPreferred: u32,
    pub SupportDistanceTracking: super::super::Foundation::BOOL,
    pub SupportContinuousTracking: super::super::Foundation::BOOL,
    pub Reserved1: u32,
    pub Reserved2: super::super::Foundation::BOOL,
    pub Reserved3: super::super::Foundation::BOOL,
    pub Reserved4: super::super::Foundation::BOOL,
    pub Reserved5: super::super::Foundation::BOOL,
    pub GeofencingSupport: u32,
    pub Reserved6: super::super::Foundation::BOOL,
    pub Reserved7: super::super::Foundation::BOOL,
    pub SupportCpLocation: super::super::Foundation::BOOL,
    pub SupportUplV2: super::super::Foundation::BOOL,
    pub SupportSuplV1: super::super::Foundation::BOOL,
    pub SupportSuplV2: super::super::Foundation::BOOL,
    pub SupportedSuplVersion: GNSS_SUPL_VERSION,
    pub MaxGeofencesSupported: u32,
    pub SupportMultipleSuplRootCert: super::super::Foundation::BOOL,
    pub GnssBreadCrumbPayloadVersion: u32,
    pub MaxGnssBreadCrumbFixes: u32,
    pub Unused: [u8; 496],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_DEVICE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_DEVICE_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_DEVICE_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_DEVICE_CAPABILITY")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("SupportMultipleFixSessions", &self.SupportMultipleFixSessions)
            .field("SupportMultipleAppSessions", &self.SupportMultipleAppSessions)
            .field("RequireAGnssInjection", &self.RequireAGnssInjection)
            .field("AgnssFormatSupported", &self.AgnssFormatSupported)
            .field("AgnssFormatPreferred", &self.AgnssFormatPreferred)
            .field("SupportDistanceTracking", &self.SupportDistanceTracking)
            .field("SupportContinuousTracking", &self.SupportContinuousTracking)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Reserved4", &self.Reserved4)
            .field("Reserved5", &self.Reserved5)
            .field("GeofencingSupport", &self.GeofencingSupport)
            .field("Reserved6", &self.Reserved6)
            .field("Reserved7", &self.Reserved7)
            .field("SupportCpLocation", &self.SupportCpLocation)
            .field("SupportUplV2", &self.SupportUplV2)
            .field("SupportSuplV1", &self.SupportSuplV1)
            .field("SupportSuplV2", &self.SupportSuplV2)
            .field("SupportedSuplVersion", &self.SupportedSuplVersion)
            .field("MaxGeofencesSupported", &self.MaxGeofencesSupported)
            .field("SupportMultipleSuplRootCert", &self.SupportMultipleSuplRootCert)
            .field("GnssBreadCrumbPayloadVersion", &self.GnssBreadCrumbPayloadVersion)
            .field("MaxGnssBreadCrumbFixes", &self.MaxGnssBreadCrumbFixes)
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_DEVICE_CAPABILITY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_DEVICE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_DEVICE_CAPABILITY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_DEVICE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_DEVICE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_DISTANCETRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MovementThreshold: u32,
}
impl ::core::marker::Copy for GNSS_DISTANCETRACKING_PARAM {}
impl ::core::clone::Clone for GNSS_DISTANCETRACKING_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_DISTANCETRACKING_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_DISTANCETRACKING_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("MovementThreshold", &self.MovementThreshold).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_DISTANCETRACKING_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_DISTANCETRACKING_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_DISTANCETRACKING_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_DISTANCETRACKING_PARAM {}
impl ::core::default::Default for GNSS_DISTANCETRACKING_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_DRIVERCOMMAND_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub CommandType: GNSS_DRIVERCOMMAND_TYPE,
    pub Reserved: u32,
    pub CommandDataSize: u32,
    pub Unused: [u8; 512],
    pub CommandData: [u8; 1],
}
impl ::core::marker::Copy for GNSS_DRIVERCOMMAND_PARAM {}
impl ::core::clone::Clone for GNSS_DRIVERCOMMAND_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_DRIVERCOMMAND_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_DRIVERCOMMAND_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("CommandType", &self.CommandType).field("Reserved", &self.Reserved).field("CommandDataSize", &self.CommandDataSize).field("Unused", &self.Unused).field("CommandData", &self.CommandData).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_DRIVERCOMMAND_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_DRIVERCOMMAND_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_DRIVERCOMMAND_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_DRIVERCOMMAND_PARAM {}
impl ::core::default::Default for GNSS_DRIVERCOMMAND_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_DRIVERCOMMAND_TYPE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SetLocationServiceEnabled: GNSS_DRIVERCOMMAND_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SetLocationNIRequestAllowed: GNSS_DRIVERCOMMAND_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_ForceSatelliteSystem: GNSS_DRIVERCOMMAND_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_ForceOperationMode: GNSS_DRIVERCOMMAND_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_ResetEngine: GNSS_DRIVERCOMMAND_TYPE = 9i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_ClearAgnssData: GNSS_DRIVERCOMMAND_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SetSuplVersion: GNSS_DRIVERCOMMAND_TYPE = 12i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SetNMEALogging: GNSS_DRIVERCOMMAND_TYPE = 13i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SetUplServerAccessInterval: GNSS_DRIVERCOMMAND_TYPE = 14i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SetNiTimeoutInterval: GNSS_DRIVERCOMMAND_TYPE = 15i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_ResetGeofencesTracking: GNSS_DRIVERCOMMAND_TYPE = 16i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SetSuplVersion2: GNSS_DRIVERCOMMAND_TYPE = 17i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_CustomCommand: GNSS_DRIVERCOMMAND_TYPE = 256i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_DRIVER_REQUEST = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const SUPL_CONFIG_DATA: GNSS_DRIVER_REQUEST = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_DRIVER_REQUEST_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Request: GNSS_DRIVER_REQUEST,
    pub RequestFlag: u32,
}
impl ::core::marker::Copy for GNSS_DRIVER_REQUEST_DATA {}
impl ::core::clone::Clone for GNSS_DRIVER_REQUEST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_DRIVER_REQUEST_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_DRIVER_REQUEST_DATA").field("Size", &self.Size).field("Version", &self.Version).field("Request", &self.Request).field("RequestFlag", &self.RequestFlag).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_DRIVER_REQUEST_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_DRIVER_REQUEST_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_DRIVER_REQUEST_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_DRIVER_REQUEST_DATA {}
impl ::core::default::Default for GNSS_DRIVER_REQUEST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_DRIVER_VERSION_1: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_DRIVER_VERSION_2: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_DRIVER_VERSION_3: u32 = 3u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_DRIVER_VERSION_4: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_DRIVER_VERSION_5: u32 = 5u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_DRIVER_VERSION_6: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_ERRORINFO {
    pub Size: u32,
    pub Version: u32,
    pub ErrorCode: u32,
    pub IsRecoverable: super::super::Foundation::BOOL,
    pub ErrorDescription: [u16; 256],
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_ERRORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_ERRORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_ERRORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_ERRORINFO").field("Size", &self.Size).field("Version", &self.Version).field("ErrorCode", &self.ErrorCode).field("IsRecoverable", &self.IsRecoverable).field("ErrorDescription", &self.ErrorDescription).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_ERRORINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_ERRORINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_ERRORINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_ERRORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_ERRORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_EVENT {
    pub Size: u32,
    pub Version: u32,
    pub EventType: GNSS_EVENT_TYPE,
    pub EventDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_EVENT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_EVENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_EVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_EVENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_EVENT_0 {
    pub FixData: GNSS_FIXDATA,
    pub AgnssRequest: GNSS_AGNSS_REQUEST_PARAM,
    pub NiRequest: GNSS_NI_REQUEST_PARAM,
    pub ErrorInformation: GNSS_ERRORINFO,
    pub NmeaData: GNSS_NMEA_DATA,
    pub GeofenceAlertData: GNSS_GEOFENCE_ALERT_DATA,
    pub BreadcrumbAlertData: GNSS_BREADCRUMBING_ALERT_DATA,
    pub GeofencesTrackingStatus: GNSS_GEOFENCES_TRACKINGSTATUS_DATA,
    pub DriverRequestData: GNSS_DRIVER_REQUEST_DATA,
    pub CustomData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_EVENT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_EVENT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_EVENT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_EVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_EVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_EVENT_2 {
    pub Size: u32,
    pub Version: u32,
    pub EventType: GNSS_EVENT_TYPE,
    pub EventDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_EVENT_2_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_EVENT_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_EVENT_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_EVENT_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_EVENT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_EVENT_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_EVENT_2_0 {
    pub FixData: GNSS_FIXDATA,
    pub FixData2: GNSS_FIXDATA_2,
    pub AgnssRequest: GNSS_AGNSS_REQUEST_PARAM,
    pub NiRequest: GNSS_NI_REQUEST_PARAM,
    pub ErrorInformation: GNSS_ERRORINFO,
    pub NmeaData: GNSS_NMEA_DATA,
    pub GeofenceAlertData: GNSS_GEOFENCE_ALERT_DATA,
    pub BreadcrumbAlertData: GNSS_BREADCRUMBING_ALERT_DATA,
    pub GeofencesTrackingStatus: GNSS_GEOFENCES_TRACKINGSTATUS_DATA,
    pub DriverRequestData: GNSS_DRIVER_REQUEST_DATA,
    pub CustomData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_EVENT_2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_EVENT_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_EVENT_2_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_EVENT_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_EVENT_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_EVENT_TYPE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_FixAvailable: GNSS_EVENT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_RequireAgnss: GNSS_EVENT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_Error: GNSS_EVENT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_NiRequest: GNSS_EVENT_TYPE = 12i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_NmeaData: GNSS_EVENT_TYPE = 13i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_GeofenceAlertData: GNSS_EVENT_TYPE = 14i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_GeofencesTrackingStatus: GNSS_EVENT_TYPE = 15i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_DriverRequest: GNSS_EVENT_TYPE = 16i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_BreadcrumbAlertEvent: GNSS_EVENT_TYPE = 17i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_FixAvailable_2: GNSS_EVENT_TYPE = 18i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Event_Custom: GNSS_EVENT_TYPE = 32768i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub IsFinalFix: super::super::Foundation::BOOL,
    pub FixStatus: super::super::Foundation::NTSTATUS,
    pub FixLevelOfDetails: u32,
    pub BasicData: GNSS_FIXDATA_BASIC,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY,
    pub SatelliteData: GNSS_FIXDATA_SATELLITE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_FIXDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_FIXDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_FIXDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA").field("Size", &self.Size).field("Version", &self.Version).field("FixSessionID", &self.FixSessionID).field("FixTimeStamp", &self.FixTimeStamp).field("IsFinalFix", &self.IsFinalFix).field("FixStatus", &self.FixStatus).field("FixLevelOfDetails", &self.FixLevelOfDetails).field("BasicData", &self.BasicData).field("AccuracyData", &self.AccuracyData).field("SatelliteData", &self.SatelliteData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_FIXDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_FIXDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_FIXDATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_FIXDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_FIXDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA_2 {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub IsFinalFix: super::super::Foundation::BOOL,
    pub FixStatus: super::super::Foundation::NTSTATUS,
    pub FixLevelOfDetails: u32,
    pub BasicData: GNSS_FIXDATA_BASIC_2,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY_2,
    pub SatelliteData: GNSS_FIXDATA_SATELLITE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_FIXDATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_FIXDATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_FIXDATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_2").field("Size", &self.Size).field("Version", &self.Version).field("FixSessionID", &self.FixSessionID).field("FixTimeStamp", &self.FixTimeStamp).field("IsFinalFix", &self.IsFinalFix).field("FixStatus", &self.FixStatus).field("FixLevelOfDetails", &self.FixLevelOfDetails).field("BasicData", &self.BasicData).field("AccuracyData", &self.AccuracyData).field("SatelliteData", &self.SatelliteData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_FIXDATA_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_FIXDATA_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_FIXDATA_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_FIXDATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_FIXDATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_FIXDATA_ACCURACY {
    pub Size: u32,
    pub Version: u32,
    pub HorizontalAccuracy: u32,
    pub HorizontalErrorMajorAxis: u32,
    pub HorizontalErrorMinorAxis: u32,
    pub HorizontalErrorAngle: u32,
    pub HeadingAccuracy: u32,
    pub AltitudeAccuracy: u32,
    pub SpeedAccuracy: u32,
    pub HorizontalConfidence: u32,
    pub HeadingConfidence: u32,
    pub AltitudeConfidence: u32,
    pub SpeedConfidence: u32,
    pub PositionDilutionOfPrecision: f32,
    pub HorizontalDilutionOfPrecision: f32,
    pub VerticalDilutionOfPrecision: f32,
}
impl ::core::marker::Copy for GNSS_FIXDATA_ACCURACY {}
impl ::core::clone::Clone for GNSS_FIXDATA_ACCURACY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_FIXDATA_ACCURACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_ACCURACY")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("HorizontalAccuracy", &self.HorizontalAccuracy)
            .field("HorizontalErrorMajorAxis", &self.HorizontalErrorMajorAxis)
            .field("HorizontalErrorMinorAxis", &self.HorizontalErrorMinorAxis)
            .field("HorizontalErrorAngle", &self.HorizontalErrorAngle)
            .field("HeadingAccuracy", &self.HeadingAccuracy)
            .field("AltitudeAccuracy", &self.AltitudeAccuracy)
            .field("SpeedAccuracy", &self.SpeedAccuracy)
            .field("HorizontalConfidence", &self.HorizontalConfidence)
            .field("HeadingConfidence", &self.HeadingConfidence)
            .field("AltitudeConfidence", &self.AltitudeConfidence)
            .field("SpeedConfidence", &self.SpeedConfidence)
            .field("PositionDilutionOfPrecision", &self.PositionDilutionOfPrecision)
            .field("HorizontalDilutionOfPrecision", &self.HorizontalDilutionOfPrecision)
            .field("VerticalDilutionOfPrecision", &self.VerticalDilutionOfPrecision)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_FIXDATA_ACCURACY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_FIXDATA_ACCURACY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_FIXDATA_ACCURACY>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_FIXDATA_ACCURACY {}
impl ::core::default::Default for GNSS_FIXDATA_ACCURACY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_FIXDATA_ACCURACY_2 {
    pub Size: u32,
    pub Version: u32,
    pub HorizontalAccuracy: f64,
    pub HorizontalErrorMajorAxis: f64,
    pub HorizontalErrorMinorAxis: f64,
    pub HorizontalErrorAngle: f64,
    pub HeadingAccuracy: f64,
    pub AltitudeAccuracy: f64,
    pub SpeedAccuracy: f64,
    pub HorizontalConfidence: u32,
    pub HeadingConfidence: u32,
    pub AltitudeConfidence: u32,
    pub SpeedConfidence: u32,
    pub PositionDilutionOfPrecision: f64,
    pub HorizontalDilutionOfPrecision: f64,
    pub VerticalDilutionOfPrecision: f64,
    pub GeometricDilutionOfPrecision: f64,
    pub TimeDilutionOfPrecision: f64,
}
impl ::core::marker::Copy for GNSS_FIXDATA_ACCURACY_2 {}
impl ::core::clone::Clone for GNSS_FIXDATA_ACCURACY_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_FIXDATA_ACCURACY_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_ACCURACY_2")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("HorizontalAccuracy", &self.HorizontalAccuracy)
            .field("HorizontalErrorMajorAxis", &self.HorizontalErrorMajorAxis)
            .field("HorizontalErrorMinorAxis", &self.HorizontalErrorMinorAxis)
            .field("HorizontalErrorAngle", &self.HorizontalErrorAngle)
            .field("HeadingAccuracy", &self.HeadingAccuracy)
            .field("AltitudeAccuracy", &self.AltitudeAccuracy)
            .field("SpeedAccuracy", &self.SpeedAccuracy)
            .field("HorizontalConfidence", &self.HorizontalConfidence)
            .field("HeadingConfidence", &self.HeadingConfidence)
            .field("AltitudeConfidence", &self.AltitudeConfidence)
            .field("SpeedConfidence", &self.SpeedConfidence)
            .field("PositionDilutionOfPrecision", &self.PositionDilutionOfPrecision)
            .field("HorizontalDilutionOfPrecision", &self.HorizontalDilutionOfPrecision)
            .field("VerticalDilutionOfPrecision", &self.VerticalDilutionOfPrecision)
            .field("GeometricDilutionOfPrecision", &self.GeometricDilutionOfPrecision)
            .field("TimeDilutionOfPrecision", &self.TimeDilutionOfPrecision)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_FIXDATA_ACCURACY_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_FIXDATA_ACCURACY_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_FIXDATA_ACCURACY_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_FIXDATA_ACCURACY_2 {}
impl ::core::default::Default for GNSS_FIXDATA_ACCURACY_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_FIXDATA_BASIC {
    pub Size: u32,
    pub Version: u32,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Speed: f64,
    pub Heading: f64,
}
impl ::core::marker::Copy for GNSS_FIXDATA_BASIC {}
impl ::core::clone::Clone for GNSS_FIXDATA_BASIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_FIXDATA_BASIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_BASIC").field("Size", &self.Size).field("Version", &self.Version).field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).field("Speed", &self.Speed).field("Heading", &self.Heading).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_FIXDATA_BASIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_FIXDATA_BASIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_FIXDATA_BASIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_FIXDATA_BASIC {}
impl ::core::default::Default for GNSS_FIXDATA_BASIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_FIXDATA_BASIC_2 {
    pub Size: u32,
    pub Version: u32,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Speed: f64,
    pub Heading: f64,
    pub AltitudeEllipsoid: f64,
}
impl ::core::marker::Copy for GNSS_FIXDATA_BASIC_2 {}
impl ::core::clone::Clone for GNSS_FIXDATA_BASIC_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_FIXDATA_BASIC_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_BASIC_2").field("Size", &self.Size).field("Version", &self.Version).field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).field("Speed", &self.Speed).field("Heading", &self.Heading).field("AltitudeEllipsoid", &self.AltitudeEllipsoid).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_FIXDATA_BASIC_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_FIXDATA_BASIC_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_FIXDATA_BASIC_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_FIXDATA_BASIC_2 {}
impl ::core::default::Default for GNSS_FIXDATA_BASIC_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA_SATELLITE {
    pub Size: u32,
    pub Version: u32,
    pub SatelliteCount: u32,
    pub SatelliteArray: [GNSS_SATELLITEINFO; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_FIXDATA_SATELLITE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_FIXDATA_SATELLITE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_FIXDATA_SATELLITE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_SATELLITE").field("Size", &self.Size).field("Version", &self.Version).field("SatelliteCount", &self.SatelliteCount).field("SatelliteArray", &self.SatelliteArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_FIXDATA_SATELLITE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_FIXDATA_SATELLITE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_FIXDATA_SATELLITE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_FIXDATA_SATELLITE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_FIXDATA_SATELLITE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_FIXDETAIL_ACCURACY: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_FIXDETAIL_BASIC: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_FIXDETAIL_SATELLITE: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_FIXSESSIONTYPE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_FixSession_SingleShot: GNSS_FIXSESSIONTYPE = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_FixSession_DistanceTracking: GNSS_FIXSESSIONTYPE = 2i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_FixSession_ContinuousTracking: GNSS_FIXSESSIONTYPE = 3i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_FixSession_LKG: GNSS_FIXSESSIONTYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_FIXSESSION_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub SessionType: GNSS_FIXSESSIONTYPE,
    pub HorizontalAccuracy: u32,
    pub HorizontalConfidence: u32,
    pub Reserved: [u32; 9],
    pub FixLevelOfDetails: u32,
    pub Anonymous: GNSS_FIXSESSION_PARAM_0,
    pub Unused: [u8; 256],
}
impl ::core::marker::Copy for GNSS_FIXSESSION_PARAM {}
impl ::core::clone::Clone for GNSS_FIXSESSION_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GNSS_FIXSESSION_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_FIXSESSION_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_FIXSESSION_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_FIXSESSION_PARAM {}
impl ::core::default::Default for GNSS_FIXSESSION_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub union GNSS_FIXSESSION_PARAM_0 {
    pub SingleShotParam: GNSS_SINGLESHOT_PARAM,
    pub DistanceParam: GNSS_DISTANCETRACKING_PARAM,
    pub ContinuousParam: GNSS_CONTINUOUSTRACKING_PARAM,
    pub LkgFixParam: GNSS_LKGFIX_PARAM,
    pub UnusedParam: [u8; 268],
}
impl ::core::marker::Copy for GNSS_FIXSESSION_PARAM_0 {}
impl ::core::clone::Clone for GNSS_FIXSESSION_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GNSS_FIXSESSION_PARAM_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_FIXSESSION_PARAM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_FIXSESSION_PARAM_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_FIXSESSION_PARAM_0 {}
impl ::core::default::Default for GNSS_FIXSESSION_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_GEOFENCESUPPORT_CIRCLE: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_GEOFENCESUPPORT_SUPPORTED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Status: super::super::Foundation::NTSTATUS,
    pub StatusTimeStamp: super::super::Foundation::FILETIME,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOFENCES_TRACKINGSTATUS_DATA").field("Size", &self.Size).field("Version", &self.Version).field("Status", &self.Status).field("StatusTimeStamp", &self.StatusTimeStamp).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_GEOFENCES_TRACKINGSTATUS_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_GEOFENCE_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub GeofenceState: GNSS_GEOFENCE_STATE,
    pub FixBasicData: GNSS_FIXDATA_BASIC,
    pub FixAccuracyData: GNSS_FIXDATA_ACCURACY,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOFENCE_ALERT_DATA {}
impl ::core::clone::Clone for GNSS_GEOFENCE_ALERT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_GEOFENCE_ALERT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOFENCE_ALERT_DATA").field("Size", &self.Size).field("Version", &self.Version).field("GeofenceID", &self.GeofenceID).field("GeofenceState", &self.GeofenceState).field("FixBasicData", &self.FixBasicData).field("FixAccuracyData", &self.FixAccuracyData).field("Unused", &self.Unused).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_GEOFENCE_ALERT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_GEOFENCE_ALERT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_GEOFENCE_ALERT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_GEOFENCE_ALERT_DATA {}
impl ::core::default::Default for GNSS_GEOFENCE_ALERT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_GEOFENCE_CREATE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub AlertTypes: u32,
    pub InitialState: GNSS_GEOFENCE_STATE,
    pub Boundary: GNSS_GEOREGION,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOFENCE_CREATE_PARAM {}
impl ::core::clone::Clone for GNSS_GEOFENCE_CREATE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GNSS_GEOFENCE_CREATE_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_GEOFENCE_CREATE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_GEOFENCE_CREATE_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_GEOFENCE_CREATE_PARAM {}
impl ::core::default::Default for GNSS_GEOFENCE_CREATE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCE_CREATE_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub CreationStatus: super::super::Foundation::NTSTATUS,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_GEOFENCE_CREATE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOFENCE_CREATE_RESPONSE").field("Size", &self.Size).field("Version", &self.Version).field("CreationStatus", &self.CreationStatus).field("GeofenceID", &self.GeofenceID).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_GEOFENCE_CREATE_RESPONSE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_GEOFENCE_CREATE_RESPONSE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_GEOFENCE_CREATE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_GEOFENCE_DELETE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOFENCE_DELETE_PARAM {}
impl ::core::clone::Clone for GNSS_GEOFENCE_DELETE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_GEOFENCE_DELETE_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOFENCE_DELETE_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("GeofenceID", &self.GeofenceID).field("Unused", &self.Unused).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_GEOFENCE_DELETE_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_GEOFENCE_DELETE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_GEOFENCE_DELETE_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_GEOFENCE_DELETE_PARAM {}
impl ::core::default::Default for GNSS_GEOFENCE_DELETE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_GEOFENCE_STATE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_GeofenceState_Unknown: GNSS_GEOFENCE_STATE = 0i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_GeofenceState_Entered: GNSS_GEOFENCE_STATE = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_GeofenceState_Exited: GNSS_GEOFENCE_STATE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_GEOREGION {
    pub Size: u32,
    pub Version: u32,
    pub GeoRegionType: GNSS_GEOREGIONTYPE,
    pub Anonymous: GNSS_GEOREGION_0,
}
impl ::core::marker::Copy for GNSS_GEOREGION {}
impl ::core::clone::Clone for GNSS_GEOREGION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GNSS_GEOREGION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_GEOREGION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_GEOREGION>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_GEOREGION {}
impl ::core::default::Default for GNSS_GEOREGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub union GNSS_GEOREGION_0 {
    pub Circle: GNSS_GEOREGION_CIRCLE,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOREGION_0 {}
impl ::core::clone::Clone for GNSS_GEOREGION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GNSS_GEOREGION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_GEOREGION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_GEOREGION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_GEOREGION_0 {}
impl ::core::default::Default for GNSS_GEOREGION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_GEOREGIONTYPE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_GeoRegion_Circle: GNSS_GEOREGIONTYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_GEOREGION_CIRCLE {
    pub Latitude: f64,
    pub Longitude: f64,
    pub RadiusInMeters: f64,
}
impl ::core::marker::Copy for GNSS_GEOREGION_CIRCLE {}
impl ::core::clone::Clone for GNSS_GEOREGION_CIRCLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_GEOREGION_CIRCLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOREGION_CIRCLE").field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("RadiusInMeters", &self.RadiusInMeters).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_GEOREGION_CIRCLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_GEOREGION_CIRCLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_GEOREGION_CIRCLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_GEOREGION_CIRCLE {}
impl ::core::default::Default for GNSS_GEOREGION_CIRCLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_LKGFIX_PARAM {
    pub Size: u32,
    pub Version: u32,
}
impl ::core::marker::Copy for GNSS_LKGFIX_PARAM {}
impl ::core::clone::Clone for GNSS_LKGFIX_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_LKGFIX_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_LKGFIX_PARAM").field("Size", &self.Size).field("Version", &self.Version).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_LKGFIX_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_LKGFIX_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_LKGFIX_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_LKGFIX_PARAM {}
impl ::core::default::Default for GNSS_LKGFIX_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_MAXSATELLITE: u32 = 64u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_NI_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_NoNotifyNoVerify: GNSS_NI_NOTIFICATION_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_NotifyOnly: GNSS_NI_NOTIFICATION_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_NotifyVerifyDefaultAllow: GNSS_NI_NOTIFICATION_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_NotifyVerifyDefaultNotAllow: GNSS_NI_NOTIFICATION_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_PrivacyOverride: GNSS_NI_NOTIFICATION_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_NI_PLANE_TYPE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_SUPL: GNSS_NI_PLANE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_CP: GNSS_NI_PLANE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_V2UPL: GNSS_NI_PLANE_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_NI_REQUEST_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub RequestId: u32,
    pub RequestType: GNSS_NI_REQUEST_TYPE,
    pub NotificationType: GNSS_NI_NOTIFICATION_TYPE,
    pub RequestPlaneType: GNSS_NI_PLANE_TYPE,
    pub Anonymous: GNSS_NI_REQUEST_PARAM_0,
    pub ResponseTimeInSec: u32,
    pub EmergencyLocation: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_NI_REQUEST_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_NI_REQUEST_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_NI_REQUEST_PARAM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_NI_REQUEST_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_NI_REQUEST_PARAM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_NI_REQUEST_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_NI_REQUEST_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_NI_REQUEST_PARAM_0 {
    pub SuplNiInfo: GNSS_SUPL_NI_INFO,
    pub CpNiInfo: GNSS_CP_NI_INFO,
    pub V2UplNiInfo: GNSS_V2UPL_NI_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_NI_REQUEST_PARAM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_NI_REQUEST_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_NI_REQUEST_PARAM_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_NI_REQUEST_PARAM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_NI_REQUEST_PARAM_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_NI_REQUEST_PARAM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_NI_REQUEST_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_NI_REQUEST_TYPE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_Request_SingleShot: GNSS_NI_REQUEST_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NI_Request_AreaTrigger: GNSS_NI_REQUEST_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_NI_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub RequestId: u32,
    pub UserResponse: GNSS_NI_USER_RESPONSE,
}
impl ::core::marker::Copy for GNSS_NI_RESPONSE {}
impl ::core::clone::Clone for GNSS_NI_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_NI_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_NI_RESPONSE").field("Size", &self.Size).field("Version", &self.Version).field("RequestId", &self.RequestId).field("UserResponse", &self.UserResponse).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_NI_RESPONSE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_NI_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_NI_RESPONSE>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_NI_RESPONSE {}
impl ::core::default::Default for GNSS_NI_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_NI_USER_RESPONSE = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Ni_UserResponseAccept: GNSS_NI_USER_RESPONSE = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Ni_UserResponseDeny: GNSS_NI_USER_RESPONSE = 2i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Ni_UserResponseTimeout: GNSS_NI_USER_RESPONSE = 3i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NMEALOGGING_ALL: u32 = 255u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_NMEALOGGING_NONE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_NMEA_DATA {
    pub Size: u32,
    pub Version: u32,
    pub NmeaSentences: [super::super::Foundation::CHAR; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_NMEA_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_NMEA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_NMEA_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_NMEA_DATA").field("Size", &self.Size).field("Version", &self.Version).field("NmeaSentences", &self.NmeaSentences).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_NMEA_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_NMEA_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_NMEA_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_NMEA_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_NMEA_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_OPERMODE_AFLT: u32 = 16u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_OPERMODE_ANY: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_OPERMODE_CELLID: u32 = 8u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_OPERMODE_MSA: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_OPERMODE_MSB: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_OPERMODE_MSS: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_OPERMODE_OTDOA: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_PLATFORM_CAPABILITY {
    pub Size: u32,
    pub Version: u32,
    pub SupportAgnssInjection: super::super::Foundation::BOOL,
    pub AgnssFormatSupported: u32,
    pub Unused: [u8; 516],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_PLATFORM_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_PLATFORM_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_PLATFORM_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_PLATFORM_CAPABILITY").field("Size", &self.Size).field("Version", &self.Version).field("SupportAgnssInjection", &self.SupportAgnssInjection).field("AgnssFormatSupported", &self.AgnssFormatSupported).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_PLATFORM_CAPABILITY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_PLATFORM_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_PLATFORM_CAPABILITY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_PLATFORM_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_PLATFORM_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SATELLITEINFO {
    pub SatelliteId: u32,
    pub UsedInPositiong: super::super::Foundation::BOOL,
    pub Elevation: f64,
    pub Azimuth: f64,
    pub SignalToNoiseRatio: f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SATELLITEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SATELLITEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_SATELLITEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SATELLITEINFO").field("SatelliteId", &self.SatelliteId).field("UsedInPositiong", &self.UsedInPositiong).field("Elevation", &self.Elevation).field("Azimuth", &self.Azimuth).field("SignalToNoiseRatio", &self.SignalToNoiseRatio).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_SATELLITEINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_SATELLITEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_SATELLITEINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_SATELLITEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_SATELLITEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SATELLITE_ANY: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SATELLITE_BEIDOU: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SATELLITE_GALILEO: u32 = 8u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SATELLITE_GLONASS: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_SATELLITE_GPS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_SELFTESTCONFIG {
    pub Size: u32,
    pub Version: u32,
    pub TestType: u32,
    pub Unused: [u8; 512],
    pub InBufLen: u32,
    pub InBuffer: [u8; 1],
}
impl ::core::marker::Copy for GNSS_SELFTESTCONFIG {}
impl ::core::clone::Clone for GNSS_SELFTESTCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SELFTESTCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SELFTESTCONFIG").field("Size", &self.Size).field("Version", &self.Version).field("TestType", &self.TestType).field("Unused", &self.Unused).field("InBufLen", &self.InBufLen).field("InBuffer", &self.InBuffer).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_SELFTESTCONFIG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_SELFTESTCONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_SELFTESTCONFIG>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_SELFTESTCONFIG {}
impl ::core::default::Default for GNSS_SELFTESTCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SELFTESTRESULT {
    pub Size: u32,
    pub Version: u32,
    pub TestResultStatus: super::super::Foundation::NTSTATUS,
    pub Result: u32,
    pub PinFailedBitMask: u32,
    pub Unused: [u8; 512],
    pub OutBufLen: u32,
    pub OutBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SELFTESTRESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SELFTESTRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_SELFTESTRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SELFTESTRESULT").field("Size", &self.Size).field("Version", &self.Version).field("TestResultStatus", &self.TestResultStatus).field("Result", &self.Result).field("PinFailedBitMask", &self.PinFailedBitMask).field("Unused", &self.Unused).field("OutBufLen", &self.OutBufLen).field("OutBuffer", &self.OutBuffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_SELFTESTRESULT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_SELFTESTRESULT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_SELFTESTRESULT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_SELFTESTRESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_SELFTESTRESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_SINGLESHOT_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub ResponseTime: u32,
}
impl ::core::marker::Copy for GNSS_SINGLESHOT_PARAM {}
impl ::core::clone::Clone for GNSS_SINGLESHOT_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SINGLESHOT_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SINGLESHOT_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("ResponseTime", &self.ResponseTime).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_SINGLESHOT_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_SINGLESHOT_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_SINGLESHOT_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_SINGLESHOT_PARAM {}
impl ::core::default::Default for GNSS_SINGLESHOT_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_STOPFIXSESSION_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_STOPFIXSESSION_PARAM {}
impl ::core::clone::Clone for GNSS_STOPFIXSESSION_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_STOPFIXSESSION_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_STOPFIXSESSION_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("FixSessionID", &self.FixSessionID).field("Unused", &self.Unused).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_STOPFIXSESSION_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_STOPFIXSESSION_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_STOPFIXSESSION_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_STOPFIXSESSION_PARAM {}
impl ::core::default::Default for GNSS_STOPFIXSESSION_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type GNSS_SUPL_CERT_ACTION = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Supl_Cert_Inject: GNSS_SUPL_CERT_ACTION = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Supl_Cert_Delete: GNSS_SUPL_CERT_ACTION = 2i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const GNSS_Supl_Cert_Purge: GNSS_SUPL_CERT_ACTION = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_CERT_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub CertAction: GNSS_SUPL_CERT_ACTION,
    pub SuplCertName: [super::super::Foundation::CHAR; 260],
    pub CertSize: u32,
    pub Unused: [u8; 512],
    pub CertData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SUPL_CERT_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SUPL_CERT_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_SUPL_CERT_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_CERT_CONFIG").field("Size", &self.Size).field("Version", &self.Version).field("CertAction", &self.CertAction).field("SuplCertName", &self.SuplCertName).field("CertSize", &self.CertSize).field("Unused", &self.Unused).field("CertData", &self.CertData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_SUPL_CERT_CONFIG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_SUPL_CERT_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_SUPL_CERT_CONFIG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_SUPL_CERT_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_SUPL_CERT_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_HSLP_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub SuplHslp: [super::super::Foundation::CHAR; 260],
    pub SuplHslpFromImsi: [super::super::Foundation::CHAR; 260],
    pub Reserved: u32,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SUPL_HSLP_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SUPL_HSLP_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_SUPL_HSLP_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_HSLP_CONFIG").field("Size", &self.Size).field("Version", &self.Version).field("SuplHslp", &self.SuplHslp).field("SuplHslpFromImsi", &self.SuplHslpFromImsi).field("Reserved", &self.Reserved).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_SUPL_HSLP_CONFIG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_SUPL_HSLP_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_SUPL_HSLP_CONFIG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_SUPL_HSLP_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_SUPL_HSLP_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub ClientName: [u16; 260],
    pub SuplNiUrl: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SUPL_NI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SUPL_NI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_SUPL_NI_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_NI_INFO").field("Size", &self.Size).field("Version", &self.Version).field("RequestorId", &self.RequestorId).field("ClientName", &self.ClientName).field("SuplNiUrl", &self.SuplNiUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_SUPL_NI_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_SUPL_NI_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_SUPL_NI_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_SUPL_NI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_SUPL_NI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_SUPL_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl ::core::marker::Copy for GNSS_SUPL_VERSION {}
impl ::core::clone::Clone for GNSS_SUPL_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SUPL_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_SUPL_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_SUPL_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_SUPL_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_SUPL_VERSION {}
impl ::core::default::Default for GNSS_SUPL_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_SUPL_VERSION_2 {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ServiceIndicator: u32,
}
impl ::core::marker::Copy for GNSS_SUPL_VERSION_2 {}
impl ::core::clone::Clone for GNSS_SUPL_VERSION_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SUPL_VERSION_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_VERSION_2").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("ServiceIndicator", &self.ServiceIndicator).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_SUPL_VERSION_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_SUPL_VERSION_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_SUPL_VERSION_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_SUPL_VERSION_2 {}
impl ::core::default::Default for GNSS_SUPL_VERSION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_V2UPL_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub MPC: [super::super::Foundation::CHAR; 260],
    pub PDE: [super::super::Foundation::CHAR; 260],
    pub ApplicationTypeIndicator_MR: u8,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_V2UPL_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_V2UPL_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_V2UPL_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_V2UPL_CONFIG").field("Size", &self.Size).field("Version", &self.Version).field("MPC", &self.MPC).field("PDE", &self.PDE).field("ApplicationTypeIndicator_MR", &self.ApplicationTypeIndicator_MR).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GNSS_V2UPL_CONFIG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_V2UPL_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_V2UPL_CONFIG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_V2UPL_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_V2UPL_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub struct GNSS_V2UPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
}
impl ::core::marker::Copy for GNSS_V2UPL_NI_INFO {}
impl ::core::clone::Clone for GNSS_V2UPL_NI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_V2UPL_NI_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_V2UPL_NI_INFO").field("Size", &self.Size).field("Version", &self.Version).field("RequestorId", &self.RequestorId).finish()
    }
}
unsafe impl ::windows::core::Abi for GNSS_V2UPL_NI_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GNSS_V2UPL_NI_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GNSS_V2UPL_NI_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for GNSS_V2UPL_NI_INFO {}
impl ::core::default::Default for GNSS_V2UPL_NI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const GUID_DEVINTERFACE_GNSS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3336e5e4_018a_4669_84c5_bd05f3bd368b);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct ICivicAddressReport(::windows::core::IUnknown);
impl ICivicAddressReport {
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetSensorID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pkey), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAddressLine1(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAddressLine2(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCity(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStateProvince(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostalCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCountryRegion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetDetailLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ICivicAddressReport> for ILocationReport {
    fn from(value: ICivicAddressReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICivicAddressReport> for ILocationReport {
    fn from(value: &ICivicAddressReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILocationReport> for ICivicAddressReport {
    fn into_param(self) -> ::windows::core::Param<'a, ILocationReport> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILocationReport> for &ICivicAddressReport {
    fn into_param(self) -> ::windows::core::Param<'a, ILocationReport> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICivicAddressReport> for ::windows::core::IUnknown {
    fn from(value: ICivicAddressReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICivicAddressReport> for ::windows::core::IUnknown {
    fn from(value: &ICivicAddressReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICivicAddressReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICivicAddressReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICivicAddressReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICivicAddressReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICivicAddressReport {}
impl ::core::fmt::Debug for ICivicAddressReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICivicAddressReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICivicAddressReport {
    type Vtable = ICivicAddressReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0b19f70_4adf_445d_87f2_cad8fd711792);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddressReportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensorid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcreationtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstraddress1: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstraddress2: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstateprovince: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpostalcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcountryregion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct ICivicAddressReportFactory(::windows::core::IUnknown);
impl ICivicAddressReportFactory {
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn ListenForReports(&self, requestedreportinterval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestedreportinterval)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn StopListeningForReports(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn SetReportInterval(&self, millisecondsrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(millisecondsrequested)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn DesiredAccuracy(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(desiredaccuracy)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwnd)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn CivicAddressReport(&self) -> ::windows::core::Result<IDispCivicAddressReport> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDispCivicAddressReport>(result__)
    }
}
impl ::core::convert::From<ICivicAddressReportFactory> for ILocationReportFactory {
    fn from(value: ICivicAddressReportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICivicAddressReportFactory> for ILocationReportFactory {
    fn from(value: &ICivicAddressReportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILocationReportFactory> for ICivicAddressReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ILocationReportFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILocationReportFactory> for &ICivicAddressReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ILocationReportFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICivicAddressReportFactory> for super::super::System::Com::IDispatch {
    fn from(value: ICivicAddressReportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICivicAddressReportFactory> for super::super::System::Com::IDispatch {
    fn from(value: &ICivicAddressReportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ICivicAddressReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &ICivicAddressReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICivicAddressReportFactory> for ::windows::core::IUnknown {
    fn from(value: ICivicAddressReportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICivicAddressReportFactory> for ::windows::core::IUnknown {
    fn from(value: &ICivicAddressReportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICivicAddressReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICivicAddressReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICivicAddressReportFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICivicAddressReportFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICivicAddressReportFactory {}
impl ::core::fmt::Debug for ICivicAddressReportFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICivicAddressReportFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICivicAddressReportFactory {
    type Vtable = ICivicAddressReportFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf773b93_c64f_4bee_beb2_67c0b8df66e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddressReportFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedreportinterval: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmilliseconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, millisecondsrequested: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesiredaccuracy: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredaccuracy: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct IDefaultLocation(::windows::core::IUnknown);
impl IDefaultLocation {
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn SetReport<'a, Param1: ::windows::core::IntoParam<'a, ILocationReport>>(&self, reporttype: *const ::windows::core::GUID, plocationreport: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), plocationreport.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetReport(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<ILocationReport> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(&mut result__)).from_abi::<ILocationReport>(result__)
    }
}
impl ::core::convert::From<IDefaultLocation> for ::windows::core::IUnknown {
    fn from(value: IDefaultLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDefaultLocation> for ::windows::core::IUnknown {
    fn from(value: &IDefaultLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDefaultLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDefaultLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDefaultLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDefaultLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDefaultLocation {}
impl ::core::fmt::Debug for IDefaultLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDefaultLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDefaultLocation {
    type Vtable = IDefaultLocationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa65af77e_969a_4a2e_8aca_33bb7cbb1235);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultLocationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, plocationreport: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pplocationreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct IDispCivicAddressReport(::windows::core::IUnknown);
impl IDispCivicAddressReport {
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddressLine1(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddressLine2(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn City(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StateProvince(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostalCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CountryRegion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn DetailLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Timestamp(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDispCivicAddressReport> for super::super::System::Com::IDispatch {
    fn from(value: IDispCivicAddressReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDispCivicAddressReport> for super::super::System::Com::IDispatch {
    fn from(value: &IDispCivicAddressReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IDispCivicAddressReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IDispCivicAddressReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDispCivicAddressReport> for ::windows::core::IUnknown {
    fn from(value: IDispCivicAddressReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDispCivicAddressReport> for ::windows::core::IUnknown {
    fn from(value: &IDispCivicAddressReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDispCivicAddressReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDispCivicAddressReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDispCivicAddressReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispCivicAddressReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispCivicAddressReport {}
impl ::core::fmt::Debug for IDispCivicAddressReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispCivicAddressReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDispCivicAddressReport {
    type Vtable = IDispCivicAddressReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16ff1a34_9e30_42c3_b44d_e22513b5767a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispCivicAddressReportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress1: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress2: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstateprovince: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppostalcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcountryregion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct IDispLatLongReport(::windows::core::IUnknown);
impl IDispLatLongReport {
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Latitude(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Longitude(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn ErrorRadius(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Altitude(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn AltitudeError(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Timestamp(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDispLatLongReport> for super::super::System::Com::IDispatch {
    fn from(value: IDispLatLongReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDispLatLongReport> for super::super::System::Com::IDispatch {
    fn from(value: &IDispLatLongReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IDispLatLongReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IDispLatLongReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDispLatLongReport> for ::windows::core::IUnknown {
    fn from(value: IDispLatLongReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDispLatLongReport> for ::windows::core::IUnknown {
    fn from(value: &IDispLatLongReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDispLatLongReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDispLatLongReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDispLatLongReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispLatLongReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispLatLongReport {}
impl ::core::fmt::Debug for IDispLatLongReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispLatLongReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDispLatLongReport {
    type Vtable = IDispLatLongReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ae32723_389b_4a11_9957_5bdd48fc9617);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispLatLongReportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct ILatLongReport(::windows::core::IUnknown);
impl ILatLongReport {
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetSensorID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pkey), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetLatitude(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetLongitude(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetErrorRadius(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetAltitude(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetAltitudeError(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
}
impl ::core::convert::From<ILatLongReport> for ILocationReport {
    fn from(value: ILatLongReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILatLongReport> for ILocationReport {
    fn from(value: &ILatLongReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILocationReport> for ILatLongReport {
    fn into_param(self) -> ::windows::core::Param<'a, ILocationReport> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILocationReport> for &ILatLongReport {
    fn into_param(self) -> ::windows::core::Param<'a, ILocationReport> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILatLongReport> for ::windows::core::IUnknown {
    fn from(value: ILatLongReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILatLongReport> for ::windows::core::IUnknown {
    fn from(value: &ILatLongReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILatLongReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILatLongReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILatLongReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILatLongReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILatLongReport {}
impl ::core::fmt::Debug for ILatLongReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILatLongReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILatLongReport {
    type Vtable = ILatLongReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fed806d_0ef8_4f07_80ac_36a0beae3134);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILatLongReportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensorid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcreationtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, platitude: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plongitude: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrorradius: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paltitude: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paltitudeerror: *mut f64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct ILatLongReportFactory(::windows::core::IUnknown);
impl ILatLongReportFactory {
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn ListenForReports(&self, requestedreportinterval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestedreportinterval)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn StopListeningForReports(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn SetReportInterval(&self, millisecondsrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(millisecondsrequested)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn DesiredAccuracy(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(desiredaccuracy)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwnd)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn LatLongReport(&self) -> ::windows::core::Result<IDispLatLongReport> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDispLatLongReport>(result__)
    }
}
impl ::core::convert::From<ILatLongReportFactory> for ILocationReportFactory {
    fn from(value: ILatLongReportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILatLongReportFactory> for ILocationReportFactory {
    fn from(value: &ILatLongReportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILocationReportFactory> for ILatLongReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ILocationReportFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILocationReportFactory> for &ILatLongReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ILocationReportFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ILatLongReportFactory> for super::super::System::Com::IDispatch {
    fn from(value: ILatLongReportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ILatLongReportFactory> for super::super::System::Com::IDispatch {
    fn from(value: &ILatLongReportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ILatLongReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &ILatLongReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILatLongReportFactory> for ::windows::core::IUnknown {
    fn from(value: ILatLongReportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILatLongReportFactory> for ::windows::core::IUnknown {
    fn from(value: &ILatLongReportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILatLongReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILatLongReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILatLongReportFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILatLongReportFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILatLongReportFactory {}
impl ::core::fmt::Debug for ILatLongReportFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILatLongReportFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILatLongReportFactory {
    type Vtable = ILatLongReportFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f0804cb_b114_447d_83dd_390174ebb082);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILatLongReportFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedreportinterval: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmilliseconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, millisecondsrequested: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesiredaccuracy: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredaccuracy: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct ILocation(::windows::core::IUnknown);
impl ILocation {
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn RegisterForReport<'a, Param0: ::windows::core::IntoParam<'a, ILocationEvents>>(&self, pevents: Param0, reporttype: *const ::windows::core::GUID, dwrequestedreportinterval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pevents.into_param().abi(), ::core::mem::transmute(reporttype), ::core::mem::transmute(dwrequestedreportinterval)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn UnregisterForReport(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetReport(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<ILocationReport> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(&mut result__)).from_abi::<ILocationReport>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetReportStatus(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<LOCATION_REPORT_STATUS> {
        let mut result__: LOCATION_REPORT_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(&mut result__)).from_abi::<LOCATION_REPORT_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetReportInterval(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn SetReportInterval(&self, reporttype: *const ::windows::core::GUID, millisecondsrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(millisecondsrequested)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Devices_Sensors'*"]
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub unsafe fn GetDesiredAccuracy(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<super::Sensors::LOCATION_DESIRED_ACCURACY> {
        let mut result__: super::Sensors::LOCATION_DESIRED_ACCURACY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(&mut result__)).from_abi::<super::Sensors::LOCATION_DESIRED_ACCURACY>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Devices_Sensors'*"]
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub unsafe fn SetDesiredAccuracy(&self, reporttype: *const ::windows::core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(desiredaccuracy)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestPermissions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hparent: Param0, preporttypes: *const ::windows::core::GUID, count: u32, fmodal: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), hparent.into_param().abi(), ::core::mem::transmute(preporttypes), ::core::mem::transmute(count), fmodal.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ILocation> for ::windows::core::IUnknown {
    fn from(value: ILocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILocation> for ::windows::core::IUnknown {
    fn from(value: &ILocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILocation {}
impl ::core::fmt::Debug for ILocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILocation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILocation {
    type Vtable = ILocationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab2ece69_56d9_4f28_b525_de1b0ee44237);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr, reporttype: *const ::windows::core::GUID, dwrequestedreportinterval: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pplocationreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pstatus: *mut LOCATION_REPORT_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pmilliseconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, millisecondsrequested: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Devices_Sensors")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pdesiredaccuracy: *mut super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Sensors"))] usize,
    #[cfg(feature = "Win32_Devices_Sensors")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Sensors"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, preporttypes: *const ::windows::core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct ILocationEvents(::windows::core::IUnknown);
impl ILocationEvents {
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn OnLocationChanged<'a, Param1: ::windows::core::IntoParam<'a, ILocationReport>>(&self, reporttype: *const ::windows::core::GUID, plocationreport: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), plocationreport.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn OnStatusChanged(&self, reporttype: *const ::windows::core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(newstatus)).ok()
    }
}
impl ::core::convert::From<ILocationEvents> for ::windows::core::IUnknown {
    fn from(value: ILocationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILocationEvents> for ::windows::core::IUnknown {
    fn from(value: &ILocationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILocationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILocationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILocationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILocationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILocationEvents {}
impl ::core::fmt::Debug for ILocationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILocationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILocationEvents {
    type Vtable = ILocationEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcae02bbf_798b_4508_a207_35a7906dc73d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, plocationreport: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct ILocationPower(::windows::core::IUnknown);
impl ILocationPower {
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Connect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ILocationPower> for ::windows::core::IUnknown {
    fn from(value: ILocationPower) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILocationPower> for ::windows::core::IUnknown {
    fn from(value: &ILocationPower) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILocationPower {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILocationPower {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILocationPower {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILocationPower {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILocationPower {}
impl ::core::fmt::Debug for ILocationPower {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILocationPower").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILocationPower {
    type Vtable = ILocationPowerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x193e7729_ab6b_4b12_8617_7596e1bb191c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationPowerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct ILocationReport(::windows::core::IUnknown);
impl ILocationReport {
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn GetSensorID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pkey), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
impl ::core::convert::From<ILocationReport> for ::windows::core::IUnknown {
    fn from(value: ILocationReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILocationReport> for ::windows::core::IUnknown {
    fn from(value: &ILocationReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILocationReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILocationReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILocationReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILocationReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILocationReport {}
impl ::core::fmt::Debug for ILocationReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILocationReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILocationReport {
    type Vtable = ILocationReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8b7f7ee_75d0_4db9_b62d_7a0f369ca456);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationReportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensorid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcreationtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct ILocationReportFactory(::windows::core::IUnknown);
impl ILocationReportFactory {
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn ListenForReports(&self, requestedreportinterval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestedreportinterval)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn StopListeningForReports(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn SetReportInterval(&self, millisecondsrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(millisecondsrequested)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn DesiredAccuracy(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(desiredaccuracy)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwnd)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ILocationReportFactory> for super::super::System::Com::IDispatch {
    fn from(value: ILocationReportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ILocationReportFactory> for super::super::System::Com::IDispatch {
    fn from(value: &ILocationReportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ILocationReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &ILocationReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILocationReportFactory> for ::windows::core::IUnknown {
    fn from(value: ILocationReportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILocationReportFactory> for ::windows::core::IUnknown {
    fn from(value: &ILocationReportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILocationReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILocationReportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILocationReportFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILocationReportFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILocationReportFactory {}
impl ::core::fmt::Debug for ILocationReportFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILocationReportFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILocationReportFactory {
    type Vtable = ILocationReportFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2daec322_90b2_47e4_bb08_0da841935a6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationReportFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedreportinterval: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmilliseconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, millisecondsrequested: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesiredaccuracy: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredaccuracy: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *const u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_CONFIG_SUPL_CERT: u32 = 2228488u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_CREATE_GEOFENCE: u32 = 2228544u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_DELETE_GEOFENCE: u32 = 2228548u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_EXECUTE_CWTEST: u32 = 2228496u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_EXECUTE_SELFTEST: u32 = 2228500u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_GET_CHIPSETINFO: u32 = 2228504u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_GET_DEVICE_CAPABILITY: u32 = 2228232u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_GET_FIXDATA: u32 = 2228300u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_INJECT_AGNSS: u32 = 2228352u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_LISTEN_AGNSS: u32 = 2228416u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_LISTEN_BREADCRUMBING_ALERT: u32 = 2228680u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_LISTEN_DRIVER_REQUEST: u32 = 2228608u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_LISTEN_ERROR: u32 = 2228420u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_LISTEN_GEOFENCES_TRACKINGSTATUS: u32 = 2228556u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_LISTEN_GEOFENCE_ALERT: u32 = 2228552u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_LISTEN_NI: u32 = 2228480u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_LISTEN_NMEA: u32 = 2228508u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_MODIFY_FIXSESSION: u32 = 2228292u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_POP_BREADCRUMBS: u32 = 2228684u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_RESPOND_NI: u32 = 2228492u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_SEND_DRIVERCOMMAND: u32 = 2228236u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_SEND_PLATFORM_CAPABILITY: u32 = 2228228u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_SET_SUPL_HSLP: u32 = 2228484u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_SET_V2UPL_CONFIG: u32 = 2228512u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_START_BREADCRUMBING: u32 = 2228672u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_START_FIXSESSION: u32 = 2228288u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_STOP_BREADCRUMBING: u32 = 2228676u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const IOCTL_GNSS_STOP_FIXSESSION: u32 = 2228296u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const LOCATION_API_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub type LOCATION_REPORT_STATUS = i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const REPORT_NOT_SUPPORTED: LOCATION_REPORT_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const REPORT_ERROR: LOCATION_REPORT_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const REPORT_ACCESS_DENIED: LOCATION_REPORT_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const REPORT_INITIALIZING: LOCATION_REPORT_STATUS = 3i32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const REPORT_RUNNING: LOCATION_REPORT_STATUS = 4i32;
pub const LatLongReport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed81c073_1f84_4ca8_a161_183c776bc651);
pub const LatLongReportFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dcc3cc8_8609_4863_bad4_03601f4c65e8);
pub const Location: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5b8e079_ee6d_4e33_a438_c87f2e959254);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const MAX_SERVER_URL_NAME: u32 = 260u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const MIN_BREADCRUMBS_SUPPORTED: u32 = 120u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
pub const MIN_GEOFENCES_REQUIRED: u32 = 100u32;
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct _ICivicAddressReportFactoryEvents(::windows::core::IUnknown);
impl _ICivicAddressReportFactoryEvents {
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_ICivicAddressReportFactoryEvents> for super::super::System::Com::IDispatch {
    fn from(value: _ICivicAddressReportFactoryEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_ICivicAddressReportFactoryEvents> for super::super::System::Com::IDispatch {
    fn from(value: &_ICivicAddressReportFactoryEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for _ICivicAddressReportFactoryEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &_ICivicAddressReportFactoryEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<_ICivicAddressReportFactoryEvents> for ::windows::core::IUnknown {
    fn from(value: _ICivicAddressReportFactoryEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_ICivicAddressReportFactoryEvents> for ::windows::core::IUnknown {
    fn from(value: &_ICivicAddressReportFactoryEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _ICivicAddressReportFactoryEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &_ICivicAddressReportFactoryEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _ICivicAddressReportFactoryEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _ICivicAddressReportFactoryEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _ICivicAddressReportFactoryEvents {}
impl ::core::fmt::Debug for _ICivicAddressReportFactoryEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ICivicAddressReportFactoryEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _ICivicAddressReportFactoryEvents {
    type Vtable = _ICivicAddressReportFactoryEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc96039ff_72ec_4617_89bd_84d88bedc722);
}
#[repr(C)]
#[doc(hidden)]
pub struct _ICivicAddressReportFactoryEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Geolocation'*"]
#[repr(transparent)]
pub struct _ILatLongReportFactoryEvents(::windows::core::IUnknown);
impl _ILatLongReportFactoryEvents {
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Geolocation', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_ILatLongReportFactoryEvents> for super::super::System::Com::IDispatch {
    fn from(value: _ILatLongReportFactoryEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_ILatLongReportFactoryEvents> for super::super::System::Com::IDispatch {
    fn from(value: &_ILatLongReportFactoryEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for _ILatLongReportFactoryEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &_ILatLongReportFactoryEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<_ILatLongReportFactoryEvents> for ::windows::core::IUnknown {
    fn from(value: _ILatLongReportFactoryEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_ILatLongReportFactoryEvents> for ::windows::core::IUnknown {
    fn from(value: &_ILatLongReportFactoryEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _ILatLongReportFactoryEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &_ILatLongReportFactoryEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _ILatLongReportFactoryEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _ILatLongReportFactoryEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _ILatLongReportFactoryEvents {}
impl ::core::fmt::Debug for _ILatLongReportFactoryEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ILatLongReportFactoryEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _ILatLongReportFactoryEvents {
    type Vtable = _ILatLongReportFactoryEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16ee6cb7_ab3c_424b_849f_269be551fcbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct _ILatLongReportFactoryEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
