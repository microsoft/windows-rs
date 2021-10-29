#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ACPI_REAL_TIME {
    pub Year: u16,
    pub Month: u8,
    pub Day: u8,
    pub Hour: u8,
    pub Minute: u8,
    pub Second: u8,
    pub Valid: u8,
    pub Milliseconds: u16,
    pub TimeZone: i16,
    pub DayLight: u8,
    pub Reserved1: [u8; 3],
}
impl ACPI_REAL_TIME {}
impl ::std::default::Default for ACPI_REAL_TIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACPI_REAL_TIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACPI_REAL_TIME")
            .field("Year", &self.Year)
            .field("Month", &self.Month)
            .field("Day", &self.Day)
            .field("Hour", &self.Hour)
            .field("Minute", &self.Minute)
            .field("Second", &self.Second)
            .field("Valid", &self.Valid)
            .field("Milliseconds", &self.Milliseconds)
            .field("TimeZone", &self.TimeZone)
            .field("DayLight", &self.DayLight)
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACPI_REAL_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.Year == other.Year
            && self.Month == other.Month
            && self.Day == other.Day
            && self.Hour == other.Hour
            && self.Minute == other.Minute
            && self.Second == other.Second
            && self.Valid == other.Valid
            && self.Milliseconds == other.Milliseconds
            && self.TimeZone == other.TimeZone
            && self.DayLight == other.DayLight
            && self.Reserved1 == other.Reserved1
    }
}
impl ::std::cmp::Eq for ACPI_REAL_TIME {}
unsafe impl ::windows::runtime::Abi for ACPI_REAL_TIME {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ACPI_TIME_ADJUST_DAYLIGHT: u32 = 1u32;
pub const ACPI_TIME_IN_DAYLIGHT: u32 = 2u32;
pub const ACPI_TIME_ZONE_UNKNOWN: u32 = 2047u32;
pub const ACTIVE_COOLING: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ADMINISTRATOR_POWER_POLICY {
    pub MinSleep: SYSTEM_POWER_STATE,
    pub MaxSleep: SYSTEM_POWER_STATE,
    pub MinVideoTimeout: u32,
    pub MaxVideoTimeout: u32,
    pub MinSpindownTimeout: u32,
    pub MaxSpindownTimeout: u32,
}
impl ADMINISTRATOR_POWER_POLICY {}
impl ::std::default::Default for ADMINISTRATOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ADMINISTRATOR_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ADMINISTRATOR_POWER_POLICY")
            .field("MinSleep", &self.MinSleep)
            .field("MaxSleep", &self.MaxSleep)
            .field("MinVideoTimeout", &self.MinVideoTimeout)
            .field("MaxVideoTimeout", &self.MaxVideoTimeout)
            .field("MinSpindownTimeout", &self.MinSpindownTimeout)
            .field("MaxSpindownTimeout", &self.MaxSpindownTimeout)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ADMINISTRATOR_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.MinSleep == other.MinSleep
            && self.MaxSleep == other.MaxSleep
            && self.MinVideoTimeout == other.MinVideoTimeout
            && self.MaxVideoTimeout == other.MaxVideoTimeout
            && self.MinSpindownTimeout == other.MinSpindownTimeout
            && self.MaxSpindownTimeout == other.MaxSpindownTimeout
    }
}
impl ::std::cmp::Eq for ADMINISTRATOR_POWER_POLICY {}
unsafe impl ::windows::runtime::Abi for ADMINISTRATOR_POWER_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BATTERY_CAPACITY_RELATIVE: u32 = 1073741824u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_CHARGER_STATUS {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub VaData: [u32; 1],
}
impl BATTERY_CHARGER_STATUS {}
impl ::std::default::Default for BATTERY_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_CHARGER_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_CHARGER_STATUS")
            .field("Type", &self.Type)
            .field("VaData", &self.VaData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_CHARGER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.VaData == other.VaData
    }
}
impl ::std::cmp::Eq for BATTERY_CHARGER_STATUS {}
unsafe impl ::windows::runtime::Abi for BATTERY_CHARGER_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BATTERY_CHARGING: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_CHARGING_SOURCE {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub MaxCurrent: u32,
}
impl BATTERY_CHARGING_SOURCE {}
impl ::std::default::Default for BATTERY_CHARGING_SOURCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_CHARGING_SOURCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_CHARGING_SOURCE")
            .field("Type", &self.Type)
            .field("MaxCurrent", &self.MaxCurrent)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_CHARGING_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.MaxCurrent == other.MaxCurrent
    }
}
impl ::std::cmp::Eq for BATTERY_CHARGING_SOURCE {}
unsafe impl ::windows::runtime::Abi for BATTERY_CHARGING_SOURCE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BATTERY_CHARGING_SOURCE_INFORMATION {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub SourceOnline: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl BATTERY_CHARGING_SOURCE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for BATTERY_CHARGING_SOURCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for BATTERY_CHARGING_SOURCE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_CHARGING_SOURCE_INFORMATION")
            .field("Type", &self.Type)
            .field("SourceOnline", &self.SourceOnline)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for BATTERY_CHARGING_SOURCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.SourceOnline == other.SourceOnline
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for BATTERY_CHARGING_SOURCE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for BATTERY_CHARGING_SOURCE_INFORMATION {
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
pub struct BATTERY_CHARGING_SOURCE_TYPE(pub i32);
pub const BatteryChargingSourceType_AC: BATTERY_CHARGING_SOURCE_TYPE =
    BATTERY_CHARGING_SOURCE_TYPE(1i32);
pub const BatteryChargingSourceType_USB: BATTERY_CHARGING_SOURCE_TYPE =
    BATTERY_CHARGING_SOURCE_TYPE(2i32);
pub const BatteryChargingSourceType_Wireless: BATTERY_CHARGING_SOURCE_TYPE =
    BATTERY_CHARGING_SOURCE_TYPE(3i32);
pub const BatteryChargingSourceType_Max: BATTERY_CHARGING_SOURCE_TYPE =
    BATTERY_CHARGING_SOURCE_TYPE(4i32);
impl ::std::convert::From<i32> for BATTERY_CHARGING_SOURCE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BATTERY_CHARGING_SOURCE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BATTERY_CLASS_MAJOR_VERSION: u32 = 1u32;
pub const BATTERY_CLASS_MINOR_VERSION: u32 = 0u32;
pub const BATTERY_CLASS_MINOR_VERSION_1: u32 = 1u32;
pub const BATTERY_CRITICAL: u32 = 8u32;
pub const BATTERY_CYCLE_COUNT_WMI_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4019772196,
        20,
        19493,
        [165, 11, 199, 36, 174, 92, 211, 113],
    );
pub const BATTERY_DISCHARGING: u32 = 2u32;
pub const BATTERY_FULL_CHARGED_CAPACITY_WMI_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1085539685,
        38647,
        17461,
        [134, 148, 151, 224, 228, 57, 89, 5],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_INFORMATION {
    pub Capabilities: u32,
    pub Technology: u8,
    pub Reserved: [u8; 3],
    pub Chemistry: [u8; 4],
    pub DesignedCapacity: u32,
    pub FullChargedCapacity: u32,
    pub DefaultAlert1: u32,
    pub DefaultAlert2: u32,
    pub CriticalBias: u32,
    pub CycleCount: u32,
}
impl BATTERY_INFORMATION {}
impl ::std::default::Default for BATTERY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_INFORMATION")
            .field("Capabilities", &self.Capabilities)
            .field("Technology", &self.Technology)
            .field("Reserved", &self.Reserved)
            .field("Chemistry", &self.Chemistry)
            .field("DesignedCapacity", &self.DesignedCapacity)
            .field("FullChargedCapacity", &self.FullChargedCapacity)
            .field("DefaultAlert1", &self.DefaultAlert1)
            .field("DefaultAlert2", &self.DefaultAlert2)
            .field("CriticalBias", &self.CriticalBias)
            .field("CycleCount", &self.CycleCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
            && self.Technology == other.Technology
            && self.Reserved == other.Reserved
            && self.Chemistry == other.Chemistry
            && self.DesignedCapacity == other.DesignedCapacity
            && self.FullChargedCapacity == other.FullChargedCapacity
            && self.DefaultAlert1 == other.DefaultAlert1
            && self.DefaultAlert2 == other.DefaultAlert2
            && self.CriticalBias == other.CriticalBias
            && self.CycleCount == other.CycleCount
    }
}
impl ::std::cmp::Eq for BATTERY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for BATTERY_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BATTERY_IS_SHORT_TERM: u32 = 536870912u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_MANUFACTURE_DATE {
    pub Day: u8,
    pub Month: u8,
    pub Year: u16,
}
impl BATTERY_MANUFACTURE_DATE {}
impl ::std::default::Default for BATTERY_MANUFACTURE_DATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_MANUFACTURE_DATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_MANUFACTURE_DATE")
            .field("Day", &self.Day)
            .field("Month", &self.Month)
            .field("Year", &self.Year)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_MANUFACTURE_DATE {
    fn eq(&self, other: &Self) -> bool {
        self.Day == other.Day && self.Month == other.Month && self.Year == other.Year
    }
}
impl ::std::cmp::Eq for BATTERY_MANUFACTURE_DATE {}
unsafe impl ::windows::runtime::Abi for BATTERY_MANUFACTURE_DATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BATTERY_MINIPORT_UPDATE_DATA_VER_1: u32 = 1u32;
pub const BATTERY_MINIPORT_UPDATE_DATA_VER_2: u32 = 2u32;
pub const BATTERY_POWER_ON_LINE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_QUERY_INFORMATION {
    pub BatteryTag: u32,
    pub InformationLevel: BATTERY_QUERY_INFORMATION_LEVEL,
    pub AtRate: u32,
}
impl BATTERY_QUERY_INFORMATION {}
impl ::std::default::Default for BATTERY_QUERY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_QUERY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_QUERY_INFORMATION")
            .field("BatteryTag", &self.BatteryTag)
            .field("InformationLevel", &self.InformationLevel)
            .field("AtRate", &self.AtRate)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_QUERY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryTag == other.BatteryTag
            && self.InformationLevel == other.InformationLevel
            && self.AtRate == other.AtRate
    }
}
impl ::std::cmp::Eq for BATTERY_QUERY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for BATTERY_QUERY_INFORMATION {
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
pub struct BATTERY_QUERY_INFORMATION_LEVEL(pub i32);
pub const BatteryInformation: BATTERY_QUERY_INFORMATION_LEVEL =
    BATTERY_QUERY_INFORMATION_LEVEL(0i32);
pub const BatteryGranularityInformation: BATTERY_QUERY_INFORMATION_LEVEL =
    BATTERY_QUERY_INFORMATION_LEVEL(1i32);
pub const BatteryTemperature: BATTERY_QUERY_INFORMATION_LEVEL =
    BATTERY_QUERY_INFORMATION_LEVEL(2i32);
pub const BatteryEstimatedTime: BATTERY_QUERY_INFORMATION_LEVEL =
    BATTERY_QUERY_INFORMATION_LEVEL(3i32);
pub const BatteryDeviceName: BATTERY_QUERY_INFORMATION_LEVEL =
    BATTERY_QUERY_INFORMATION_LEVEL(4i32);
pub const BatteryManufactureDate: BATTERY_QUERY_INFORMATION_LEVEL =
    BATTERY_QUERY_INFORMATION_LEVEL(5i32);
pub const BatteryManufactureName: BATTERY_QUERY_INFORMATION_LEVEL =
    BATTERY_QUERY_INFORMATION_LEVEL(6i32);
pub const BatteryUniqueID: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(7i32);
pub const BatterySerialNumber: BATTERY_QUERY_INFORMATION_LEVEL =
    BATTERY_QUERY_INFORMATION_LEVEL(8i32);
impl ::std::convert::From<i32> for BATTERY_QUERY_INFORMATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BATTERY_QUERY_INFORMATION_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_REPORTING_SCALE {
    pub Granularity: u32,
    pub Capacity: u32,
}
impl BATTERY_REPORTING_SCALE {}
impl ::std::default::Default for BATTERY_REPORTING_SCALE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_REPORTING_SCALE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_REPORTING_SCALE")
            .field("Granularity", &self.Granularity)
            .field("Capacity", &self.Capacity)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_REPORTING_SCALE {
    fn eq(&self, other: &Self) -> bool {
        self.Granularity == other.Granularity && self.Capacity == other.Capacity
    }
}
impl ::std::cmp::Eq for BATTERY_REPORTING_SCALE {}
unsafe impl ::windows::runtime::Abi for BATTERY_REPORTING_SCALE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BATTERY_RUNTIME_WMI_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1398421351,
        6850,
        18876,
        [160, 119, 63, 122, 2, 228, 10, 236],
    );
pub const BATTERY_SEALED: u32 = 268435456u32;
pub const BATTERY_SET_CHARGER_ID_SUPPORTED: u32 = 8u32;
pub const BATTERY_SET_CHARGE_SUPPORTED: u32 = 1u32;
pub const BATTERY_SET_CHARGINGSOURCE_SUPPORTED: u32 = 4u32;
pub const BATTERY_SET_DISCHARGE_SUPPORTED: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_SET_INFORMATION {
    pub BatteryTag: u32,
    pub InformationLevel: BATTERY_SET_INFORMATION_LEVEL,
    pub Buffer: [u8; 1],
}
impl BATTERY_SET_INFORMATION {}
impl ::std::default::Default for BATTERY_SET_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_SET_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_SET_INFORMATION")
            .field("BatteryTag", &self.BatteryTag)
            .field("InformationLevel", &self.InformationLevel)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_SET_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryTag == other.BatteryTag
            && self.InformationLevel == other.InformationLevel
            && self.Buffer == other.Buffer
    }
}
impl ::std::cmp::Eq for BATTERY_SET_INFORMATION {}
unsafe impl ::windows::runtime::Abi for BATTERY_SET_INFORMATION {
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
pub struct BATTERY_SET_INFORMATION_LEVEL(pub i32);
pub const BatteryCriticalBias: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(0i32);
pub const BatteryCharge: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(1i32);
pub const BatteryDischarge: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(2i32);
pub const BatteryChargingSource: BATTERY_SET_INFORMATION_LEVEL =
    BATTERY_SET_INFORMATION_LEVEL(3i32);
pub const BatteryChargerId: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(4i32);
pub const BatteryChargerStatus: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(5i32);
impl ::std::convert::From<i32> for BATTERY_SET_INFORMATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BATTERY_SET_INFORMATION_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BATTERY_STATIC_DATA_WMI_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        98690147,
        58594,
        20137,
        [128, 203, 155, 212, 179, 202, 6, 85],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_STATUS {
    pub PowerState: u32,
    pub Capacity: u32,
    pub Voltage: u32,
    pub Rate: i32,
}
impl BATTERY_STATUS {}
impl ::std::default::Default for BATTERY_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_STATUS")
            .field("PowerState", &self.PowerState)
            .field("Capacity", &self.Capacity)
            .field("Voltage", &self.Voltage)
            .field("Rate", &self.Rate)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.PowerState == other.PowerState
            && self.Capacity == other.Capacity
            && self.Voltage == other.Voltage
            && self.Rate == other.Rate
    }
}
impl ::std::cmp::Eq for BATTERY_STATUS {}
unsafe impl ::windows::runtime::Abi for BATTERY_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BATTERY_STATUS_CHANGE_WMI_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3453984963,
        31835,
        20035,
        [160, 52, 5, 159, 165, 184, 67, 100],
    );
pub const BATTERY_STATUS_WMI_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4232474833,
    60351,
    16750,
    [135, 206, 55, 74, 78, 188, 17, 26],
);
pub const BATTERY_SYSTEM_BATTERY: u32 = 2147483648u32;
pub const BATTERY_TAG_CHANGE_WMI_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1579118105,
        34694,
        19747,
        [148, 252, 158, 116, 107, 213, 216, 136],
    );
pub const BATTERY_TAG_INVALID: u32 = 0u32;
pub const BATTERY_TEMPERATURE_WMI_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        441622861,
        44494,
        19012,
        [154, 62, 200, 216, 241, 95, 242, 194],
    );
pub const BATTERY_UNKNOWN_CAPACITY: u32 = 4294967295u32;
pub const BATTERY_UNKNOWN_CURRENT: u32 = 4294967295u32;
pub const BATTERY_UNKNOWN_RATE: u32 = 2147483648u32;
pub const BATTERY_UNKNOWN_TIME: u32 = 4294967295u32;
pub const BATTERY_UNKNOWN_VOLTAGE: u32 = 4294967295u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_USB_CHARGER_STATUS {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub Reserved: u32,
    pub Flags: u32,
    pub MaxCurrent: u32,
    pub Voltage: u32,
    pub PortType: USB_CHARGER_PORT,
    pub PortId: u64,
    pub PowerSourceInformation: *mut ::std::ffi::c_void,
    pub OemCharger: ::windows::runtime::GUID,
}
impl BATTERY_USB_CHARGER_STATUS {}
impl ::std::default::Default for BATTERY_USB_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_USB_CHARGER_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_USB_CHARGER_STATUS")
            .field("Type", &self.Type)
            .field("Reserved", &self.Reserved)
            .field("Flags", &self.Flags)
            .field("MaxCurrent", &self.MaxCurrent)
            .field("Voltage", &self.Voltage)
            .field("PortType", &self.PortType)
            .field("PortId", &self.PortId)
            .field("PowerSourceInformation", &self.PowerSourceInformation)
            .field("OemCharger", &self.OemCharger)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_USB_CHARGER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.Reserved == other.Reserved
            && self.Flags == other.Flags
            && self.MaxCurrent == other.MaxCurrent
            && self.Voltage == other.Voltage
            && self.PortType == other.PortType
            && self.PortId == other.PortId
            && self.PowerSourceInformation == other.PowerSourceInformation
            && self.OemCharger == other.OemCharger
    }
}
impl ::std::cmp::Eq for BATTERY_USB_CHARGER_STATUS {}
unsafe impl ::windows::runtime::Abi for BATTERY_USB_CHARGER_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BATTERY_USB_CHARGER_STATUS_FN_DEFAULT_USB: u32 = 1u32;
pub const BATTERY_USB_CHARGER_STATUS_UCM_PD: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_WAIT_STATUS {
    pub BatteryTag: u32,
    pub Timeout: u32,
    pub PowerState: u32,
    pub LowCapacity: u32,
    pub HighCapacity: u32,
}
impl BATTERY_WAIT_STATUS {}
impl ::std::default::Default for BATTERY_WAIT_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BATTERY_WAIT_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BATTERY_WAIT_STATUS")
            .field("BatteryTag", &self.BatteryTag)
            .field("Timeout", &self.Timeout)
            .field("PowerState", &self.PowerState)
            .field("LowCapacity", &self.LowCapacity)
            .field("HighCapacity", &self.HighCapacity)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BATTERY_WAIT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryTag == other.BatteryTag
            && self.Timeout == other.Timeout
            && self.PowerState == other.PowerState
            && self.LowCapacity == other.LowCapacity
            && self.HighCapacity == other.HighCapacity
    }
}
impl ::std::cmp::Eq for BATTERY_WAIT_STATUS {}
unsafe impl ::windows::runtime::Abi for BATTERY_WAIT_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn CallNtPowerInformation(
    informationlevel: super::SystemServices::POWER_INFORMATION_LEVEL,
    inputbuffer: *const ::std::ffi::c_void,
    inputbufferlength: u32,
    outputbuffer: *mut ::std::ffi::c_void,
    outputbufferlength: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CallNtPowerInformation(
                informationlevel: super::SystemServices::POWER_INFORMATION_LEVEL,
                inputbuffer: *const ::std::ffi::c_void,
                inputbufferlength: u32,
                outputbuffer: *mut ::std::ffi::c_void,
                outputbufferlength: u32,
            ) -> i32;
        }
        ::std::mem::transmute(CallNtPowerInformation(
            ::std::mem::transmute(informationlevel),
            ::std::mem::transmute(inputbuffer),
            ::std::mem::transmute(inputbufferlength),
            ::std::mem::transmute(outputbuffer),
            ::std::mem::transmute(outputbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CanUserWritePwrScheme() -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CanUserWritePwrScheme() -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(CanUserWritePwrScheme())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DEVICEPOWER_AND_OPERATION: u32 = 1073741824u32;
pub const DEVICEPOWER_CLEAR_WAKEENABLED: u32 = 2u32;
pub const DEVICEPOWER_FILTER_DEVICES_PRESENT: u32 = 536870912u32;
pub const DEVICEPOWER_FILTER_HARDWARE: u32 = 268435456u32;
pub const DEVICEPOWER_FILTER_ON_NAME: u32 = 33554432u32;
pub const DEVICEPOWER_FILTER_WAKEENABLED: u32 = 134217728u32;
pub const DEVICEPOWER_FILTER_WAKEPROGRAMMABLE: u32 = 67108864u32;
pub const DEVICEPOWER_HARDWAREID: u32 = 2147483648u32;
pub const DEVICEPOWER_SET_WAKEENABLED: u32 = 1u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    pub Callback: ::std::option::Option<PDEVICE_NOTIFY_CALLBACK_ROUTINE>,
    pub Context: *mut ::std::ffi::c_void,
}
impl DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {}
impl ::std::default::Default for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS")
            .field("Context", &self.Context)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize)
            && self.Context == other.Context
    }
}
impl ::std::cmp::Eq for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePwrScheme(uiid: u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeletePwrScheme(uiid: u32) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(DeletePwrScheme(::std::mem::transmute(uiid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DevicePowerClose() -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevicePowerClose() -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(DevicePowerClose())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DevicePowerEnumDevices(
    queryindex: u32,
    queryinterpretationflags: u32,
    queryflags: u32,
    preturnbuffer: *mut u8,
    pbuffersize: *mut u32,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevicePowerEnumDevices(
                queryindex: u32,
                queryinterpretationflags: u32,
                queryflags: u32,
                preturnbuffer: *mut u8,
                pbuffersize: *mut u32,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(DevicePowerEnumDevices(
            ::std::mem::transmute(queryindex),
            ::std::mem::transmute(queryinterpretationflags),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(preturnbuffer),
            ::std::mem::transmute(pbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DevicePowerOpen(debugmask: u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevicePowerOpen(debugmask: u32) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(DevicePowerOpen(::std::mem::transmute(debugmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DevicePowerSetDeviceState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    devicedescription: Param0,
    setflags: u32,
    setdata: *const ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevicePowerSetDeviceState(
                devicedescription: super::super::Foundation::PWSTR,
                setflags: u32,
                setdata: *const ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(DevicePowerSetDeviceState(
            devicedescription.into_param().abi(),
            ::std::mem::transmute(setflags),
            ::std::mem::transmute(setdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub struct EFFECTIVE_POWER_MODE(pub i32);
pub const EffectivePowerModeBatterySaver: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(0i32);
pub const EffectivePowerModeBetterBattery: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(1i32);
pub const EffectivePowerModeBalanced: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(2i32);
pub const EffectivePowerModeHighPerformance: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(3i32);
pub const EffectivePowerModeMaxPerformance: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(4i32);
pub const EffectivePowerModeGameMode: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(5i32);
pub const EffectivePowerModeMixedReality: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(6i32);
impl ::std::convert::From<i32> for EFFECTIVE_POWER_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EFFECTIVE_POWER_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
pub type EFFECTIVE_POWER_MODE_CALLBACK =
    unsafe extern "system" fn(mode: EFFECTIVE_POWER_MODE, context: *const ::std::ffi::c_void);
pub const EFFECTIVE_POWER_MODE_V1: u32 = 1u32;
pub const EFFECTIVE_POWER_MODE_V2: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EMI_CHANNEL_MEASUREMENT_DATA {
    pub AbsoluteEnergy: u64,
    pub AbsoluteTime: u64,
}
impl EMI_CHANNEL_MEASUREMENT_DATA {}
impl ::std::default::Default for EMI_CHANNEL_MEASUREMENT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EMI_CHANNEL_MEASUREMENT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EMI_CHANNEL_MEASUREMENT_DATA")
            .field("AbsoluteEnergy", &self.AbsoluteEnergy)
            .field("AbsoluteTime", &self.AbsoluteTime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EMI_CHANNEL_MEASUREMENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.AbsoluteEnergy == other.AbsoluteEnergy && self.AbsoluteTime == other.AbsoluteTime
    }
}
impl ::std::cmp::Eq for EMI_CHANNEL_MEASUREMENT_DATA {}
unsafe impl ::windows::runtime::Abi for EMI_CHANNEL_MEASUREMENT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EMI_CHANNEL_V2 {
    pub MeasurementUnit: EMI_MEASUREMENT_UNIT,
    pub ChannelNameSize: u16,
    pub ChannelName: [u16; 1],
}
impl EMI_CHANNEL_V2 {}
impl ::std::default::Default for EMI_CHANNEL_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EMI_CHANNEL_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EMI_CHANNEL_V2")
            .field("MeasurementUnit", &self.MeasurementUnit)
            .field("ChannelNameSize", &self.ChannelNameSize)
            .field("ChannelName", &self.ChannelName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EMI_CHANNEL_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.MeasurementUnit == other.MeasurementUnit
            && self.ChannelNameSize == other.ChannelNameSize
            && self.ChannelName == other.ChannelName
    }
}
impl ::std::cmp::Eq for EMI_CHANNEL_V2 {}
unsafe impl ::windows::runtime::Abi for EMI_CHANNEL_V2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EMI_MEASUREMENT_DATA_V2 {
    pub ChannelData: [EMI_CHANNEL_MEASUREMENT_DATA; 1],
}
impl EMI_MEASUREMENT_DATA_V2 {}
impl ::std::default::Default for EMI_MEASUREMENT_DATA_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EMI_MEASUREMENT_DATA_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EMI_MEASUREMENT_DATA_V2")
            .field("ChannelData", &self.ChannelData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EMI_MEASUREMENT_DATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelData == other.ChannelData
    }
}
impl ::std::cmp::Eq for EMI_MEASUREMENT_DATA_V2 {}
unsafe impl ::windows::runtime::Abi for EMI_MEASUREMENT_DATA_V2 {
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
pub struct EMI_MEASUREMENT_UNIT(pub i32);
pub const EmiMeasurementUnitPicowattHours: EMI_MEASUREMENT_UNIT = EMI_MEASUREMENT_UNIT(0i32);
impl ::std::convert::From<i32> for EMI_MEASUREMENT_UNIT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EMI_MEASUREMENT_UNIT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EMI_METADATA_SIZE {
    pub MetadataSize: u32,
}
impl EMI_METADATA_SIZE {}
impl ::std::default::Default for EMI_METADATA_SIZE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EMI_METADATA_SIZE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EMI_METADATA_SIZE")
            .field("MetadataSize", &self.MetadataSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EMI_METADATA_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.MetadataSize == other.MetadataSize
    }
}
impl ::std::cmp::Eq for EMI_METADATA_SIZE {}
unsafe impl ::windows::runtime::Abi for EMI_METADATA_SIZE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EMI_METADATA_V1 {
    pub MeasurementUnit: EMI_MEASUREMENT_UNIT,
    pub HardwareOEM: [u16; 16],
    pub HardwareModel: [u16; 16],
    pub HardwareRevision: u16,
    pub MeteredHardwareNameSize: u16,
    pub MeteredHardwareName: [u16; 1],
}
impl EMI_METADATA_V1 {}
impl ::std::default::Default for EMI_METADATA_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EMI_METADATA_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EMI_METADATA_V1")
            .field("MeasurementUnit", &self.MeasurementUnit)
            .field("HardwareOEM", &self.HardwareOEM)
            .field("HardwareModel", &self.HardwareModel)
            .field("HardwareRevision", &self.HardwareRevision)
            .field("MeteredHardwareNameSize", &self.MeteredHardwareNameSize)
            .field("MeteredHardwareName", &self.MeteredHardwareName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EMI_METADATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.MeasurementUnit == other.MeasurementUnit
            && self.HardwareOEM == other.HardwareOEM
            && self.HardwareModel == other.HardwareModel
            && self.HardwareRevision == other.HardwareRevision
            && self.MeteredHardwareNameSize == other.MeteredHardwareNameSize
            && self.MeteredHardwareName == other.MeteredHardwareName
    }
}
impl ::std::cmp::Eq for EMI_METADATA_V1 {}
unsafe impl ::windows::runtime::Abi for EMI_METADATA_V1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EMI_METADATA_V2 {
    pub HardwareOEM: [u16; 16],
    pub HardwareModel: [u16; 16],
    pub HardwareRevision: u16,
    pub ChannelCount: u16,
    pub Channels: [EMI_CHANNEL_V2; 1],
}
impl EMI_METADATA_V2 {}
impl ::std::default::Default for EMI_METADATA_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EMI_METADATA_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EMI_METADATA_V2")
            .field("HardwareOEM", &self.HardwareOEM)
            .field("HardwareModel", &self.HardwareModel)
            .field("HardwareRevision", &self.HardwareRevision)
            .field("ChannelCount", &self.ChannelCount)
            .field("Channels", &self.Channels)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EMI_METADATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.HardwareOEM == other.HardwareOEM
            && self.HardwareModel == other.HardwareModel
            && self.HardwareRevision == other.HardwareRevision
            && self.ChannelCount == other.ChannelCount
            && self.Channels == other.Channels
    }
}
impl ::std::cmp::Eq for EMI_METADATA_V2 {}
unsafe impl ::windows::runtime::Abi for EMI_METADATA_V2 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EMI_NAME_MAX: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EMI_VERSION {
    pub EmiVersion: u16,
}
impl EMI_VERSION {}
impl ::std::default::Default for EMI_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EMI_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EMI_VERSION")
            .field("EmiVersion", &self.EmiVersion)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EMI_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.EmiVersion == other.EmiVersion
    }
}
impl ::std::cmp::Eq for EMI_VERSION {}
unsafe impl ::windows::runtime::Abi for EMI_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EMI_VERSION_V1: u32 = 1u32;
pub const EMI_VERSION_V2: u32 = 2u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EXECUTION_STATE(pub u32);
pub const ES_AWAYMODE_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(64u32);
pub const ES_CONTINUOUS: EXECUTION_STATE = EXECUTION_STATE(2147483648u32);
pub const ES_DISPLAY_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(2u32);
pub const ES_SYSTEM_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(1u32);
pub const ES_USER_PRESENT: EXECUTION_STATE = EXECUTION_STATE(4u32);
impl ::std::convert::From<u32> for EXECUTION_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EXECUTION_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for EXECUTION_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for EXECUTION_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for EXECUTION_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for EXECUTION_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for EXECUTION_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const EnableMultiBatteryDisplay: u32 = 2u32;
pub const EnablePasswordLogon: u32 = 4u32;
pub const EnableSysTrayBatteryMeter: u32 = 1u32;
pub const EnableVideoDimDisplay: u32 = 16u32;
pub const EnableWakeOnRing: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPwrSchemes<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    lpfn: ::std::option::Option<PWRSCHEMESENUMPROC>,
    lparam: Param1,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumPwrSchemes(
                lpfn: ::windows::runtime::RawPtr,
                lparam: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(EnumPwrSchemes(
            ::std::mem::transmute(lpfn),
            lparam.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GLOBAL_MACHINE_POWER_POLICY {
    pub Revision: u32,
    pub LidOpenWakeAc: SYSTEM_POWER_STATE,
    pub LidOpenWakeDc: SYSTEM_POWER_STATE,
    pub BroadcastCapacityResolution: u32,
}
impl GLOBAL_MACHINE_POWER_POLICY {}
impl ::std::default::Default for GLOBAL_MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GLOBAL_MACHINE_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GLOBAL_MACHINE_POWER_POLICY")
            .field("Revision", &self.Revision)
            .field("LidOpenWakeAc", &self.LidOpenWakeAc)
            .field("LidOpenWakeDc", &self.LidOpenWakeDc)
            .field(
                "BroadcastCapacityResolution",
                &self.BroadcastCapacityResolution,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for GLOBAL_MACHINE_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.LidOpenWakeAc == other.LidOpenWakeAc
            && self.LidOpenWakeDc == other.LidOpenWakeDc
            && self.BroadcastCapacityResolution == other.BroadcastCapacityResolution
    }
}
impl ::std::cmp::Eq for GLOBAL_MACHINE_POWER_POLICY {}
unsafe impl ::windows::runtime::Abi for GLOBAL_MACHINE_POWER_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GLOBAL_POWER_POLICY {
    pub user: GLOBAL_USER_POWER_POLICY,
    pub mach: GLOBAL_MACHINE_POWER_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl GLOBAL_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GLOBAL_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GLOBAL_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GLOBAL_POWER_POLICY")
            .field("user", &self.user)
            .field("mach", &self.mach)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GLOBAL_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user && self.mach == other.mach
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GLOBAL_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GLOBAL_POWER_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GLOBAL_USER_POWER_POLICY {
    pub Revision: u32,
    pub PowerButtonAc: POWER_ACTION_POLICY,
    pub PowerButtonDc: POWER_ACTION_POLICY,
    pub SleepButtonAc: POWER_ACTION_POLICY,
    pub SleepButtonDc: POWER_ACTION_POLICY,
    pub LidCloseAc: POWER_ACTION_POLICY,
    pub LidCloseDc: POWER_ACTION_POLICY,
    pub DischargePolicy: [SYSTEM_POWER_LEVEL; 4],
    pub GlobalFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl GLOBAL_USER_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GLOBAL_USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GLOBAL_USER_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GLOBAL_USER_POWER_POLICY")
            .field("Revision", &self.Revision)
            .field("PowerButtonAc", &self.PowerButtonAc)
            .field("PowerButtonDc", &self.PowerButtonDc)
            .field("SleepButtonAc", &self.SleepButtonAc)
            .field("SleepButtonDc", &self.SleepButtonDc)
            .field("LidCloseAc", &self.LidCloseAc)
            .field("LidCloseDc", &self.LidCloseDc)
            .field("DischargePolicy", &self.DischargePolicy)
            .field("GlobalFlags", &self.GlobalFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GLOBAL_USER_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.PowerButtonAc == other.PowerButtonAc
            && self.PowerButtonDc == other.PowerButtonDc
            && self.SleepButtonAc == other.SleepButtonAc
            && self.SleepButtonDc == other.SleepButtonDc
            && self.LidCloseAc == other.LidCloseAc
            && self.LidCloseDc == other.LidCloseDc
            && self.DischargePolicy == other.DischargePolicy
            && self.GlobalFlags == other.GlobalFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GLOBAL_USER_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GLOBAL_USER_POWER_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const GUID_CLASS_INPUT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(1293833650, 61807, 4559, [136, 203, 0, 17, 17, 0, 0, 48]);
pub const GUID_DEVICE_ACPI_TIME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2549718006,
    17559,
    20248,
    [187, 34, 75, 159, 178, 251, 239, 156],
);
pub const GUID_DEVICE_APPLICATIONLAUNCH_BUTTON: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1654085870,
        39022,
        19870,
        [142, 71, 222, 39, 248, 171, 5, 77],
    );
pub const GUID_DEVICE_BATTERY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1919098452,
    30884,
    4560,
    [188, 247, 0, 170, 0, 183, 179, 42],
);
pub const GUID_DEVICE_ENERGY_METER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1170047812,
        32470,
        18895,
        [164, 64, 194, 118, 201, 51, 176, 83],
    );
pub const GUID_DEVICE_FAN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    99406141,
    33242,
    18986,
    [138, 76, 82, 79, 35, 221, 77, 201],
);
pub const GUID_DEVICE_LID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1257913682,
    29863,
    4560,
    [190, 94, 0, 160, 201, 6, 40, 87],
);
pub const GUID_DEVICE_MEMORY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1070657597,
    37600,
    17915,
    [183, 92, 94, 216, 255, 176, 16, 33],
);
pub const GUID_DEVICE_MESSAGE_INDICATOR: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3444089701,
        64148,
        19682,
        [162, 50, 161, 183, 100, 229, 216, 180],
    );
pub const GUID_DEVICE_PROCESSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2549799696,
    20019,
    16558,
    [53, 156, 139, 239, 2, 157, 189, 208],
);
pub const GUID_DEVICE_SYS_BUTTON: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1257913683,
    29863,
    4560,
    [190, 94, 0, 160, 201, 6, 40, 87],
);
pub const GUID_DEVICE_THERMAL_ZONE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1257913681,
        29863,
        4560,
        [190, 94, 0, 160, 201, 6, 40, 87],
    );
pub const GUID_DEVINTERFACE_THERMAL_COOLING: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3689166653,
        15489,
        16587,
        [172, 228, 224, 229, 208, 95, 12, 159],
    );
pub const GUID_DEVINTERFACE_THERMAL_MANAGER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2457780371,
        27044,
        19392,
        [189, 2, 113, 22, 100, 113, 68, 99],
    );
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetActivePwrScheme(puiid: *mut u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetActivePwrScheme(puiid: *mut u32) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(GetActivePwrScheme(::std::mem::transmute(puiid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPowerPolicies(
    pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY,
    ppowerpolicy: *mut POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPowerPolicies(
                pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY,
                ppowerpolicy: *mut POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(GetCurrentPowerPolicies(
            ::std::mem::transmute(pglobalpowerpolicy),
            ::std::mem::transmute(ppowerpolicy),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDevicePowerState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hdevice: Param0,
    pfon: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDevicePowerState(
                hdevice: super::super::Foundation::HANDLE,
                pfon: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDevicePowerState(
            hdevice.into_param().abi(),
            ::std::mem::transmute(pfon),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPwrCapabilities(
    lpspc: *mut SYSTEM_POWER_CAPABILITIES,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPwrCapabilities(
                lpspc: *mut SYSTEM_POWER_CAPABILITIES,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(GetPwrCapabilities(::std::mem::transmute(lpspc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPwrDiskSpindownRange(
    puimax: *mut u32,
    puimin: *mut u32,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPwrDiskSpindownRange(
                puimax: *mut u32,
                puimin: *mut u32,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(GetPwrDiskSpindownRange(
            ::std::mem::transmute(puimax),
            ::std::mem::transmute(puimin),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemPowerStatus(
    lpsystempowerstatus: *mut SYSTEM_POWER_STATUS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemPowerStatus(
                lpsystempowerstatus: *mut SYSTEM_POWER_STATUS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSystemPowerStatus(::std::mem::transmute(
            lpsystempowerstatus,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HPOWERNOTIFY(pub isize);
impl ::std::default::Default for HPOWERNOTIFY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HPOWERNOTIFY {}
unsafe impl ::windows::runtime::Abi for HPOWERNOTIFY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IOCTL_ACPI_GET_REAL_TIME: u32 = 2703888u32;
pub const IOCTL_ACPI_SET_REAL_TIME: u32 = 2720276u32;
pub const IOCTL_BATTERY_CHARGING_SOURCE_CHANGE: u32 = 2703440u32;
pub const IOCTL_BATTERY_QUERY_INFORMATION: u32 = 2703428u32;
pub const IOCTL_BATTERY_QUERY_STATUS: u32 = 2703436u32;
pub const IOCTL_BATTERY_QUERY_TAG: u32 = 2703424u32;
pub const IOCTL_BATTERY_SET_INFORMATION: u32 = 2719816u32;
pub const IOCTL_EMI_GET_MEASUREMENT: u32 = 2244620u32;
pub const IOCTL_EMI_GET_METADATA: u32 = 2244616u32;
pub const IOCTL_EMI_GET_METADATA_SIZE: u32 = 2244612u32;
pub const IOCTL_EMI_GET_VERSION: u32 = 2244608u32;
pub const IOCTL_GET_PROCESSOR_OBJ_INFO: u32 = 2703744u32;
pub const IOCTL_GET_SYS_BUTTON_CAPS: u32 = 2703680u32;
pub const IOCTL_GET_SYS_BUTTON_EVENT: u32 = 2703684u32;
pub const IOCTL_GET_WAKE_ALARM_POLICY: u32 = 2736652u32;
pub const IOCTL_GET_WAKE_ALARM_SYSTEM_POWERSTATE: u32 = 2703896u32;
pub const IOCTL_GET_WAKE_ALARM_VALUE: u32 = 2736648u32;
pub const IOCTL_NOTIFY_SWITCH_EVENT: u32 = 2703616u32;
pub const IOCTL_QUERY_LID: u32 = 2703552u32;
pub const IOCTL_RUN_ACTIVE_COOLING_METHOD: u32 = 2719880u32;
pub const IOCTL_SET_SYS_MESSAGE_INDICATOR: u32 = 2720192u32;
pub const IOCTL_SET_WAKE_ALARM_POLICY: u32 = 2720260u32;
pub const IOCTL_SET_WAKE_ALARM_VALUE: u32 = 2720256u32;
pub const IOCTL_THERMAL_QUERY_INFORMATION: u32 = 2703488u32;
pub const IOCTL_THERMAL_READ_POLICY: u32 = 2703508u32;
pub const IOCTL_THERMAL_READ_TEMPERATURE: u32 = 2703504u32;
pub const IOCTL_THERMAL_SET_COOLING_POLICY: u32 = 2719876u32;
pub const IOCTL_THERMAL_SET_PASSIVE_LIMIT: u32 = 2719884u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsAdminOverrideActive(
    papp: *const ADMINISTRATOR_POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsAdminOverrideActive(
                papp: *const ADMINISTRATOR_POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(IsAdminOverrideActive(::std::mem::transmute(papp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsPwrHibernateAllowed() -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsPwrHibernateAllowed() -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(IsPwrHibernateAllowed())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsPwrShutdownAllowed() -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsPwrShutdownAllowed() -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(IsPwrShutdownAllowed())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsPwrSuspendAllowed() -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsPwrSuspendAllowed() -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(IsPwrSuspendAllowed())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsSystemResumeAutomatic() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsSystemResumeAutomatic() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsSystemResumeAutomatic())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub struct LATENCY_TIME(pub i32);
pub const LT_DONT_CARE: LATENCY_TIME = LATENCY_TIME(0i32);
pub const LT_LOWEST_LATENCY: LATENCY_TIME = LATENCY_TIME(1i32);
impl ::std::convert::From<i32> for LATENCY_TIME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LATENCY_TIME {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MACHINE_POWER_POLICY {
    pub Revision: u32,
    pub MinSleepAc: SYSTEM_POWER_STATE,
    pub MinSleepDc: SYSTEM_POWER_STATE,
    pub ReducedLatencySleepAc: SYSTEM_POWER_STATE,
    pub ReducedLatencySleepDc: SYSTEM_POWER_STATE,
    pub DozeTimeoutAc: u32,
    pub DozeTimeoutDc: u32,
    pub DozeS4TimeoutAc: u32,
    pub DozeS4TimeoutDc: u32,
    pub MinThrottleAc: u8,
    pub MinThrottleDc: u8,
    pub pad1: [u8; 2],
    pub OverThrottledAc: POWER_ACTION_POLICY,
    pub OverThrottledDc: POWER_ACTION_POLICY,
}
impl MACHINE_POWER_POLICY {}
impl ::std::default::Default for MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MACHINE_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MACHINE_POWER_POLICY")
            .field("Revision", &self.Revision)
            .field("MinSleepAc", &self.MinSleepAc)
            .field("MinSleepDc", &self.MinSleepDc)
            .field("ReducedLatencySleepAc", &self.ReducedLatencySleepAc)
            .field("ReducedLatencySleepDc", &self.ReducedLatencySleepDc)
            .field("DozeTimeoutAc", &self.DozeTimeoutAc)
            .field("DozeTimeoutDc", &self.DozeTimeoutDc)
            .field("DozeS4TimeoutAc", &self.DozeS4TimeoutAc)
            .field("DozeS4TimeoutDc", &self.DozeS4TimeoutDc)
            .field("MinThrottleAc", &self.MinThrottleAc)
            .field("MinThrottleDc", &self.MinThrottleDc)
            .field("pad1", &self.pad1)
            .field("OverThrottledAc", &self.OverThrottledAc)
            .field("OverThrottledDc", &self.OverThrottledDc)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MACHINE_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.MinSleepAc == other.MinSleepAc
            && self.MinSleepDc == other.MinSleepDc
            && self.ReducedLatencySleepAc == other.ReducedLatencySleepAc
            && self.ReducedLatencySleepDc == other.ReducedLatencySleepDc
            && self.DozeTimeoutAc == other.DozeTimeoutAc
            && self.DozeTimeoutDc == other.DozeTimeoutDc
            && self.DozeS4TimeoutAc == other.DozeS4TimeoutAc
            && self.DozeS4TimeoutDc == other.DozeS4TimeoutDc
            && self.MinThrottleAc == other.MinThrottleAc
            && self.MinThrottleDc == other.MinThrottleDc
            && self.pad1 == other.pad1
            && self.OverThrottledAc == other.OverThrottledAc
            && self.OverThrottledDc == other.OverThrottledDc
    }
}
impl ::std::cmp::Eq for MACHINE_POWER_POLICY {}
unsafe impl ::windows::runtime::Abi for MACHINE_POWER_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MACHINE_PROCESSOR_POWER_POLICY {
    pub Revision: u32,
    pub ProcessorPolicyAc: PROCESSOR_POWER_POLICY,
    pub ProcessorPolicyDc: PROCESSOR_POWER_POLICY,
}
impl MACHINE_PROCESSOR_POWER_POLICY {}
impl ::std::default::Default for MACHINE_PROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MACHINE_PROCESSOR_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MACHINE_PROCESSOR_POWER_POLICY")
            .field("Revision", &self.Revision)
            .field("ProcessorPolicyAc", &self.ProcessorPolicyAc)
            .field("ProcessorPolicyDc", &self.ProcessorPolicyDc)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MACHINE_PROCESSOR_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.ProcessorPolicyAc == other.ProcessorPolicyAc
            && self.ProcessorPolicyDc == other.ProcessorPolicyDc
    }
}
impl ::std::cmp::Eq for MACHINE_PROCESSOR_POWER_POLICY {}
unsafe impl ::windows::runtime::Abi for MACHINE_PROCESSOR_POWER_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MAX_ACTIVE_COOLING_LEVELS: u32 = 10u32;
pub const MAX_BATTERY_STRING_SIZE: u32 = 128u32;
pub const PASSIVE_COOLING: u32 = 1u32;
pub const PDCAP_S0_SUPPORTED: u32 = 65536u32;
pub const PDCAP_S1_SUPPORTED: u32 = 131072u32;
pub const PDCAP_S2_SUPPORTED: u32 = 262144u32;
pub const PDCAP_S3_SUPPORTED: u32 = 524288u32;
pub const PDCAP_S4_SUPPORTED: u32 = 16777216u32;
pub const PDCAP_S5_SUPPORTED: u32 = 33554432u32;
pub const PDCAP_WAKE_FROM_S0_SUPPORTED: u32 = 1048576u32;
pub const PDCAP_WAKE_FROM_S1_SUPPORTED: u32 = 2097152u32;
pub const PDCAP_WAKE_FROM_S2_SUPPORTED: u32 = 4194304u32;
pub const PDCAP_WAKE_FROM_S3_SUPPORTED: u32 = 8388608u32;
pub type PDEVICE_NOTIFY_CALLBACK_ROUTINE = unsafe extern "system" fn(
    context: *const ::std::ffi::c_void,
    r#type: u32,
    setting: *const ::std::ffi::c_void,
) -> u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct POWERBROADCAST_SETTING {
    pub PowerSetting: ::windows::runtime::GUID,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl POWERBROADCAST_SETTING {}
impl ::std::default::Default for POWERBROADCAST_SETTING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for POWERBROADCAST_SETTING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POWERBROADCAST_SETTING")
            .field("PowerSetting", &self.PowerSetting)
            .field("DataLength", &self.DataLength)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::std::cmp::PartialEq for POWERBROADCAST_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.PowerSetting == other.PowerSetting
            && self.DataLength == other.DataLength
            && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for POWERBROADCAST_SETTING {}
unsafe impl ::windows::runtime::Abi for POWERBROADCAST_SETTING {
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
pub struct POWER_ACTION(pub i32);
pub const PowerActionNone: POWER_ACTION = POWER_ACTION(0i32);
pub const PowerActionReserved: POWER_ACTION = POWER_ACTION(1i32);
pub const PowerActionSleep: POWER_ACTION = POWER_ACTION(2i32);
pub const PowerActionHibernate: POWER_ACTION = POWER_ACTION(3i32);
pub const PowerActionShutdown: POWER_ACTION = POWER_ACTION(4i32);
pub const PowerActionShutdownReset: POWER_ACTION = POWER_ACTION(5i32);
pub const PowerActionShutdownOff: POWER_ACTION = POWER_ACTION(6i32);
pub const PowerActionWarmEject: POWER_ACTION = POWER_ACTION(7i32);
pub const PowerActionDisplayOff: POWER_ACTION = POWER_ACTION(8i32);
impl ::std::convert::From<i32> for POWER_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POWER_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct POWER_ACTION_POLICY {
    pub Action: POWER_ACTION,
    pub Flags: u32,
    pub EventCode: POWER_ACTION_POLICY_EVENT_CODE,
}
impl POWER_ACTION_POLICY {}
impl ::std::default::Default for POWER_ACTION_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for POWER_ACTION_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POWER_ACTION_POLICY")
            .field("Action", &self.Action)
            .field("Flags", &self.Flags)
            .field("EventCode", &self.EventCode)
            .finish()
    }
}
impl ::std::cmp::PartialEq for POWER_ACTION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Action == other.Action
            && self.Flags == other.Flags
            && self.EventCode == other.EventCode
    }
}
impl ::std::cmp::Eq for POWER_ACTION_POLICY {}
unsafe impl ::windows::runtime::Abi for POWER_ACTION_POLICY {
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
pub struct POWER_ACTION_POLICY_EVENT_CODE(pub u32);
pub const POWER_FORCE_TRIGGER_RESET: POWER_ACTION_POLICY_EVENT_CODE =
    POWER_ACTION_POLICY_EVENT_CODE(2147483648u32);
pub const POWER_LEVEL_USER_NOTIFY_EXEC: POWER_ACTION_POLICY_EVENT_CODE =
    POWER_ACTION_POLICY_EVENT_CODE(4u32);
pub const POWER_LEVEL_USER_NOTIFY_SOUND: POWER_ACTION_POLICY_EVENT_CODE =
    POWER_ACTION_POLICY_EVENT_CODE(2u32);
pub const POWER_LEVEL_USER_NOTIFY_TEXT: POWER_ACTION_POLICY_EVENT_CODE =
    POWER_ACTION_POLICY_EVENT_CODE(1u32);
pub const POWER_USER_NOTIFY_BUTTON: POWER_ACTION_POLICY_EVENT_CODE =
    POWER_ACTION_POLICY_EVENT_CODE(8u32);
pub const POWER_USER_NOTIFY_SHUTDOWN: POWER_ACTION_POLICY_EVENT_CODE =
    POWER_ACTION_POLICY_EVENT_CODE(16u32);
impl ::std::convert::From<u32> for POWER_ACTION_POLICY_EVENT_CODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POWER_ACTION_POLICY_EVENT_CODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for POWER_ACTION_POLICY_EVENT_CODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for POWER_ACTION_POLICY_EVENT_CODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const POWER_ATTRIBUTE_HIDE: u32 = 1u32;
pub const POWER_ATTRIBUTE_SHOW_AOAC: u32 = 2u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct POWER_DATA_ACCESSOR(pub i32);
pub const ACCESS_AC_POWER_SETTING_INDEX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(0i32);
pub const ACCESS_DC_POWER_SETTING_INDEX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(1i32);
pub const ACCESS_FRIENDLY_NAME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(2i32);
pub const ACCESS_DESCRIPTION: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(3i32);
pub const ACCESS_POSSIBLE_POWER_SETTING: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(4i32);
pub const ACCESS_POSSIBLE_POWER_SETTING_FRIENDLY_NAME: POWER_DATA_ACCESSOR =
    POWER_DATA_ACCESSOR(5i32);
pub const ACCESS_POSSIBLE_POWER_SETTING_DESCRIPTION: POWER_DATA_ACCESSOR =
    POWER_DATA_ACCESSOR(6i32);
pub const ACCESS_DEFAULT_AC_POWER_SETTING: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(7i32);
pub const ACCESS_DEFAULT_DC_POWER_SETTING: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(8i32);
pub const ACCESS_POSSIBLE_VALUE_MIN: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(9i32);
pub const ACCESS_POSSIBLE_VALUE_MAX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(10i32);
pub const ACCESS_POSSIBLE_VALUE_INCREMENT: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(11i32);
pub const ACCESS_POSSIBLE_VALUE_UNITS: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(12i32);
pub const ACCESS_ICON_RESOURCE: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(13i32);
pub const ACCESS_DEFAULT_SECURITY_DESCRIPTOR: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(14i32);
pub const ACCESS_ATTRIBUTES: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(15i32);
pub const ACCESS_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(16i32);
pub const ACCESS_SUBGROUP: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(17i32);
pub const ACCESS_INDIVIDUAL_SETTING: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(18i32);
pub const ACCESS_ACTIVE_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(19i32);
pub const ACCESS_CREATE_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(20i32);
pub const ACCESS_AC_POWER_SETTING_MAX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(21i32);
pub const ACCESS_DC_POWER_SETTING_MAX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(22i32);
pub const ACCESS_AC_POWER_SETTING_MIN: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(23i32);
pub const ACCESS_DC_POWER_SETTING_MIN: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(24i32);
pub const ACCESS_PROFILE: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(25i32);
pub const ACCESS_OVERLAY_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(26i32);
pub const ACCESS_ACTIVE_OVERLAY_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(27i32);
impl ::std::convert::From<i32> for POWER_DATA_ACCESSOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POWER_DATA_ACCESSOR {
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
pub struct POWER_PLATFORM_ROLE(pub i32);
pub const PlatformRoleUnspecified: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(0i32);
pub const PlatformRoleDesktop: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(1i32);
pub const PlatformRoleMobile: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(2i32);
pub const PlatformRoleWorkstation: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(3i32);
pub const PlatformRoleEnterpriseServer: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(4i32);
pub const PlatformRoleSOHOServer: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(5i32);
pub const PlatformRoleAppliancePC: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(6i32);
pub const PlatformRolePerformanceServer: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(7i32);
pub const PlatformRoleSlate: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(8i32);
pub const PlatformRoleMaximum: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(9i32);
impl ::std::convert::From<i32> for POWER_PLATFORM_ROLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POWER_PLATFORM_ROLE {
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
pub struct POWER_PLATFORM_ROLE_VERSION(pub u32);
pub const POWER_PLATFORM_ROLE_V1: POWER_PLATFORM_ROLE_VERSION = POWER_PLATFORM_ROLE_VERSION(1u32);
pub const POWER_PLATFORM_ROLE_V2: POWER_PLATFORM_ROLE_VERSION = POWER_PLATFORM_ROLE_VERSION(2u32);
impl ::std::convert::From<u32> for POWER_PLATFORM_ROLE_VERSION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POWER_PLATFORM_ROLE_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for POWER_PLATFORM_ROLE_VERSION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for POWER_PLATFORM_ROLE_VERSION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for POWER_PLATFORM_ROLE_VERSION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for POWER_PLATFORM_ROLE_VERSION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for POWER_PLATFORM_ROLE_VERSION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct POWER_POLICY {
    pub user: USER_POWER_POLICY,
    pub mach: MACHINE_POWER_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POWER_POLICY")
            .field("user", &self.user)
            .field("mach", &self.mach)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user && self.mach == other.mach
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for POWER_POLICY {
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
pub struct POWER_SETTING_REGISTER_NOTIFICATION_FLAGS(pub u32);
pub const DEVICE_NOTIFY_SERVICE_HANDLE: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS =
    POWER_SETTING_REGISTER_NOTIFICATION_FLAGS(1u32);
pub const DEVICE_NOTIFY_CALLBACK: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS =
    POWER_SETTING_REGISTER_NOTIFICATION_FLAGS(2u32);
pub const DEVICE_NOTIFY_WINDOW_HANDLE: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS =
    POWER_SETTING_REGISTER_NOTIFICATION_FLAGS(0u32);
impl ::std::convert::From<u32> for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
pub const PROCESSOR_NUMBER_PKEY: super::PropertiesSystem::PROPERTYKEY =
    super::PropertiesSystem::PROPERTYKEY {
        fmtid: ::windows::runtime::GUID::from_values(
            1462028317,
            54703,
            19487,
            [161, 3, 160, 110, 40, 242, 4, 198],
        ),
        pid: 1u32,
    };
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESSOR_OBJECT_INFO {
    pub PhysicalID: u32,
    pub PBlkAddress: u32,
    pub PBlkLength: u8,
}
impl PROCESSOR_OBJECT_INFO {}
impl ::std::default::Default for PROCESSOR_OBJECT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESSOR_OBJECT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESSOR_OBJECT_INFO")
            .field("PhysicalID", &self.PhysicalID)
            .field("PBlkAddress", &self.PBlkAddress)
            .field("PBlkLength", &self.PBlkLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESSOR_OBJECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalID == other.PhysicalID
            && self.PBlkAddress == other.PBlkAddress
            && self.PBlkLength == other.PBlkLength
    }
}
impl ::std::cmp::Eq for PROCESSOR_OBJECT_INFO {}
unsafe impl ::windows::runtime::Abi for PROCESSOR_OBJECT_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESSOR_OBJECT_INFO_EX {
    pub PhysicalID: u32,
    pub PBlkAddress: u32,
    pub PBlkLength: u8,
    pub InitialApicId: u32,
}
impl PROCESSOR_OBJECT_INFO_EX {}
impl ::std::default::Default for PROCESSOR_OBJECT_INFO_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESSOR_OBJECT_INFO_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESSOR_OBJECT_INFO_EX")
            .field("PhysicalID", &self.PhysicalID)
            .field("PBlkAddress", &self.PBlkAddress)
            .field("PBlkLength", &self.PBlkLength)
            .field("InitialApicId", &self.InitialApicId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESSOR_OBJECT_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalID == other.PhysicalID
            && self.PBlkAddress == other.PBlkAddress
            && self.PBlkLength == other.PBlkLength
            && self.InitialApicId == other.InitialApicId
    }
}
impl ::std::cmp::Eq for PROCESSOR_OBJECT_INFO_EX {}
unsafe impl ::windows::runtime::Abi for PROCESSOR_OBJECT_INFO_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESSOR_POWER_POLICY {
    pub Revision: u32,
    pub DynamicThrottle: u8,
    pub Spare: [u8; 3],
    pub _bitfield: u32,
    pub PolicyCount: u32,
    pub Policy: [PROCESSOR_POWER_POLICY_INFO; 3],
}
impl PROCESSOR_POWER_POLICY {}
impl ::std::default::Default for PROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESSOR_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESSOR_POWER_POLICY")
            .field("Revision", &self.Revision)
            .field("DynamicThrottle", &self.DynamicThrottle)
            .field("Spare", &self.Spare)
            .field("_bitfield", &self._bitfield)
            .field("PolicyCount", &self.PolicyCount)
            .field("Policy", &self.Policy)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESSOR_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.DynamicThrottle == other.DynamicThrottle
            && self.Spare == other.Spare
            && self._bitfield == other._bitfield
            && self.PolicyCount == other.PolicyCount
            && self.Policy == other.Policy
    }
}
impl ::std::cmp::Eq for PROCESSOR_POWER_POLICY {}
unsafe impl ::windows::runtime::Abi for PROCESSOR_POWER_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESSOR_POWER_POLICY_INFO {
    pub TimeCheck: u32,
    pub DemoteLimit: u32,
    pub PromoteLimit: u32,
    pub DemotePercent: u8,
    pub PromotePercent: u8,
    pub Spare: [u8; 2],
    pub _bitfield: u32,
}
impl PROCESSOR_POWER_POLICY_INFO {}
impl ::std::default::Default for PROCESSOR_POWER_POLICY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESSOR_POWER_POLICY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESSOR_POWER_POLICY_INFO")
            .field("TimeCheck", &self.TimeCheck)
            .field("DemoteLimit", &self.DemoteLimit)
            .field("PromoteLimit", &self.PromoteLimit)
            .field("DemotePercent", &self.DemotePercent)
            .field("PromotePercent", &self.PromotePercent)
            .field("Spare", &self.Spare)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESSOR_POWER_POLICY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TimeCheck == other.TimeCheck
            && self.DemoteLimit == other.DemoteLimit
            && self.PromoteLimit == other.PromoteLimit
            && self.DemotePercent == other.DemotePercent
            && self.PromotePercent == other.PromotePercent
            && self.Spare == other.Spare
            && self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for PROCESSOR_POWER_POLICY_INFO {}
unsafe impl ::windows::runtime::Abi for PROCESSOR_POWER_POLICY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PWRSCHEMESENUMPROC = unsafe extern "system" fn(
    index: u32,
    namesize: u32,
    name: super::super::Foundation::PWSTR,
    descriptionsize: u32,
    description: super::super::Foundation::PWSTR,
    policy: *const POWER_POLICY,
    context: super::super::Foundation::LPARAM,
) -> super::super::Foundation::BOOLEAN;
#[cfg(feature = "Win32_Foundation")]
pub type PWRSCHEMESENUMPROC_V1 = unsafe extern "system" fn(
    index: u32,
    namesize: u32,
    name: *const i8,
    descriptionsize: u32,
    description: *const i8,
    policy: *const POWER_POLICY,
    context: super::super::Foundation::LPARAM,
) -> super::super::Foundation::BOOLEAN;
#[inline]
pub unsafe fn PowerCanRestoreIndividualDefaultPowerScheme(
    schemeguid: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerCanRestoreIndividualDefaultPowerScheme(
                schemeguid: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerCanRestoreIndividualDefaultPowerScheme(
            ::std::mem::transmute(schemeguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn PowerClearRequest<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    powerrequest: Param0,
    requesttype: super::SystemServices::POWER_REQUEST_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerClearRequest(
                powerrequest: super::super::Foundation::HANDLE,
                requesttype: super::SystemServices::POWER_REQUEST_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PowerClearRequest(
            powerrequest.into_param().abi(),
            ::std::mem::transmute(requesttype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerCreatePossibleSetting<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootsystempowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    possiblesettingindex: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerCreatePossibleSetting(
                rootsystempowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                possiblesettingindex: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerCreatePossibleSetting(
            rootsystempowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(possiblesettingindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn PowerCreateRequest(
    context: *const super::SystemServices::REASON_CONTEXT,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerCreateRequest(
                context: *const super::SystemServices::REASON_CONTEXT,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(PowerCreateRequest(::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerCreateSetting<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootsystempowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerCreateSetting(
                rootsystempowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerCreateSetting(
            rootsystempowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerDeleteScheme<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerDeleteScheme(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerDeleteScheme(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerDeterminePlatformRole() -> POWER_PLATFORM_ROLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerDeterminePlatformRole() -> POWER_PLATFORM_ROLE;
        }
        ::std::mem::transmute(PowerDeterminePlatformRole())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerDeterminePlatformRoleEx(
    version: POWER_PLATFORM_ROLE_VERSION,
) -> POWER_PLATFORM_ROLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerDeterminePlatformRoleEx(
                version: POWER_PLATFORM_ROLE_VERSION,
            ) -> POWER_PLATFORM_ROLE;
        }
        ::std::mem::transmute(PowerDeterminePlatformRoleEx(::std::mem::transmute(version)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerDuplicateScheme<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    sourceschemeguid: *const ::windows::runtime::GUID,
    destinationschemeguid: *mut *mut ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerDuplicateScheme(
                rootpowerkey: super::Registry::HKEY,
                sourceschemeguid: *const ::windows::runtime::GUID,
                destinationschemeguid: *mut *mut ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerDuplicateScheme(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(sourceschemeguid),
            ::std::mem::transmute(destinationschemeguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerEnumerate<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    accessflags: POWER_DATA_ACCESSOR,
    index: u32,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerEnumerate(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                accessflags: POWER_DATA_ACCESSOR,
                index: u32,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerEnumerate(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(accessflags),
            ::std::mem::transmute(index),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerGetActiveScheme<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    userrootpowerkey: Param0,
    activepolicyguid: *mut *mut ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerGetActiveScheme(
                userrootpowerkey: super::Registry::HKEY,
                activepolicyguid: *mut *mut ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerGetActiveScheme(
            userrootpowerkey.into_param().abi(),
            ::std::mem::transmute(activepolicyguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn PowerImportPowerScheme<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    rootpowerkey: Param0,
    importfilenamepath: Param1,
    destinationschemeguid: *mut *mut ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerImportPowerScheme(
                rootpowerkey: super::Registry::HKEY,
                importfilenamepath: super::super::Foundation::PWSTR,
                destinationschemeguid: *mut *mut ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerImportPowerScheme(
            rootpowerkey.into_param().abi(),
            importfilenamepath.into_param().abi(),
            ::std::mem::transmute(destinationschemeguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PowerIsSettingRangeDefined(
    subkeyguid: *const ::windows::runtime::GUID,
    settingguid: *const ::windows::runtime::GUID,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerIsSettingRangeDefined(
                subkeyguid: *const ::windows::runtime::GUID,
                settingguid: *const ::windows::runtime::GUID,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(PowerIsSettingRangeDefined(
            ::std::mem::transmute(subkeyguid),
            ::std::mem::transmute(settingguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn PowerOpenSystemPowerKey<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    phsystempowerkey: *mut super::Registry::HKEY,
    access: u32,
    openexisting: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerOpenSystemPowerKey(
                phsystempowerkey: *mut super::Registry::HKEY,
                access: u32,
                openexisting: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(PowerOpenSystemPowerKey(
            ::std::mem::transmute(phsystempowerkey),
            ::std::mem::transmute(access),
            openexisting.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn PowerOpenUserPowerKey<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    phuserpowerkey: *mut super::Registry::HKEY,
    access: u32,
    openexisting: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerOpenUserPowerKey(
                phuserpowerkey: *mut super::Registry::HKEY,
                access: u32,
                openexisting: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(PowerOpenUserPowerKey(
            ::std::mem::transmute(phuserpowerkey),
            ::std::mem::transmute(access),
            openexisting.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadACDefaultIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemepersonalityguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    acdefaultindex: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadACDefaultIndex(
                rootpowerkey: super::Registry::HKEY,
                schemepersonalityguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                acdefaultindex: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadACDefaultIndex(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemepersonalityguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(acdefaultindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadACValue<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    r#type: *mut u32,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadACValue(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                r#type: *mut u32,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadACValue(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadACValueIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    acvalueindex: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadACValueIndex(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                acvalueindex: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadACValueIndex(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(acvalueindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDCDefaultIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemepersonalityguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    dcdefaultindex: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadDCDefaultIndex(
                rootpowerkey: super::Registry::HKEY,
                schemepersonalityguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                dcdefaultindex: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadDCDefaultIndex(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemepersonalityguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(dcdefaultindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDCValue<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    r#type: *mut u32,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadDCValue(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                r#type: *mut u32,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadDCValue(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDCValueIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    dcvalueindex: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadDCValueIndex(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                dcvalueindex: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadDCValueIndex(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(dcvalueindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDescription<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadDescription(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadDescription(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadFriendlyName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadFriendlyName(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadFriendlyName(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadIconResourceSpecifier<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadIconResourceSpecifier(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadIconResourceSpecifier(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadPossibleDescription<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    possiblesettingindex: u32,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadPossibleDescription(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                possiblesettingindex: u32,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadPossibleDescription(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(possiblesettingindex),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadPossibleFriendlyName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    possiblesettingindex: u32,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadPossibleFriendlyName(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                possiblesettingindex: u32,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadPossibleFriendlyName(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(possiblesettingindex),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadPossibleValue<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    r#type: *mut u32,
    possiblesettingindex: u32,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadPossibleValue(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                r#type: *mut u32,
                possiblesettingindex: u32,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadPossibleValue(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(possiblesettingindex),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerReadSettingAttributes(
    subgroupguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadSettingAttributes(
                subgroupguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadSettingAttributes(
            ::std::mem::transmute(subgroupguid),
            ::std::mem::transmute(powersettingguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueIncrement<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    valueincrement: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadValueIncrement(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                valueincrement: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadValueIncrement(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(valueincrement),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueMax<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    valuemaximum: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadValueMax(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                valuemaximum: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadValueMax(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(valuemaximum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueMin<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    valueminimum: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadValueMin(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                valueminimum: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadValueMin(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(valueminimum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueUnitsSpecifier<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    buffer: *mut u8,
    buffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadValueUnitsSpecifier(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                buffer: *mut u8,
                buffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerReadValueUnitsSpecifier(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn PowerRegisterForEffectivePowerModeNotifications(
    version: u32,
    callback: ::std::option::Option<EFFECTIVE_POWER_MODE_CALLBACK>,
    context: *const ::std::ffi::c_void,
    registrationhandle: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerRegisterForEffectivePowerModeNotifications(
                version: u32,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                registrationhandle: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        PowerRegisterForEffectivePowerModeNotifications(
            ::std::mem::transmute(version),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(registrationhandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PowerRegisterSuspendResumeNotification<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    flags: u32,
    recipient: Param1,
    registrationhandle: *mut *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerRegisterSuspendResumeNotification(
                flags: u32,
                recipient: super::super::Foundation::HANDLE,
                registrationhandle: *mut *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(PowerRegisterSuspendResumeNotification(
            ::std::mem::transmute(flags),
            recipient.into_param().abi(),
            ::std::mem::transmute(registrationhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerRemovePowerSetting(
    powersettingsubkeyguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerRemovePowerSetting(
                powersettingsubkeyguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerRemovePowerSetting(
            ::std::mem::transmute(powersettingsubkeyguid),
            ::std::mem::transmute(powersettingguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerReplaceDefaultPowerSchemes() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReplaceDefaultPowerSchemes() -> u32;
        }
        ::std::mem::transmute(PowerReplaceDefaultPowerSchemes())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PowerReportThermalEvent(event: *const THERMAL_EVENT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReportThermalEvent(event: *const THERMAL_EVENT) -> u32;
        }
        ::std::mem::transmute(PowerReportThermalEvent(::std::mem::transmute(event)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerRestoreDefaultPowerSchemes() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerRestoreDefaultPowerSchemes() -> u32;
        }
        ::std::mem::transmute(PowerRestoreDefaultPowerSchemes())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerRestoreIndividualDefaultPowerScheme(
    schemeguid: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerRestoreIndividualDefaultPowerScheme(
                schemeguid: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerRestoreIndividualDefaultPowerScheme(
            ::std::mem::transmute(schemeguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerSetActiveScheme<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    userrootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSetActiveScheme(
                userrootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerSetActiveScheme(
            userrootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn PowerSetRequest<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    powerrequest: Param0,
    requesttype: super::SystemServices::POWER_REQUEST_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSetRequest(
                powerrequest: super::super::Foundation::HANDLE,
                requesttype: super::SystemServices::POWER_REQUEST_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PowerSetRequest(
            powerrequest.into_param().abi(),
            ::std::mem::transmute(requesttype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerSettingAccessCheck(
    accessflags: POWER_DATA_ACCESSOR,
    powerguid: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSettingAccessCheck(
                accessflags: POWER_DATA_ACCESSOR,
                powerguid: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(PowerSettingAccessCheck(
            ::std::mem::transmute(accessflags),
            ::std::mem::transmute(powerguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerSettingAccessCheckEx(
    accessflags: POWER_DATA_ACCESSOR,
    powerguid: *const ::windows::runtime::GUID,
    accesstype: super::Registry::REG_SAM_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSettingAccessCheckEx(
                accessflags: POWER_DATA_ACCESSOR,
                powerguid: *const ::windows::runtime::GUID,
                accesstype: super::Registry::REG_SAM_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(PowerSettingAccessCheckEx(
            ::std::mem::transmute(accessflags),
            ::std::mem::transmute(powerguid),
            ::std::mem::transmute(accesstype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PowerSettingRegisterNotification<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    settingguid: *const ::windows::runtime::GUID,
    flags: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS,
    recipient: Param2,
    registrationhandle: *mut *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSettingRegisterNotification(
                settingguid: *const ::windows::runtime::GUID,
                flags: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS,
                recipient: super::super::Foundation::HANDLE,
                registrationhandle: *mut *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(PowerSettingRegisterNotification(
            ::std::mem::transmute(settingguid),
            ::std::mem::transmute(flags),
            recipient.into_param().abi(),
            ::std::mem::transmute(registrationhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerSettingUnregisterNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HPOWERNOTIFY>,
>(
    registrationhandle: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSettingUnregisterNotification(registrationhandle: HPOWERNOTIFY) -> u32;
        }
        ::std::mem::transmute(PowerSettingUnregisterNotification(
            registrationhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn PowerUnregisterFromEffectivePowerModeNotifications(
    registrationhandle: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerUnregisterFromEffectivePowerModeNotifications(
                registrationhandle: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        PowerUnregisterFromEffectivePowerModeNotifications(::std::mem::transmute(
            registrationhandle,
        ))
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerUnregisterSuspendResumeNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HPOWERNOTIFY>,
>(
    registrationhandle: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerUnregisterSuspendResumeNotification(registrationhandle: HPOWERNOTIFY) -> u32;
        }
        ::std::mem::transmute(PowerUnregisterSuspendResumeNotification(
            registrationhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteACDefaultIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootsystempowerkey: Param0,
    schemepersonalityguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    defaultacindex: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteACDefaultIndex(
                rootsystempowerkey: super::Registry::HKEY,
                schemepersonalityguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                defaultacindex: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteACDefaultIndex(
            rootsystempowerkey.into_param().abi(),
            ::std::mem::transmute(schemepersonalityguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(defaultacindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteACValueIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    acvalueindex: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteACValueIndex(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                acvalueindex: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteACValueIndex(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(acvalueindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteDCDefaultIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootsystempowerkey: Param0,
    schemepersonalityguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    defaultdcindex: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteDCDefaultIndex(
                rootsystempowerkey: super::Registry::HKEY,
                schemepersonalityguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                defaultdcindex: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteDCDefaultIndex(
            rootsystempowerkey.into_param().abi(),
            ::std::mem::transmute(schemepersonalityguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(defaultdcindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteDCValueIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    dcvalueindex: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteDCValueIndex(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                dcvalueindex: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteDCValueIndex(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(dcvalueindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteDescription<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    buffer: *const u8,
    buffersize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteDescription(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                buffer: *const u8,
                buffersize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteDescription(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteFriendlyName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    buffer: *const u8,
    buffersize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteFriendlyName(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                buffer: *const u8,
                buffersize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteFriendlyName(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteIconResourceSpecifier<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    schemeguid: *const ::windows::runtime::GUID,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    buffer: *const u8,
    buffersize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteIconResourceSpecifier(
                rootpowerkey: super::Registry::HKEY,
                schemeguid: *const ::windows::runtime::GUID,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                buffer: *const u8,
                buffersize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteIconResourceSpecifier(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(schemeguid),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWritePossibleDescription<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    possiblesettingindex: u32,
    buffer: *const u8,
    buffersize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWritePossibleDescription(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                possiblesettingindex: u32,
                buffer: *const u8,
                buffersize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWritePossibleDescription(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(possiblesettingindex),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWritePossibleFriendlyName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    possiblesettingindex: u32,
    buffer: *const u8,
    buffersize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWritePossibleFriendlyName(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                possiblesettingindex: u32,
                buffer: *const u8,
                buffersize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWritePossibleFriendlyName(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(possiblesettingindex),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWritePossibleValue<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    r#type: u32,
    possiblesettingindex: u32,
    buffer: *const u8,
    buffersize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWritePossibleValue(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                r#type: u32,
                possiblesettingindex: u32,
                buffer: *const u8,
                buffersize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWritePossibleValue(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(possiblesettingindex),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerWriteSettingAttributes(
    subgroupguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    attributes: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteSettingAttributes(
                subgroupguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                attributes: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteSettingAttributes(
            ::std::mem::transmute(subgroupguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(attributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueIncrement<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    valueincrement: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteValueIncrement(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                valueincrement: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteValueIncrement(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(valueincrement),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueMax<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    valuemaximum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteValueMax(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                valuemaximum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteValueMax(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(valuemaximum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueMin<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    valueminimum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteValueMin(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                valueminimum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteValueMin(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(valueminimum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueUnitsSpecifier<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
>(
    rootpowerkey: Param0,
    subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
    powersettingguid: *const ::windows::runtime::GUID,
    buffer: *const u8,
    buffersize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteValueUnitsSpecifier(
                rootpowerkey: super::Registry::HKEY,
                subgroupofpowersettingsguid: *const ::windows::runtime::GUID,
                powersettingguid: *const ::windows::runtime::GUID,
                buffer: *const u8,
                buffersize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(PowerWriteValueUnitsSpecifier(
            rootpowerkey.into_param().abi(),
            ::std::mem::transmute(subgroupofpowersettingsguid),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadGlobalPwrPolicy(
    pglobalpowerpolicy: *const GLOBAL_POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadGlobalPwrPolicy(
                pglobalpowerpolicy: *const GLOBAL_POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(ReadGlobalPwrPolicy(::std::mem::transmute(
            pglobalpowerpolicy,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadProcessorPwrScheme(
    uiid: u32,
    pmachineprocessorpowerpolicy: *mut MACHINE_PROCESSOR_POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadProcessorPwrScheme(
                uiid: u32,
                pmachineprocessorpowerpolicy: *mut MACHINE_PROCESSOR_POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(ReadProcessorPwrScheme(
            ::std::mem::transmute(uiid),
            ::std::mem::transmute(pmachineprocessorpowerpolicy),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadPwrScheme(
    uiid: u32,
    ppowerpolicy: *mut POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadPwrScheme(
                uiid: u32,
                ppowerpolicy: *mut POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(ReadPwrScheme(
            ::std::mem::transmute(uiid),
            ::std::mem::transmute(ppowerpolicy),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterPowerSettingNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hrecipient: Param0,
    powersettingguid: *const ::windows::runtime::GUID,
    flags: u32,
) -> HPOWERNOTIFY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterPowerSettingNotification(
                hrecipient: super::super::Foundation::HANDLE,
                powersettingguid: *const ::windows::runtime::GUID,
                flags: u32,
            ) -> HPOWERNOTIFY;
        }
        ::std::mem::transmute(RegisterPowerSettingNotification(
            hrecipient.into_param().abi(),
            ::std::mem::transmute(powersettingguid),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterSuspendResumeNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hrecipient: Param0,
    flags: u32,
) -> HPOWERNOTIFY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterSuspendResumeNotification(
                hrecipient: super::super::Foundation::HANDLE,
                flags: u32,
            ) -> HPOWERNOTIFY;
        }
        ::std::mem::transmute(RegisterSuspendResumeNotification(
            hrecipient.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RequestWakeupLatency(latency: LATENCY_TIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RequestWakeupLatency(latency: LATENCY_TIME) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RequestWakeupLatency(::std::mem::transmute(latency)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_BATTERY_STATE {
    pub AcOnLine: super::super::Foundation::BOOLEAN,
    pub BatteryPresent: super::super::Foundation::BOOLEAN,
    pub Charging: super::super::Foundation::BOOLEAN,
    pub Discharging: super::super::Foundation::BOOLEAN,
    pub Spare1: [super::super::Foundation::BOOLEAN; 3],
    pub Tag: u8,
    pub MaxCapacity: u32,
    pub RemainingCapacity: u32,
    pub Rate: u32,
    pub EstimatedTime: u32,
    pub DefaultAlert1: u32,
    pub DefaultAlert2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SYSTEM_BATTERY_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SYSTEM_BATTERY_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SYSTEM_BATTERY_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_BATTERY_STATE")
            .field("AcOnLine", &self.AcOnLine)
            .field("BatteryPresent", &self.BatteryPresent)
            .field("Charging", &self.Charging)
            .field("Discharging", &self.Discharging)
            .field("Spare1", &self.Spare1)
            .field("Tag", &self.Tag)
            .field("MaxCapacity", &self.MaxCapacity)
            .field("RemainingCapacity", &self.RemainingCapacity)
            .field("Rate", &self.Rate)
            .field("EstimatedTime", &self.EstimatedTime)
            .field("DefaultAlert1", &self.DefaultAlert1)
            .field("DefaultAlert2", &self.DefaultAlert2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SYSTEM_BATTERY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.AcOnLine == other.AcOnLine
            && self.BatteryPresent == other.BatteryPresent
            && self.Charging == other.Charging
            && self.Discharging == other.Discharging
            && self.Spare1 == other.Spare1
            && self.Tag == other.Tag
            && self.MaxCapacity == other.MaxCapacity
            && self.RemainingCapacity == other.RemainingCapacity
            && self.Rate == other.Rate
            && self.EstimatedTime == other.EstimatedTime
            && self.DefaultAlert1 == other.DefaultAlert1
            && self.DefaultAlert2 == other.DefaultAlert2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SYSTEM_BATTERY_STATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SYSTEM_BATTERY_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_POWER_CAPABILITIES {
    pub PowerButtonPresent: super::super::Foundation::BOOLEAN,
    pub SleepButtonPresent: super::super::Foundation::BOOLEAN,
    pub LidPresent: super::super::Foundation::BOOLEAN,
    pub SystemS1: super::super::Foundation::BOOLEAN,
    pub SystemS2: super::super::Foundation::BOOLEAN,
    pub SystemS3: super::super::Foundation::BOOLEAN,
    pub SystemS4: super::super::Foundation::BOOLEAN,
    pub SystemS5: super::super::Foundation::BOOLEAN,
    pub HiberFilePresent: super::super::Foundation::BOOLEAN,
    pub FullWake: super::super::Foundation::BOOLEAN,
    pub VideoDimPresent: super::super::Foundation::BOOLEAN,
    pub ApmPresent: super::super::Foundation::BOOLEAN,
    pub UpsPresent: super::super::Foundation::BOOLEAN,
    pub ThermalControl: super::super::Foundation::BOOLEAN,
    pub ProcessorThrottle: super::super::Foundation::BOOLEAN,
    pub ProcessorMinThrottle: u8,
    pub ProcessorMaxThrottle: u8,
    pub FastSystemS4: super::super::Foundation::BOOLEAN,
    pub Hiberboot: super::super::Foundation::BOOLEAN,
    pub WakeAlarmPresent: super::super::Foundation::BOOLEAN,
    pub AoAc: super::super::Foundation::BOOLEAN,
    pub DiskSpinDown: super::super::Foundation::BOOLEAN,
    pub HiberFileType: u8,
    pub AoAcConnectivitySupported: super::super::Foundation::BOOLEAN,
    pub spare3: [u8; 6],
    pub SystemBatteriesPresent: super::super::Foundation::BOOLEAN,
    pub BatteriesAreShortTerm: super::super::Foundation::BOOLEAN,
    pub BatteryScale: [BATTERY_REPORTING_SCALE; 3],
    pub AcOnLineWake: SYSTEM_POWER_STATE,
    pub SoftLidWake: SYSTEM_POWER_STATE,
    pub RtcWake: SYSTEM_POWER_STATE,
    pub MinDeviceWakeState: SYSTEM_POWER_STATE,
    pub DefaultLowLatencyWake: SYSTEM_POWER_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl SYSTEM_POWER_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SYSTEM_POWER_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SYSTEM_POWER_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_POWER_CAPABILITIES")
            .field("PowerButtonPresent", &self.PowerButtonPresent)
            .field("SleepButtonPresent", &self.SleepButtonPresent)
            .field("LidPresent", &self.LidPresent)
            .field("SystemS1", &self.SystemS1)
            .field("SystemS2", &self.SystemS2)
            .field("SystemS3", &self.SystemS3)
            .field("SystemS4", &self.SystemS4)
            .field("SystemS5", &self.SystemS5)
            .field("HiberFilePresent", &self.HiberFilePresent)
            .field("FullWake", &self.FullWake)
            .field("VideoDimPresent", &self.VideoDimPresent)
            .field("ApmPresent", &self.ApmPresent)
            .field("UpsPresent", &self.UpsPresent)
            .field("ThermalControl", &self.ThermalControl)
            .field("ProcessorThrottle", &self.ProcessorThrottle)
            .field("ProcessorMinThrottle", &self.ProcessorMinThrottle)
            .field("ProcessorMaxThrottle", &self.ProcessorMaxThrottle)
            .field("FastSystemS4", &self.FastSystemS4)
            .field("Hiberboot", &self.Hiberboot)
            .field("WakeAlarmPresent", &self.WakeAlarmPresent)
            .field("AoAc", &self.AoAc)
            .field("DiskSpinDown", &self.DiskSpinDown)
            .field("HiberFileType", &self.HiberFileType)
            .field("AoAcConnectivitySupported", &self.AoAcConnectivitySupported)
            .field("spare3", &self.spare3)
            .field("SystemBatteriesPresent", &self.SystemBatteriesPresent)
            .field("BatteriesAreShortTerm", &self.BatteriesAreShortTerm)
            .field("BatteryScale", &self.BatteryScale)
            .field("AcOnLineWake", &self.AcOnLineWake)
            .field("SoftLidWake", &self.SoftLidWake)
            .field("RtcWake", &self.RtcWake)
            .field("MinDeviceWakeState", &self.MinDeviceWakeState)
            .field("DefaultLowLatencyWake", &self.DefaultLowLatencyWake)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SYSTEM_POWER_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.PowerButtonPresent == other.PowerButtonPresent
            && self.SleepButtonPresent == other.SleepButtonPresent
            && self.LidPresent == other.LidPresent
            && self.SystemS1 == other.SystemS1
            && self.SystemS2 == other.SystemS2
            && self.SystemS3 == other.SystemS3
            && self.SystemS4 == other.SystemS4
            && self.SystemS5 == other.SystemS5
            && self.HiberFilePresent == other.HiberFilePresent
            && self.FullWake == other.FullWake
            && self.VideoDimPresent == other.VideoDimPresent
            && self.ApmPresent == other.ApmPresent
            && self.UpsPresent == other.UpsPresent
            && self.ThermalControl == other.ThermalControl
            && self.ProcessorThrottle == other.ProcessorThrottle
            && self.ProcessorMinThrottle == other.ProcessorMinThrottle
            && self.ProcessorMaxThrottle == other.ProcessorMaxThrottle
            && self.FastSystemS4 == other.FastSystemS4
            && self.Hiberboot == other.Hiberboot
            && self.WakeAlarmPresent == other.WakeAlarmPresent
            && self.AoAc == other.AoAc
            && self.DiskSpinDown == other.DiskSpinDown
            && self.HiberFileType == other.HiberFileType
            && self.AoAcConnectivitySupported == other.AoAcConnectivitySupported
            && self.spare3 == other.spare3
            && self.SystemBatteriesPresent == other.SystemBatteriesPresent
            && self.BatteriesAreShortTerm == other.BatteriesAreShortTerm
            && self.BatteryScale == other.BatteryScale
            && self.AcOnLineWake == other.AcOnLineWake
            && self.SoftLidWake == other.SoftLidWake
            && self.RtcWake == other.RtcWake
            && self.MinDeviceWakeState == other.MinDeviceWakeState
            && self.DefaultLowLatencyWake == other.DefaultLowLatencyWake
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SYSTEM_POWER_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SYSTEM_POWER_CAPABILITIES {
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
pub struct SYSTEM_POWER_CONDITION(pub i32);
pub const PoAc: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(0i32);
pub const PoDc: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(1i32);
pub const PoHot: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(2i32);
pub const PoConditionMaximum: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(3i32);
impl ::std::convert::From<i32> for SYSTEM_POWER_CONDITION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYSTEM_POWER_CONDITION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_POWER_LEVEL {
    pub Enable: super::super::Foundation::BOOLEAN,
    pub Spare: [u8; 3],
    pub BatteryLevel: u32,
    pub PowerPolicy: POWER_ACTION_POLICY,
    pub MinSystemState: SYSTEM_POWER_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl SYSTEM_POWER_LEVEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SYSTEM_POWER_LEVEL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SYSTEM_POWER_LEVEL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_POWER_LEVEL")
            .field("Enable", &self.Enable)
            .field("Spare", &self.Spare)
            .field("BatteryLevel", &self.BatteryLevel)
            .field("PowerPolicy", &self.PowerPolicy)
            .field("MinSystemState", &self.MinSystemState)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SYSTEM_POWER_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable
            && self.Spare == other.Spare
            && self.BatteryLevel == other.BatteryLevel
            && self.PowerPolicy == other.PowerPolicy
            && self.MinSystemState == other.MinSystemState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SYSTEM_POWER_LEVEL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SYSTEM_POWER_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_POWER_POLICY {
    pub Revision: u32,
    pub PowerButton: POWER_ACTION_POLICY,
    pub SleepButton: POWER_ACTION_POLICY,
    pub LidClose: POWER_ACTION_POLICY,
    pub LidOpenWake: SYSTEM_POWER_STATE,
    pub Reserved: u32,
    pub Idle: POWER_ACTION_POLICY,
    pub IdleTimeout: u32,
    pub IdleSensitivity: u8,
    pub DynamicThrottle: u8,
    pub Spare2: [u8; 2],
    pub MinSleep: SYSTEM_POWER_STATE,
    pub MaxSleep: SYSTEM_POWER_STATE,
    pub ReducedLatencySleep: SYSTEM_POWER_STATE,
    pub WinLogonFlags: u32,
    pub Spare3: u32,
    pub DozeS4Timeout: u32,
    pub BroadcastCapacityResolution: u32,
    pub DischargePolicy: [SYSTEM_POWER_LEVEL; 4],
    pub VideoTimeout: u32,
    pub VideoDimDisplay: super::super::Foundation::BOOLEAN,
    pub VideoReserved: [u32; 3],
    pub SpindownTimeout: u32,
    pub OptimizeForPower: super::super::Foundation::BOOLEAN,
    pub FanThrottleTolerance: u8,
    pub ForcedThrottle: u8,
    pub MinThrottle: u8,
    pub OverThrottled: POWER_ACTION_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl SYSTEM_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SYSTEM_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SYSTEM_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_POWER_POLICY")
            .field("Revision", &self.Revision)
            .field("PowerButton", &self.PowerButton)
            .field("SleepButton", &self.SleepButton)
            .field("LidClose", &self.LidClose)
            .field("LidOpenWake", &self.LidOpenWake)
            .field("Reserved", &self.Reserved)
            .field("Idle", &self.Idle)
            .field("IdleTimeout", &self.IdleTimeout)
            .field("IdleSensitivity", &self.IdleSensitivity)
            .field("DynamicThrottle", &self.DynamicThrottle)
            .field("Spare2", &self.Spare2)
            .field("MinSleep", &self.MinSleep)
            .field("MaxSleep", &self.MaxSleep)
            .field("ReducedLatencySleep", &self.ReducedLatencySleep)
            .field("WinLogonFlags", &self.WinLogonFlags)
            .field("Spare3", &self.Spare3)
            .field("DozeS4Timeout", &self.DozeS4Timeout)
            .field(
                "BroadcastCapacityResolution",
                &self.BroadcastCapacityResolution,
            )
            .field("DischargePolicy", &self.DischargePolicy)
            .field("VideoTimeout", &self.VideoTimeout)
            .field("VideoDimDisplay", &self.VideoDimDisplay)
            .field("VideoReserved", &self.VideoReserved)
            .field("SpindownTimeout", &self.SpindownTimeout)
            .field("OptimizeForPower", &self.OptimizeForPower)
            .field("FanThrottleTolerance", &self.FanThrottleTolerance)
            .field("ForcedThrottle", &self.ForcedThrottle)
            .field("MinThrottle", &self.MinThrottle)
            .field("OverThrottled", &self.OverThrottled)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SYSTEM_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.PowerButton == other.PowerButton
            && self.SleepButton == other.SleepButton
            && self.LidClose == other.LidClose
            && self.LidOpenWake == other.LidOpenWake
            && self.Reserved == other.Reserved
            && self.Idle == other.Idle
            && self.IdleTimeout == other.IdleTimeout
            && self.IdleSensitivity == other.IdleSensitivity
            && self.DynamicThrottle == other.DynamicThrottle
            && self.Spare2 == other.Spare2
            && self.MinSleep == other.MinSleep
            && self.MaxSleep == other.MaxSleep
            && self.ReducedLatencySleep == other.ReducedLatencySleep
            && self.WinLogonFlags == other.WinLogonFlags
            && self.Spare3 == other.Spare3
            && self.DozeS4Timeout == other.DozeS4Timeout
            && self.BroadcastCapacityResolution == other.BroadcastCapacityResolution
            && self.DischargePolicy == other.DischargePolicy
            && self.VideoTimeout == other.VideoTimeout
            && self.VideoDimDisplay == other.VideoDimDisplay
            && self.VideoReserved == other.VideoReserved
            && self.SpindownTimeout == other.SpindownTimeout
            && self.OptimizeForPower == other.OptimizeForPower
            && self.FanThrottleTolerance == other.FanThrottleTolerance
            && self.ForcedThrottle == other.ForcedThrottle
            && self.MinThrottle == other.MinThrottle
            && self.OverThrottled == other.OverThrottled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SYSTEM_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SYSTEM_POWER_POLICY {
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
pub struct SYSTEM_POWER_STATE(pub i32);
pub const PowerSystemUnspecified: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(0i32);
pub const PowerSystemWorking: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(1i32);
pub const PowerSystemSleeping1: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(2i32);
pub const PowerSystemSleeping2: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(3i32);
pub const PowerSystemSleeping3: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(4i32);
pub const PowerSystemHibernate: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(5i32);
pub const PowerSystemShutdown: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(6i32);
pub const PowerSystemMaximum: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(7i32);
impl ::std::convert::From<i32> for SYSTEM_POWER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYSTEM_POWER_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_POWER_STATUS {
    pub ACLineStatus: u8,
    pub BatteryFlag: u8,
    pub BatteryLifePercent: u8,
    pub SystemStatusFlag: u8,
    pub BatteryLifeTime: u32,
    pub BatteryFullLifeTime: u32,
}
impl SYSTEM_POWER_STATUS {}
impl ::std::default::Default for SYSTEM_POWER_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_POWER_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_POWER_STATUS")
            .field("ACLineStatus", &self.ACLineStatus)
            .field("BatteryFlag", &self.BatteryFlag)
            .field("BatteryLifePercent", &self.BatteryLifePercent)
            .field("SystemStatusFlag", &self.SystemStatusFlag)
            .field("BatteryLifeTime", &self.BatteryLifeTime)
            .field("BatteryFullLifeTime", &self.BatteryFullLifeTime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_POWER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ACLineStatus == other.ACLineStatus
            && self.BatteryFlag == other.BatteryFlag
            && self.BatteryLifePercent == other.BatteryLifePercent
            && self.SystemStatusFlag == other.SystemStatusFlag
            && self.BatteryLifeTime == other.BatteryLifeTime
            && self.BatteryFullLifeTime == other.BatteryFullLifeTime
    }
}
impl ::std::cmp::Eq for SYSTEM_POWER_STATUS {}
unsafe impl ::windows::runtime::Abi for SYSTEM_POWER_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SYS_BUTTON_LID: u32 = 4u32;
pub const SYS_BUTTON_LID_CHANGED: u32 = 524288u32;
pub const SYS_BUTTON_LID_CLOSED: u32 = 131072u32;
pub const SYS_BUTTON_LID_INITIAL: u32 = 262144u32;
pub const SYS_BUTTON_LID_OPEN: u32 = 65536u32;
pub const SYS_BUTTON_LID_STATE_MASK: u32 = 196608u32;
pub const SYS_BUTTON_POWER: u32 = 1u32;
pub const SYS_BUTTON_SLEEP: u32 = 2u32;
pub const SYS_BUTTON_WAKE: u32 = 2147483648u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetActivePwrScheme(
    uiid: u32,
    pglobalpowerpolicy: *const GLOBAL_POWER_POLICY,
    ppowerpolicy: *const POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetActivePwrScheme(
                uiid: u32,
                pglobalpowerpolicy: *const GLOBAL_POWER_POLICY,
                ppowerpolicy: *const POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(SetActivePwrScheme(
            ::std::mem::transmute(uiid),
            ::std::mem::transmute(pglobalpowerpolicy),
            ::std::mem::transmute(ppowerpolicy),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSuspendState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    bhibernate: Param0,
    bforce: Param1,
    bwakeupeventsdisabled: Param2,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSuspendState(
                bhibernate: super::super::Foundation::BOOLEAN,
                bforce: super::super::Foundation::BOOLEAN,
                bwakeupeventsdisabled: super::super::Foundation::BOOLEAN,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(SetSuspendState(
            bhibernate.into_param().abi(),
            bforce.into_param().abi(),
            bwakeupeventsdisabled.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemPowerState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    fsuspend: Param0,
    fforce: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemPowerState(
                fsuspend: super::super::Foundation::BOOL,
                fforce: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSystemPowerState(
            fsuspend.into_param().abi(),
            fforce.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetThreadExecutionState(esflags: EXECUTION_STATE) -> EXECUTION_STATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadExecutionState(esflags: EXECUTION_STATE) -> EXECUTION_STATE;
        }
        ::std::mem::transmute(SetThreadExecutionState(::std::mem::transmute(esflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const THERMAL_COOLING_INTERFACE_VERSION: u32 = 1u32;
pub const THERMAL_DEVICE_INTERFACE_VERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct THERMAL_EVENT {
    pub Version: u32,
    pub Size: u32,
    pub Type: u32,
    pub Temperature: u32,
    pub TripPointTemperature: u32,
    pub Initiator: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl THERMAL_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for THERMAL_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for THERMAL_EVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("THERMAL_EVENT")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Type", &self.Type)
            .field("Temperature", &self.Temperature)
            .field("TripPointTemperature", &self.TripPointTemperature)
            .field("Initiator", &self.Initiator)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for THERMAL_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.Type == other.Type
            && self.Temperature == other.Temperature
            && self.TripPointTemperature == other.TripPointTemperature
            && self.Initiator == other.Initiator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for THERMAL_EVENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for THERMAL_EVENT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const THERMAL_EVENT_VERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct THERMAL_INFORMATION {
    pub ThermalStamp: u32,
    pub ThermalConstant1: u32,
    pub ThermalConstant2: u32,
    pub Processors: usize,
    pub SamplingPeriod: u32,
    pub CurrentTemperature: u32,
    pub PassiveTripPoint: u32,
    pub CriticalTripPoint: u32,
    pub ActiveTripPointCount: u8,
    pub ActiveTripPoint: [u32; 10],
}
impl THERMAL_INFORMATION {}
impl ::std::default::Default for THERMAL_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for THERMAL_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("THERMAL_INFORMATION")
            .field("ThermalStamp", &self.ThermalStamp)
            .field("ThermalConstant1", &self.ThermalConstant1)
            .field("ThermalConstant2", &self.ThermalConstant2)
            .field("Processors", &self.Processors)
            .field("SamplingPeriod", &self.SamplingPeriod)
            .field("CurrentTemperature", &self.CurrentTemperature)
            .field("PassiveTripPoint", &self.PassiveTripPoint)
            .field("CriticalTripPoint", &self.CriticalTripPoint)
            .field("ActiveTripPointCount", &self.ActiveTripPointCount)
            .field("ActiveTripPoint", &self.ActiveTripPoint)
            .finish()
    }
}
impl ::std::cmp::PartialEq for THERMAL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ThermalStamp == other.ThermalStamp
            && self.ThermalConstant1 == other.ThermalConstant1
            && self.ThermalConstant2 == other.ThermalConstant2
            && self.Processors == other.Processors
            && self.SamplingPeriod == other.SamplingPeriod
            && self.CurrentTemperature == other.CurrentTemperature
            && self.PassiveTripPoint == other.PassiveTripPoint
            && self.CriticalTripPoint == other.CriticalTripPoint
            && self.ActiveTripPointCount == other.ActiveTripPointCount
            && self.ActiveTripPoint == other.ActiveTripPoint
    }
}
impl ::std::cmp::Eq for THERMAL_INFORMATION {}
unsafe impl ::windows::runtime::Abi for THERMAL_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct THERMAL_POLICY {
    pub Version: u32,
    pub WaitForUpdate: super::super::Foundation::BOOLEAN,
    pub Hibernate: super::super::Foundation::BOOLEAN,
    pub Critical: super::super::Foundation::BOOLEAN,
    pub ThermalStandby: super::super::Foundation::BOOLEAN,
    pub ActivationReasons: u32,
    pub PassiveLimit: u32,
    pub ActiveLevel: u32,
    pub OverThrottled: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl THERMAL_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for THERMAL_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for THERMAL_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("THERMAL_POLICY")
            .field("Version", &self.Version)
            .field("WaitForUpdate", &self.WaitForUpdate)
            .field("Hibernate", &self.Hibernate)
            .field("Critical", &self.Critical)
            .field("ThermalStandby", &self.ThermalStandby)
            .field("ActivationReasons", &self.ActivationReasons)
            .field("PassiveLimit", &self.PassiveLimit)
            .field("ActiveLevel", &self.ActiveLevel)
            .field("OverThrottled", &self.OverThrottled)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for THERMAL_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.WaitForUpdate == other.WaitForUpdate
            && self.Hibernate == other.Hibernate
            && self.Critical == other.Critical
            && self.ThermalStandby == other.ThermalStandby
            && self.ActivationReasons == other.ActivationReasons
            && self.PassiveLimit == other.PassiveLimit
            && self.ActiveLevel == other.ActiveLevel
            && self.OverThrottled == other.OverThrottled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for THERMAL_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for THERMAL_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const THERMAL_POLICY_VERSION_1: u32 = 1u32;
pub const THERMAL_POLICY_VERSION_2: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct THERMAL_WAIT_READ {
    pub Timeout: u32,
    pub LowTemperature: u32,
    pub HighTemperature: u32,
}
impl THERMAL_WAIT_READ {}
impl ::std::default::Default for THERMAL_WAIT_READ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for THERMAL_WAIT_READ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("THERMAL_WAIT_READ")
            .field("Timeout", &self.Timeout)
            .field("LowTemperature", &self.LowTemperature)
            .field("HighTemperature", &self.HighTemperature)
            .finish()
    }
}
impl ::std::cmp::PartialEq for THERMAL_WAIT_READ {
    fn eq(&self, other: &Self) -> bool {
        self.Timeout == other.Timeout
            && self.LowTemperature == other.LowTemperature
            && self.HighTemperature == other.HighTemperature
    }
}
impl ::std::cmp::Eq for THERMAL_WAIT_READ {}
unsafe impl ::windows::runtime::Abi for THERMAL_WAIT_READ {
    type Abi = Self;
    type DefaultType = Self;
}
pub const TZ_ACTIVATION_REASON_CURRENT: u32 = 2u32;
pub const TZ_ACTIVATION_REASON_THERMAL: u32 = 1u32;
pub const UNKNOWN_CAPACITY: u32 = 4294967295u32;
pub const UNKNOWN_CURRENT: u32 = 4294967295u32;
pub const UNKNOWN_RATE: u32 = 2147483648u32;
pub const UNKNOWN_VOLTAGE: u32 = 4294967295u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct USB_CHARGER_PORT(pub i32);
pub const UsbChargerPort_Legacy: USB_CHARGER_PORT = USB_CHARGER_PORT(0i32);
pub const UsbChargerPort_TypeC: USB_CHARGER_PORT = USB_CHARGER_PORT(1i32);
pub const UsbChargerPort_Max: USB_CHARGER_PORT = USB_CHARGER_PORT(2i32);
impl ::std::convert::From<i32> for USB_CHARGER_PORT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for USB_CHARGER_PORT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_POWER_POLICY {
    pub Revision: u32,
    pub IdleAc: POWER_ACTION_POLICY,
    pub IdleDc: POWER_ACTION_POLICY,
    pub IdleTimeoutAc: u32,
    pub IdleTimeoutDc: u32,
    pub IdleSensitivityAc: u8,
    pub IdleSensitivityDc: u8,
    pub ThrottlePolicyAc: u8,
    pub ThrottlePolicyDc: u8,
    pub MaxSleepAc: SYSTEM_POWER_STATE,
    pub MaxSleepDc: SYSTEM_POWER_STATE,
    pub Reserved: [u32; 2],
    pub VideoTimeoutAc: u32,
    pub VideoTimeoutDc: u32,
    pub SpindownTimeoutAc: u32,
    pub SpindownTimeoutDc: u32,
    pub OptimizeForPowerAc: super::super::Foundation::BOOLEAN,
    pub OptimizeForPowerDc: super::super::Foundation::BOOLEAN,
    pub FanThrottleToleranceAc: u8,
    pub FanThrottleToleranceDc: u8,
    pub ForcedThrottleAc: u8,
    pub ForcedThrottleDc: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl USER_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for USER_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("USER_POWER_POLICY")
            .field("Revision", &self.Revision)
            .field("IdleAc", &self.IdleAc)
            .field("IdleDc", &self.IdleDc)
            .field("IdleTimeoutAc", &self.IdleTimeoutAc)
            .field("IdleTimeoutDc", &self.IdleTimeoutDc)
            .field("IdleSensitivityAc", &self.IdleSensitivityAc)
            .field("IdleSensitivityDc", &self.IdleSensitivityDc)
            .field("ThrottlePolicyAc", &self.ThrottlePolicyAc)
            .field("ThrottlePolicyDc", &self.ThrottlePolicyDc)
            .field("MaxSleepAc", &self.MaxSleepAc)
            .field("MaxSleepDc", &self.MaxSleepDc)
            .field("Reserved", &self.Reserved)
            .field("VideoTimeoutAc", &self.VideoTimeoutAc)
            .field("VideoTimeoutDc", &self.VideoTimeoutDc)
            .field("SpindownTimeoutAc", &self.SpindownTimeoutAc)
            .field("SpindownTimeoutDc", &self.SpindownTimeoutDc)
            .field("OptimizeForPowerAc", &self.OptimizeForPowerAc)
            .field("OptimizeForPowerDc", &self.OptimizeForPowerDc)
            .field("FanThrottleToleranceAc", &self.FanThrottleToleranceAc)
            .field("FanThrottleToleranceDc", &self.FanThrottleToleranceDc)
            .field("ForcedThrottleAc", &self.ForcedThrottleAc)
            .field("ForcedThrottleDc", &self.ForcedThrottleDc)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for USER_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.IdleAc == other.IdleAc
            && self.IdleDc == other.IdleDc
            && self.IdleTimeoutAc == other.IdleTimeoutAc
            && self.IdleTimeoutDc == other.IdleTimeoutDc
            && self.IdleSensitivityAc == other.IdleSensitivityAc
            && self.IdleSensitivityDc == other.IdleSensitivityDc
            && self.ThrottlePolicyAc == other.ThrottlePolicyAc
            && self.ThrottlePolicyDc == other.ThrottlePolicyDc
            && self.MaxSleepAc == other.MaxSleepAc
            && self.MaxSleepDc == other.MaxSleepDc
            && self.Reserved == other.Reserved
            && self.VideoTimeoutAc == other.VideoTimeoutAc
            && self.VideoTimeoutDc == other.VideoTimeoutDc
            && self.SpindownTimeoutAc == other.SpindownTimeoutAc
            && self.SpindownTimeoutDc == other.SpindownTimeoutDc
            && self.OptimizeForPowerAc == other.OptimizeForPowerAc
            && self.OptimizeForPowerDc == other.OptimizeForPowerDc
            && self.FanThrottleToleranceAc == other.FanThrottleToleranceAc
            && self.FanThrottleToleranceDc == other.FanThrottleToleranceDc
            && self.ForcedThrottleAc == other.ForcedThrottleAc
            && self.ForcedThrottleDc == other.ForcedThrottleDc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for USER_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for USER_POWER_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterPowerSettingNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HPOWERNOTIFY>,
>(
    handle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterPowerSettingNotification(
                handle: HPOWERNOTIFY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterPowerSettingNotification(
            handle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterSuspendResumeNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HPOWERNOTIFY>,
>(
    handle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterSuspendResumeNotification(
                handle: HPOWERNOTIFY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterSuspendResumeNotification(
            handle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ValidatePowerPolicies(
    pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY,
    ppowerpolicy: *mut POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ValidatePowerPolicies(
                pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY,
                ppowerpolicy: *mut POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(ValidatePowerPolicies(
            ::std::mem::transmute(pglobalpowerpolicy),
            ::std::mem::transmute(ppowerpolicy),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WAKE_ALARM_INFORMATION {
    pub TimerIdentifier: u32,
    pub Timeout: u32,
}
impl WAKE_ALARM_INFORMATION {}
impl ::std::default::Default for WAKE_ALARM_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WAKE_ALARM_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WAKE_ALARM_INFORMATION")
            .field("TimerIdentifier", &self.TimerIdentifier)
            .field("Timeout", &self.Timeout)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WAKE_ALARM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TimerIdentifier == other.TimerIdentifier && self.Timeout == other.Timeout
    }
}
impl ::std::cmp::Eq for WAKE_ALARM_INFORMATION {}
unsafe impl ::windows::runtime::Abi for WAKE_ALARM_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteGlobalPwrPolicy(
    pglobalpowerpolicy: *const GLOBAL_POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteGlobalPwrPolicy(
                pglobalpowerpolicy: *const GLOBAL_POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(WriteGlobalPwrPolicy(::std::mem::transmute(
            pglobalpowerpolicy,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProcessorPwrScheme(
    uiid: u32,
    pmachineprocessorpowerpolicy: *const MACHINE_PROCESSOR_POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProcessorPwrScheme(
                uiid: u32,
                pmachineprocessorpowerpolicy: *const MACHINE_PROCESSOR_POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(WriteProcessorPwrScheme(
            ::std::mem::transmute(uiid),
            ::std::mem::transmute(pmachineprocessorpowerpolicy),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePwrScheme<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    puiid: *const u32,
    lpszschemename: Param1,
    lpszdescription: Param2,
    lpscheme: *const POWER_POLICY,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePwrScheme(
                puiid: *const u32,
                lpszschemename: super::super::Foundation::PWSTR,
                lpszdescription: super::super::Foundation::PWSTR,
                lpscheme: *const POWER_POLICY,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(WritePwrScheme(
            ::std::mem::transmute(puiid),
            lpszschemename.into_param().abi(),
            lpszdescription.into_param().abi(),
            ::std::mem::transmute(lpscheme),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
