#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const BREADCRUMBING_UNSUPPORTED: u32 = 0u32;
pub const BREADCRUMBING_VERSION_1: u32 = 1u32;
pub const CivicAddressReport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3550378973,
    32005,
    18104,
    [135, 33, 128, 207, 3, 95, 87, 215],
);
pub const CivicAddressReportFactory: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        705819692,
        16001,
        19156,
        [156, 190, 69, 87, 157, 137, 103, 26],
    );
pub const DefaultLocation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2340405216,
    23767,
    18762,
    [175, 140, 40, 58, 101, 112, 117, 6],
);
pub const DispCivicAddressReport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1280928492,
    34116,
    16514,
    [186, 159, 235, 10, 125, 142, 101, 198],
);
pub const DispLatLongReport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2054959735,
    36740,
    17974,
    [149, 178, 235, 181, 80, 127, 247, 126],
);
pub const GNSS_AGNSSFORMAT_LTO: u32 = 4u32;
pub const GNSS_AGNSSFORMAT_XTRA1: u32 = 1u32;
pub const GNSS_AGNSSFORMAT_XTRA2: u32 = 2u32;
pub const GNSS_AGNSSFORMAT_XTRA3: u32 = 8u32;
pub const GNSS_AGNSSFORMAT_XTRA3_1: u32 = 16u32;
pub const GNSS_AGNSSFORMAT_XTRA3_2: u32 = 32u32;
pub const GNSS_AGNSSFORMAT_XTRA_INT: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_AGNSS_INJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_AGNSS_INJECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_AGNSS_INJECT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_AGNSS_INJECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_AGNSS_INJECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_AGNSS_INJECT_0 {
    pub Time: GNSS_AGNSS_INJECTTIME,
    pub Position: GNSS_AGNSS_INJECTPOSITION,
    pub BlobData: GNSS_AGNSS_INJECTBLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_AGNSS_INJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_AGNSS_INJECT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_AGNSS_INJECT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_AGNSS_INJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_AGNSS_INJECT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_AGNSS_INJECTBLOB {
    pub Size: u32,
    pub Version: u32,
    pub BlobOui: u32,
    pub BlobVersion: u32,
    pub AgnssFormat: u32,
    pub BlobSize: u32,
    pub BlobData: [u8; 1],
}
impl GNSS_AGNSS_INJECTBLOB {}
impl ::std::default::Default for GNSS_AGNSS_INJECTBLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_AGNSS_INJECTBLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_AGNSS_INJECTBLOB")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("BlobOui", &self.BlobOui)
            .field("BlobVersion", &self.BlobVersion)
            .field("AgnssFormat", &self.AgnssFormat)
            .field("BlobSize", &self.BlobSize)
            .field("BlobData", &self.BlobData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_AGNSS_INJECTBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.BlobOui == other.BlobOui
            && self.BlobVersion == other.BlobVersion
            && self.AgnssFormat == other.AgnssFormat
            && self.BlobSize == other.BlobSize
            && self.BlobData == other.BlobData
    }
}
impl ::std::cmp::Eq for GNSS_AGNSS_INJECTBLOB {}
unsafe impl ::windows::runtime::Abi for GNSS_AGNSS_INJECTBLOB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_AGNSS_INJECTPOSITION {
    pub Size: u32,
    pub Version: u32,
    pub Age: u32,
    pub BasicData: GNSS_FIXDATA_BASIC,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY,
}
impl GNSS_AGNSS_INJECTPOSITION {}
impl ::std::default::Default for GNSS_AGNSS_INJECTPOSITION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_AGNSS_INJECTPOSITION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_AGNSS_INJECTPOSITION")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("Age", &self.Age)
            .field("BasicData", &self.BasicData)
            .field("AccuracyData", &self.AccuracyData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_AGNSS_INJECTPOSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.Age == other.Age
            && self.BasicData == other.BasicData
            && self.AccuracyData == other.AccuracyData
    }
}
impl ::std::cmp::Eq for GNSS_AGNSS_INJECTPOSITION {}
unsafe impl ::windows::runtime::Abi for GNSS_AGNSS_INJECTPOSITION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_AGNSS_INJECTTIME {
    pub Size: u32,
    pub Version: u32,
    pub UtcTime: super::super::Foundation::FILETIME,
    pub TimeUncertainty: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_AGNSS_INJECTTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_AGNSS_INJECTTIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_AGNSS_INJECTTIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_AGNSS_INJECTTIME")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("UtcTime", &self.UtcTime)
            .field("TimeUncertainty", &self.TimeUncertainty)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_AGNSS_INJECTTIME {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.UtcTime == other.UtcTime
            && self.TimeUncertainty == other.TimeUncertainty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_AGNSS_INJECTTIME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_AGNSS_INJECTTIME {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_AGNSS_REQUEST_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub RequestType: GNSS_AGNSS_REQUEST_TYPE,
    pub BlobFormat: u32,
}
impl GNSS_AGNSS_REQUEST_PARAM {}
impl ::std::default::Default for GNSS_AGNSS_REQUEST_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_AGNSS_REQUEST_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_AGNSS_REQUEST_PARAM")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("RequestType", &self.RequestType)
            .field("BlobFormat", &self.BlobFormat)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_AGNSS_REQUEST_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.RequestType == other.RequestType
            && self.BlobFormat == other.BlobFormat
    }
}
impl ::std::cmp::Eq for GNSS_AGNSS_REQUEST_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_AGNSS_REQUEST_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_AGNSS_REQUEST_TYPE(pub i32);
pub const GNSS_AGNSS_TimeInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(1i32);
pub const GNSS_AGNSS_PositionInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(2i32);
pub const GNSS_AGNSS_BlobInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(3i32);
impl ::std::convert::From<i32> for GNSS_AGNSS_REQUEST_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_AGNSS_REQUEST_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_BREADCRUMBING_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Unused: [u8; 512],
}
impl GNSS_BREADCRUMBING_ALERT_DATA {}
impl ::std::default::Default for GNSS_BREADCRUMBING_ALERT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_BREADCRUMBING_ALERT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_BREADCRUMBING_ALERT_DATA")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("Unused", &self.Unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_BREADCRUMBING_ALERT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Unused == other.Unused
    }
}
impl ::std::cmp::Eq for GNSS_BREADCRUMBING_ALERT_DATA {}
unsafe impl ::windows::runtime::Abi for GNSS_BREADCRUMBING_ALERT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_BREADCRUMBING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MaximumHorizontalUncertainty: u32,
    pub MinDistanceBetweenFixes: u32,
    pub MaximumErrorTimeoutMs: u32,
    pub Unused: [u8; 512],
}
impl GNSS_BREADCRUMBING_PARAM {}
impl ::std::default::Default for GNSS_BREADCRUMBING_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_BREADCRUMBING_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_BREADCRUMBING_PARAM")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field(
                "MaximumHorizontalUncertainty",
                &self.MaximumHorizontalUncertainty,
            )
            .field("MinDistanceBetweenFixes", &self.MinDistanceBetweenFixes)
            .field("MaximumErrorTimeoutMs", &self.MaximumErrorTimeoutMs)
            .field("Unused", &self.Unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_BREADCRUMBING_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.MaximumHorizontalUncertainty == other.MaximumHorizontalUncertainty
            && self.MinDistanceBetweenFixes == other.MinDistanceBetweenFixes
            && self.MaximumErrorTimeoutMs == other.MaximumErrorTimeoutMs
            && self.Unused == other.Unused
    }
}
impl ::std::cmp::Eq for GNSS_BREADCRUMBING_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_BREADCRUMBING_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_BREADCRUMB_LIST {
    pub Size: u32,
    pub Version: u32,
    pub NumCrumbs: u32,
    pub Anonymous: GNSS_BREADCRUMB_LIST_0,
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_BREADCRUMB_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_BREADCRUMB_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_BREADCRUMB_LIST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_BREADCRUMB_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_BREADCRUMB_LIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_BREADCRUMB_LIST_0 {
    pub v1: [GNSS_BREADCRUMB_V1; 50],
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_BREADCRUMB_LIST_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_BREADCRUMB_LIST_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_BREADCRUMB_LIST_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_BREADCRUMB_LIST_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_BREADCRUMB_LIST_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_BREADCRUMB_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_BREADCRUMB_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_BREADCRUMB_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_BREADCRUMB_V1")
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
impl ::std::cmp::PartialEq for GNSS_BREADCRUMB_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.FixTimeStamp == other.FixTimeStamp
            && self.Latitude == other.Latitude
            && self.Longitude == other.Longitude
            && self.HorizontalAccuracy == other.HorizontalAccuracy
            && self.Speed == other.Speed
            && self.SpeedAccuracy == other.SpeedAccuracy
            && self.Altitude == other.Altitude
            && self.AltitudeAccuracy == other.AltitudeAccuracy
            && self.Heading == other.Heading
            && self.HeadingAccuracy == other.HeadingAccuracy
            && self.FixSuccess == other.FixSuccess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_BREADCRUMB_V1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_BREADCRUMB_V1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_CHIPSETINFO {
    pub Size: u32,
    pub Version: u32,
    pub ManufacturerID: [u16; 25],
    pub HardwareID: [u16; 25],
    pub FirmwareVersion: [u16; 20],
    pub Unused: [u8; 512],
}
impl GNSS_CHIPSETINFO {}
impl ::std::default::Default for GNSS_CHIPSETINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_CHIPSETINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_CHIPSETINFO")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("ManufacturerID", &self.ManufacturerID)
            .field("HardwareID", &self.HardwareID)
            .field("FirmwareVersion", &self.FirmwareVersion)
            .field("Unused", &self.Unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_CHIPSETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.ManufacturerID == other.ManufacturerID
            && self.HardwareID == other.HardwareID
            && self.FirmwareVersion == other.FirmwareVersion
            && self.Unused == other.Unused
    }
}
impl ::std::cmp::Eq for GNSS_CHIPSETINFO {}
unsafe impl ::windows::runtime::Abi for GNSS_CHIPSETINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_CONTINUOUSTRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub PreferredInterval: u32,
}
impl GNSS_CONTINUOUSTRACKING_PARAM {}
impl ::std::default::Default for GNSS_CONTINUOUSTRACKING_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_CONTINUOUSTRACKING_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_CONTINUOUSTRACKING_PARAM")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("PreferredInterval", &self.PreferredInterval)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_CONTINUOUSTRACKING_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.PreferredInterval == other.PreferredInterval
    }
}
impl ::std::cmp::Eq for GNSS_CONTINUOUSTRACKING_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_CONTINUOUSTRACKING_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_CP_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub NotificationText: [u16; 260],
}
impl GNSS_CP_NI_INFO {}
impl ::std::default::Default for GNSS_CP_NI_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_CP_NI_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_CP_NI_INFO")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("RequestorId", &self.RequestorId)
            .field("NotificationText", &self.NotificationText)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_CP_NI_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.RequestorId == other.RequestorId
            && self.NotificationText == other.NotificationText
    }
}
impl ::std::cmp::Eq for GNSS_CP_NI_INFO {}
unsafe impl ::windows::runtime::Abi for GNSS_CP_NI_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_CWTESTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_CWTESTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_CWTESTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_CWTESTDATA")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("TestResultStatus", &self.TestResultStatus)
            .field("SignalToNoiseRatio", &self.SignalToNoiseRatio)
            .field("Frequency", &self.Frequency)
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_CWTESTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.TestResultStatus == other.TestResultStatus
            && self.SignalToNoiseRatio == other.SignalToNoiseRatio
            && self.Frequency == other.Frequency
            && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_CWTESTDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_CWTESTDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_DEVICE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_DEVICE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_DEVICE_CAPABILITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_DEVICE_CAPABILITY")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field(
                "SupportMultipleFixSessions",
                &self.SupportMultipleFixSessions,
            )
            .field(
                "SupportMultipleAppSessions",
                &self.SupportMultipleAppSessions,
            )
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
            .field(
                "SupportMultipleSuplRootCert",
                &self.SupportMultipleSuplRootCert,
            )
            .field(
                "GnssBreadCrumbPayloadVersion",
                &self.GnssBreadCrumbPayloadVersion,
            )
            .field("MaxGnssBreadCrumbFixes", &self.MaxGnssBreadCrumbFixes)
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_DEVICE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.SupportMultipleFixSessions == other.SupportMultipleFixSessions
            && self.SupportMultipleAppSessions == other.SupportMultipleAppSessions
            && self.RequireAGnssInjection == other.RequireAGnssInjection
            && self.AgnssFormatSupported == other.AgnssFormatSupported
            && self.AgnssFormatPreferred == other.AgnssFormatPreferred
            && self.SupportDistanceTracking == other.SupportDistanceTracking
            && self.SupportContinuousTracking == other.SupportContinuousTracking
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.Reserved3 == other.Reserved3
            && self.Reserved4 == other.Reserved4
            && self.Reserved5 == other.Reserved5
            && self.GeofencingSupport == other.GeofencingSupport
            && self.Reserved6 == other.Reserved6
            && self.Reserved7 == other.Reserved7
            && self.SupportCpLocation == other.SupportCpLocation
            && self.SupportUplV2 == other.SupportUplV2
            && self.SupportSuplV1 == other.SupportSuplV1
            && self.SupportSuplV2 == other.SupportSuplV2
            && self.SupportedSuplVersion == other.SupportedSuplVersion
            && self.MaxGeofencesSupported == other.MaxGeofencesSupported
            && self.SupportMultipleSuplRootCert == other.SupportMultipleSuplRootCert
            && self.GnssBreadCrumbPayloadVersion == other.GnssBreadCrumbPayloadVersion
            && self.MaxGnssBreadCrumbFixes == other.MaxGnssBreadCrumbFixes
            && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_DEVICE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_DEVICE_CAPABILITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_DISTANCETRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MovementThreshold: u32,
}
impl GNSS_DISTANCETRACKING_PARAM {}
impl ::std::default::Default for GNSS_DISTANCETRACKING_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_DISTANCETRACKING_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_DISTANCETRACKING_PARAM")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("MovementThreshold", &self.MovementThreshold)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_DISTANCETRACKING_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.MovementThreshold == other.MovementThreshold
    }
}
impl ::std::cmp::Eq for GNSS_DISTANCETRACKING_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_DISTANCETRACKING_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_DRIVERCOMMAND_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub CommandType: GNSS_DRIVERCOMMAND_TYPE,
    pub Reserved: u32,
    pub CommandDataSize: u32,
    pub Unused: [u8; 512],
    pub CommandData: [u8; 1],
}
impl GNSS_DRIVERCOMMAND_PARAM {}
impl ::std::default::Default for GNSS_DRIVERCOMMAND_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_DRIVERCOMMAND_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_DRIVERCOMMAND_PARAM")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("CommandType", &self.CommandType)
            .field("Reserved", &self.Reserved)
            .field("CommandDataSize", &self.CommandDataSize)
            .field("Unused", &self.Unused)
            .field("CommandData", &self.CommandData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_DRIVERCOMMAND_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.CommandType == other.CommandType
            && self.Reserved == other.Reserved
            && self.CommandDataSize == other.CommandDataSize
            && self.Unused == other.Unused
            && self.CommandData == other.CommandData
    }
}
impl ::std::cmp::Eq for GNSS_DRIVERCOMMAND_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_DRIVERCOMMAND_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for GNSS_DRIVERCOMMAND_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_DRIVERCOMMAND_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_DRIVER_REQUEST(pub i32);
pub const SUPL_CONFIG_DATA: GNSS_DRIVER_REQUEST = GNSS_DRIVER_REQUEST(1i32);
impl ::std::convert::From<i32> for GNSS_DRIVER_REQUEST {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_DRIVER_REQUEST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_DRIVER_REQUEST_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Request: GNSS_DRIVER_REQUEST,
    pub RequestFlag: u32,
}
impl GNSS_DRIVER_REQUEST_DATA {}
impl ::std::default::Default for GNSS_DRIVER_REQUEST_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_DRIVER_REQUEST_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_DRIVER_REQUEST_DATA")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("Request", &self.Request)
            .field("RequestFlag", &self.RequestFlag)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_DRIVER_REQUEST_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.Request == other.Request
            && self.RequestFlag == other.RequestFlag
    }
}
impl ::std::cmp::Eq for GNSS_DRIVER_REQUEST_DATA {}
unsafe impl ::windows::runtime::Abi for GNSS_DRIVER_REQUEST_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GNSS_DRIVER_VERSION_1: u32 = 1u32;
pub const GNSS_DRIVER_VERSION_2: u32 = 2u32;
pub const GNSS_DRIVER_VERSION_3: u32 = 3u32;
pub const GNSS_DRIVER_VERSION_4: u32 = 4u32;
pub const GNSS_DRIVER_VERSION_5: u32 = 5u32;
pub const GNSS_DRIVER_VERSION_6: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_ERRORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_ERRORINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_ERRORINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_ERRORINFO")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("ErrorCode", &self.ErrorCode)
            .field("IsRecoverable", &self.IsRecoverable)
            .field("ErrorDescription", &self.ErrorDescription)
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_ERRORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.ErrorCode == other.ErrorCode
            && self.IsRecoverable == other.IsRecoverable
            && self.ErrorDescription == other.ErrorDescription
            && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_ERRORINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_ERRORINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_EVENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_EVENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_EVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_EVENT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_EVENT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_EVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_EVENT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_EVENT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_EVENT_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_EVENT_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_EVENT_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_EVENT_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_EVENT_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_EVENT_2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_EVENT_2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_EVENT_2_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_EVENT_2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for GNSS_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_EVENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_FIXDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_FIXDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_FIXDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_FIXDATA")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("FixSessionID", &self.FixSessionID)
            .field("FixTimeStamp", &self.FixTimeStamp)
            .field("IsFinalFix", &self.IsFinalFix)
            .field("FixStatus", &self.FixStatus)
            .field("FixLevelOfDetails", &self.FixLevelOfDetails)
            .field("BasicData", &self.BasicData)
            .field("AccuracyData", &self.AccuracyData)
            .field("SatelliteData", &self.SatelliteData)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_FIXDATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.FixSessionID == other.FixSessionID
            && self.FixTimeStamp == other.FixTimeStamp
            && self.IsFinalFix == other.IsFinalFix
            && self.FixStatus == other.FixStatus
            && self.FixLevelOfDetails == other.FixLevelOfDetails
            && self.BasicData == other.BasicData
            && self.AccuracyData == other.AccuracyData
            && self.SatelliteData == other.SatelliteData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_FIXDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_FIXDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_FIXDATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_FIXDATA_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_FIXDATA_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_FIXDATA_2")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("FixSessionID", &self.FixSessionID)
            .field("FixTimeStamp", &self.FixTimeStamp)
            .field("IsFinalFix", &self.IsFinalFix)
            .field("FixStatus", &self.FixStatus)
            .field("FixLevelOfDetails", &self.FixLevelOfDetails)
            .field("BasicData", &self.BasicData)
            .field("AccuracyData", &self.AccuracyData)
            .field("SatelliteData", &self.SatelliteData)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_FIXDATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.FixSessionID == other.FixSessionID
            && self.FixTimeStamp == other.FixTimeStamp
            && self.IsFinalFix == other.IsFinalFix
            && self.FixStatus == other.FixStatus
            && self.FixLevelOfDetails == other.FixLevelOfDetails
            && self.BasicData == other.BasicData
            && self.AccuracyData == other.AccuracyData
            && self.SatelliteData == other.SatelliteData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_FIXDATA_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_FIXDATA_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_FIXDATA_ACCURACY {}
impl ::std::default::Default for GNSS_FIXDATA_ACCURACY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_FIXDATA_ACCURACY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_FIXDATA_ACCURACY")
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
            .field(
                "PositionDilutionOfPrecision",
                &self.PositionDilutionOfPrecision,
            )
            .field(
                "HorizontalDilutionOfPrecision",
                &self.HorizontalDilutionOfPrecision,
            )
            .field(
                "VerticalDilutionOfPrecision",
                &self.VerticalDilutionOfPrecision,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_FIXDATA_ACCURACY {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.HorizontalAccuracy == other.HorizontalAccuracy
            && self.HorizontalErrorMajorAxis == other.HorizontalErrorMajorAxis
            && self.HorizontalErrorMinorAxis == other.HorizontalErrorMinorAxis
            && self.HorizontalErrorAngle == other.HorizontalErrorAngle
            && self.HeadingAccuracy == other.HeadingAccuracy
            && self.AltitudeAccuracy == other.AltitudeAccuracy
            && self.SpeedAccuracy == other.SpeedAccuracy
            && self.HorizontalConfidence == other.HorizontalConfidence
            && self.HeadingConfidence == other.HeadingConfidence
            && self.AltitudeConfidence == other.AltitudeConfidence
            && self.SpeedConfidence == other.SpeedConfidence
            && self.PositionDilutionOfPrecision == other.PositionDilutionOfPrecision
            && self.HorizontalDilutionOfPrecision == other.HorizontalDilutionOfPrecision
            && self.VerticalDilutionOfPrecision == other.VerticalDilutionOfPrecision
    }
}
impl ::std::cmp::Eq for GNSS_FIXDATA_ACCURACY {}
unsafe impl ::windows::runtime::Abi for GNSS_FIXDATA_ACCURACY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_FIXDATA_ACCURACY_2 {}
impl ::std::default::Default for GNSS_FIXDATA_ACCURACY_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_FIXDATA_ACCURACY_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_FIXDATA_ACCURACY_2")
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
            .field(
                "PositionDilutionOfPrecision",
                &self.PositionDilutionOfPrecision,
            )
            .field(
                "HorizontalDilutionOfPrecision",
                &self.HorizontalDilutionOfPrecision,
            )
            .field(
                "VerticalDilutionOfPrecision",
                &self.VerticalDilutionOfPrecision,
            )
            .field(
                "GeometricDilutionOfPrecision",
                &self.GeometricDilutionOfPrecision,
            )
            .field("TimeDilutionOfPrecision", &self.TimeDilutionOfPrecision)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_FIXDATA_ACCURACY_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.HorizontalAccuracy == other.HorizontalAccuracy
            && self.HorizontalErrorMajorAxis == other.HorizontalErrorMajorAxis
            && self.HorizontalErrorMinorAxis == other.HorizontalErrorMinorAxis
            && self.HorizontalErrorAngle == other.HorizontalErrorAngle
            && self.HeadingAccuracy == other.HeadingAccuracy
            && self.AltitudeAccuracy == other.AltitudeAccuracy
            && self.SpeedAccuracy == other.SpeedAccuracy
            && self.HorizontalConfidence == other.HorizontalConfidence
            && self.HeadingConfidence == other.HeadingConfidence
            && self.AltitudeConfidence == other.AltitudeConfidence
            && self.SpeedConfidence == other.SpeedConfidence
            && self.PositionDilutionOfPrecision == other.PositionDilutionOfPrecision
            && self.HorizontalDilutionOfPrecision == other.HorizontalDilutionOfPrecision
            && self.VerticalDilutionOfPrecision == other.VerticalDilutionOfPrecision
            && self.GeometricDilutionOfPrecision == other.GeometricDilutionOfPrecision
            && self.TimeDilutionOfPrecision == other.TimeDilutionOfPrecision
    }
}
impl ::std::cmp::Eq for GNSS_FIXDATA_ACCURACY_2 {}
unsafe impl ::windows::runtime::Abi for GNSS_FIXDATA_ACCURACY_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_FIXDATA_BASIC {
    pub Size: u32,
    pub Version: u32,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Speed: f64,
    pub Heading: f64,
}
impl GNSS_FIXDATA_BASIC {}
impl ::std::default::Default for GNSS_FIXDATA_BASIC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_FIXDATA_BASIC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_FIXDATA_BASIC")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("Latitude", &self.Latitude)
            .field("Longitude", &self.Longitude)
            .field("Altitude", &self.Altitude)
            .field("Speed", &self.Speed)
            .field("Heading", &self.Heading)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_FIXDATA_BASIC {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.Latitude == other.Latitude
            && self.Longitude == other.Longitude
            && self.Altitude == other.Altitude
            && self.Speed == other.Speed
            && self.Heading == other.Heading
    }
}
impl ::std::cmp::Eq for GNSS_FIXDATA_BASIC {}
unsafe impl ::windows::runtime::Abi for GNSS_FIXDATA_BASIC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_FIXDATA_BASIC_2 {}
impl ::std::default::Default for GNSS_FIXDATA_BASIC_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_FIXDATA_BASIC_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_FIXDATA_BASIC_2")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("Latitude", &self.Latitude)
            .field("Longitude", &self.Longitude)
            .field("Altitude", &self.Altitude)
            .field("Speed", &self.Speed)
            .field("Heading", &self.Heading)
            .field("AltitudeEllipsoid", &self.AltitudeEllipsoid)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_FIXDATA_BASIC_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.Latitude == other.Latitude
            && self.Longitude == other.Longitude
            && self.Altitude == other.Altitude
            && self.Speed == other.Speed
            && self.Heading == other.Heading
            && self.AltitudeEllipsoid == other.AltitudeEllipsoid
    }
}
impl ::std::cmp::Eq for GNSS_FIXDATA_BASIC_2 {}
unsafe impl ::windows::runtime::Abi for GNSS_FIXDATA_BASIC_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA_SATELLITE {
    pub Size: u32,
    pub Version: u32,
    pub SatelliteCount: u32,
    pub SatelliteArray: [GNSS_SATELLITEINFO; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_FIXDATA_SATELLITE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_FIXDATA_SATELLITE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_FIXDATA_SATELLITE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_FIXDATA_SATELLITE")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("SatelliteCount", &self.SatelliteCount)
            .field("SatelliteArray", &self.SatelliteArray)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_FIXDATA_SATELLITE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.SatelliteCount == other.SatelliteCount
            && self.SatelliteArray == other.SatelliteArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_FIXDATA_SATELLITE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_FIXDATA_SATELLITE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GNSS_FIXDETAIL_ACCURACY: u32 = 2u32;
pub const GNSS_FIXDETAIL_BASIC: u32 = 1u32;
pub const GNSS_FIXDETAIL_SATELLITE: u32 = 4u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_FIXSESSIONTYPE(pub i32);
pub const GNSS_FixSession_SingleShot: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(1i32);
pub const GNSS_FixSession_DistanceTracking: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(2i32);
pub const GNSS_FixSession_ContinuousTracking: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(3i32);
pub const GNSS_FixSession_LKG: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(4i32);
impl ::std::convert::From<i32> for GNSS_FIXSESSIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_FIXSESSIONTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_FIXSESSION_PARAM {}
impl ::std::default::Default for GNSS_FIXSESSION_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for GNSS_FIXSESSION_PARAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for GNSS_FIXSESSION_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_FIXSESSION_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union GNSS_FIXSESSION_PARAM_0 {
    pub SingleShotParam: GNSS_SINGLESHOT_PARAM,
    pub DistanceParam: GNSS_DISTANCETRACKING_PARAM,
    pub ContinuousParam: GNSS_CONTINUOUSTRACKING_PARAM,
    pub LkgFixParam: GNSS_LKGFIX_PARAM,
    pub UnusedParam: [u8; 268],
}
impl GNSS_FIXSESSION_PARAM_0 {}
impl ::std::default::Default for GNSS_FIXSESSION_PARAM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for GNSS_FIXSESSION_PARAM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for GNSS_FIXSESSION_PARAM_0 {}
unsafe impl ::windows::runtime::Abi for GNSS_FIXSESSION_PARAM_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GNSS_GEOFENCESUPPORT_CIRCLE: u32 = 2u32;
pub const GNSS_GEOFENCESUPPORT_SUPPORTED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Status: super::super::Foundation::NTSTATUS,
    pub StatusTimeStamp: super::super::Foundation::FILETIME,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_GEOFENCES_TRACKINGSTATUS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_GEOFENCES_TRACKINGSTATUS_DATA")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("Status", &self.Status)
            .field("StatusTimeStamp", &self.StatusTimeStamp)
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.Status == other.Status
            && self.StatusTimeStamp == other.StatusTimeStamp
            && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_GEOFENCE_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub GeofenceState: GNSS_GEOFENCE_STATE,
    pub FixBasicData: GNSS_FIXDATA_BASIC,
    pub FixAccuracyData: GNSS_FIXDATA_ACCURACY,
    pub Unused: [u8; 512],
}
impl GNSS_GEOFENCE_ALERT_DATA {}
impl ::std::default::Default for GNSS_GEOFENCE_ALERT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_GEOFENCE_ALERT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_GEOFENCE_ALERT_DATA")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("GeofenceID", &self.GeofenceID)
            .field("GeofenceState", &self.GeofenceState)
            .field("FixBasicData", &self.FixBasicData)
            .field("FixAccuracyData", &self.FixAccuracyData)
            .field("Unused", &self.Unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_GEOFENCE_ALERT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.GeofenceID == other.GeofenceID
            && self.GeofenceState == other.GeofenceState
            && self.FixBasicData == other.FixBasicData
            && self.FixAccuracyData == other.FixAccuracyData
            && self.Unused == other.Unused
    }
}
impl ::std::cmp::Eq for GNSS_GEOFENCE_ALERT_DATA {}
unsafe impl ::windows::runtime::Abi for GNSS_GEOFENCE_ALERT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_GEOFENCE_CREATE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub AlertTypes: u32,
    pub InitialState: GNSS_GEOFENCE_STATE,
    pub Boundary: GNSS_GEOREGION,
    pub Unused: [u8; 512],
}
impl GNSS_GEOFENCE_CREATE_PARAM {}
impl ::std::default::Default for GNSS_GEOFENCE_CREATE_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for GNSS_GEOFENCE_CREATE_PARAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for GNSS_GEOFENCE_CREATE_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_GEOFENCE_CREATE_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCE_CREATE_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub CreationStatus: super::super::Foundation::NTSTATUS,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_GEOFENCE_CREATE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_GEOFENCE_CREATE_RESPONSE")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("CreationStatus", &self.CreationStatus)
            .field("GeofenceID", &self.GeofenceID)
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.CreationStatus == other.CreationStatus
            && self.GeofenceID == other.GeofenceID
            && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_GEOFENCE_CREATE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_GEOFENCE_CREATE_RESPONSE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_GEOFENCE_DELETE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
impl GNSS_GEOFENCE_DELETE_PARAM {}
impl ::std::default::Default for GNSS_GEOFENCE_DELETE_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_GEOFENCE_DELETE_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_GEOFENCE_DELETE_PARAM")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("GeofenceID", &self.GeofenceID)
            .field("Unused", &self.Unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_GEOFENCE_DELETE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.GeofenceID == other.GeofenceID
            && self.Unused == other.Unused
    }
}
impl ::std::cmp::Eq for GNSS_GEOFENCE_DELETE_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_GEOFENCE_DELETE_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_GEOFENCE_STATE(pub i32);
pub const GNSS_GeofenceState_Unknown: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(0i32);
pub const GNSS_GeofenceState_Entered: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(1i32);
pub const GNSS_GeofenceState_Exited: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(2i32);
impl ::std::convert::From<i32> for GNSS_GEOFENCE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_GEOFENCE_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_GEOREGION {
    pub Size: u32,
    pub Version: u32,
    pub GeoRegionType: GNSS_GEOREGIONTYPE,
    pub Anonymous: GNSS_GEOREGION_0,
}
impl GNSS_GEOREGION {}
impl ::std::default::Default for GNSS_GEOREGION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for GNSS_GEOREGION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for GNSS_GEOREGION {}
unsafe impl ::windows::runtime::Abi for GNSS_GEOREGION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union GNSS_GEOREGION_0 {
    pub Circle: GNSS_GEOREGION_CIRCLE,
    pub Unused: [u8; 512],
}
impl GNSS_GEOREGION_0 {}
impl ::std::default::Default for GNSS_GEOREGION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for GNSS_GEOREGION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for GNSS_GEOREGION_0 {}
unsafe impl ::windows::runtime::Abi for GNSS_GEOREGION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_GEOREGIONTYPE(pub i32);
pub const GNSS_GeoRegion_Circle: GNSS_GEOREGIONTYPE = GNSS_GEOREGIONTYPE(1i32);
impl ::std::convert::From<i32> for GNSS_GEOREGIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_GEOREGIONTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_GEOREGION_CIRCLE {
    pub Latitude: f64,
    pub Longitude: f64,
    pub RadiusInMeters: f64,
}
impl GNSS_GEOREGION_CIRCLE {}
impl ::std::default::Default for GNSS_GEOREGION_CIRCLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_GEOREGION_CIRCLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_GEOREGION_CIRCLE")
            .field("Latitude", &self.Latitude)
            .field("Longitude", &self.Longitude)
            .field("RadiusInMeters", &self.RadiusInMeters)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_GEOREGION_CIRCLE {
    fn eq(&self, other: &Self) -> bool {
        self.Latitude == other.Latitude
            && self.Longitude == other.Longitude
            && self.RadiusInMeters == other.RadiusInMeters
    }
}
impl ::std::cmp::Eq for GNSS_GEOREGION_CIRCLE {}
unsafe impl ::windows::runtime::Abi for GNSS_GEOREGION_CIRCLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_LKGFIX_PARAM {
    pub Size: u32,
    pub Version: u32,
}
impl GNSS_LKGFIX_PARAM {}
impl ::std::default::Default for GNSS_LKGFIX_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_LKGFIX_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_LKGFIX_PARAM")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_LKGFIX_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version
    }
}
impl ::std::cmp::Eq for GNSS_LKGFIX_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_LKGFIX_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GNSS_MAXSATELLITE: u32 = 64u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_NI_NOTIFICATION_TYPE(pub i32);
pub const GNSS_NI_NoNotifyNoVerify: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(1i32);
pub const GNSS_NI_NotifyOnly: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(2i32);
pub const GNSS_NI_NotifyVerifyDefaultAllow: GNSS_NI_NOTIFICATION_TYPE =
    GNSS_NI_NOTIFICATION_TYPE(3i32);
pub const GNSS_NI_NotifyVerifyDefaultNotAllow: GNSS_NI_NOTIFICATION_TYPE =
    GNSS_NI_NOTIFICATION_TYPE(4i32);
pub const GNSS_NI_PrivacyOverride: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(5i32);
impl ::std::convert::From<i32> for GNSS_NI_NOTIFICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_NI_NOTIFICATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_NI_PLANE_TYPE(pub i32);
pub const GNSS_NI_SUPL: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(1i32);
pub const GNSS_NI_CP: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(2i32);
pub const GNSS_NI_V2UPL: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(3i32);
impl ::std::convert::From<i32> for GNSS_NI_PLANE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_NI_PLANE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_NI_REQUEST_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_NI_REQUEST_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_NI_REQUEST_PARAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_NI_REQUEST_PARAM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_NI_REQUEST_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_NI_REQUEST_PARAM_0 {
    pub SuplNiInfo: GNSS_SUPL_NI_INFO,
    pub CpNiInfo: GNSS_CP_NI_INFO,
    pub V2UplNiInfo: GNSS_V2UPL_NI_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_NI_REQUEST_PARAM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_NI_REQUEST_PARAM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_NI_REQUEST_PARAM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_NI_REQUEST_PARAM_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_NI_REQUEST_PARAM_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_NI_REQUEST_TYPE(pub i32);
pub const GNSS_NI_Request_SingleShot: GNSS_NI_REQUEST_TYPE = GNSS_NI_REQUEST_TYPE(1i32);
pub const GNSS_NI_Request_AreaTrigger: GNSS_NI_REQUEST_TYPE = GNSS_NI_REQUEST_TYPE(2i32);
impl ::std::convert::From<i32> for GNSS_NI_REQUEST_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_NI_REQUEST_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_NI_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub RequestId: u32,
    pub UserResponse: GNSS_NI_USER_RESPONSE,
}
impl GNSS_NI_RESPONSE {}
impl ::std::default::Default for GNSS_NI_RESPONSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_NI_RESPONSE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_NI_RESPONSE")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("RequestId", &self.RequestId)
            .field("UserResponse", &self.UserResponse)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_NI_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.RequestId == other.RequestId
            && self.UserResponse == other.UserResponse
    }
}
impl ::std::cmp::Eq for GNSS_NI_RESPONSE {}
unsafe impl ::windows::runtime::Abi for GNSS_NI_RESPONSE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_NI_USER_RESPONSE(pub i32);
pub const GNSS_Ni_UserResponseAccept: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(1i32);
pub const GNSS_Ni_UserResponseDeny: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(2i32);
pub const GNSS_Ni_UserResponseTimeout: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(3i32);
impl ::std::convert::From<i32> for GNSS_NI_USER_RESPONSE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_NI_USER_RESPONSE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GNSS_NMEALOGGING_ALL: u32 = 255u32;
pub const GNSS_NMEALOGGING_NONE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_NMEA_DATA {
    pub Size: u32,
    pub Version: u32,
    pub NmeaSentences: [super::super::Foundation::CHAR; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_NMEA_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_NMEA_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_NMEA_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_NMEA_DATA")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("NmeaSentences", &self.NmeaSentences)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_NMEA_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.NmeaSentences == other.NmeaSentences
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_NMEA_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_NMEA_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GNSS_OPERMODE_AFLT: u32 = 16u32;
pub const GNSS_OPERMODE_ANY: u32 = 0u32;
pub const GNSS_OPERMODE_CELLID: u32 = 8u32;
pub const GNSS_OPERMODE_MSA: u32 = 1u32;
pub const GNSS_OPERMODE_MSB: u32 = 2u32;
pub const GNSS_OPERMODE_MSS: u32 = 4u32;
pub const GNSS_OPERMODE_OTDOA: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_PLATFORM_CAPABILITY {
    pub Size: u32,
    pub Version: u32,
    pub SupportAgnssInjection: super::super::Foundation::BOOL,
    pub AgnssFormatSupported: u32,
    pub Unused: [u8; 516],
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_PLATFORM_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_PLATFORM_CAPABILITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_PLATFORM_CAPABILITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_PLATFORM_CAPABILITY")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("SupportAgnssInjection", &self.SupportAgnssInjection)
            .field("AgnssFormatSupported", &self.AgnssFormatSupported)
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_PLATFORM_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.SupportAgnssInjection == other.SupportAgnssInjection
            && self.AgnssFormatSupported == other.AgnssFormatSupported
            && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_PLATFORM_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_PLATFORM_CAPABILITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SATELLITEINFO {
    pub SatelliteId: u32,
    pub UsedInPositiong: super::super::Foundation::BOOL,
    pub Elevation: f64,
    pub Azimuth: f64,
    pub SignalToNoiseRatio: f64,
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_SATELLITEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_SATELLITEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_SATELLITEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_SATELLITEINFO")
            .field("SatelliteId", &self.SatelliteId)
            .field("UsedInPositiong", &self.UsedInPositiong)
            .field("Elevation", &self.Elevation)
            .field("Azimuth", &self.Azimuth)
            .field("SignalToNoiseRatio", &self.SignalToNoiseRatio)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_SATELLITEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.SatelliteId == other.SatelliteId
            && self.UsedInPositiong == other.UsedInPositiong
            && self.Elevation == other.Elevation
            && self.Azimuth == other.Azimuth
            && self.SignalToNoiseRatio == other.SignalToNoiseRatio
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_SATELLITEINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_SATELLITEINFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GNSS_SATELLITE_ANY: u32 = 0u32;
pub const GNSS_SATELLITE_BEIDOU: u32 = 4u32;
pub const GNSS_SATELLITE_GALILEO: u32 = 8u32;
pub const GNSS_SATELLITE_GLONASS: u32 = 2u32;
pub const GNSS_SATELLITE_GPS: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_SELFTESTCONFIG {
    pub Size: u32,
    pub Version: u32,
    pub TestType: u32,
    pub Unused: [u8; 512],
    pub InBufLen: u32,
    pub InBuffer: [u8; 1],
}
impl GNSS_SELFTESTCONFIG {}
impl ::std::default::Default for GNSS_SELFTESTCONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_SELFTESTCONFIG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_SELFTESTCONFIG")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("TestType", &self.TestType)
            .field("Unused", &self.Unused)
            .field("InBufLen", &self.InBufLen)
            .field("InBuffer", &self.InBuffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_SELFTESTCONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.TestType == other.TestType
            && self.Unused == other.Unused
            && self.InBufLen == other.InBufLen
            && self.InBuffer == other.InBuffer
    }
}
impl ::std::cmp::Eq for GNSS_SELFTESTCONFIG {}
unsafe impl ::windows::runtime::Abi for GNSS_SELFTESTCONFIG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_SELFTESTRESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_SELFTESTRESULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_SELFTESTRESULT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_SELFTESTRESULT")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("TestResultStatus", &self.TestResultStatus)
            .field("Result", &self.Result)
            .field("PinFailedBitMask", &self.PinFailedBitMask)
            .field("Unused", &self.Unused)
            .field("OutBufLen", &self.OutBufLen)
            .field("OutBuffer", &self.OutBuffer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_SELFTESTRESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.TestResultStatus == other.TestResultStatus
            && self.Result == other.Result
            && self.PinFailedBitMask == other.PinFailedBitMask
            && self.Unused == other.Unused
            && self.OutBufLen == other.OutBufLen
            && self.OutBuffer == other.OutBuffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_SELFTESTRESULT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_SELFTESTRESULT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_SINGLESHOT_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub ResponseTime: u32,
}
impl GNSS_SINGLESHOT_PARAM {}
impl ::std::default::Default for GNSS_SINGLESHOT_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_SINGLESHOT_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_SINGLESHOT_PARAM")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("ResponseTime", &self.ResponseTime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_SINGLESHOT_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.ResponseTime == other.ResponseTime
    }
}
impl ::std::cmp::Eq for GNSS_SINGLESHOT_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_SINGLESHOT_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_STOPFIXSESSION_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub Unused: [u8; 512],
}
impl GNSS_STOPFIXSESSION_PARAM {}
impl ::std::default::Default for GNSS_STOPFIXSESSION_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_STOPFIXSESSION_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_STOPFIXSESSION_PARAM")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("FixSessionID", &self.FixSessionID)
            .field("Unused", &self.Unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_STOPFIXSESSION_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.FixSessionID == other.FixSessionID
            && self.Unused == other.Unused
    }
}
impl ::std::cmp::Eq for GNSS_STOPFIXSESSION_PARAM {}
unsafe impl ::windows::runtime::Abi for GNSS_STOPFIXSESSION_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GNSS_SUPL_CERT_ACTION(pub i32);
pub const GNSS_Supl_Cert_Inject: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(1i32);
pub const GNSS_Supl_Cert_Delete: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(2i32);
pub const GNSS_Supl_Cert_Purge: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(3i32);
impl ::std::convert::From<i32> for GNSS_SUPL_CERT_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GNSS_SUPL_CERT_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_SUPL_CERT_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_SUPL_CERT_CONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_SUPL_CERT_CONFIG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_SUPL_CERT_CONFIG")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("CertAction", &self.CertAction)
            .field("SuplCertName", &self.SuplCertName)
            .field("CertSize", &self.CertSize)
            .field("Unused", &self.Unused)
            .field("CertData", &self.CertData)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_SUPL_CERT_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.CertAction == other.CertAction
            && self.SuplCertName == other.SuplCertName
            && self.CertSize == other.CertSize
            && self.Unused == other.Unused
            && self.CertData == other.CertData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_SUPL_CERT_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_SUPL_CERT_CONFIG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_SUPL_HSLP_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_SUPL_HSLP_CONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_SUPL_HSLP_CONFIG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_SUPL_HSLP_CONFIG")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("SuplHslp", &self.SuplHslp)
            .field("SuplHslpFromImsi", &self.SuplHslpFromImsi)
            .field("Reserved", &self.Reserved)
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_SUPL_HSLP_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.SuplHslp == other.SuplHslp
            && self.SuplHslpFromImsi == other.SuplHslpFromImsi
            && self.Reserved == other.Reserved
            && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_SUPL_HSLP_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_SUPL_HSLP_CONFIG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub ClientName: [u16; 260],
    pub SuplNiUrl: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl GNSS_SUPL_NI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_SUPL_NI_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_SUPL_NI_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_SUPL_NI_INFO")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("RequestorId", &self.RequestorId)
            .field("ClientName", &self.ClientName)
            .field("SuplNiUrl", &self.SuplNiUrl)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_SUPL_NI_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.RequestorId == other.RequestorId
            && self.ClientName == other.ClientName
            && self.SuplNiUrl == other.SuplNiUrl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_SUPL_NI_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_SUPL_NI_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_SUPL_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl GNSS_SUPL_VERSION {}
impl ::std::default::Default for GNSS_SUPL_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_SUPL_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_SUPL_VERSION")
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_SUPL_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::std::cmp::Eq for GNSS_SUPL_VERSION {}
unsafe impl ::windows::runtime::Abi for GNSS_SUPL_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_SUPL_VERSION_2 {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ServiceIndicator: u32,
}
impl GNSS_SUPL_VERSION_2 {}
impl ::std::default::Default for GNSS_SUPL_VERSION_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_SUPL_VERSION_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_SUPL_VERSION_2")
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("ServiceIndicator", &self.ServiceIndicator)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_SUPL_VERSION_2 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion
            && self.MinorVersion == other.MinorVersion
            && self.ServiceIndicator == other.ServiceIndicator
    }
}
impl ::std::cmp::Eq for GNSS_SUPL_VERSION_2 {}
unsafe impl ::windows::runtime::Abi for GNSS_SUPL_VERSION_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl GNSS_V2UPL_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GNSS_V2UPL_CONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GNSS_V2UPL_CONFIG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_V2UPL_CONFIG")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("MPC", &self.MPC)
            .field("PDE", &self.PDE)
            .field(
                "ApplicationTypeIndicator_MR",
                &self.ApplicationTypeIndicator_MR,
            )
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GNSS_V2UPL_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.MPC == other.MPC
            && self.PDE == other.PDE
            && self.ApplicationTypeIndicator_MR == other.ApplicationTypeIndicator_MR
            && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GNSS_V2UPL_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GNSS_V2UPL_CONFIG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GNSS_V2UPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
}
impl GNSS_V2UPL_NI_INFO {}
impl ::std::default::Default for GNSS_V2UPL_NI_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GNSS_V2UPL_NI_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GNSS_V2UPL_NI_INFO")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("RequestorId", &self.RequestorId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GNSS_V2UPL_NI_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.RequestorId == other.RequestorId
    }
}
impl ::std::cmp::Eq for GNSS_V2UPL_NI_INFO {}
unsafe impl ::windows::runtime::Abi for GNSS_V2UPL_NI_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GUID_DEVINTERFACE_GNSS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    859235812,
    394,
    18025,
    [132, 197, 189, 5, 243, 189, 54, 139],
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICivicAddressReport(::windows::runtime::IUnknown);
impl ICivicAddressReport {
    pub unsafe fn GetSensorID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: <super::super::Foundation::SYSTEMTIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation",
        feature = "Win32_System_PropertiesSystem"
    ))]
    pub unsafe fn GetValue(
        &self,
        pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY,
    ) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ : < super::super::System::Com::StructuredStorage:: PROPVARIANT as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pkey),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAddressLine1(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAddressLine2(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCity(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStateProvince(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostalCode(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCountryRegion(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetDetailLevel(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICivicAddressReport {
    type Vtable = ICivicAddressReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3232866160,
        19167,
        17501,
        [135, 242, 202, 216, 253, 113, 23, 146],
    );
}
impl ::std::convert::From<ICivicAddressReport> for ::windows::runtime::IUnknown {
    fn from(value: ICivicAddressReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICivicAddressReport> for ::windows::runtime::IUnknown {
    fn from(value: &ICivicAddressReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICivicAddressReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICivicAddressReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ICivicAddressReport> for ILocationReport {
    fn from(value: ICivicAddressReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICivicAddressReport> for ILocationReport {
    fn from(value: &ICivicAddressReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILocationReport> for ICivicAddressReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILocationReport> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILocationReport>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILocationReport> for &ICivicAddressReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILocationReport> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILocationReport>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddressReport_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psensorid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcreationtime: *mut super::super::Foundation::SYSTEMTIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation",
        feature = "Win32_System_PropertiesSystem"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY,
        pvalue: *mut ::std::mem::ManuallyDrop<
            super::super::System::Com::StructuredStorage::PROPVARIANT,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation",
        feature = "Win32_System_PropertiesSystem"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstraddress1: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstraddress2: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrcity: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrstateprovince: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpostalcode: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrcountryregion: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdetaillevel: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICivicAddressReportFactory(::windows::runtime::IUnknown);
impl ICivicAddressReportFactory {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__ : < super::super::System::Ole::Automation:: ITypeInfo as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::super::System::Com::VARIANT,
        pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn ListenForReports(
        &self,
        requestedreportinterval: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(requestedreportinterval),
        )
        .ok()
    }
    pub unsafe fn StopListeningForReports(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn ReportInterval(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetReportInterval(
        &self,
        millisecondsrequested: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(millisecondsrequested),
        )
        .ok()
    }
    pub unsafe fn DesiredAccuracy(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetDesiredAccuracy(
        &self,
        desiredaccuracy: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(desiredaccuracy),
        )
        .ok()
    }
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hwnd),
        )
        .ok()
    }
    pub unsafe fn CivicAddressReport(&self) -> ::windows::runtime::Result<IDispCivicAddressReport> {
        let mut result__: <IDispCivicAddressReport as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDispCivicAddressReport>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICivicAddressReportFactory {
    type Vtable = ICivicAddressReportFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3212262291,
        50767,
        19438,
        [190, 178, 103, 192, 184, 223, 102, 224],
    );
}
impl ::std::convert::From<ICivicAddressReportFactory> for ::windows::runtime::IUnknown {
    fn from(value: ICivicAddressReportFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICivicAddressReportFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ICivicAddressReportFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICivicAddressReportFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ICivicAddressReportFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ICivicAddressReportFactory> for ILocationReportFactory {
    fn from(value: ICivicAddressReportFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICivicAddressReportFactory> for ILocationReportFactory {
    fn from(value: &ICivicAddressReportFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILocationReportFactory> for ICivicAddressReportFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILocationReportFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILocationReportFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILocationReportFactory> for &ICivicAddressReportFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILocationReportFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILocationReportFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<ICivicAddressReportFactory>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: ICivicAddressReportFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&ICivicAddressReportFactory>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &ICivicAddressReportFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for ICivicAddressReportFactory
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &ICivicAddressReportFactory
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddressReportFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        requestedreportinterval: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmilliseconds: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        millisecondsrequested: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesiredaccuracy: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        desiredaccuracy: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwnd: *const u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDefaultLocation(::windows::runtime::IUnknown);
impl IDefaultLocation {
    pub unsafe fn SetReport<'a, Param1: ::windows::runtime::IntoParam<'a, ILocationReport>>(
        &self,
        reporttype: *const ::windows::runtime::GUID,
        plocationreport: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            plocationreport.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetReport(
        &self,
        reporttype: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<ILocationReport> {
        let mut result__: <ILocationReport as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            &mut result__,
        )
        .from_abi::<ILocationReport>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDefaultLocation {
    type Vtable = IDefaultLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2790979454,
        38554,
        18990,
        [138, 202, 51, 187, 124, 187, 18, 53],
    );
}
impl ::std::convert::From<IDefaultLocation> for ::windows::runtime::IUnknown {
    fn from(value: IDefaultLocation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDefaultLocation> for ::windows::runtime::IUnknown {
    fn from(value: &IDefaultLocation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDefaultLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDefaultLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultLocation_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        plocationreport: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        pplocationreport: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDispCivicAddressReport(::windows::runtime::IUnknown);
impl IDispCivicAddressReport {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddressLine1(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddressLine2(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn City(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StateProvince(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostalCode(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CountryRegion(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn DetailLevel(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn Timestamp(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDispCivicAddressReport {
    type Vtable = IDispCivicAddressReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        385817140,
        40496,
        17091,
        [180, 77, 226, 37, 19, 181, 118, 122],
    );
}
impl ::std::convert::From<IDispCivicAddressReport> for ::windows::runtime::IUnknown {
    fn from(value: IDispCivicAddressReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDispCivicAddressReport> for ::windows::runtime::IUnknown {
    fn from(value: &IDispCivicAddressReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDispCivicAddressReport
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDispCivicAddressReport
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IDispCivicAddressReport>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: IDispCivicAddressReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IDispCivicAddressReport>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &IDispCivicAddressReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for IDispCivicAddressReport
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &IDispCivicAddressReport
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispCivicAddressReport_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        paddress1: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        paddress2: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcity: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstateprovince: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppostalcode: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcountryregion: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdetaillevel: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDispLatLongReport(::windows::runtime::IUnknown);
impl IDispLatLongReport {
    pub unsafe fn Latitude(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    pub unsafe fn Longitude(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    pub unsafe fn ErrorRadius(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    pub unsafe fn Altitude(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    pub unsafe fn AltitudeError(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    pub unsafe fn Timestamp(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDispLatLongReport {
    type Vtable = IDispLatLongReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2330142499,
        14491,
        18961,
        [153, 87, 91, 221, 72, 252, 150, 23],
    );
}
impl ::std::convert::From<IDispLatLongReport> for ::windows::runtime::IUnknown {
    fn from(value: IDispLatLongReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDispLatLongReport> for ::windows::runtime::IUnknown {
    fn from(value: &IDispLatLongReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDispLatLongReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDispLatLongReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IDispLatLongReport> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IDispLatLongReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IDispLatLongReport>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &IDispLatLongReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for IDispLatLongReport
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &IDispLatLongReport
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispLatLongReport_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ILatLongReport(::windows::runtime::IUnknown);
impl ILatLongReport {
    pub unsafe fn GetSensorID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: <super::super::Foundation::SYSTEMTIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation",
        feature = "Win32_System_PropertiesSystem"
    ))]
    pub unsafe fn GetValue(
        &self,
        pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY,
    ) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ : < super::super::System::Com::StructuredStorage:: PROPVARIANT as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pkey),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    pub unsafe fn GetLatitude(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    pub unsafe fn GetLongitude(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    pub unsafe fn GetErrorRadius(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    pub unsafe fn GetAltitude(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    pub unsafe fn GetAltitudeError(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILatLongReport {
    type Vtable = ILatLongReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2146271341,
        3832,
        20231,
        [128, 172, 54, 160, 190, 174, 49, 52],
    );
}
impl ::std::convert::From<ILatLongReport> for ::windows::runtime::IUnknown {
    fn from(value: ILatLongReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILatLongReport> for ::windows::runtime::IUnknown {
    fn from(value: &ILatLongReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILatLongReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ILatLongReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ILatLongReport> for ILocationReport {
    fn from(value: ILatLongReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILatLongReport> for ILocationReport {
    fn from(value: &ILatLongReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILocationReport> for ILatLongReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILocationReport> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILocationReport>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILocationReport> for &ILatLongReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILocationReport> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILocationReport>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILatLongReport_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psensorid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcreationtime: *mut super::super::Foundation::SYSTEMTIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation",
        feature = "Win32_System_PropertiesSystem"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY,
        pvalue: *mut ::std::mem::ManuallyDrop<
            super::super::System::Com::StructuredStorage::PROPVARIANT,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation",
        feature = "Win32_System_PropertiesSystem"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        platitude: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plongitude: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        perrorradius: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        paltitude: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        paltitudeerror: *mut f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ILatLongReportFactory(::windows::runtime::IUnknown);
impl ILatLongReportFactory {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__ : < super::super::System::Ole::Automation:: ITypeInfo as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::super::System::Com::VARIANT,
        pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn ListenForReports(
        &self,
        requestedreportinterval: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(requestedreportinterval),
        )
        .ok()
    }
    pub unsafe fn StopListeningForReports(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn ReportInterval(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetReportInterval(
        &self,
        millisecondsrequested: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(millisecondsrequested),
        )
        .ok()
    }
    pub unsafe fn DesiredAccuracy(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetDesiredAccuracy(
        &self,
        desiredaccuracy: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(desiredaccuracy),
        )
        .ok()
    }
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hwnd),
        )
        .ok()
    }
    pub unsafe fn LatLongReport(&self) -> ::windows::runtime::Result<IDispLatLongReport> {
        let mut result__: <IDispLatLongReport as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDispLatLongReport>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILatLongReportFactory {
    type Vtable = ILatLongReportFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1057490123,
        45332,
        17533,
        [131, 221, 57, 1, 116, 235, 176, 130],
    );
}
impl ::std::convert::From<ILatLongReportFactory> for ::windows::runtime::IUnknown {
    fn from(value: ILatLongReportFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILatLongReportFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ILatLongReportFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILatLongReportFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ILatLongReportFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ILatLongReportFactory> for ILocationReportFactory {
    fn from(value: ILatLongReportFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILatLongReportFactory> for ILocationReportFactory {
    fn from(value: &ILatLongReportFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILocationReportFactory> for ILatLongReportFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILocationReportFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILocationReportFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILocationReportFactory> for &ILatLongReportFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILocationReportFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILocationReportFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<ILatLongReportFactory>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: ILatLongReportFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&ILatLongReportFactory>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &ILatLongReportFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for ILatLongReportFactory
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &ILatLongReportFactory
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILatLongReportFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        requestedreportinterval: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmilliseconds: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        millisecondsrequested: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesiredaccuracy: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        desiredaccuracy: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwnd: *const u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ILocation(::windows::runtime::IUnknown);
impl ILocation {
    pub unsafe fn RegisterForReport<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ILocationEvents>,
    >(
        &self,
        pevents: Param0,
        reporttype: *const ::windows::runtime::GUID,
        dwrequestedreportinterval: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pevents.into_param().abi(),
            ::std::mem::transmute(reporttype),
            ::std::mem::transmute(dwrequestedreportinterval),
        )
        .ok()
    }
    pub unsafe fn UnregisterForReport(
        &self,
        reporttype: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
        )
        .ok()
    }
    pub unsafe fn GetReport(
        &self,
        reporttype: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<ILocationReport> {
        let mut result__: <ILocationReport as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            &mut result__,
        )
        .from_abi::<ILocationReport>(result__)
    }
    pub unsafe fn GetReportStatus(
        &self,
        reporttype: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<LOCATION_REPORT_STATUS> {
        let mut result__: <LOCATION_REPORT_STATUS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            &mut result__,
        )
        .from_abi::<LOCATION_REPORT_STATUS>(result__)
    }
    pub unsafe fn GetReportInterval(
        &self,
        reporttype: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetReportInterval(
        &self,
        reporttype: *const ::windows::runtime::GUID,
        millisecondsrequested: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            ::std::mem::transmute(millisecondsrequested),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub unsafe fn GetDesiredAccuracy(
        &self,
        reporttype: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<super::Sensors::LOCATION_DESIRED_ACCURACY> {
        let mut result__ : < super::Sensors:: LOCATION_DESIRED_ACCURACY as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            &mut result__,
        )
        .from_abi::<super::Sensors::LOCATION_DESIRED_ACCURACY>(result__)
    }
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub unsafe fn SetDesiredAccuracy(
        &self,
        reporttype: *const ::windows::runtime::GUID,
        desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            ::std::mem::transmute(desiredaccuracy),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestPermissions<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hparent: Param0,
        preporttypes: *const ::windows::runtime::GUID,
        count: u32,
        fmodal: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            hparent.into_param().abi(),
            ::std::mem::transmute(preporttypes),
            ::std::mem::transmute(count),
            fmodal.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILocation {
    type Vtable = ILocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2871971433,
        22233,
        20264,
        [181, 37, 222, 27, 14, 228, 66, 55],
    );
}
impl ::std::convert::From<ILocation> for ::windows::runtime::IUnknown {
    fn from(value: ILocation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILocation> for ::windows::runtime::IUnknown {
    fn from(value: &ILocation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ILocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocation_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pevents: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        dwrequestedreportinterval: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        pplocationreport: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        pstatus: *mut LOCATION_REPORT_STATUS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        pmilliseconds: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        millisecondsrequested: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        pdesiredaccuracy: *mut super::Sensors::LOCATION_DESIRED_ACCURACY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Sensors"))] usize,
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Sensors"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hparent: super::super::Foundation::HWND,
        preporttypes: *const ::windows::runtime::GUID,
        count: u32,
        fmodal: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ILocationEvents(::windows::runtime::IUnknown);
impl ILocationEvents {
    pub unsafe fn OnLocationChanged<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ILocationReport>,
    >(
        &self,
        reporttype: *const ::windows::runtime::GUID,
        plocationreport: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            plocationreport.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnStatusChanged(
        &self,
        reporttype: *const ::windows::runtime::GUID,
        newstatus: LOCATION_REPORT_STATUS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reporttype),
            ::std::mem::transmute(newstatus),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILocationEvents {
    type Vtable = ILocationEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3403688895,
        31115,
        17672,
        [162, 7, 53, 167, 144, 109, 199, 61],
    );
}
impl ::std::convert::From<ILocationEvents> for ::windows::runtime::IUnknown {
    fn from(value: ILocationEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILocationEvents> for ::windows::runtime::IUnknown {
    fn from(value: &ILocationEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILocationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ILocationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationEvents_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        plocationreport: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reporttype: *const ::windows::runtime::GUID,
        newstatus: LOCATION_REPORT_STATUS,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ILocationPower(::windows::runtime::IUnknown);
impl ILocationPower {
    pub unsafe fn Connect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILocationPower {
    type Vtable = ILocationPower_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        423524137,
        43883,
        19218,
        [134, 23, 117, 150, 225, 187, 25, 28],
    );
}
impl ::std::convert::From<ILocationPower> for ::windows::runtime::IUnknown {
    fn from(value: ILocationPower) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILocationPower> for ::windows::runtime::IUnknown {
    fn from(value: &ILocationPower) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILocationPower {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ILocationPower {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationPower_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ILocationReport(::windows::runtime::IUnknown);
impl ILocationReport {
    pub unsafe fn GetSensorID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: <super::super::Foundation::SYSTEMTIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation",
        feature = "Win32_System_PropertiesSystem"
    ))]
    pub unsafe fn GetValue(
        &self,
        pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY,
    ) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ : < super::super::System::Com::StructuredStorage:: PROPVARIANT as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pkey),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILocationReport {
    type Vtable = ILocationReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3367499758,
        30160,
        19897,
        [182, 45, 122, 15, 54, 156, 164, 86],
    );
}
impl ::std::convert::From<ILocationReport> for ::windows::runtime::IUnknown {
    fn from(value: ILocationReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILocationReport> for ::windows::runtime::IUnknown {
    fn from(value: &ILocationReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILocationReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ILocationReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationReport_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psensorid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcreationtime: *mut super::super::Foundation::SYSTEMTIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation",
        feature = "Win32_System_PropertiesSystem"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pkey: *const super::super::System::PropertiesSystem::PROPERTYKEY,
        pvalue: *mut ::std::mem::ManuallyDrop<
            super::super::System::Com::StructuredStorage::PROPVARIANT,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation",
        feature = "Win32_System_PropertiesSystem"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ILocationReportFactory(::windows::runtime::IUnknown);
impl ILocationReportFactory {
    pub unsafe fn ListenForReports(
        &self,
        requestedreportinterval: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(requestedreportinterval),
        )
        .ok()
    }
    pub unsafe fn StopListeningForReports(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn ReportInterval(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetReportInterval(
        &self,
        millisecondsrequested: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(millisecondsrequested),
        )
        .ok()
    }
    pub unsafe fn DesiredAccuracy(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetDesiredAccuracy(
        &self,
        desiredaccuracy: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(desiredaccuracy),
        )
        .ok()
    }
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hwnd),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILocationReportFactory {
    type Vtable = ILocationReportFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        766427938,
        37042,
        18404,
        [187, 8, 13, 168, 65, 147, 90, 107],
    );
}
impl ::std::convert::From<ILocationReportFactory> for ::windows::runtime::IUnknown {
    fn from(value: ILocationReportFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILocationReportFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ILocationReportFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ILocationReportFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ILocationReportFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<ILocationReportFactory>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: ILocationReportFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&ILocationReportFactory>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &ILocationReportFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for ILocationReportFactory
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &ILocationReportFactory
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationReportFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        requestedreportinterval: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmilliseconds: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        millisecondsrequested: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdesiredaccuracy: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        desiredaccuracy: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwnd: *const u32,
    ) -> ::windows::runtime::HRESULT,
);
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
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LOCATION_REPORT_STATUS(pub i32);
pub const REPORT_NOT_SUPPORTED: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(0i32);
pub const REPORT_ERROR: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(1i32);
pub const REPORT_ACCESS_DENIED: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(2i32);
pub const REPORT_INITIALIZING: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(3i32);
pub const REPORT_RUNNING: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(4i32);
impl ::std::convert::From<i32> for LOCATION_REPORT_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOCATION_REPORT_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const LatLongReport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3984703603,
    8068,
    19624,
    [161, 97, 24, 60, 119, 107, 198, 81],
);
pub const LatLongReportFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2647407816,
    34313,
    18531,
    [186, 212, 3, 96, 31, 76, 101, 232],
);
pub const Location: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3854098553,
    61037,
    20019,
    [164, 56, 200, 127, 46, 149, 146, 84],
);
pub const MAX_SERVER_URL_NAME: u32 = 260u32;
pub const MIN_BREADCRUMBS_SUPPORTED: u32 = 120u32;
pub const MIN_GEOFENCES_REQUIRED: u32 = 100u32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct _ICivicAddressReportFactoryEvents(::windows::runtime::IUnknown);
impl _ICivicAddressReportFactoryEvents {}
unsafe impl ::windows::runtime::Interface for _ICivicAddressReportFactoryEvents {
    type Vtable = _ICivicAddressReportFactoryEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3378526719,
        29420,
        17943,
        [137, 189, 132, 216, 139, 237, 199, 34],
    );
}
impl ::std::convert::From<_ICivicAddressReportFactoryEvents> for ::windows::runtime::IUnknown {
    fn from(value: _ICivicAddressReportFactoryEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&_ICivicAddressReportFactoryEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_ICivicAddressReportFactoryEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for _ICivicAddressReportFactoryEvents
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &_ICivicAddressReportFactoryEvents
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_ICivicAddressReportFactoryEvents>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: _ICivicAddressReportFactoryEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_ICivicAddressReportFactoryEvents>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &_ICivicAddressReportFactoryEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for _ICivicAddressReportFactoryEvents
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &_ICivicAddressReportFactoryEvents
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _ICivicAddressReportFactoryEvents_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct _ILatLongReportFactoryEvents(::windows::runtime::IUnknown);
impl _ILatLongReportFactoryEvents {}
unsafe impl ::windows::runtime::Interface for _ILatLongReportFactoryEvents {
    type Vtable = _ILatLongReportFactoryEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        384724151,
        43836,
        16971,
        [132, 159, 38, 155, 229, 81, 252, 188],
    );
}
impl ::std::convert::From<_ILatLongReportFactoryEvents> for ::windows::runtime::IUnknown {
    fn from(value: _ILatLongReportFactoryEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&_ILatLongReportFactoryEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_ILatLongReportFactoryEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for _ILatLongReportFactoryEvents
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &_ILatLongReportFactoryEvents
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_ILatLongReportFactoryEvents>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: _ILatLongReportFactoryEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_ILatLongReportFactoryEvents>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &_ILatLongReportFactoryEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for _ILatLongReportFactoryEvents
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &_ILatLongReportFactoryEvents
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _ILatLongReportFactoryEvents_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
